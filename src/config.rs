use crate::OkErr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Collector {
    pub shell_checks: Vec<ShellCheck>,
    pub shell_metrics: Vec<ShellMetric>,
}

// TODO: Delete `Config` suffix?
#[derive(Debug, Deserialize, Serialize)]
pub struct ShellCheck {
    pub name: String,
    pub cmd: String,
    pub interval: Duration,
}

// TODO: Delete `Config` suffix?
#[derive(Debug, Deserialize, Serialize)]
pub struct ShellMetric {
    pub name: String,
    pub cmd: String,
    pub interval: Duration,
    pub check: MetricCheck,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Duration {
    Seconds(u32),
    Minutes(u32),
}

impl Duration {
    pub fn as_chrono_duration(&self) -> chrono::Duration {
        match self {
            Duration::Seconds(x) => chrono::Duration::seconds(*x as i64),
            Duration::Minutes(x) => chrono::Duration::minutes(*x as i64),
        }
    }
}
