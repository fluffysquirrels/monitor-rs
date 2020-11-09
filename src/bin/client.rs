#[macro_use]
extern crate log;

use monitor::{
    connect_all_checks_to_notifier,
    config::{self, ShellCheck, ShellMetric},
    Continue,
    create_shell_checks,
    create_shell_metrics,
    DataPoint,
    Host,
    LogStore,
    MetricCheck,
    MetricKey,
    MetricStore,
    MetricValue,
    Notifier,
    OkErr,
    RemoteHost,
    remote_sync,
    scheduler::Scheduler,
};
use gio::prelude::*;
use gtk::prelude::*;
use std::{
    cell::Cell,
    collections::BTreeMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct Ui {
    metrics: BTreeMap<MetricKey, MetricUi>,
}

#[derive(Clone)]
struct MetricUi {
    label_status: gtk::Label,
    graph: Rc<rt_graph::GraphWithControls>,
    showing_graph: Rc<Cell<bool>>,
    show_graph_btn: gtk::Button,
}

fn shell_check_configs() -> Vec<ShellCheck> {
    vec![
        ShellCheck {
            name: "ping.mf".to_owned(),
            cmd: "ping -c 1 -W 5 mf".to_owned(),
            interval: config::Duration::Seconds(5),
        },
        ShellCheck {
            name: "ping.f1".to_owned(),
            cmd: "ping -c 1 -W 5 f1".to_owned(),
            interval: config::Duration::Seconds(60),
        },
        ShellCheck {
            name: "apt.plato.upgradable".to_owned(),
            cmd: "/home/alex/Code/rust/monitor/scripts/apt-upgradable.py".to_owned(),
            interval: config::Duration::Minutes(10),
        },
        ShellCheck {
            name: "apt.mf.upgradable".to_owned(),
            cmd: "ssh mf /home/alex/Code/apt-upgradable.py".to_owned(),
            interval: config::Duration::Minutes(10),
        },
        ShellCheck {
            name: "internet.up.gstatic".to_owned(),
            cmd: "curl http://connectivitycheck.gstatic.com/generate_204 -v -f -s".to_owned(),
            interval: config::Duration::Minutes(2),
        },
        ShellCheck {
            name: "zfs.mf.healthy".to_owned(),
            cmd: "ssh mf /sbin/zpool status -x | grep 'all pools are healthy'".to_owned(),
            interval: config::Duration::Minutes(2),
        },
        // check_travis("github", "fluffysquirrels/mqtt-async-client-rs", "master"),
        // check_travis("github", "fluffysquirrels/webdriver_client_rust", "master"),
        // check_travis("github", "fluffysquirrels/framed-rs", "master"),
    ]
}

fn shell_metric_configs() -> Vec<ShellMetric> {
    vec![
        ShellMetric {
            cmd: "df -h / | awk '{print $5}' | egrep -o '[0-9]+'".to_owned(),
            name: "df.local.root".to_owned(),
            interval: config::Duration::Minutes(5),
            check: MetricCheck::Max(80),
        },
        ShellMetric {
            cmd: "ssh mf df -h / | awk '{print $5}' | egrep -o '[0-9]+'".to_owned(),
            name: "df.mf.root".to_owned(),
            interval: config::Duration::Minutes(5),
            check: MetricCheck::Max(80),
        },
        ShellMetric {
            cmd: "ssh mf df -h /mnt/monster | awk '{print $5}' | egrep -o '[0-9]+'".to_owned(),
            name: "df.mf.monster".to_owned(),
            interval: config::Duration::Minutes(5),
            check: MetricCheck::Max(80),
        },
    ]
}

fn check_travis(provider: &str, repo: &str, branch: &str) -> ShellCheck {
    ShellCheck {
        name: format!("travis.{}.{}.{}.passed", provider, repo, branch),
        cmd: format!(
"curl -f -s -H 'Travis-API-Version: 3' 'https://api.travis-ci.com/repo/{}/{}/branch/{}' |
tee /dev/stderr |
jq '.last_build.state' |
egrep '^\"passed\"$'",
            provider,
            repo.replace('/', "%2F"),
            branch),
        interval: config::Duration::Minutes(30),
    }
}

fn config() -> config::Client {
    config::Client {
        shell_checks: shell_check_configs(),
        shell_metrics: shell_metric_configs(),
        remote_syncs: vec![
            config::RemoteSync {
                url: "https://mf:8080".to_owned(),
                server_ca: config::TlsCertificate {
                    cert_path: "/home/alex/Code/rust/monitor/cert/ok/ca.cert".to_owned(),
                },
                client_identity: config::TlsIdentity {
                    cert_path: "/home/alex/Code/rust/monitor/cert/ok/plato.fullchain".to_owned(),
                    key_path:  "/home/alex/Code/rust/monitor/cert/ok/plato.key".to_owned(),
                },
            },
            config::RemoteSync {
                url: "https://f1:80".to_owned(),
                server_ca: config::TlsCertificate {
                    cert_path: "/home/alex/Code/rust/monitor/cert/ok/ca.cert".to_owned(),
                },
                client_identity: config::TlsIdentity {
                    cert_path: "/home/alex/Code/rust/monitor/cert/ok/plato.fullchain".to_owned(),
                    key_path:  "/home/alex/Code/rust/monitor/cert/ok/plato.key".to_owned(),
                },
            },
        ],
        remote_checks: vec![
            config::RemoteCheck {
                name: "travis.github.fluffysquirrels/framed-rs.master.passed".to_owned(),
                host_name: "instance-1".to_owned(),
            },
            config::RemoteCheck {
                name: "travis.github.fluffysquirrels/mqtt-async-client-rs.master.passed"
                          .to_owned(),
                host_name: "instance-1".to_owned(),
            },
            config::RemoteCheck {
                name: "travis.github.fluffysquirrels/webdriver_client_rust.master.passed"
                          .to_owned(),
                host_name: "instance-1".to_owned(),
            },
            config::RemoteCheck {
                name: "zfs.mf.healthy".to_owned(),
                host_name: "MicroFridge".to_owned(),
            },
        ]
    }
}

fn main() {
    env_logger::Builder::new()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned()))
        .format_timestamp_micros()
        .init();

    let ls = Arc::new(Mutex::new(LogStore::new()));
    let ms = Arc::new(Mutex::new(MetricStore::new()));
    let n = Arc::new(Mutex::new(Notifier::new()));
    let sched = Arc::new(Mutex::new(Scheduler::new()));

    let config = config();
    trace!("client config in rudano = {}",
           rudano::to_string_pretty(&config).expect("rudano serialisation"));

    create_shell_checks(&config.shell_checks, &ls, &ms, &sched);
    create_shell_metrics(&config.shell_metrics, &ls, &ms, &n, &sched);

    connect_all_checks_to_notifier(&ms, &n);

    for rsc in config.remote_syncs.iter() {
        remote_sync::spawn_job_streaming(&rsc, &ms);
    }

    sched.lock().unwrap().spawn();

    let application =
        gtk::Application::new(Some("com.github.fluffysquirrels.monitor"),
                              gio::ApplicationFlags::default())
            .expect("Application::new failed");

    application.connect_activate(move |app| {
        let ms = ms.clone();
        let sc = sched.clone();
        let ls = ls.clone();
        let config = config.clone();
        build_ui(&config, app, ms, sc, ls);
    });

    application.run(&std::env::args().collect::<Vec<_>>());
}

