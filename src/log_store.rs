use crate::{
    MetricKey,
    Signal,
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

    pub fn update_signal(&mut self) -> &mut Signal<Log> {
        &mut self.update_signal
    }
}
