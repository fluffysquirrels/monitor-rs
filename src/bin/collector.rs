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
            interval: chrono::Duration::minutes(2),
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
        _request: tonic::Request<collector::GetMetricsRequest>,
    ) -> Result<tonic::Response<collector::MetricsReply>, tonic::Status>
    {
        trace!("CollectorService::get_metrics");
        let metrics = self.metric_store.lock().unwrap().query_all();
        // TODO: Remove this unwrap and return an error.
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
}
