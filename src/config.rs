use crate::{BoxError, Host, MetricKey, OkErr, RemoteHost};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Collector {
    pub host_name: String,
    pub listen_addr: String,
    pub shell_checks: Vec<ShellCheck>,
    pub shell_metrics: Vec<ShellMetric>,
    pub server_tls_identity: Option<TlsIdentity>,
    pub client_tls_ca_cert: Option<TlsCertificate>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GtkClient {
    pub shell_checks: Vec<ShellCheck>,
    pub shell_metrics: Vec<ShellMetric>,
    pub remote_syncs: Vec<RemoteSync>,
    pub remote_checks: Vec<RemoteCheck>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TlsIdentity {
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TlsCertificate {
    pub cert_path: String,
}

impl TlsIdentity {
    pub fn load(&self) -> Result<tonic::transport::Identity, BoxError> {
        Ok(tonic::transport::Identity::from_pem(
              std::fs::read(&self.cert_path)?,
              std::fs::read(&self.key_path)?))
    }
}

impl TlsCertificate {
    pub fn load(&self) -> Result<tonic::transport::Certificate, BoxError> {
        Ok(tonic::transport::Certificate::from_pem(
              std::fs::read(&self.cert_path)?))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShellCheck {
    pub name: String,
    pub cmd: String,
    pub interval: Duration,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSync {
    pub url: String,
    pub server_ca: TlsCertificate,
    pub client_identity: TlsIdentity,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteCheck {
    pub name: String,
    pub host_name: String,
}

impl RemoteCheck {
    pub fn to_metric_key(&self) -> MetricKey {
        MetricKey {
            name: self.name.clone(),
            host: Host::Remote(RemoteHost {
                name: self.host_name.clone(),
            }),
        }
    }
}

impl RemoteSync {
    pub fn metrics_sync_key(&self) -> MetricKey {
        MetricKey {
            name: format!("sync.{}.metrics", self.url),
            host: Host::Local,
        }
    }

    pub fn logs_sync_key(&self) -> MetricKey {
        MetricKey {
            name: format!("sync.{}.logs", self.url),
            host: Host::Local,
        }
    }
}
