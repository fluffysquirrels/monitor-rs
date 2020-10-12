use crate::{
    DataPoint,
    Signal,
};

#[derive(Clone, Debug)]
pub struct Metric {
    latest: Option<DataPoint>,
    name: String,
}

pub struct MetricStore {
    metrics: Vec<Metric>,
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
            metrics: vec![],
            update_signal: Signal::new(),
        }
    }

    pub fn update(&mut self, metric_name: &str, point: DataPoint) {
        let existing = self.metrics.iter_mut().find(|m| m.name == metric_name);
        let metric: &mut Metric = match existing {
            Some(e) => e,
            None => {
                self.metrics.push(Metric {
                    name: metric_name.to_owned(),
                    latest: None,
                });
                self.metrics.last_mut().unwrap()
            }
        };
        metric.latest = Some(point);
        self.update_signal.raise(metric.clone());
    }

    pub fn update_signal(&mut self) -> &mut Signal<Metric> {
        &mut self.update_signal
    }

    pub fn query_one(&self, metric_name: &str) -> Option<Metric> {
        self.metrics.iter()
                    .find(|m| m.name == metric_name)
                    .cloned()
    }

    pub fn query_all(&self) -> Vec<Metric> {
        self.metrics.iter().cloned().collect()
    }
}
