#[macro_use]
extern crate log;

use monitor::{
    collector::{self, collector_server},
    create_shell_checks,
    // create_shell_metrics,
    DataPoint,
    LogStore,
    MetricCheck,
    MetricStore,
    MetricValue,
    OkErr,
    scheduler::Scheduler,
    ShellCheckConfig,
    ShellMetricConfig,
};
use std::sync::{Arc, Mutex};

fn shell_check_configs() -> Vec<ShellCheckConfig> {
    vec![
        ShellCheckConfig {
            name: "zfs.mf.healthy".to_owned(),
            cmd: "/sbin/zpool status -x | grep 'all pools are healthy'".to_owned(),
            interval: chrono::Duration::seconds(5),
        },
    ]
}

// fn shell_metric_configs() -> Vec<ShellMetricConfig> {
//     vec![]
// }

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or("info".to_owned()))
        .format_timestamp_micros()
        .init();

    let ls = Arc::new(Mutex::new(LogStore::new()));
    let ms = Arc::new(Mutex::new(MetricStore::new()));
    // let n = Arc::new(Mutex::new(Notifier::new()));
    let sched = Arc::new(Mutex::new(Scheduler::new()));

    let check_configs = shell_check_configs();
    create_shell_checks(&check_configs, &ls, &ms, &sched);

    // let metric_configs = shell_metric_configs();
    // create_shell_metrics(&metric_configs, &ls, &ms, &n, &sched);

    // connect_all_checks_to_notifier(&ms, &n);

    sched.lock().unwrap().spawn();

    let addr: std::net::SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let collector_service = CollectorService {
        metric_store: ms.clone(),
    };
    // let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    info!("Listening on http://{}", addr);
    tonic::transport::Server::builder()
        .add_service(collector::collector_server::CollectorServer::new(collector_service))
        .serve(addr)
        .await.unwrap();
}

struct CollectorService {
    metric_store: Arc<Mutex<MetricStore>>,
}

#[tonic::async_trait]
impl collector_server::Collector for CollectorService {
    async fn get_metrics(
        &self,
        request: tonic::Request<collector::GetMetricsRequest>,
    ) -> Result<tonic::Response<collector::MetricsReply>, tonic::Status>
    {
        trace!("CollectorService::get_metrics");
        trace!("request remote_addr={:?}, \n  {:#?}", request.remote_addr(), request);
        let metrics = self.metric_store.lock().unwrap().query_all();
        let metrics = metrics.iter().map(|m| m.as_protobuf())
                             .collect::<Result<Vec<collector::Metric>, String>>();
        if let Err(e) = metrics {
            error!("metrics as_protobuf error: {}", e);
            return Err(tonic::Status::internal("Internal error."));
        }
        let metrics = metrics.unwrap();
        trace!("num_metrics = {}", metrics.len());
        Ok(tonic::Response::new(collector::MetricsReply {
            metrics: metrics,
        }))
    }

    type StreamMetricsStream =
        tokio::sync::mpsc::Receiver<Result<collector::Metric, tonic::Status>>;
    async fn stream_metrics(
        &self,
        request: tonic::Request<collector::StreamMetricsRequest>,
    ) -> Result<tonic::Response<Self::StreamMetricsStream>, tonic::Status>
    {
        trace!("CollectorService::stream_metrics");
        trace!("request remote_addr={:?}, \n  {:#?}", request.remote_addr(), request);

        // Lock the metric store until we've sent all initial values and registered our listener
        let mut lock = self.metric_store.lock().unwrap();
        let metrics = lock.query_all()
                          .iter().map(|m| m.as_protobuf())
                          .collect::<Result<Vec<collector::Metric>, String>>();
        if let Err(e) = metrics {
            error!("metrics as_protobuf error: {}", e);
            return Err(tonic::Status::internal("Internal error."));
        }
        let metrics = metrics.unwrap();
        let (mut tx, rx) = tokio::sync::mpsc::channel(metrics.len() + 10);
        for m in metrics.into_iter() {
            tx.try_send(Ok(m)).expect("channel to have capacity for all initial values");
        }
        trace!("CollectorService::stream_metrics sent initial values");
        // Wrap in a mutex to keep the borrow checker happy that we're using a
        // &mut method on a captured variable in an Fn closure.
        let tx = Mutex::new(tx);
        lock.update_signal_for_all().connect(move |metric| {
            let metric_proto = match metric.as_protobuf() {
                Err(e) => {
                    error!("stream_metrics: failed to encode as protobuf: {}", e);
                    return
                },
                Ok(m) => m,
            };
            match tx.lock().unwrap().try_send(Ok(metric_proto)) {
                Err(tokio::sync::mpsc::error::TrySendError::Full(_)) =>
                    error!("stream_metrics: channel full key={}",
                           metric.key().display_name()),
                Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) =>
                    error!("stream_metrics: channel closed, not implemented yet!"),
                Ok(()) =>
                    trace!("stream_metrics: sent metric key=`{}'", metric.key().display_name()),
            };
        });
        Ok(tonic::Response::new(rx))
    }
}
