use crate::{
    BoxError,
    collector,
    collector_pool,
    config,
    DataPoint,
    Host,
    Log,
    LogStore,
    MetricKey,
    MetricStore,
    MetricValue,
    OkErr,
    RemoteHost,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

pub struct Remotes {
    by_host_name: BTreeMap<String, Remote>,
}

impl Remotes {
    pub fn from_configs(configs: &[config::RemoteSync]) -> Result<Remotes, BoxError> {
        let mut by_host_name = BTreeMap::<String, Remote>::new();
        for config in configs.iter() {
            let url = config.url.parse::<tonic::transport::Uri>()?;
            let host = url.host().ok_or("Missing endpoint URL host")?;
            let new_remote = Remote::from_config(config.clone())?;
            if let Some(_old) = by_host_name.insert(host.to_owned(), new_remote) {
                return Err(format!("Duplicate remote host `{}'", host).into());
            }
        }
        Ok(Remotes {
            by_host_name,
        })
    }

    pub fn find_by_host(&self, host: &str) -> Option<&Remote> {
        self.by_host_name.get(host)
    }
}

#[derive(Clone)]
pub struct Remote {
    config: config::RemoteSync,
    pool: Arc<collector_pool::Pool>,
}

impl Remote {
    pub fn from_config(config: config::RemoteSync) -> Result<Remote, BoxError> {
        let endpoint = sync_endpoint(&config)?;
        let pool = Arc::new(collector_pool::Pool::new(&config.url, endpoint));
        Ok(Remote {
            config,
            pool,
        })
    }

    pub fn pool(&self) -> Arc<collector_pool::Pool> {
        self.pool.clone()
    }

    pub fn config(&self) -> &config::RemoteSync {
        &self.config
    }
}

pub fn spawn_sync_jobs(
    remotes: &Remotes,
    ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>
) {
    for remote in remotes.by_host_name.values() {
        spawn_one_sync_jobs(remote, ls, ms);
    }
}

fn spawn_one_sync_jobs(
    remote: &Remote,
    ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>
) {
    let ms = ms.clone();
    let ls = ls.clone();
    let remote = remote.clone();
    std::thread::Builder::new()
        .name(format!("remote-sync {}", &remote.config.url))
        .spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                tokio::join!(run_metric_sync(&remote.config, remote.pool.clone(),
                                             ls.clone(), ms.clone()),
                             run_log_sync(&remote.config, remote.pool.clone(), ls, ms))
            });
        }).unwrap();
}

async fn run_metric_sync(
    config: &config::RemoteSync,
    pool: Arc<collector_pool::Pool>,
    ls: Arc<Mutex<LogStore>>,
    ms: Arc<Mutex<MetricStore>>,
) {
    let log_ctx = format!("metric-sync {}", &config.url);
    'retry_all: loop {
        let mut client = match pool.get().await {
            Err(e) => {
                error!("{} connect error: {}", log_ctx, &e);
                set_sync_metric_log(Err(format!("{}",e)), &config.metrics_sync_key(), &ls, &ms);
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                continue 'retry_all;
            },
            Ok(c) => c,
        };
        if let Err(e) = run_metric_sync_inner(config, client.get(), &ls, &ms).await {
            error!("{} error: {}", log_ctx, &e);
            pool.discard_faulted(client).await;
            set_sync_metric_log(Err(e), &config.metrics_sync_key(), &ls, &ms);
            tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
            continue 'retry_all;
        }
        // Unreachable, because run_metric_sync_inner only returns on error.
    }
    // Unreachable, because we never exit the 'retry_all loop.
}

async fn run_metric_sync_inner(
    config: &config::RemoteSync,
    client: &mut collector_pool::Client,
    ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>,
) -> Result<(), String> {
    let log_ctx = format!("metric-sync {}", &config.url);
    let req = collector::StreamMetricsRequest {};
    let mut stream = client.stream_metrics(req).await
                           .map_err(|e| format!("stream_metrics: {}", e))?
                           .into_inner();
    'next_message: loop {
        match stream.message().await {
            Err(e) => return Err(format!("message(): {}", e)),
            Ok(None) => return Err("message() was None, stream should be infinite".to_owned()),
            Ok(Some(metric)) => {
                let metric = match crate::metric_store::Metric::from_protobuf(&metric) {
                    Err(e) => {
                        error!("{} error converting protobuf: {}", log_ctx, e);
                        continue 'next_message;
                    }
                    Ok(m) => m,
                };
                trace!("{} got key=`{}'", log_ctx, metric.key.display_name());
                if let Some(latest) = metric.latest {
                    ms.lock().unwrap()
                      .update(&metric.key, latest.clone());
                    set_sync_metric_log(Ok(()), &config.metrics_sync_key(), &ls, &ms);
                }
            },
        };
    }
    // Unreachable, because we never break out of the 'next_message loop.
}

