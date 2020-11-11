use crate::{
    collector,
    config,
    LogStore,
    MetricStore,
};
use std::sync::{Arc, Mutex};

// pub fn add_remote_sync_job_polling(
//     config: &config::RemoteSync,
//     ms: &Arc<Mutex<MetricStore>>,
//     sched: &Arc<Mutex<Scheduler>>)
// {
//     // TODO: Cache connection and re-use between invocations.
//
//     let config = config.clone();
//     let ms = ms.clone();
//     let j = scheduler::JobDefinition {
//         interval: chrono::Duration::seconds(5),
//         name: format!("remote-sync.{}", &config.url),
//         f: Arc::new(Mutex::new(move |_rc| {
//             debug!("Remote sync connecting endpoint url: {}", &config.url);
//             let endpoint = tonic::transport::Endpoint::from_shared(config.url.clone()).unwrap();
//             let config = config.clone();
//             let ms = ms.clone();
//             let fut = async move {
//                 let mut client = collector::collector_client::CollectorClient::connect(endpoint)
//                     .await?;
//                 debug!("Remote sync polling `{}'", config.url);
//                 let req = collector::GetMetricsRequest {};
//                 let metrics = client.get_metrics(req).await?;
//                 trace!("Remote sync got metrics");
//                 trace!("metrics: {:#?}", metrics);
//                 let metrics = metrics.into_inner().metrics.iter()
//                                      .map(|m| metric_store::Metric::from_protobuf(m))
//                                      .collect::<Result<Vec<metric_store::Metric>, String>>()?;
//                 debug!("Remote sync unmarshalled metrics");
//                 trace!("metrics: {:#?}", metrics);
//
//                 { // Scope the lock on ms.
//                     let mut msl = ms.lock().unwrap();
//                     for m in metrics.iter() {
//                         if let Some(latest) = m.latest() {
//                             msl.update(&m.key(), latest.clone())
//                         }
//                     }
//                 }
//                 Ok(())
//             };
//             // Spinning up a tokio runtime takes ~300us, so caching a
//             // runtime somewhere might be nice but isn't required.
//             let mut rt = tokio::runtime::Runtime::new().unwrap();
//             let res: Result<(), Box<dyn std::error::Error>> = rt.block_on(fut);
//             if let Err(e) = res {
//                 error!("Remote sync error = {}", e);
//             }
//         }))
//     };
//
//     sched.lock().unwrap().add(j);
// }

pub fn spawn_jobs_streaming(
    config: &config::RemoteSync,
    ls: &Arc<Mutex<LogStore>>,
    ms: &Arc<Mutex<MetricStore>>
) {
    let config = config.clone();
    let ms = ms.clone();
    let ls = ls.clone();
    std::thread::Builder::new()
        .name(format!("metric-sync {}", config.url))
        .spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                tokio::join!(run_metric_sync(&config, ms),
                             run_log_sync(&config, ls))
            });
        }).unwrap();
}

async fn run_metric_sync(config: &config::RemoteSync, ms: Arc<Mutex<MetricStore>>) {
    debug!("metric-sync connecting endpoint url: {}", &config.url);
    let endpoint = sync_endpoint(config).expect("Building endpoint");
    'retry_all: loop {
        let client =
            collector::collector_client::CollectorClient::connect(endpoint.clone())
            .await;
        let mut client = match client {
            Err(e) => {
                error!("metric-sync connect error: {}", e);
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                continue 'retry_all;
            },
            Ok(c) => c,
        };
        debug!("metric-sync connected `{}'", config.url);
        let req = collector::StreamMetricsRequest {};
        let mut stream = match client.stream_metrics(req).await {
            Err(e) => {
                error!("metric-sync stream_metrics error: {}", e);
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                continue 'retry_all;
            },
            Ok(s) => s.into_inner(),
        };
        'next_message: loop {
            match stream.message().await {
                Err(e) => {
                    error!("metric-sync message() error: {}", e);
                    tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                    continue 'retry_all;
                }
                Ok(None) => {
                    error!("metric-sync message() was None, stream should be infinte");
                    tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                    continue 'retry_all;
                }
                Ok(Some(metric)) => {
                    let metric = match crate::metric_store::Metric::from_protobuf(&metric) {
                        Err(e) => {
                            error!("metric-sync error converting protobuf: {}", e);
                            continue 'next_message;
                        }
                        Ok(m) => m,
                    };
                    trace!("metric-sync got a metric key=`{}'", metric.key().display_name());
                    if let Some(latest) = metric.latest() {
                        ms.lock().unwrap()
                            .update(&metric.key(), latest.clone())
                    }
                },
            };
        }
    }
}

async fn run_log_sync(config: &config::RemoteSync, ls: Arc<Mutex<LogStore>>) {
    debug!("log-sync connecting endpoint url: {}", &config.url);
    let endpoint = sync_endpoint(config).expect("Building endpoint");
    'retry_all: loop {
        let client =
            collector::collector_client::CollectorClient::connect(endpoint.clone())
            .await;
        let mut client = match client {
            Err(e) => {
                error!("log-sync connect error: {}", e);
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                continue 'retry_all;
            },
            Ok(c) => c,
        };
        debug!("log-sync connected `{}'", config.url);
        let req = collector::StreamLogsRequest {};
        let mut stream = match client.stream_logs(req).await {
            Err(e) => {
                error!("log-sync stream_logs error: {}", e);
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                continue 'retry_all;
            },
            Ok(s) => s.into_inner(),
        };
        'next_message: loop {
            match stream.message().await {
                Err(e) => {
                    error!("log-sync message() error: {}", e);
                    tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                    continue 'retry_all;
                }
                Ok(None) => {
                    error!("log-sync message() was None, stream should be infinte");
                    tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                    continue 'retry_all;
                }
                Ok(Some(log)) => {
                    let log = match crate::log_store::Log::from_protobuf(&log) {
                        Err(e) => {
                            error!("log-sync error converting protobuf: {}", e);
                            continue 'next_message;
                        }
                        Ok(m) => m,
                    };
                    trace!("log-sync got a log key=`{}'", log.key.display_name());
                    ls.lock().unwrap().update(log);
                },
            };
        }
    }
}

fn sync_endpoint(config: &config::RemoteSync
) -> Result<tonic::transport::Endpoint, Box<dyn std::error::Error>> {
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
