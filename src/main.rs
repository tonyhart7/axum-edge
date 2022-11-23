use axum::{
    extract::Extension,routing::{get, post, put, delete}, Router,
};

use sqlx::postgres::PgPoolOptions;
use tokio::signal;
use std::{net::SocketAddr, time::Duration};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod errors;
mod models;
mod controllers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());


    let pool = PgPoolOptions::new()
    .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");

    let app = Router::new()
        .route("/hello", get(root))
        .route("/tasks", get(controllers::task::all_tasks))
        .route("/task", post(controllers::task::new_task))
        .route("/task/:id",get(controllers::task::task))
        .route("/task/:id", put(controllers::task::update_task))
        .route("/task/:id", delete(controllers::task::delete_task))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

        Ok(())

}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}