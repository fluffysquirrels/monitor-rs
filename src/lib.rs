#[macro_use]
extern crate log;

mod log_store;
mod metric_store;
mod notifier;
pub mod scheduler;
mod signal;

// pub use to fix compiler dead_code warnings.
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
