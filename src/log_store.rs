use crate::{
    chrono_datetime_from_protobuf,
    chrono_datetime_to_protobuf,
    collector,
    MetricKey,
    Signal,
    std_time_duration_from_protobuf,
    std_time_duration_to_protobuf,
};
use std::collections::BTreeMap;

pub struct LogStore {
    logs: BTreeMap<MetricKey, Log>,
    update_signal: Signal<Log>,
}

#[derive(Clone, Debug)]
pub struct Log {
    pub start: chrono::DateTime<chrono::Utc>,
    pub finish: chrono::DateTime<chrono::Utc>,
    pub duration: std::time::Duration,
    pub log: String,
    pub key: MetricKey,
}

impl LogStore {
    pub fn new() -> LogStore {
        LogStore {
            logs: BTreeMap::new(),
            update_signal: Signal::new(),
        }
    }

    pub fn update(&mut self, log: Log) {
        self.logs.insert(log.key.clone(), log.clone());
        self.update_signal.raise(log);
    }

    pub fn get(&self, key: &MetricKey) -> Option<&Log> {
        self.logs.get(key)
    }

    pub fn query_all(&self) -> impl Iterator<Item = &Log> {
        self.logs.values()
    }

    pub fn update_signal(&mut self) -> &mut Signal<Log> {
        &mut self.update_signal
    }
}

impl Default for LogStore {
    fn default() -> LogStore {
        LogStore::new()
    }
}

impl Log {
    pub fn to_protobuf(&self) -> Result<collector::Log, String> {
        Ok(collector::Log {
            start: Some(chrono_datetime_to_protobuf(&self.start)?),
            finish: Some(chrono_datetime_to_protobuf(&self.finish)?),
            duration: Some(std_time_duration_to_protobuf(&self.duration)?),
            log: self.log.clone(),
            key: Some(self.key.to_protobuf()?),
        })
    }

    pub fn from_protobuf(l: &collector::Log) -> Result<Log, String> {
        let rv = Log {
            start: chrono_datetime_from_protobuf(
                       l.start.as_ref()
                        .ok_or_else(|| "protobuf Log missing .start".to_owned())?)?,
            finish: chrono_datetime_from_protobuf(
                       l.finish.as_ref()
                        .ok_or_else(|| "protobuf Log missing .finish".to_owned())?)?,
            duration: std_time_duration_from_protobuf(
                          l.duration.as_ref()
                           .ok_or_else(|| "protobuf Log missing .duration".to_owned())?)?,
            log: l.log.clone(),
            key: MetricKey::from_protobuf(
                     l.key.as_ref()
                      .ok_or_else(|| "protobuf Log missing .key".to_owned())?)?,
        };

        Ok(rv)
    }
}
