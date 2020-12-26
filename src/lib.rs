#[macro_use]
extern crate log;

pub mod collector_pool;
pub mod config;
mod core_types;
pub mod log_store;
pub mod metric_store;
mod notifier;
pub mod remote;
pub mod scheduler;
mod signal;
mod time_utils;

pub mod collector {
    //! Protobuf types for the Collector service

    tonic::include_proto!("collector");
}

pub mod monitor_core_types {
    //! Core Protobuf types

    tonic::include_proto!("monitor_core_types");
}

pub use crate::{
    config::MetricCheck,
    core_types::{DataPoint, Host, MetricKey, MetricValue, OkErr, RemoteHost},
    log_store::{Log, LogStore},
    metric_store::MetricStore,
    notifier::Notifier,
    scheduler::Scheduler,
    signal::{Continue, Signal},
};

use process_control::{ChildExt, Timeout};
use std::sync::{Arc, Mutex};

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

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
        log,
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

pub fn create_shell_checks(
    check_configs: &[config::ShellCheck],
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
    metric_configs: &[config::ShellMetric],
    ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>,
    n: Option<&Arc<Mutex<Notifier>>>,
    sched: &Arc<Mutex<Scheduler>>)
{
    for smc in metric_configs.iter() {
        add_shell_metric_job(smc,
                             ls.clone(),
                             ms.clone(),
                             sched.clone());
        if let Some(n) = n {
            connect_metric_to_notifier(&smc, &ms, &n);
        }
    }
}

// TODO: Ugly duplication between this and add_shell_metric_job.
pub fn add_shell_check_job(
    config: &config::ShellCheck,
    ls: Arc<Mutex<LogStore>>,
    ms: Arc<Mutex<MetricStore>>,
    sched: Arc<Mutex<Scheduler>>,
) {
    let cmd = config.cmd.to_owned();
    let name = config.name.to_owned();
    let j = scheduler::JobDefinition {
        interval: config.interval.as_chrono_duration(),
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
                        val: MetricValue::None,
                        ok: res.ok,
                    });
                    ls.lock().unwrap().update(Log {
                        start: res.start_time,
                        finish: res.finish_time,
                        duration: res.duration,
                        log: res.log,
                        key,
                    });
                },
                Err(e) => {
                    error!("run_shell cmd=`{}' error={}", &cmd, e);
                    ms.lock().unwrap().update(&key, DataPoint {
                        time: chrono::Utc::now(),
                        val: MetricValue::None,
                        ok: OkErr::Err,
                    });
                    ls.lock().unwrap().update(Log {
                        start, finish,
                        // Susceptible to shifts in time, e.g. leap seconds.
                        duration: std::time::Duration::from_millis(
                            (finish - start).num_milliseconds() as u64),
                        log: format!("Error={}", e),
                        key,
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
    config: &config::ShellMetric,
    ls: Arc<Mutex<LogStore>>,
    ms: Arc<Mutex<MetricStore>>,
    sched: Arc<Mutex<Scheduler>>,
) {
    let cmd = config.cmd.to_owned();
    let name = config.name.to_owned();
    let check = config.check.clone();
    let j = scheduler::JobDefinition {
        interval: config.interval.as_chrono_duration(),
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
                                val: MetricValue::I64(i),
                                ok: check.is_value_ok(i),
                            }),
                        Err(e) => {
                            error!("Error parsing run_shell stdout: {}", e);
                            ms.lock().unwrap().update(&key, DataPoint {
                                time: chrono::Utc::now(),
                                val: MetricValue::None,
                                ok: OkErr::Err,
                            });
                        }
                    };
                    ls.lock().unwrap().update(Log {
                        start: res.start_time,
                        finish: res.finish_time,
                        duration: res.duration,
                        log: res.log,
                        key,
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
                        key,
                    });
                }
            }
        })),
    };
    sched.lock().unwrap()
         .add(j);
}

pub fn connect_metric_to_notifier(
    smc: &config::ShellMetric,
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
                    let dp = m.latest.as_ref().unwrap();
                    match dp.ok {
                        OkErr::Err =>
                            nc.lock().unwrap().update_metric(&m.key.display_name(), OkErr::Err),
                        OkErr::Ok => {
                            let val: i64 = m.latest.as_ref().unwrap()
                                .val.as_i64().expect("Only int checks so far");
                            let ok = check.is_value_ok(val);
                            debug!("Metric check name=`{}' value={:?} check={:?} ok={:?}",
                                   m.key.display_name(), val, check, ok);
                            nc.lock().unwrap().update_metric(&m.key.display_name(), ok);
                        },
                    }
                    Continue::Continue
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
                     if let Some(DataPoint { ok, .. })
                             = m.latest.as_ref() {
                         nc.lock().unwrap().update_metric(&m.key.display_name(), *ok);
                     }
                     Continue::Continue
                 });
}

pub fn force_check(mk: &MetricKey, remotes: &Arc<remote::Remotes>, sched: &Arc<Mutex<Scheduler>>) {
    match &mk.host {
        Host::Local => {
            if let Err(e) = sched.lock().unwrap().force_run(&mk.name) {
                error!("Error on force run: {}", e);
            }
        },
        Host::Remote(_) => {
            remote::force_check_remote(mk, remotes);
        }
    }
}