fn build_ui(
    config: &config::Client,
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
        .unwrap_or_else(|_| ".".to_string());
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

    let mut metrics = BTreeMap::<MetricKey, MetricUi>::new();

    for config in config.shell_checks.iter() {
        let key = MetricKey { name: config.name.clone(), host: Host::Local };
        let metric_ui = ui_for_metric(&metrics_box, &gdk_window, &key, &ls, &ms, &sched);
        metrics.insert(key, metric_ui);
    }

    for config in config.shell_metrics.iter() {
        let key = MetricKey { name: config.name.clone(), host: Host::Local };
        let metric_ui = ui_for_metric(&metrics_box, &gdk_window, &key, &ls, &ms, &sched);
        metrics.insert(key, metric_ui);
    }

    for config in config.remote_checks.iter() {
        let key = config.to_metric_key();
        let metric_ui = ui_for_metric(&metrics_box, &gdk_window, &key, &ls, &ms, &sched);
        metrics.insert(key, metric_ui);
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
            if tx.send(log).is_err() {
                error!("LogStore UI channel send error");
            }
            Continue::Continue
        });

        let uic = ui.clone();
        rx.attach(Some(&ui_thread), move |log| {
            let metric = uic.metrics.get(&log.key);
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
            if tx.send(metric).is_err() {
                error!("MetricStore UI channel send error");
            }
            Continue::Continue
        });

        rx.attach(Some(&ui_thread), move |metric| {
            if let Some(DataPoint { val, .. }) = metric.latest() {
                let ui_metric = ui.metrics.get(metric.key());
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
    container: &C,
    gdk_window: &gdk::Window,
    metric_key: &MetricKey,
    _ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>,
    sched: &Arc<Mutex<Scheduler>>,
) -> MetricUi
    where C: IsA<gtk::Container> + IsA<gtk::Widget>
{
    let label_box = gtk::BoxBuilder::new()
        .parent(container)
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let _label = gtk::LabelBuilder::new()
        .label(&metric_key.display_name())
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
    let metric_key_clone = metric_key.to_owned();
    force_btn.connect_clicked(move |_btn| {
        if let Host::Local = metric_key_clone.host {
            sched.lock().unwrap().force_run(&metric_key_clone.name);
        } else {
            error!("Forcing remote checks is not yet supported");
        }
    });

    let show_graph_btn = gtk::ButtonBuilder::new()
        .label("Show graph")
        .parent(&buttons_box)
        .halign(gtk::Align::Start)
        .build();

    let config = rt_graph::ConfigBuilder::default()
        .data_source(MetricStoreDataSource::new(metric_key, ms.clone()))
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
    metric_key: MetricKey,
    ms: Arc<Mutex<MetricStore>>,
    last: Option<DataPoint>,
    t: u32,
}

impl std::fmt::Debug for MetricStoreDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.debug_struct("MetricStoreDataSource")
            .field("metric_key", &self.metric_key)
            .finish()
    }
}

impl MetricStoreDataSource {
    fn new(metric_key: &MetricKey, ms: Arc<Mutex<MetricStore>>) -> MetricStoreDataSource {
        MetricStoreDataSource {
            metric_key: metric_key.clone(),
            ms,
            t: 1,
            last: None,
        }
    }
}

impl rt_graph::DataSource for MetricStoreDataSource {
    fn get_data(&mut self) -> Result<Vec<rt_graph::Point>, rt_graph::Error> {
        let m = self.ms.lock().unwrap().query_one(&self.metric_key);
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
                                    MetricValue::F64(_f) => unimplemented!(),
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
