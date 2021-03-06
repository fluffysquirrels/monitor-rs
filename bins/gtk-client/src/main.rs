#[macro_use]
extern crate log;

use monitor::{
    connect_all_checks_to_notifier,
    config::{self, ShellCheck, ShellMetric},
    Continue,
    create_shell_checks,
    create_shell_metrics,
    DataPoint,
    force_check,
    Host,
    LogStore,
    metric_store,
    MetricCheck,
    MetricKey,
    MetricStore,
    MetricValue,
    Notifier,
    OkErr,
    remote,
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

struct Ui {
    metrics: BTreeMap<MetricKey, Rc<MetricUi>>,
    summary_label: gtk::Label,
    thread: glib::MainContext,
    display_checks: Cell<DisplayChecks>,
}

struct MetricUi {
    label_status: gtk::Label,
    graph: rt_graph::GraphWithControls,
    showing_graph: Cell<bool>,
    show_graph_btn: gtk::Button,
    metric_box: gtk::Box,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum DisplayChecks {
    All,
    Err,
}

const OK_COLOR: &'static str = "#00cc00";
const ERR_COLOR: &'static str = "#cc0000";
const MISSING_COLOR: &'static str = "#cccc00";

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
            name: "ping.f1-vpn".to_owned(),
            cmd: "ping -c 1 -W 5 192.168.1.2".to_owned(),
            interval: config::Duration::Seconds(60),
        },
        ShellCheck {
            name: "internet.up.gstatic".to_owned(),
            cmd: "curl http://connectivitycheck.gstatic.com/generate_204 -v -f -s".to_owned(),
            interval: config::Duration::Minutes(2),
        },
        // ShellCheck {
        //     name: "zfs.mf.healthy".to_owned(),
        //     cmd: "ssh mf /sbin/zpool status -x | grep 'all pools are healthy'".to_owned(),
        //     interval: config::Duration::Minutes(2),
        // },
        // check_travis("github", "fluffysquirrels/mqtt-async-client-rs", "master"),
        // check_travis("github", "fluffysquirrels/webdriver_client_rust", "master"),
        // check_travis("github", "fluffysquirrels/framed-rs", "master"),
    ]
}

fn shell_metric_configs() -> Vec<ShellMetric> {
    vec![
        ShellMetric {
            cmd: "df -h / | awk '{print $5}' | egrep -o '[0-9]+'".to_owned(),
            name: "df.plato.root".to_owned(),
            interval: config::Duration::Minutes(5),
            check: MetricCheck::Max(80),
        },
        ShellMetric {
            name: "apt.plato.upgradable".to_owned(),
            cmd: "/home/alex/Code/rust/monitor/scripts/apt-upgradable.py".to_owned(),
            interval: config::Duration::Minutes(10),
            check: MetricCheck::Max(0),
        },
        // ShellMetric {
        //     name: "apt.mf.upgradable".to_owned(),
        //     cmd: "ssh mf /home/alex/Code/apt-upgradable.py".to_owned(),
        //     interval: config::Duration::Minutes(10),
        //     check: MetricCheck::Max(0),
        // },
        // ShellMetric {
        //     cmd: "ssh mf df -h / | awk '{print $5}' | egrep -o '[0-9]+'".to_owned(),
        //     name: "df.mf.root".to_owned(),
        //     interval: config::Duration::Minutes(5),
        //     check: MetricCheck::Max(80),
        // },
        // ShellMetric {
        //     cmd: "ssh mf df -h /mnt/monster | awk '{print $5}' | egrep -o '[0-9]+'".to_owned(),
        //     name: "df.mf.monster".to_owned(),
        //     interval: config::Duration::Minutes(5),
        //     check: MetricCheck::Max(80),
        // },
    ]
}