async fn run_log_sync(
    config: &config::RemoteSync,
    pool: Arc<collector_pool::Pool>,
    ls: Arc<Mutex<LogStore>>,
    ms: Arc<Mutex<MetricStore>>,
) {
    let log_ctx = format!("log-sync {}", &config.url);
    'retry_all: loop {
        let mut client = match pool.get().await {
            Err(e) => {
                error!("{} connect error: {}", log_ctx, &e);
                set_sync_metric_log(Err(format!("{}",e)), &config.logs_sync_key(), &ls, &ms);
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                continue 'retry_all;
            },
            Ok(c) => c,
        };
        if let Err(e) = run_log_sync_inner(config, client.get(), &ls, &ms).await {
            error!("{} error: {}", log_ctx, &e);
            pool.discard_faulted(client).await;
            set_sync_metric_log(Err(e), &config.logs_sync_key(), &ls, &ms);
            tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
            continue 'retry_all;
        }
        // Unreachable, because run_log_sync_inner only returns on error.
    }
    // Unreachable, because we never exit the 'retry_all loop.
}

async fn run_log_sync_inner(
    config: &config::RemoteSync,
    client: &mut collector_pool::Client,
    ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>,
) -> Result<(), String> {
    let log_ctx = format!("log-sync {}", &config.url);
    let req = collector::StreamLogsRequest {};
    let mut stream = client.stream_logs(req).await
                           .map_err(|e| format!("stream_logs error: {}", e))?
                           .into_inner();
    'next_message: loop {
        match stream.message().await {
            Err(e) => return Err(format!("message(): {}", e)),
            Ok(None) => return Err("message() was None, stream should be infinite".to_owned()),
            Ok(Some(log)) => {
                let log = match crate::log_store::Log::from_protobuf(&log) {
                    Err(e) => {
                        error!("{} error converting protobuf: {}", log_ctx, e);
                        continue 'next_message;
                    }
                    Ok(m) => m,
                };
                trace!("{} got a log key=`{}'", log_ctx, log.key.display_name());
                ls.lock().unwrap().update(log);
                set_sync_metric_log(Ok(()), &config.logs_sync_key(), ls, ms);
            },
        };
    }
    // Unreachable, because we never break out of the 'next_message loop.
}

fn set_sync_metric_log(
    res: Result<(), String>,
    key: &MetricKey,
    ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>,
) {
    ms.lock().unwrap()
      .update(key,
              DataPoint {
                  time: chrono::Utc::now(),
                  val: MetricValue::OkErr(OkErr::from(res.as_ref())),
              });
    ls.lock().unwrap()
      .update(Log {
          start: chrono::Utc::now(),
          finish: chrono::Utc::now(),
          duration: std::time::Duration::from_secs(0),
          log: match res {
              Ok(()) => "Ok".to_owned(),
              Err(e) => e,
          },
          key: key.clone(),
      });
}

pub fn force_check_remote(mk: &MetricKey, remotes: &Arc<Remotes>) {
    let host_name = match &mk.host {
        Host::Local => panic!("Host::Local in force_check_remote"),
        Host::Remote(RemoteHost { name, }) => name,
    };
    let remote = match remotes.find_by_host(&host_name) {
        Some(r) => r,
        None => {
            error!("Remote not found for host `{}'", &host_name);
            return;
        }
    };
    let pool = remote.pool();
    let url = remote.config().url.clone();
    let mk = mk.clone();
    // TODO: Probably re-use some existing tokio runtime.
    std::thread::Builder::new()
        .name(format!("force-check {}", &url))
        .spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let log_ctx = format!("force-check {}", url);
                trace!("{} getting connection", log_ctx);
                // TODO: Add retries?
                let mut conn = match pool.get().await {
                    Ok(c) => c,
                    Err(e) => {
                        error!("{} error getting connection: {}", log_ctx, e);
                        return;
                    }
                };
                let req = collector::ForceRunRequest {
                    job_name: mk.name.clone(),
                };
                trace!("{} running RPC", log_ctx);
                if let Err(e) = conn.get().force_run(req).await {
                    error!("{} force_run error: {}", log_ctx, e);
                    return;
                }
                trace!("{} done", log_ctx);
            });
        }).unwrap();
}

fn sync_endpoint(config: &config::RemoteSync
) -> Result<tonic::transport::Endpoint, BoxError> {
    let url = config.url.parse::<tonic::transport::Uri>()?;
    let host = url.host().ok_or("Missing endpoint URL host")?;
    let scheme = url.scheme().ok_or("Missing endpoint URL host")?;
    let endpoint =
        tonic::transport::Endpoint::from_shared(config.url.clone())?
        .http2_keep_alive_interval(std::time::Duration::from_secs(60))
        .keep_alive_timeout(std::time::Duration::from_secs(15))
        .keep_alive_while_idle(true);
    let client_id = config.client_identity.load()?;
    let server_ca_cert = config.server_ca.load()?;
    let endpoint = if *scheme == http::uri::Scheme::HTTPS {
        let client_tls = tonic::transport::ClientTlsConfig::new()
            .domain_name(host)
            .ca_certificate(server_ca_cert)
            .identity(client_id);
        endpoint.tls_config(client_tls)?
    } else {
        endpoint
    };
    Ok(endpoint)
}
