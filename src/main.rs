use clap::Parser;

mod docker;
mod models;
mod prometheus;

use docker::fetch_container_stats;
use prometheus::json_to_prometheus;

use axum::{Router, extract::State, response::Json, routing::get};
use models::{ContainerInfo, ContainerStats};
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "/etc/docker/docker.sock")]
    socket_path: String,

    #[arg(long, short, default_value = "3000")]
    port: u32,
}

async fn get_container_metrics_json_handler(
    State(state): State<Arc<Args>>,
) -> Json<Vec<ContainerInfo>> {
    let (containers, container_stats) =
        fetch_container_stats(&state.socket_path);
    Json::from(
        container_stats
            .into_iter()
            .zip(containers)
            .map(|f| ContainerInfo {
                id: f.1.id,
                names: f.1.names,
                stats: f.0,
            })
            .collect::<Vec<ContainerInfo>>(),
    )
}

async fn get_container_metrics_prometheus_handler(
    State(state): State<Arc<Args>>,
) -> String {
    println!("Inside get_container_metrics_prometheus_handler");
    let (containers, container_stats) =
        fetch_container_stats(&state.socket_path);
    json_to_prometheus(containers, container_stats)
}

#[tokio::main]
async fn main() {
    let args = Arc::new(Args::parse());
    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let app = Router::new()
        .route("/json", get(get_container_metrics_json_handler))
        .route("/metrics", get(get_container_metrics_prometheus_handler))
        .with_state(args);

    println!("Listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
