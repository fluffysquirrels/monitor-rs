use crate::OkErr;

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
