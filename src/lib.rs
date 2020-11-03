#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};

mod log_store;
mod metric_store;
mod notifier;
pub mod scheduler;
mod signal;

pub use crate::{
    log_store::{Log, LogStore},
    metric_store::MetricStore,
    notifier::Notifier,
    scheduler::Scheduler,
    signal::Signal,
};
use process_control::{ChildExt, Timeout};

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
                            ms.clone(),
                            sched.clone(),
                            ls.clone());
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
                             ms.clone(),
                             sched.clone(),
                             ls.clone());
        connect_metric_to_notifier(&smc, &ms, &n);
    }
}

// TODO: Ugly duplication between this and add_shell_metric_job.
pub fn add_shell_check_job(
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
pub fn add_shell_metric_job(
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
                         nc.lock().unwrap().update_metric(m.name(), *ok);
                     }
                 });
}
