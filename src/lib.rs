#[macro_use]
extern crate log;

mod log_store;
mod metric_store;
mod notifier;
pub mod scheduler;
mod signal;

pub mod collector {
    tonic::include_proto!("collector");
}

pub use crate::{
    log_store::{Log, LogStore},
    metric_store::MetricStore,
    notifier::Notifier,
    scheduler::Scheduler,
    signal::Signal,
};

use chrono::TimeZone;
use process_control::{ChildExt, Timeout};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OkErr {
    Ok,
    Err,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MetricKey {
    pub name: String,
    pub host: Host,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Host {
    Local,
    Remote(RemoteHost),
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct RemoteHost {
    pub name: String,
}

impl MetricKey {
    pub fn display_name(&self) -> String {
        format!("{}@{}",
                self.name,
                match &self.host {
                    Host::Local => "local",
                    Host::Remote(RemoteHost { name: hostname }) => &hostname,
                })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MetricValue {
    OkErr(OkErr),
    I64(i64),
    F64(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataPoint {
    pub time: chrono::DateTime<chrono::Utc>,
    pub val: MetricValue,
}

pub struct ShellCheckConfig {
    pub name: String,
    pub cmd: String,
    pub interval: chrono::Duration,
}

pub struct ShellMetricConfig {
    pub name: String,
    pub cmd: String,
    pub interval: chrono::Duration,
    pub check: MetricCheck,
}

#[derive(Clone, Debug)]
pub enum MetricCheck {
    None,
    Min(i64),
    Max(i64),
}

impl MetricCheck {
    pub fn is_value_ok(&self, value: i64) -> OkErr {
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

#[derive(Clone, Debug)]
pub struct RunShellResult {
    pub log: String,
    pub ok: OkErr,
    pub exit_code: Option<i64>,
    pub stdout: String,
    pub stderr: String,
    pub duration: std::time::Duration,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub finish_time: chrono::DateTime<chrono::Utc>,
}

pub fn run_shell(mut cmd: std::process::Command) -> Result<RunShellResult, std::io::Error> {
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
                          start_utc.to_rfc3339_opts(chrono::SecondsFormat::Micros, true)));
    log.push_str(&format!("finish: {}",
                          finish_utc.to_rfc3339_opts(chrono::SecondsFormat::Micros, true)));

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
            MetricValue::F64(float) => f.write_str(&format!("{}", float)),
        }
    }
}

impl MetricValue {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            MetricValue::I64(i) => Some(*i),
            _ => None,
        }
    }
}

pub fn create_shell_checks(
    check_configs: &[ShellCheckConfig],
    ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>,
    sched: &Arc<Mutex<Scheduler>>)
{
    for scc in check_configs.iter() {
        add_shell_check_job(scc,
                            ls.clone(),
                            ms.clone(),
                            sched.clone());
    }
}

pub fn create_shell_metrics(
    metric_configs: &[ShellMetricConfig],
    ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>,
    n: &Arc<Mutex<Notifier>>,
    sched: &Arc<Mutex<Scheduler>>)
{
    for smc in metric_configs.iter() {
        add_shell_metric_job(smc,
                             ls.clone(),
                             ms.clone(),
                             sched.clone());
        connect_metric_to_notifier(&smc, &ms, &n);
    }
}

// TODO: Ugly duplication between this and add_shell_metric_job.
pub fn add_shell_check_job(
    config: &ShellCheckConfig,
    ls: Arc<Mutex<LogStore>>,
    ms: Arc<Mutex<MetricStore>>,
    sched: Arc<Mutex<Scheduler>>,
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

            let key = MetricKey {
                name: name.clone(),
                host: Host::Local,
            };
            match res {
                Ok(res) => {
                    debug!("run_shell cmd=`{}' log=\n{}", &cmd, res.log);
                    ms.lock().unwrap().update(&key, DataPoint {
                        time: chrono::Utc::now(),
                        val: MetricValue::OkErr(res.ok)
                    });
                    ls.lock().unwrap().update(Log {
                        start: res.start_time,
                        finish: res.finish_time,
                        duration: res.duration,
                        log: res.log,
                        key: key.clone(),
                    });
                },
                Err(e) => {
                    error!("run_shell cmd=`{}' error={}", &cmd, e);
                    ms.lock().unwrap().update(&key, DataPoint {
                        time: chrono::Utc::now(),
                        val: MetricValue::OkErr(OkErr::Err),
                    });
                    ls.lock().unwrap().update(Log {
                        start, finish,
                        // Susceptible to shifts in time, e.g. leap seconds.
                        duration: std::time::Duration::from_millis(
                            (finish - start).num_milliseconds() as u64),
                        log: format!("Error={}", e),
                        key: key.clone(),
                    });
                }
            }
        })),
    };
    sched.lock().unwrap()
         .add(j);
}

