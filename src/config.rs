use crate::OkErr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Collector {
    pub shell_checks: Vec<ShellCheck>,
    pub shell_metrics: Vec<ShellMetric>,
    pub server_tls_identity: Option<TlsIdentity>,
    pub client_tls_ca_cert: Option<TlsCertificate>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Client {
    pub shell_checks: Vec<ShellCheck>,
    pub shell_metrics: Vec<ShellMetric>,
    pub remote_syncs: Vec<RemoteSync>,
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
    pub fn load(&self) -> Result<tonic::transport::Identity, Box<dyn std::error::Error>> {
        Ok(tonic::transport::Identity::from_pem(
              std::fs::read(&self.cert_path)?,
              std::fs::read(&self.key_path)?))
    }
}

impl TlsCertificate {
    pub fn load(&self) -> Result<tonic::transport::Certificate, Box<dyn std::error::Error>> {
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
