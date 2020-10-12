#[macro_use]
extern crate log;

mod scheduler;
mod signal;

use crate::signal::Signal;
use gio::prelude::*;
use gtk::prelude::*;
use rt_graph;
use subprocess;
use std::{
    env::args,
    sync::{Arc, Mutex},
};

fn main() {
    env_logger::init();

    // let nh = notify_rust::Notification::new()
    //     .summary("monitor")
    //     .body("Starting!")
    //     .timeout(notify_rust::Timeout::Milliseconds(2000))
    //     .show().unwrap();
    //
    // std::thread::sleep(std::time::Duration::from_secs(5));
    // nh.close();

    let ms = Arc::new(Mutex::new(MetricStore::new()));
    let n = Arc::new(Mutex::new(Notifier::new()));
    let mut sched = scheduler::Scheduler::new();

    sched.add(shell_check_job("ping.mf",
                              "ping -c 1 -W 5 mf",
                              chrono::Duration::seconds(5),
                              ms.clone()));

    sched.add(shell_check_job("apt.upgradable",
                              "/home/alex/Code/rust/monitor/scripts/apt-upgradable.py",
                              chrono::Duration::seconds(600),
                              ms.clone()));

    sched.add(shell_check_job("mf.apt.upgradable",
                              "ssh mf /home/alex/Code/apt-upgradable.py",
                              chrono::Duration::seconds(600),
                              ms.clone()));

    let msc = ms.clone();
    sched.add(scheduler::JobDefinition {
        f: Arc::new(Mutex::new(move |_rc| {
            for m in msc.lock().unwrap().query_all() {
                debug!("{:?}", m);
            }
        })),
        interval: chrono::Duration::seconds(5),
        name: String::from("show-metrics"),
    });

    let nc = n.clone();
    ms.lock().unwrap()
        .update_signal()
        .connect(move |m|
                 {
                     if let Some(DataPoint { val: MetricValue::OkErr(ok),.. }) = m.latest {
                         nc.lock().unwrap().update_metric(&m.name, ok);
                     }
                 });
    sched.spawn();

    let application =
        gtk::Application::new(Some("com.github.fluffysquirrels.monitor"),
                              gio::ApplicationFlags::default())
            .expect("Application::new failed");

    let msc = ms.clone();
    application.connect_activate(move |app| {
        let ms = msc.clone();
        build_ui(app, ms);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn build_ui(application: &gtk::Application, ms: Arc<Mutex<MetricStore>>) {
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

    graph_for_metric(&ms, &graphs, &gdk_window, "ping.mf");
    graph_for_metric(&ms, &graphs, &gdk_window, "apt.upgradable");
    graph_for_metric(&ms, &graphs, &gdk_window, "mf.apt.upgradable");

    window.show_all();
}

fn graph_for_metric<C>(
    ms: &Arc<Mutex<MetricStore>>,
    graphs: &C,
    gdk_window: &gdk::Window,
    metric_name: &str
)
    where C: IsA<gtk::Container> + IsA<gtk::Widget>
{
    gtk::LabelBuilder::new()
        .label(metric_name)
        .parent(graphs)
        .build();
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
                 vs: vec![match m.latest {
                     Some(DataPoint { val: MetricValue::OkErr(ok), .. }) =>
                         match ok {
                             OkErr::Ok => 50000,
                             OkErr::Err => 10000,
                         },
                     // TODO.
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

fn shell_check_job(
    name: &str,
    cmd: &str,
    interval: chrono::Duration,
    ms: Arc<Mutex<MetricStore>>,
) -> scheduler::JobDefinition {
    let cmd = cmd.to_owned();
    let name2 = name.to_owned();
    scheduler::JobDefinition {
        f: Arc::new(Mutex::new(move |_rc| {
            let res = shell_check(subprocess::Exec::shell(&cmd));
            ms.lock().unwrap().update(&name2, DataPoint {
                time: chrono::Utc::now(),
                val: MetricValue::OkErr(res.ok)
            });
        })),
        interval: interval,
        name: String::from(name),
    }
}

#[derive(Clone, Debug)]
struct ShellCheckResult {
    log: String,
    ok: OkErr,
    exit_code: Option<u32>,
}

fn shell_check(cmd: subprocess::Exec) -> ShellCheckResult {
    let res = cmd.capture().unwrap();
    let mut log = String::new();
    log.push_str("stdout:\n=======\n");
    log.push_str(&*res.stdout_str());
    log.push_str("=======\n");

    log.push_str("stderr:\n=======\n");
    log.push_str(&*res.stderr_str());
    log.push_str("=======\n");
    log.push_str(&format!("exit_status: {:?}", res.exit_status));

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
    };
    debug!("shell_check res={:#?}", res);
    res
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OkErr {
    Ok,
    Err,
}

#[derive(Clone, Debug)]
enum MetricValue {
    OkErr(OkErr),
    _I64(i64),
    _F64(f64),
}

#[derive(Clone, Debug)]
struct DataPoint {
    time: chrono::DateTime<chrono::Utc>,
    val: MetricValue,
}

#[derive(Clone, Debug)]
struct Metric {
    latest: Option<DataPoint>,
    name: String,
}

struct MetricStore {
    metrics: Vec<Metric>,
    update_signal: Signal<Metric>,
}

impl MetricStore {
    pub fn new() -> MetricStore {
        MetricStore {
            metrics: vec![],
            update_signal: Signal::new(),
        }
    }

    pub fn update(&mut self, metric_name: &str, point: DataPoint) {
        let existing = self.metrics.iter_mut().find(|m| m.name == metric_name);
        let metric: &mut Metric = match existing {
            Some(e) => e,
            None => {
                self.metrics.push(Metric {
                    name: metric_name.to_owned(),
                    latest: None,
                });
                self.metrics.last_mut().unwrap()
            }
        };
        metric.latest = Some(point);
        self.update_signal.raise(metric.clone());
    }

    pub fn update_signal(&mut self) -> &mut Signal<Metric> {
        &mut self.update_signal
    }

    pub fn query_one(&self, metric_name: &str) -> Option<Metric> {
        self.metrics.iter()
                    .find(|m| m.name == metric_name)
                    .cloned()
    }

    pub fn query_all(&self) -> Vec<Metric> {
        self.metrics.iter().cloned().collect()
    }
}

pub struct Notifier {
    metrics: Vec<NotifierMetric>,
}

struct NotifierMetric {
    name: String,
    last_value: OkErr,
    last_notification: Option<chrono::DateTime<chrono::Utc>>,
}

const NOTIFICATION_REFRESH_SECS: i64 = 5 * 60; // 5 minutes

impl Notifier {
    pub fn new() -> Notifier {
        Notifier {
            metrics: vec![],
        }
    }

    pub fn update_metric(&mut self, name: &str, new_value: OkErr) {
        let existing = self.metrics.iter_mut().find(|m| m.name == name);
        let metric: &mut NotifierMetric = match existing {
            Some(m) => m,
            None => {
                self.metrics.push(NotifierMetric {
                    name: name.to_owned(),
                    last_value: OkErr::Ok,
                    last_notification: None,
                });
                self.metrics.last_mut().unwrap()
            }
        };
        let last_value = metric.last_value;
        metric.last_value = new_value;

        let is_changed = last_value != new_value;
        let is_old = metric.last_notification.is_none()
            || ((chrono::Utc::now() - metric.last_notification.unwrap()) >
                chrono::Duration::seconds(NOTIFICATION_REFRESH_SECS));
        let is_old_error = (last_value == OkErr::Err) && is_old;
        if is_changed || is_old_error {
            trace!("Notifier: is_changed={} is_old_error={}", is_changed, is_old_error);
            let res = notify_rust::Notification::new()
                .summary("monitor")
                .body(&format!("metric `{}` is {:?}", metric.name, metric.last_value))
                .timeout(notify_rust::Timeout::Milliseconds(2000))
                .show();
            metric.last_notification = Some(chrono::Utc::now());
            if let Err(e) = res {
                error!("Showing notification: {}", e);
            }

            // TODO: Close after n seconds. NB: NotificationHandle is !Send.

            // std::thread::sleep(std::time::Duration::from_secs(5));
            // nh.close();
        }
    }
}
