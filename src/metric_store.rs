use crate::{
    chrono_datetime_from_protobuf,
    chrono_datetime_to_protobuf,
    collector,
    DataPoint,
    Host,
    MetricKey,
    MetricValue,
    OkErr,
    RemoteHost,
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
        trace!("update key=`{}', point={:?}", key.display_name(), point);
        let mut metric_state =
            self.metrics.entry(key.clone())
                .or_insert_with(|| MetricState::with_key(key));
        let old_value = metric_state.latest.clone();
        metric_state.latest = Some(point);
        if old_value != metric_state.latest {
            trace!("raising signals for key=`{}'", key.display_name());
            let metric = metric_state.as_metric();
            metric_state.update_signal.raise(metric.clone());
            self.update_signal_for_all.raise(metric);
        }
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
                    .or_insert_with(|| MetricState::with_key(key))
    }
}

impl Default for MetricStore {
    fn default() -> MetricStore {
        MetricStore::new()
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

impl Metric {
    pub fn from_protobuf(metric: &collector::Metric) -> Result<Metric, String> {
        let rv = Metric {
            latest: match &metric.latest {
                None => None,
                Some(dp) => Some(DataPoint {
                    time: chrono_datetime_from_protobuf(
                              dp.time.as_ref()
                                     .ok_or_else(|| "protobuf DataPoint missing .time"
                                                    .to_owned())?)?,
                    val: match dp.value.as_ref()
                                 .ok_or_else(|| "protobuf DataPoint missing .value".to_owned())? {
                        collector::data_point::Value::Ok(true) => MetricValue::OkErr(OkErr::Ok),
                        collector::data_point::Value::Ok(false) => MetricValue::OkErr(OkErr::Err),
                        collector::data_point::Value::I64(x) => MetricValue::I64(*x),
                        collector::data_point::Value::F64(x) => MetricValue::F64(*x),
                    },
                }),
            },
            key: MetricKey {
                name: metric.name.clone(),
                host: Host::Remote(RemoteHost {
                    name: metric.from_host.as_ref()
                                .ok_or_else(|| "protobuf Metric missing .from_host".to_owned())?
                                .name.clone(),
                })
            },
        };

        if rv.key.name == "" {
            return Err("protobuf Metric empty .key.name".to_owned());
        }
        match rv.key.host {
            Host::Remote(RemoteHost { name }) if name == "" => {
                return Err("protobuf Metric empty .key.host.as_RemoteHost.name".to_owned());
            }
            _ => (),
        }

        Ok(rv)
    }

    pub fn as_protobuf(&self) -> Result<collector::Metric, String> {
        Ok(collector::Metric {
            name: self.key.name.clone(),
            from_host: match &self.key.host {
                Host::Local => Some(collector::Host{
                    // TODO: This should probably not be hidden in here.
                    name: hostname::get().unwrap().into_string().unwrap()
                }),
                Host::Remote(RemoteHost { name, }) => Some(collector::Host {
                    name: name.clone(),
                }),
            },
            latest: match self.latest.as_ref() {
                None => None,
                Some(dp) => Some(collector::DataPoint {
                    time: Some(chrono_datetime_to_protobuf(&dp.time)?),
                    value: Some(match dp.val {
                        MetricValue::OkErr(OkErr::Ok) =>
                            collector::data_point::Value::Ok(true),
                        MetricValue::OkErr(OkErr::Err) =>
                            collector::data_point::Value::Ok(false),
                        MetricValue::I64(x) =>
                            collector::data_point::Value::I64(x),
                        MetricValue::F64(x) =>
                            collector::data_point::Value::F64(x),
                    }),
                }),
            },
        })
    }
}
