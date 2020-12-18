use crate::collector;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OkErr {
    Ok,
    Err,
}

impl<T, E> From<Result<T, E>> for OkErr {
    fn from(res: Result<T, E>) -> OkErr {
        match res {
            Ok(_) => OkErr::Ok,
            Err(_) => OkErr::Err,
        }
    }
}

impl std::fmt::Display for OkErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            OkErr::Ok => "Ok",
            OkErr::Err => "Err",
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MetricKey {
    pub name: String,
    pub host: Host,
}

impl MetricKey {
    pub fn display_name(&self) -> String {
        format!("{}@{}",
                self.name,
                match &self.host {
                    Host::Local => "local",
                    Host::Remote(RemoteHost { name: hostname }) => &hostname,
                })
    }

    pub fn to_protobuf(&self) -> Result<collector::MetricKey, String> {
        Ok(collector::MetricKey {
            name: self.name.clone(),
            from_host: Some(collector::Host {
                name: match &self.host {
                    Host::Local =>
                        return Err("Missing hostname in MetricKey::to_protobuf".to_owned()),
                    Host::Remote(RemoteHost { name, }) => name.clone(),
                },
            }),
        })
    }

    pub fn from_protobuf(p: &collector::MetricKey) -> Result<MetricKey, String> {
        let rv = MetricKey {
            name: p.name.clone(),
            host: Host::Remote(RemoteHost {
                name: p.from_host.as_ref()
                       .ok_or_else(|| "protobuf MetricKey missing .from_host".to_owned())?
                       .name.clone(),
            })
        };

        if rv.name == "" {
            return Err("protobuf MetricKey empty .name".to_owned());
        }
        match rv.host {
            Host::Remote(RemoteHost { name }) if name == "" => {
                return Err("protobuf MetricKey empty .host.as_RemoteHost.name".to_owned());
            }
            _ => (),
        }

        Ok(rv)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Host {
    Local,
    Remote(RemoteHost),
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct RemoteHost {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MetricValue {
    None,
    I64(i64),
    F64(f64),
}

impl MetricValue {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            MetricValue::I64(i) => Some(*i),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataPoint {
    pub time: chrono::DateTime<chrono::Utc>,
    pub val: MetricValue,
    pub ok: OkErr,
}

impl DataPoint {
    pub fn value_string(&self) -> String {
        match self.val {
            MetricValue::None => self.ok.to_string(),
            MetricValue::I64(x) => x.to_string(),
            MetricValue::F64(x) => x.to_string(),
        }
    }
}