// TODO: Ugly duplication between this and add_shell_check_job.
pub fn add_shell_metric_job(
    config: &ShellMetricConfig,
    ls: Arc<Mutex<LogStore>>,
    ms: Arc<Mutex<MetricStore>>,
    sched: Arc<Mutex<Scheduler>>,
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

            let key = MetricKey {
                name: name.clone(),
                host: Host::Local,
            };
            match res {
                Ok(res) => {
                    debug!("run_shell cmd=`{}' log=\n{}", &cmd, res.log);
                    match res.stdout.trim().parse::<i64>() {
                        Ok(i) =>
                            ms.lock().unwrap().update(&key, DataPoint {
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
                        key: key.clone(),
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
                        key: key.clone(),
                    });
                }
            }
        })),
    };
    sched.lock().unwrap()
         .add(j);
}

pub fn connect_metric_to_notifier(
    smc: &ShellMetricConfig,
    ms: &Arc<Mutex<MetricStore>>,
    n: &Arc<Mutex<Notifier>>
) {
    match &smc.check {
        MetricCheck::None => (),
        _ => {
            let nc = n.clone();
            let check = smc.check.clone();
            ms.lock().unwrap()
                .update_signal_for_one(&MetricKey {
                    name: smc.name.clone(),
                    host: Host::Local,
                })
                .connect(move |m| {
                    let val: i64 = m.latest().unwrap()
                        .val.as_i64().expect("Only int checks so far");
                    let ok = check.is_value_ok(val);
                    debug!("Metric check name=`{}' value={} check={:?} ok={:?}",
                           m.key().display_name(), val, check, ok);
                    nc.lock().unwrap().update_metric(&m.key().display_name(), ok);
                });
        },
    };
}

pub fn connect_all_checks_to_notifier(
    ms: &Arc<Mutex<MetricStore>>,
    n: &Arc<Mutex<Notifier>>
) {
    let nc = n.clone();
    ms.lock().unwrap()
        .update_signal_for_all()
        .connect(move |m|
                 {
                     if let Some(DataPoint { val: MetricValue::OkErr(ok),.. }) = m.latest() {
                         nc.lock().unwrap().update_metric(&m.key().display_name(), *ok);
                     }
                 });
}

#[derive(Clone)]
pub struct RemoteSyncConfig {
    // TODO: Use a more specific type here.
    pub url: String,
}

