use crate::{
    BoxError,
    collector,
    collector_pool,
    config,
    LogStore,
    MetricStore,
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
}

#[derive(Clone)]
pub struct Remote {
    config: config::RemoteSync,
    pool: Arc<collector_pool::Pool>,
}

impl Remote {
    pub fn from_config(config: config::RemoteSync) -> Result<Remote, BoxError> {
        let endpoint = sync_endpoint(&config)?;
        let pool = Arc::new(collector_pool::Pool::new(endpoint));
        Ok(Remote {
            config,
            pool,
        })
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
                tokio::join!(run_metric_sync(&remote.config, remote.pool.clone(), ms),
                             run_log_sync(&remote.config, remote.pool.clone(), ls))
            });
        }).unwrap();
}

async fn run_metric_sync(
    config: &config::RemoteSync,
    pool: Arc<collector_pool::Pool>,
    ms: Arc<Mutex<MetricStore>>
) {
    let log_ctx = format!("metric-sync {}", &config.url);
    'retry_all: loop {
        debug!("{} connecting", log_ctx);
        let mut client = match pool.get().await {
            Err(e) => {
                error!("{} connect error: {}", log_ctx, e);
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                continue 'retry_all;
            },
            Ok(c) => c,
        };
        debug!("{}: connected", log_ctx);
        let req = collector::StreamMetricsRequest {};
        let mut stream = match client.get().stream_metrics(req).await {
            Err(e) => {
                error!("{} stream_metrics error: {}", log_ctx, e);
                pool.discard_faulted(client).await;
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                continue 'retry_all;
            },
            Ok(s) => s.into_inner(),
        };
        'next_message: loop {
            match stream.message().await {
                Err(e) => {
                    error!("{} message() error: {}", log_ctx, e);
                    pool.discard_faulted(client).await;
                    tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                    continue 'retry_all;
                }
                Ok(None) => {
                    error!("{} message() was None, stream should be infinte", log_ctx);
                    pool.discard_faulted(client).await;
                    tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                    continue 'retry_all;
                }
                Ok(Some(metric)) => {
                    let metric = match crate::metric_store::Metric::from_protobuf(&metric) {
                        Err(e) => {
                            error!("{} error converting protobuf: {}", log_ctx, e);
                            continue 'next_message;
                        }
                        Ok(m) => m,
                    };
                    trace!("{} got a metric key=`{}'", log_ctx, metric.key.display_name());
                    if let Some(latest) = metric.latest {
                        ms.lock().unwrap()
                            .update(&metric.key, latest.clone())
                    }
                },
            };
        }
    }
}

async fn run_log_sync(
    config: &config::RemoteSync,
    pool: Arc<collector_pool::Pool>,
    ls: Arc<Mutex<LogStore>>
) {
    let log_ctx = format!("log-sync {}", &config.url);
    'retry_all: loop {
        debug!("{} connecting", log_ctx);
        let mut client = match pool.get().await {
            Err(e) => {
                error!("{} connect error: {}", log_ctx, e);
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                continue 'retry_all;
            },
            Ok(c) => c,
        };
        debug!("{} connected", log_ctx);
        let req = collector::StreamLogsRequest {};
        let mut stream = match client.get().stream_logs(req).await {
            Err(e) => {
                error!("{} stream_logs error: {}", log_ctx, e);
                pool.discard_faulted(client).await;
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                continue 'retry_all;
            },
            Ok(s) => s.into_inner(),
        };
        'next_message: loop {
            match stream.message().await {
                Err(e) => {
                    error!("{} message() error: {}", log_ctx, e);
                    pool.discard_faulted(client).await;
                    tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                    continue 'retry_all;
                }
                Ok(None) => {
                    error!("{} message() was None, stream should be infinte", log_ctx);
                    pool.discard_faulted(client).await;
                    tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                    continue 'retry_all;
                }
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
                },
            };
        }
    }
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
