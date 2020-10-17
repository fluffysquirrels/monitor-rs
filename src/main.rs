#[macro_use]
extern crate log;

mod log_store;
mod metric_store;
mod notifier;
mod scheduler;
mod signal;

use crate::{
    log_store::{Log, LogStore},
    metric_store::MetricStore,
    notifier::Notifier,
    scheduler::Scheduler,
    signal::Signal,
};
use gio::prelude::*;
use glib;
use gtk::prelude::*;
use rt_graph;
use subprocess;
use std::{
    collections::BTreeMap,
    env::args,
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
    _I64(i64),
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
    label: gtk::Label,
}

fn main() {
    env_logger::init();

    let ms = Arc::new(Mutex::new(MetricStore::new()));
    let n = Arc::new(Mutex::new(Notifier::new()));
    let sched = Arc::new(Mutex::new(Scheduler::new()));
    let ls = Arc::new(Mutex::new(LogStore::new()));

    add_shell_check_job("ping.mf",
                        "ping -c 1 -W 5 mf",
                        chrono::Duration::seconds(5),
                        ms.clone(),
                        sched.clone(),
                        ls.clone());

    add_shell_check_job("apt.upgradable",
                        "/home/alex/Code/rust/monitor/scripts/apt-upgradable.py",
                        chrono::Duration::seconds(600),
                        ms.clone(),
                        sched.clone(),
                        ls.clone());

    add_shell_check_job("mf.apt.upgradable",
                        "ssh mf /home/alex/Code/apt-upgradable.py",
                        chrono::Duration::seconds(600),
                        ms.clone(),
                        sched.clone(),
                        ls.clone());

    add_shell_check_job("internet.up.gstatic",
                        "curl http://connectivitycheck.gstatic.com/generate_204 -v -f -s",
                        chrono::Duration::seconds(120),
                        ms.clone(),
                        sched.clone(),
                        ls.clone());

    let msc = ms.clone();
    sched.lock().unwrap().add(scheduler::JobDefinition {
        f: Arc::new(Mutex::new(move |_rc| {
            for m in msc.lock().unwrap().query_all() {
                debug!("{:?}", m);
            }
        })),
        interval: chrono::Duration::seconds(5),
        name: String::from("show-metrics"),
    });

    sched.lock().unwrap().spawn();

    // Listen to metrics and connect the Notifier.
    let nc = n.clone();
    ms.lock().unwrap()
        .update_signal()
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
        build_ui(app, ms, sc, ls);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn build_ui(
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

    // Show the (gtk) window so we can get a gdk::window below.
    window.show();
    let gdk_window = window.get_window().unwrap();

    let graphs = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .build();
    window.add(&graphs);

    let metric_names = vec![
        "ping.mf",
        "internet.up.gstatic",
        "apt.upgradable",
        "mf.apt.upgradable",
    ];

    let metrics = metric_names.iter().map(|name| {
        let metric_ui = ui_for_metric(&ms, &sched, &ls, &graphs, &gdk_window, name);
        (String::from(*name), metric_ui)
    }).collect::<BTreeMap<String, MetricUi>>();

    window.show_all();

    let ui = Ui {
        metrics,
    };

    // Subscribe to log messages and send them over a channel to the UI thread,
    // which can then update the UI.

    let (tx, rx) = glib::MainContext::sync_channel(glib::source::Priority::default(), 50);

    ls.lock().unwrap().update_signal().connect(move |log| {
        match tx.send(log) {
            Err(_) => error!("LogStore UI channel send error"),
            Ok(_) => (),
        }
    });

    let ui_thread = glib::MainContext::ref_thread_default();
    let uic = ui.clone();
    rx.attach(Some(&ui_thread), move |log| {
        let metric = uic.metrics.get(&log.name);
        if let Some(metric) = metric {
            metric.label.set_tooltip_text(Some(&log.log));
        }
        glib::source::Continue(true)
    });
}

fn ui_for_metric<C>(
    ms: &Arc<Mutex<MetricStore>>,
    sched: &Arc<Mutex<Scheduler>>,
    _ls: &Arc<Mutex<LogStore>>,
    graphs: &C,
    gdk_window: &gdk::Window,
    metric_name: &str
) -> MetricUi
    where C: IsA<gtk::Container> + IsA<gtk::Widget>
{
    let label = gtk::LabelBuilder::new()
        .label(metric_name)
        .parent(graphs)
        .build();

    let force_btn = gtk::ButtonBuilder::new()
        .label("Force")
        .parent(graphs)
        .halign(gtk::Align::Start)
        .build();
    let sched = sched.clone();
    let metric_name_clone = metric_name.to_owned();
    force_btn.connect_clicked(move |_btn| {
        sched.lock().unwrap().force_run(&metric_name_clone);
    });

    let config = rt_graph::ConfigBuilder::default()
        .data_source(MetricStoreDataSource {
            metric_name: String::from(metric_name),
            ms: ms.clone(),
            t: 1,
        })
        .base_zoom_x(10.0)
        .graph_height(100)
        .build()
        .unwrap();
    let mut _g = rt_graph::GraphWithControls::build_ui(config, graphs, &gdk_window);

    MetricUi {
        label,
    }
}

struct MetricStoreDataSource {
    metric_name: String,
    ms: Arc<Mutex<MetricStore>>,
    t: u32,
}

impl std::fmt::Debug for MetricStoreDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.debug_struct("MetricStoreDataSource")
            .field("metric_name", &self.metric_name)
            .finish()
    }
}

