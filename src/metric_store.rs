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

struct MetricState {
    latest: Option<DataPoint>,
    name: String,
    update_signal: Signal<Metric>,
}

pub struct MetricStore {
    metrics: BTreeMap<String, MetricState>,
    update_signal_for_all: Signal<Metric>,
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
            update_signal_for_all: Signal::new(),
        }
    }

    pub fn update(&mut self, metric_name: &str, point: DataPoint) {
        let metric = {
            let mut metric_state = self.get_or_insert_metric(metric_name);
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
    pub fn update_signal_for_one(&mut self, metric_name: &str) -> &mut Signal<Metric> {
        let metric = self.get_or_insert_metric(metric_name);
        &mut metric.update_signal
    }

    pub fn query_one(&self, metric_name: &str) -> Option<Metric> {
        self.metrics.get(metric_name)
                    .map(|ms| ms.as_metric())
    }

    pub fn query_all(&self) -> Vec<Metric> {
        self.metrics.values()
                    .map(|ms| ms.as_metric())
                    .collect::<Vec<Metric>>()
    }

    fn get_or_insert_metric(&mut self, metric_name: &str) -> &mut MetricState {
        self.metrics.entry(String::from(metric_name))
                    .or_insert(MetricState::named(metric_name))
    }
}

impl MetricState {
    fn named(name: &str) -> MetricState {
        MetricState {
            name: name.to_owned(),
            latest: None,
            update_signal: Signal::new(),
        }
    }

    fn as_metric(&self) -> Metric {
        Metric {
            latest: self.latest.clone(),
            name: self.name.clone(),
        }
    }
}
