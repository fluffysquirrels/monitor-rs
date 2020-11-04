use crate::{
    DataPoint,
    MetricKey,
    Signal,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Metric {
    latest: Option<DataPoint>,
    key: MetricKey,
}

struct MetricState {
    latest: Option<DataPoint>,
    key: MetricKey,
    update_signal: Signal<Metric>,
}

pub struct MetricStore {
    metrics: BTreeMap<MetricKey, MetricState>,
    update_signal_for_all: Signal<Metric>,
}

impl Metric {
    pub fn latest(&self) -> Option<&DataPoint> {
        self.latest.as_ref()
    }

    pub fn key(&self) -> &MetricKey {
        &self.key
    }
}

impl MetricStore {
    pub fn new() -> MetricStore {
        MetricStore {
            metrics: BTreeMap::new(),
            update_signal_for_all: Signal::new(),
        }
    }

    pub fn update(&mut self, key: &MetricKey, point: DataPoint) {
        let metric = {
            let mut metric_state = self.get_or_insert_metric(key);
            metric_state.latest = Some(point);
            let metric = metric_state.as_metric();
            metric_state.update_signal.raise(metric.clone());
            metric
        };
        self.update_signal_for_all.raise(metric);
    }

    /// Returns a signal to listen to updates for all metrics.
    pub fn update_signal_for_all(&mut self) -> &mut Signal<Metric> {
        &mut self.update_signal_for_all
    }

    /// Returns a signal to listen to updates for one metric
    pub fn update_signal_for_one(&mut self, key: &MetricKey) -> &mut Signal<Metric> {
        let metric = self.get_or_insert_metric(key);
        &mut metric.update_signal
    }

    pub fn query_one(&self, key: &MetricKey) -> Option<Metric> {
        self.metrics.get(key)
                    .map(|ms| ms.as_metric())
    }

    pub fn query_all(&self) -> Vec<Metric> {
        self.metrics.values()
                    .map(|ms| ms.as_metric())
                    .collect::<Vec<Metric>>()
    }

    fn get_or_insert_metric(&mut self, key: &MetricKey) -> &mut MetricState {
        self.metrics.entry(key.clone())
                    .or_insert(MetricState::with_key(key))
    }
}

impl MetricState {
    fn with_key(key: &MetricKey) -> MetricState {
        MetricState {
            key: key.clone(),
            latest: None,
            update_signal: Signal::new(),
        }
    }

    fn as_metric(&self) -> Metric {
        Metric {
            latest: self.latest.clone(),
            key: self.key.clone(),
        }
    }
}