impl rt_graph::DataSource for MetricStoreDataSource {
    fn get_data(&mut self) -> Result<Vec<rt_graph::Point>, rt_graph::Error> {
        // TODO: Only return points when they're new.
        let m = self.ms.lock().unwrap().query_one(&self.metric_name);
        let res =
            m.iter()
             .map(|m| rt_graph::Point {
                 t: self.t, // TODO: Use time.
                 vs: vec![match m.latest() {
                     Some(DataPoint { val: MetricValue::OkErr(ok), .. }) =>
                         match ok {
                             OkErr::Ok => 50000,
                             OkErr::Err => 10000,
                         },
                     // TODO: Add other metric types.
                     _ => 0,
                 }],
             })
             .collect::<Vec<rt_graph::Point>>();
        self.t += 1;
        Ok(res)
    }

    fn get_num_values(&self) -> Result<usize, rt_graph::Error> {
        Ok(1)
    }

}

fn add_shell_check_job(
    name: &str,
    cmd: &str,
    interval: chrono::Duration,
    ms: Arc<Mutex<MetricStore>>,
    sched: Arc<Mutex<Scheduler>>,
    ls: Arc<Mutex<LogStore>>,
) {
    let cmd = cmd.to_owned();
    let name2 = name.to_owned();
    let j = scheduler::JobDefinition {
        f: Arc::new(Mutex::new(move |_rc| {
            let start = chrono::Utc::now();
            let res = shell_check(subprocess::Exec::shell(&cmd));
            let finish = chrono::Utc::now();

            debug!("shell_check cmd=`{}' log=\n{}", &cmd, res.log);
            ms.lock().unwrap().update(&name2, DataPoint {
                time: chrono::Utc::now(),
                val: MetricValue::OkErr(res.ok)
            });
            ls.lock().unwrap().update(Log {
                start, finish,
                duration: res.duration,
                log: res.log,
                name: String::from(&name2),
            });
        })),
        interval: interval,
        name: String::from(name),
    };
    sched.lock().unwrap()
         .add(j);
}

#[derive(Clone, Debug)]
struct ShellCheckResult {
    log: String,
    ok: OkErr,
    exit_code: Option<u32>,
    duration: std::time::Duration,
}

fn shell_check(cmd: subprocess::Exec) -> ShellCheckResult {
    let start = std::time::Instant::now();
    let res = cmd
        .stdout(subprocess::Redirection::Pipe)
        .stderr(subprocess::Redirection::Merge)
        .capture().unwrap();
    let end = std::time::Instant::now();
    let duration: std::time::Duration = end - start;

    let mut log = String::new();
    log.push_str("stdout & stderr:\n=======\n");
    log.push_str(&*res.stdout_str());
    log.push_str("=======\n");

    log.push_str(&format!("exit_status: {:?}\n", res.exit_status));
    log.push_str(&format!("duration: {}ms", duration.as_millis()));

    let res = ShellCheckResult {
        log: log,
        exit_code: match res.exit_status {
            subprocess::ExitStatus::Exited(code) => Some(code),
            _ => None,
        },
        ok: match res.exit_status {
            subprocess::ExitStatus::Exited(0) => OkErr::Ok,
            _ => OkErr::Err,
        },
        duration,
    };
    res
}
