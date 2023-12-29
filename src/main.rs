use bytes::Bytes;
use clap::Parser;
use fetcher::fetch_cluster_status;
use futures::join;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use metrics::{process_metrics, MetricsConvertible};
use prometheus::{Encoder, TextEncoder};

use std::convert::Infallible;
use std::net::SocketAddr;
use std::num::ParseIntError;
use std::path::PathBuf;

use tokio::{
    net::TcpListener,
    time::{sleep, Duration},
};
use tracing::{error, info};

mod fetcher;
mod metrics;
mod status_models;

async fn metrics(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Ok(Response::new(Full::new(buffer.into())))
}

async fn run_http_server(config: &CommandArgs) -> Result<(), anyhow::Error> {
    let addr: SocketAddr = ([0, 0, 0, 0], config.port).into();
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);
    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(metrics))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

/// Run a loop which will fetch regularly fdbcli status, to fetch current state
/// of the cluster.
async fn run_status_fetcher(config: &CommandArgs) -> Result<(), anyhow::Error> {
    let mut opts = ["--exec", "status json"].to_vec();

    if let Some(cluster) = &config.cluster {
        opts.push("-C");
        // Fairly safe as we already parse it to a valid PathBuf in config
        opts.push(cluster.to_str().unwrap());
    }
    let cmd = String::from("fdbcli");
    loop {
        match fetch_cluster_status(&cmd, &opts) {
            Ok(status) => process_metrics(status),
            Err(e) => e.to_metrics(&[]),
        }
        sleep(config.delay_sec).await;
    }
}

/// FoundationDB exporter for metrics parsed from status
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandArgs {
    /// Listening port of the web server
    #[arg(short, long, default_value_t = 9090, env = "FDB_EXPORTER_PORT")]
    port: u16,

    /// Location of fdb.cluster file
    #[arg(short, long, env = "FDB_CLUSTER_FILE")]
    cluster: Option<PathBuf>,

    /// Delay in seconds between two update of the status & metrics
    #[arg(short, long, env = "FDB_EXPORTER_DELAY", value_parser = parse_duration, default_value = "5")]
    delay_sec: Duration,
}

fn parse_duration(arg: &str) -> Result<Duration, ParseIntError> {
    let seconds = arg.parse()?;
    Ok(Duration::from_secs(seconds))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();
    let cli = CommandArgs::parse();

    let (server_rs, fetcher_rs) = join!(run_http_server(&cli), run_status_fetcher(&cli));

    if let Err(err) = server_rs {
        error!("HTTP server thread failed, {:?}", err);
    }

    if let Err(err) = fetcher_rs {
        error!("HTTP fetcher thread failed, {:?}", err);
    }

    Ok(())
}