pub fn add_remote_sync_job_polling(
    config: &RemoteSyncConfig,
    ms: &Arc<Mutex<MetricStore>>,
    sched: &Arc<Mutex<Scheduler>>)
{
    // TODO: Cache connection and re-use between invocations.

    let config = config.clone();
    let ms = ms.clone();
    let j = scheduler::JobDefinition {
        interval: chrono::Duration::seconds(5),
        name: format!("remote-sync.{}", &config.url),
        f: Arc::new(Mutex::new(move |_rc| {
            debug!("Remote sync connecting endpoint url: {}", &config.url);
            let endpoint = tonic::transport::Endpoint::from_shared(config.url.clone()).unwrap();
            let config = config.clone();
            let ms = ms.clone();
            let fut = async move {
                let mut client = collector::collector_client::CollectorClient::connect(endpoint)
                    .await?;
                debug!("Remote sync polling `{}'", config.url);
                let req = collector::GetMetricsRequest {};
                let metrics = client.get_metrics(req).await?;
                trace!("Remote sync got metrics");
                trace!("metrics: {:#?}", metrics);
                let metrics = metrics.into_inner().metrics.iter()
                                     .map(|m| metric_store::Metric::from_protobuf(m))
                                     .collect::<Result<Vec<metric_store::Metric>, String>>()?;
                debug!("Remote sync unmarshalled metrics");
                trace!("metrics: {:#?}", metrics);

                { // Scope the lock on ms.
                    let mut msl = ms.lock().unwrap();
                    for m in metrics.iter() {
                        if let Some(latest) = m.latest() {
                            msl.update(&m.key(), latest.clone())
                        }
                    }
                }
                Ok(())
            };
            // Spinning up a tokio runtime takes ~300us, so caching a
            // runtime somewhere might be nice but isn't required.
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            let res: Result<(), Box<dyn std::error::Error>> = rt.block_on(fut);
            if let Err(e) = res {
                error!("Remote sync error = {}", e);
            }
        }))
    };

    sched.lock().unwrap().add(j);
}

pub fn spawn_remote_sync_job_streaming(config: &RemoteSyncConfig, ms: &Arc<Mutex<MetricStore>>) {
    let config = config.clone();
    let ms = ms.clone();
    std::thread::Builder::new()
        .name(format!("remote-sync {}", config.url))
        .spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                debug!("Remote sync connecting endpoint url: {}", &config.url);
                let endpoint =
                    tonic::transport::Endpoint::from_shared(config.url.clone()).unwrap();
                'retry_all: loop {
                    let client =
                        collector::collector_client::CollectorClient::connect(endpoint.clone())
                        .await;
                    let mut client = match client {
                        Err(e) => {
                            error!("remote-sync connect error: {}", e);
                            tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                            continue 'retry_all;
                        },
                        Ok(c) => c,
                    };
                    debug!("Remote sync connected `{}'", config.url);
                    let req = collector::StreamMetricsRequest {};
                    let stream = client.stream_metrics(req).await;
                    let stream = match stream {
                        Err(e) => {
                            error!("remote-sync stream_metrics error: {}", e);
                            tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                            continue 'retry_all;
                        },
                        Ok(s) => s,
                    };
                    let mut stream = stream.into_inner();
                    'next_message: loop {
                        match stream.message().await {
                            Err(e) => {
                                error!("remote-sync message() error: {}", e);
                                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                                continue 'retry_all;
                            }
                            Ok(None) => {
                                error!("remote-sync message() was None, stream should be infinte");
                                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                                continue 'retry_all;
                            }
                            Ok(Some(metric)) => {
                                let metric =
                                    match crate::metric_store::Metric::from_protobuf(&metric) {
                                        Err(e) => {
                                            error!("remote-sync converting protobuf: {}", e);
                                            continue 'next_message;
                                        }
                                        Ok(m) => m,
                                    };
                                debug!("remote-sync got a metric key=`{}'",
                                       metric.key().display_name());
                                if let Some(latest) = metric.latest() {
                                    ms.lock().unwrap()
                                      .update(&metric.key(), latest.clone())
                                }
                            },
                        };
                    }
                }
            });
        }).unwrap();
}

fn chrono_datetime_from_protobuf(t: &collector::Time
) -> Result<chrono::DateTime<chrono::Utc>, String> {
    let epoch = chrono::Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);
    Ok(epoch
       + chrono::Duration::milliseconds(t.epoch_millis)
       + chrono::Duration::nanoseconds(t.nanos as i64)
    )
}

fn chrono_datetime_to_protobuf(t: &chrono::DateTime<chrono::Utc>
) -> Result<collector::Time, String> {
    Ok(collector::Time {
        epoch_millis: t.timestamp_millis(),
        nanos: t.timestamp_subsec_nanos() % 1_000_000,
    })
}
