use crate::Signal;
use std::collections::BTreeMap;

pub struct LogStore {
    logs: BTreeMap<String, Log>,
    update_signal: Signal<Log>,
}

#[derive(Clone, Debug)]
pub struct Log {
    pub start: chrono::DateTime<chrono::Utc>,
    pub finish: chrono::DateTime<chrono::Utc>,
    pub duration: std::time::Duration,
    pub log: String,
    pub name: String,
}

impl LogStore {
    pub fn new() -> LogStore {
        LogStore {
            logs: BTreeMap::new(),
            update_signal: Signal::new(),
        }
    }

    pub fn update(&mut self, log: Log) {
        let log2 = log.clone();
        self.logs.insert(log.name.clone(), log);
        self.update_signal.raise(log2);
    }

    pub fn get(&self, name: &str) -> Option<&Log> {
        self.logs.get(name)
    }

    pub fn update_signal(&mut self) -> &mut Signal<Log> {
        &mut self.update_signal
    }
}