fn _check_travis(provider: &str, repo: &str, branch: &str) -> ShellCheck {
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

fn config() -> config::GtkClient {
    config::GtkClient {
        shell_checks: shell_check_configs(),
        shell_metrics: shell_metric_configs(),
        remote_syncs: vec![
            config::RemoteSync {
                url: "https://mf:6443".to_owned(),
                server_ca: config::TlsCertificate {
                    cert_path: "/home/alex/Code/rust/monitor/cert/ok/ca.cert".to_owned(),
                },
                client_identity: config::TlsIdentity {
                    cert_path: "/home/alex/Code/rust/monitor/cert/ok/plato.fullchain".to_owned(),
                    key_path:  "/home/alex/Code/rust/monitor/cert/ok/plato.key".to_owned(),
                },
            },
            config::RemoteSync {
                url: "https://f1:6443".to_owned(),
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
                name: "apt.f1.upgradable".to_owned(),
                host_name: "f1".to_owned(),
            },
            config::RemoteCheck {
                name: "apt.mf.upgradable".to_owned(),
                host_name: "mf".to_owned(),
            },
            config::RemoteCheck {
                name: "df.f1.root".to_owned(),
                host_name: "f1".to_owned(),
            },
            config::RemoteCheck {
                name: "df.mf.monster".to_owned(),
                host_name: "mf".to_owned(),
            },
            config::RemoteCheck {
                name: "df.mf.root".to_owned(),
                host_name: "mf".to_owned(),
            },
            config::RemoteCheck {
                name: "jellyfin.mf.http".to_owned(),
                host_name: "mf".to_owned(),
            },
            config::RemoteCheck {
                name: "travis.github.fluffysquirrels/framed-rs.master.passed".to_owned(),
                host_name: "f1".to_owned(),
            },
            config::RemoteCheck {
                name: "travis.github.fluffysquirrels/mqtt-async-client-rs.master.passed"
                          .to_owned(),
                host_name: "f1".to_owned(),
            },
            config::RemoteCheck {
                name: "travis.github.fluffysquirrels/webdriver_client_rust.master.passed"
                          .to_owned(),
                host_name: "f1".to_owned(),
            },
            config::RemoteCheck {
                name: "zfs.mf.healthy".to_owned(),
                host_name: "mf".to_owned(),
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
    create_shell_metrics(&config.shell_metrics, &ls, &ms, Some(&n), &sched);

    connect_all_checks_to_notifier(&ms, &n);

    let remotes = remote::Remotes::from_configs(&config.remote_syncs)
                                  .expect("RemoteSync configs OK");
    let remotes = Arc::new(remotes);

    let application =
        gtk::Application::new(Some("com.github.fluffysquirrels.monitor"),
                              gio::ApplicationFlags::default())
            .expect("Application::new failed");

    application.connect_activate(move |app| {
        let ms = ms.clone();
        let sc = sched.clone();
        let ls = ls.clone();
        let config = config.clone();
        let remotes = remotes.clone();
        build_ui(&config, app, ls.clone(), ms.clone(), remotes.clone(), sc);

        remote::spawn_sync_jobs(&remotes, &ls, &ms);
        sched.lock().unwrap().spawn();
    });

    application.run(&std::env::args().collect::<Vec<_>>());
}

fn build_ui(
    config: &config::GtkClient,
    application: &gtk::Application,
    ls: Arc<Mutex<LogStore>>,
    ms: Arc<Mutex<MetricStore>>,
    remotes: Arc<remote::Remotes>,
    sched: Arc<Mutex<Scheduler>>,
) {
    let window = gtk::ApplicationWindowBuilder::new()
        .application(application)
        .title("monitor")
        .border_width(8)
        .window_position(gtk::WindowPosition::Center)
        .default_height(900)
        .default_width(500)
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

    let window_box = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .spacing(8)
        .parent(&window)
        .build();

    let summary_label = gtk::LabelBuilder::new()
        .parent(&window_box)
        .build();

    let top_controls = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Horizontal)
        .parent(&window_box)
        .build();

    let all_checks = gtk::RadioButtonBuilder::new()
        .label("All checks")
        .active(true)
        .parent(&top_controls)
        .build();

    let err_checks = gtk::RadioButtonBuilder::new()
        .label("Err checks")
        .active(false)
        .parent(&top_controls)
        .build();
    err_checks.join_group(Some(&all_checks));

    let scrollable = gtk::ScrolledWindowBuilder::new()
        .parent(&window_box)
        .vexpand(true)
        .build();

    let metrics_box = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .spacing(4)
        .parent(&scrollable)
        .build();

    let metrics = metrics(config, &metrics_box, &gdk_window, &ls, &ms, &remotes, &sched);

    window.show_all();
    for mui in metrics.values() {
        mui.graph.hide();
    }

    let ui = Rc::new(Ui {
        metrics,
        summary_label,
        thread: glib::MainContext::ref_thread_default(),
        display_checks: Cell::new(DisplayChecks::All),
    });

    // Connect handlers

    connect_log_updates(&ui, &ls);
    connect_metric_updates(&ui, &ms);

    let uic = ui.clone();
    let msc = ms.clone();
    all_checks.connect_toggled(move |_rad| on_visible_radio_click(DisplayChecks::All, &uic, &msc));

    let uic = ui.clone();
    let msc = ms.clone();
    err_checks.connect_toggled(move |_rad| on_visible_radio_click(DisplayChecks::Err, &uic, &msc));
}

fn on_visible_radio_click(checks: DisplayChecks, ui: &Rc<Ui>, ms: &Arc<Mutex<MetricStore>>) {
    ui.display_checks.set(checks);
    for (key, mui) in ui.metrics.iter() {
        match checks {
            DisplayChecks::All => mui.metric_box.show(),
            DisplayChecks::Err => {
                match ms.lock().unwrap().query_one(key) {
                    Some(metric_store::Metric {
                        latest: Some(DataPoint {ok: OkErr::Err, ..}),
                        ..
                    }) => mui.metric_box.show(),
                    _ => mui.metric_box.hide(),
                }
            },
        }
    }
}

fn update_summary_label(ui: &Ui, ms: &Arc<Mutex<MetricStore>>) {
    let counts = ms.lock().unwrap().count_ok();
    ui.summary_label.set_markup(
        &format!(
            concat!("{} <span fgcolor='{}'>Ok</span> and ",
                    "{} <span fgcolor='{}'>Err</span> check(s)"),
            counts.ok, OK_COLOR,
            counts.err, ERR_COLOR));
}

fn metrics(
    config: &config::GtkClient,
    metrics_box: &gtk::Box,
    gdk_window: &gdk::Window,
    ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>,
    remotes: &Arc<remote::Remotes>,
    sched: &Arc<Mutex<Scheduler>>
) -> BTreeMap<MetricKey, Rc<MetricUi>>
{
    let mut metrics = BTreeMap::<MetricKey, Rc<MetricUi>>::new();

    for config in config.shell_checks.iter() {
        let key = MetricKey { name: config.name.clone(), host: Host::Local };
        let metric_ui = ui_for_metric(
            metrics_box, &gdk_window, &key,
            &ls, &ms, &remotes, &sched);
        metrics.insert(key, metric_ui);
    }

    for config in config.shell_metrics.iter() {
        let key = MetricKey { name: config.name.clone(), host: Host::Local };
        let metric_ui = ui_for_metric(
            metrics_box, &gdk_window, &key,
            &ls, &ms, &remotes, &sched);
        metrics.insert(key, metric_ui);
    }

    for config in config.remote_checks.iter() {
        let key = config.to_metric_key();
        let metric_ui = ui_for_metric(
            metrics_box, &gdk_window, &key,
            &ls, &ms, &remotes, &sched);
        metrics.insert(key, metric_ui);
    }

    for config in config.remote_syncs.iter() {
        let metrics_key = config.metrics_sync_key();
        let metrics_metric_ui = ui_for_metric(
            metrics_box, &gdk_window, &metrics_key,
            &ls, &ms, &remotes, &sched);
        metrics.insert(metrics_key, metrics_metric_ui);

        let logs_key = config.logs_sync_key();
        let logs_metric_ui = ui_for_metric(
            metrics_box, &gdk_window, &logs_key,
            &ls, &ms, &remotes, &sched);
        metrics.insert(logs_key, logs_metric_ui);
    }

    metrics
}

fn connect_log_updates(ui: &Rc<Ui>, ls: &Arc<Mutex<LogStore>>) {
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
    rx.attach(Some(&ui.thread), move |log| {
        let metric = uic.metrics.get(&log.key);
        if let Some(metric) = metric {
            metric.label_status.set_tooltip_text(Some(&log.log));
        }
        glib::source::Continue(true)
    });
}

/// Subscribe to metric updates and send them over a channel to the UI thread,
/// which can then update the UI with metric status.
fn connect_metric_updates(ui: &Rc<Ui>, ms: &Arc<Mutex<MetricStore>>) {
    let uic = ui.clone();
    let msc = ms.clone();

    let (tx, rx) = glib::MainContext::sync_channel(glib::source::Priority::default(), 50);

    msc.lock().unwrap().update_signal_for_all().connect(move |metric| {
        if tx.send(metric).is_err() {
            error!("MetricStore UI channel send error");
        }
        Continue::Continue
    });
    let thread = uic.thread.clone();
    rx.attach(Some(&thread), move |metric| {
        trace!("gui thread got metric = {:?}", &metric);
        if let Some(dp) = &metric.latest {
            let ui_metric = uic.metrics.get(&metric.key);
            if let Some(ui_metric) = ui_metric {
                let fgcolor = match dp.ok {
                    OkErr::Ok => OK_COLOR,
                    OkErr::Err => ERR_COLOR,
                };
                ui_metric.label_status.set_markup(
                    &format!("<span fgcolor='{}'>{}</span>", fgcolor, dp.value_string()));
                if uic.display_checks.get() == DisplayChecks::Err {
                    match dp.ok {
                        OkErr::Ok => ui_metric.metric_box.hide(),
                        OkErr::Err => ui_metric.metric_box.show(),
                    }
                }
            } else {
                warn!("Couldn't find ui_metric for key '{}'", &metric.key.display_name());
            }
        }
        update_summary_label(&uic, &msc);
        glib::source::Continue(true)
    });
}

fn ui_for_metric<C>(
    container: &C,
    gdk_window: &gdk::Window,
    metric_key: &MetricKey,
    _ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>,
    remotes: &Arc<remote::Remotes>,
    sched: &Arc<Mutex<Scheduler>>,
) -> Rc<MetricUi>
    where C: IsA<gtk::Container> + IsA<gtk::Widget>
{
    let metric_box = gtk::BoxBuilder::new()
        .parent(container)
        .orientation(gtk::Orientation::Vertical)
        .build();

    let label_box = gtk::BoxBuilder::new()
        .parent(&metric_box)
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let label_status = gtk::LabelBuilder::new()
        .parent(&label_box)
        .build();
    label_status.set_markup(&format!("<span fgcolor='{}'>?</span>", MISSING_COLOR));
    let _label = gtk::LabelBuilder::new()
        .label(&format!(" = {}", metric_key.display_name()))
        .parent(&label_box)
        .build();

    let buttons_box = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Horizontal)
        .parent(&metric_box)
        .spacing(8)
        .build();

    let force_btn = gtk::ButtonBuilder::new()
        .label("Force")
        .parent(&buttons_box)
        .halign(gtk::Align::Start)
        .build();
    let schedc = sched.clone();
    let remotesc = remotes.clone();
    let metric_key_clone = metric_key.to_owned();
    force_btn.connect_clicked(move |_btn| {
        force_check(&metric_key_clone, &remotesc, &schedc)
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
    let graph = rt_graph::GraphWithControls::build_ui(config, &metric_box, &gdk_window);

    let metric_ui = Rc::new(MetricUi {
        label_status,
        graph: graph,
        showing_graph: Cell::new(false),
        show_graph_btn,
        metric_box,
    });

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
                let dp = m.latest.as_ref();
                match dp {
                    Some(DataPoint { ok, val, time }) => {
                        if self.last.is_none() ||
                            (self.last.is_some() && *time != self.last.as_ref().unwrap().time) {
                            self.t += 1;
                            self.last = Some(dp.unwrap().clone());
                            vec![rt_graph::Point {
                                t: self.t, // TODO: Use time.
                                vs: vec![match (ok, val) {
                                    (OkErr::Err, _) => 10000,
                                    (_, MetricValue::None) => 50000,
                                    (_, MetricValue::I64(i)) => *i as u16, // TODO: Handle overflow
                                    (_, MetricValue::F64(_f)) => todo!(),
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
