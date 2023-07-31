mod common;
mod disk_store;
mod routes;
mod screee;

use axum::{
    routing::{any, get, post},
    Router,
};
use common::{Links, DEFAULT_HTTP_ADDR, DEFAULT_HTTP_PORT};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tokio::{signal, sync::Mutex};
use tower_http::trace::{self, TraceLayer};
use tracing::{event, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let links = disk_store::load();
    let links_mutex: Arc<Mutex<Links>> = Arc::new(Mutex::new(links.clone()));

    let addr: String = std::env::var("HTTP_ADDR").unwrap_or(DEFAULT_HTTP_ADDR.to_string());
    let port: u16 = match std::env::vars().find(|(name, _)| name == "HTTP_PORT") {
        Some((_, value)) => value.parse::<u16>().unwrap(),
        None => DEFAULT_HTTP_PORT,
    };
    let socket = SocketAddr::new(addr.parse::<IpAddr>().unwrap(), port);

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/", post(routes::create_screee))
        .route("/:screee", any(routes::use_screee))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(links_mutex.clone());

    event!(Level::INFO, "Starting server on {}", socket.to_string());
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_handler(&links_mutex))
        .await
        .unwrap()
}

async fn shutdown(links: &Arc<Mutex<Links>>) {
    event!(Level::INFO, "Signal received, shutting down");

    let data = links.lock().await;
    disk_store::save(&data);
}

async fn signal_handler(links: &Arc<Mutex<Links>>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            shutdown(links).await;
        },
        _ = terminate => {
            shutdown(links).await;
        },
    }
}
