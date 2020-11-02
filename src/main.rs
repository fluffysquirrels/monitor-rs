#[macro_use]
extern crate log;

mod log_store;
mod metric_store;
mod notifier;
mod scheduler;
mod signal;

// pub use to fix compiler dead_code warnings.
pub use crate::{
    log_store::{Log, LogStore},
    metric_store::MetricStore,
    notifier::Notifier,
    scheduler::Scheduler,
    signal::Signal,
};
use gio::prelude::*;
use glib;
use gtk::prelude::*;
use process_control::{ChildExt, Timeout};
use rt_graph;
use std::{
    cell::Cell,
    collections::BTreeMap,
    env::args,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OkErr {
    Ok,
    Err,
}

#[derive(Clone, Debug)]
pub enum MetricValue {
    OkErr(OkErr),
    I64(i64),
    _F64(f64),
}

#[derive(Clone, Debug)]
pub struct DataPoint {
    time: chrono::DateTime<chrono::Utc>,
    val: MetricValue,
}

#[derive(Clone)]
struct Ui {
    metrics: BTreeMap<String, MetricUi>,
}

#[derive(Clone)]
struct MetricUi {
    label_status: gtk::Label,
    graph: Rc<rt_graph::GraphWithControls>,
    showing_graph: Rc<Cell<bool>>,
    show_graph_btn: gtk::Button,
}

struct ShellCheckConfig {
    name: String,
    cmd: String,
    interval: chrono::Duration,
}

struct ShellMetricConfig {
    name: String,
    cmd: String,
    interval: chrono::Duration,
    check: MetricCheck,
}

#[derive(Clone, Debug)]
enum MetricCheck {
    #[allow(dead_code)] // Not used yet.
    None,
    Min(i64),
    #[allow(dead_code)] // Not used yet.
    Max(i64),
}

impl MetricCheck {
    fn is_value_ok(&self, value: i64) -> OkErr {
        match self {
            MetricCheck::None => OkErr::Ok,
            MetricCheck::Min(min) if value >= *min => OkErr::Ok,
            MetricCheck::Min(min) if value <  *min => OkErr::Err,
            MetricCheck::Max(max) if value <= *max => OkErr::Ok,
            MetricCheck::Max(max) if value >  *max => OkErr::Err,
            _ => panic!("Not reached"),
        }
    }
}

fn shell_check_configs() -> Vec<ShellCheckConfig> {
    vec![
        ShellCheckConfig {
            name: "ping.mf".to_owned(),
            cmd: "ping -c 1 -W 5 mf".to_owned(),
            interval: chrono::Duration::seconds(5),
        },
        ShellCheckConfig {
            name: "apt.upgradable".to_owned(),
            cmd: "/home/alex/Code/rust/monitor/scripts/apt-upgradable.py".to_owned(),
            interval: chrono::Duration::minutes(10),
        },
        ShellCheckConfig {
            name: "mf.apt.upgradable".to_owned(),
            cmd: "ssh mf /home/alex/Code/apt-upgradable.py".to_owned(),
            interval: chrono::Duration::minutes(10),
        },
        ShellCheckConfig {
            name: "internet.up.gstatic".to_owned(),
            cmd: "curl http://connectivitycheck.gstatic.com/generate_204 -v -f -s".to_owned(),
            interval: chrono::Duration::minutes(2),
        },
        ShellCheckConfig {
            name: "zfs.mf.healthy".to_owned(),
            cmd: "ssh mf /sbin/zpool status -x | grep 'all pools are healthy'".to_owned(),
            interval: chrono::Duration::minutes(2),
        },
        check_travis("github", "fluffysquirrels/mqtt-async-client-rs", "master"),
        check_travis("github", "fluffysquirrels/webdriver_client_rust", "master"),
        check_travis("github", "fluffysquirrels/framed-rs", "master"),
    ]
}

fn shell_metric_configs() -> Vec<ShellMetricConfig> {
    vec![
        ShellMetricConfig {
            cmd: "df -h / | awk '{print $5}' | egrep -o '[0-9]+'".to_owned(),
            name: "df.local.root".to_owned(),
            interval: chrono::Duration::minutes(5),
            check: MetricCheck::Min(20),
        },
    ]
}

fn check_travis(provider: &str, repo: &str, branch: &str) -> ShellCheckConfig {
    ShellCheckConfig {
        name: format!("travis.{}.{}.{}.passed", provider, repo, branch),
        cmd: format!(
"curl -f -s -H 'Travis-API-Version: 3' 'https://api.travis-ci.com/repo/{}/{}/branch/{}' |
tee /dev/stderr |
jq '.last_build.state' |
egrep '^\"passed\"$'",
            provider,
            repo.replace('/', "%2F"),
            branch),
        interval: chrono::Duration::minutes(30),
    }
}

fn main() {
    env_logger::init();

    let ms = Arc::new(Mutex::new(MetricStore::new()));
    let n = Arc::new(Mutex::new(Notifier::new()));
    let sched = Arc::new(Mutex::new(Scheduler::new()));
    let ls = Arc::new(Mutex::new(LogStore::new()));

    let check_configs = shell_check_configs();
    for scc in check_configs.iter() {
        add_shell_check_job(scc,
                            ms.clone(),
                            sched.clone(),
                            ls.clone());
    }

    let metric_configs = shell_metric_configs();
    for smc in metric_configs.iter() {
        add_shell_metric_job(smc,
                             ms.clone(),
                             sched.clone(),
                             ls.clone());
        match &smc.check {
            MetricCheck::None => (),
            _ => {
                let nc = n.clone();
                let check = smc.check.clone();
                ms.lock().unwrap()
                  .update_signal_for_one(&smc.name)
                  .connect(move |m| {
                      let val: i64 = m.latest().unwrap()
                                      .val.as_i64().expect("Only int checks so far");
                      let ok = check.is_value_ok(val);
                      debug!("Metric check name=`{}' value={} check={:?} ok={:?}",
                             m.name(), val, check, ok);
                      nc.lock().unwrap().update_metric(m.name(), ok);
                  });
            },
        };
    }

    sched.lock().unwrap().spawn();

    // Listen to metrics and connect the Notifier.
    let nc = n.clone();
    ms.lock().unwrap()
        .update_signal_for_all()
        .connect(move |m|
                 {
                     if let Some(DataPoint { val: MetricValue::OkErr(ok),.. }) = m.latest() {
                         nc.lock().unwrap().update_metric(m.name(), *ok);
                     }
                 });

    let application =
        gtk::Application::new(Some("com.github.fluffysquirrels.monitor"),
                              gio::ApplicationFlags::default())
            .expect("Application::new failed");

    let msc = ms.clone();
    let sc = sched.clone();
    let lsc = ls.clone();
    application.connect_activate(move |app| {
        let ms = msc.clone();
        let sc = sc.clone();
        let ls = lsc.clone();
        build_ui(&check_configs, &metric_configs, app, ms, sc, ls);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn build_ui(
    check_configs: &[ShellCheckConfig],
    metric_configs: &[ShellMetricConfig],
    application: &gtk::Application,
    ms: Arc<Mutex<MetricStore>>,
    sched: Arc<Mutex<Scheduler>>,
    ls: Arc<Mutex<LogStore>>,
) {
    let window = gtk::ApplicationWindowBuilder::new()
        .application(application)
        .title("monitor")
        .border_width(8)
        .window_position(gtk::WindowPosition::Center)
        .build();

    // Load icon relative to Cargo provided package root or if that's
    // unavailable, the current directory.
    let mut icon_path = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or(".".to_string());
    icon_path.push_str("/third_party/gnome-icon-theme/utilities-system-monitor.png");
    match window.set_icon_from_file(icon_path) {
        Ok(()) => (),
        Err(e) => error!("Unable to load icon, error: {}", e),
    };

    // Show the (gtk) window so we can get a gdk::window below.
    window.show();
    let gdk_window = window.get_window().unwrap();

    let metrics_box = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .spacing(8)
        .parent(&window)
        .build();

    let mut metrics = BTreeMap::<String,MetricUi>::new();

    for config in check_configs.iter() {
        let metric_ui = ui_for_metric(&ms, &sched, &ls, &metrics_box, &gdk_window, &config.name);
        metrics.insert(String::from(&config.name), metric_ui);
    }

    for config in metric_configs.iter() {
        let metric_ui = ui_for_metric(&ms, &sched, &ls, &metrics_box, &gdk_window, &config.name);
        metrics.insert(String::from(&config.name), metric_ui);
    }

    window.show_all();
    for mui in metrics.values() {
        mui.graph.hide();
    }

    let ui = Ui {
        metrics,
    };

    let ui_thread = glib::MainContext::ref_thread_default();

    {
        // Subscribe to log messages and send them over a channel to the UI thread,
        // which can then update the UI with logs.

        let (tx, rx) = glib::MainContext::sync_channel(glib::source::Priority::default(), 50);

        ls.lock().unwrap().update_signal().connect(move |log| {
            match tx.send(log) {
                Err(_) => error!("LogStore UI channel send error"),
                Ok(_) => (),
            }
        });

        let uic = ui.clone();
        rx.attach(Some(&ui_thread), move |log| {
            let metric = uic.metrics.get(&log.name);
            if let Some(metric) = metric {
                metric.label_status.set_tooltip_text(Some(&log.log));
            }
            glib::source::Continue(true)
        });
    }

    {
        // Subscribe to metric updates and send them over a channel to the UI thread,
        // which can then update the UI with metric status.

        let (tx, rx) = glib::MainContext::sync_channel(glib::source::Priority::default(), 50);

        ms.lock().unwrap().update_signal_for_all().connect(move |metric| {
            match tx.send(metric) {
                Err(_) => error!("MetricStore UI channel send error"),
                Ok(_) => (),
            }
        });

        let uic = ui.clone();
        rx.attach(Some(&ui_thread), move |metric| {
            if let Some(DataPoint { val, .. }) = metric.latest() {
                let ui_metric = uic.metrics.get(metric.name());
                if let Some(ui_metric) = ui_metric {
                    if let MetricValue::OkErr(ok) = val {
                        ui_metric.label_status.set_markup(match ok {
                            OkErr::Ok  => " = <span fgcolor='#00cc00'>Ok</span>",
                            OkErr::Err => " = <span fgcolor='#cc0000'>Err</span>",
                        });
                    } else {
                        ui_metric.label_status.set_text(&format!(" = {}", val));
                    }
                }
            }
            glib::source::Continue(true)
        });
    }
}

fn ui_for_metric<C>(
    ms: &Arc<Mutex<MetricStore>>,
    sched: &Arc<Mutex<Scheduler>>,
    _ls: &Arc<Mutex<LogStore>>,
    container: &C,
    gdk_window: &gdk::Window,
    metric_name: &str
) -> MetricUi
    where C: IsA<gtk::Container> + IsA<gtk::Widget>
{
    let label_box = gtk::BoxBuilder::new()
        .parent(container)
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let _label = gtk::LabelBuilder::new()
        .label(metric_name)
        .parent(&label_box)
        .build();
    let label_status = gtk::LabelBuilder::new()
        .parent(&label_box)
        .build();
    label_status.set_markup(" = <span fgcolor='#cccc00'>?</span>");

    let buttons_box = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Horizontal)
        .parent(container)
        .spacing(8)
        .build();

    let force_btn = gtk::ButtonBuilder::new()
        .label("Force")
        .parent(&buttons_box)
        .halign(gtk::Align::Start)
        .build();
    let sched = sched.clone();
    let metric_name_clone = metric_name.to_owned();
    force_btn.connect_clicked(move |_btn| {
        sched.lock().unwrap().force_run(&metric_name_clone);
    });

    let show_graph_btn = gtk::ButtonBuilder::new()
        .label("Show graph")
        .parent(&buttons_box)
        .halign(gtk::Align::Start)
        .build();

    let config = rt_graph::ConfigBuilder::default()
        .data_source(MetricStoreDataSource::new(metric_name, ms.clone()))
        .base_zoom_x(1.0)
        .max_zoom_x(0.1)
        .graph_height(100)
        .windows_to_store(2)
        .point_style(rt_graph::PointStyle::Cross)
        .build()
        .unwrap();
    let graph = rt_graph::GraphWithControls::build_ui(config, container, &gdk_window);

    let metric_ui = MetricUi {
        label_status,
        graph: Rc::new(graph),
        showing_graph: Rc::new(Cell::new(false)),
        show_graph_btn,
    };

    let muc = metric_ui.clone();
    metric_ui.show_graph_btn.connect_clicked(move |_btn| {
        match muc.showing_graph.get() {
            true => {
                muc.showing_graph.set(false);
                muc.show_graph_btn.set_label("Show graph");
                muc.graph.hide();
            },
            false => {
                muc.showing_graph.set(true);
                muc.show_graph_btn.set_label("Hide graph");
                muc.graph.show();
            },

        }
    });

    metric_ui
}

struct MetricStoreDataSource {
    metric_name: String,
    ms: Arc<Mutex<MetricStore>>,
    last: Option<DataPoint>,
    t: u32,
}

impl std::fmt::Debug for MetricStoreDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.debug_struct("MetricStoreDataSource")
            .field("metric_name", &self.metric_name)
            .finish()
    }
}

impl MetricStoreDataSource {
    fn new(metric_name: &str, ms: Arc<Mutex<MetricStore>>) -> MetricStoreDataSource {
        MetricStoreDataSource {
            metric_name: String::from(metric_name),
            ms,
            t: 1,
            last: None,
        }
    }
}

impl rt_graph::DataSource for MetricStoreDataSource {
    fn get_data(&mut self) -> Result<Vec<rt_graph::Point>, rt_graph::Error> {
        let m = self.ms.lock().unwrap().query_one(&self.metric_name);
        let res = match m {
            Some(m) => {
                let dp = m.latest();
                match dp {
                    Some(DataPoint { val, time }) => {
                        if self.last.is_none() ||
                            (self.last.is_some() && *time != self.last.as_ref().unwrap().time) {
                            self.t += 1;
                            self.last = Some(dp.unwrap().clone());
                            vec![rt_graph::Point {
                                t: self.t, // TODO: Use time.
                                vs: vec![match val {
                                    MetricValue::OkErr(OkErr::Ok)  => 50000,
                                    MetricValue::OkErr(OkErr::Err) => 10000,
                                    MetricValue::I64(i) => *i as u16, // TODO: Handle overflow
                                    MetricValue::_F64(_f) => unimplemented!(),
                                },
                            ]}]
                        } else {
                            vec![]
                        }
                    },
                    _ => vec![],
                }
            }
            None => vec![],
        };
        Ok(res)
    }

    fn get_num_values(&self) -> Result<usize, rt_graph::Error> {
        Ok(1)
    }

}

// TODO: Ugly duplication between this and add_shell_metric_job.
fn add_shell_check_job(
    config: &ShellCheckConfig,
    ms: Arc<Mutex<MetricStore>>,
    sched: Arc<Mutex<Scheduler>>,
    ls: Arc<Mutex<LogStore>>,
) {
    let cmd = config.cmd.to_owned();
    let name = config.name.to_owned();
    let j = scheduler::JobDefinition {
        interval: config.interval,
        name: String::from(&name),
        f: Arc::new(Mutex::new(move |_rc| {
            let mut command = std::process::Command::new("sh");
            command.arg("-c");
            command.arg(&cmd);

            // Ugly: calculates UTC time twice, once in run_shell and once here.
            let start = chrono::Utc::now();
            let res = run_shell(command);
            let finish = chrono::Utc::now();

            match res {
                Ok(res) => {
                    debug!("run_shell cmd=`{}' log=\n{}", &cmd, res.log);
                    ms.lock().unwrap().update(&name, DataPoint {
                        time: chrono::Utc::now(),
                        val: MetricValue::OkErr(res.ok)
                    });
                    ls.lock().unwrap().update(Log {
                        start: res.start_time,
                        finish: res.finish_time,
                        duration: res.duration,
                        log: res.log,
                        name: String::from(&name),
                    });
                },
                Err(e) => {
                    error!("run_shell cmd=`{}' error={}", &cmd, e);
                    ms.lock().unwrap().update(&name, DataPoint {
                        time: chrono::Utc::now(),
                        val: MetricValue::OkErr(OkErr::Err),
                    });
                    ls.lock().unwrap().update(Log {
                        start, finish,
                        // Susceptible to shifts in time, e.g. leap seconds.
                        duration: std::time::Duration::from_millis(
                            (finish - start).num_milliseconds() as u64),
                        log: format!("Error={}", e),
                        name: String::from(&name),
                    });
                }
            }
        })),
    };
    sched.lock().unwrap()
         .add(j);
}

// TODO: Ugly duplication between this and add_shell_check_job.
fn add_shell_metric_job(
    config: &ShellMetricConfig,
    ms: Arc<Mutex<MetricStore>>,
    sched: Arc<Mutex<Scheduler>>,
    ls: Arc<Mutex<LogStore>>,
) {
    let cmd = config.cmd.to_owned();
    let name = config.name.to_owned();
    let j = scheduler::JobDefinition {
        interval: config.interval,
        name: String::from(&name),
        f: Arc::new(Mutex::new(move |_rc| {
            let mut command = std::process::Command::new("sh");
            command.arg("-c");
            command.arg(&cmd);

            // Ugly: calculates UTC time twice, once in run_shell and once here.
            let start = chrono::Utc::now();
            let res = run_shell(command);
            let finish = chrono::Utc::now();

            match res {
                Ok(res) => {
                    debug!("run_shell cmd=`{}' log=\n{}", &cmd, res.log);
                    match res.stdout.trim().parse::<i64>() {
                        Ok(i) =>
                            ms.lock().unwrap().update(&name, DataPoint {
                                time: chrono::Utc::now(),
                                val: MetricValue::I64(i)
                            }),
                        Err(e) =>
                            error!("Error parsing run_shell stdout: {}", e),
                    };
                    ls.lock().unwrap().update(Log {
                        start: res.start_time,
                        finish: res.finish_time,
                        duration: res.duration,
                        log: res.log,
                        name: String::from(&name),
                    });
                },
                Err(e) => {
                    error!("run_shell cmd=`{}' error={}", &cmd, e);
                    ls.lock().unwrap().update(Log {
                        start, finish,
                        // Susceptible to shifts in time, e.g. leap seconds.
                        duration: std::time::Duration::from_millis(
                            (finish - start).num_milliseconds() as u64),
                        log: format!("Error={}", e),
                        name: String::from(&name),
                    });
                }
            }
        })),
    };
    sched.lock().unwrap()
         .add(j);
}

#[derive(Clone, Debug)]
struct RunShellResult {
    log: String,
    ok: OkErr,
    exit_code: Option<i64>,
    stdout: String,
    stderr: String,
    duration: std::time::Duration,
    start_time: chrono::DateTime<chrono::Utc>,
    finish_time: chrono::DateTime<chrono::Utc>,
}

fn run_shell(mut cmd: std::process::Command) -> Result<RunShellResult, std::io::Error> {
    let cmd = cmd.stdout(std::process::Stdio::piped())
                 .stderr(std::process::Stdio::piped());
    let start = std::time::Instant::now();
    let start_utc = chrono::Utc::now();
    let res = cmd.spawn()?
                 .with_output_timeout(std::time::Duration::from_secs(15))
                 .terminating()
                 .wait()?
                 .ok_or_else(|| {
                     std::io::Error::new(std::io::ErrorKind::TimedOut, "Process timed out")
                 })?;
    let finish = std::time::Instant::now();
    let finish_utc = chrono::Utc::now();

    let duration: std::time::Duration = finish - start;

    let stdout_string = String::from_utf8_lossy(&res.stdout);
    let stderr_string = String::from_utf8_lossy(&res.stderr);

    let mut log = String::new();
    log.push_str("stdout:\n=======\n");
    log.push_str(&stdout_string);
    log.push_str("=======\n");
    log.push_str("stderr:\n=======\n");
    log.push_str(&stderr_string);
    log.push_str("=======\n");

    log.push_str(&format!("exit_status: {:?}\n", res.status));
    log.push_str(&format!("duration: {}ms\n", duration.as_millis()));
    log.push_str(&format!("start: {}\n",
                          start_utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)));
    log.push_str(&format!("finish: {}",
                          finish_utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)));

    let res = RunShellResult {
        log: log,
        exit_code: res.status.code(),
        ok: match res.status.success() {
            false => OkErr::Err,
            true => OkErr::Ok,
        },
        stdout: String::from(stdout_string),
        stderr: String::from(stderr_string),
        duration,
        start_time: start_utc,
        finish_time: finish_utc,
    };
    Ok(res)
}

impl std::fmt::Display for MetricValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            MetricValue::OkErr(OkErr::Ok) => f.write_str("Ok"),
            MetricValue::OkErr(OkErr::Err) => f.write_str("Err"),
            MetricValue::I64(int) => f.write_str(&format!("{}", int)),
            MetricValue::_F64(float) => f.write_str(&format!("{}", float)),
        }
    }
}

impl MetricValue {
    fn as_i64(&self) -> Option<i64> {
        match self {
            MetricValue::I64(i) => Some(*i),
            _ => None,
        }
    }
}
