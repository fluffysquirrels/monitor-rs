use crate::{
    DataPoint,
    Signal,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Metric {
    latest: Option<DataPoint>,
    name: String,
}

pub struct MetricStore {
    metrics: BTreeMap<String, Metric>,
    update_signal: Signal<Metric>,
}

impl Metric {
    pub fn latest(&self) -> Option<&DataPoint> {
        self.latest.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl MetricStore {
    pub fn new() -> MetricStore {
        MetricStore {
            metrics: BTreeMap::new(),
            update_signal: Signal::new(),
        }
    }

    pub fn update(&mut self, metric_name: &str, point: DataPoint) {
        let metric: &mut Metric =
            self.metrics.entry(String::from(metric_name))
                        .or_insert(Metric {
                            name: metric_name.to_owned(),
                            latest: None,
                        });
        metric.latest = Some(point);
        self.update_signal.raise(metric.clone());
    }

    pub fn update_signal(&mut self) -> &mut Signal<Metric> {
        &mut self.update_signal
    }

    pub fn query_one(&self, metric_name: &str) -> Option<Metric> {
        self.metrics.get(metric_name)
                    .cloned()
    }

    pub fn query_all(&self) -> Vec<Metric> {
        self.metrics.values().cloned().collect()
    }
}
