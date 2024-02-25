use dotenv::dotenv;
use repository::StreamSessionRepository;
use std::env;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::net::TcpListener;
use tonic::transport::Server;

use axum::{
    routing::{delete, get, post},
    BoxError, Router,
};

use log::info;

mod grpc_server;
mod handlers;
mod models;
mod repository;

pub mod service {
    tonic::include_proto!("session_manager");
}

type AppState = Arc<InnerAppState>;

struct InnerAppState {
    pub redis_client: StreamSessionRepository,
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    dotenv().ok();
    env_logger::init();

    let redis_url = env::var("REDIS_URL").unwrap();

    let redis_url = format!("redis://{}", redis_url);

    let connection = StreamSessionRepository::new(redis_url).await?;
    let state: AppState = Arc::new(InnerAppState {
        redis_client: connection,
    });

    let router: Router = Router::<AppState>::new()
        .route("/stream", get(handlers::get_stream))
        .route("/stream", post(handlers::create_stream))
        .route("/stream", delete(handlers::remove_stream))
        .with_state(state.clone());

    let http_addr = "0.0.0.0:8888";
    let grpc_addr = "0.0.0.0:50052";

    let http_listener = TcpListener::bind(http_addr).await?;

    axum::serve(http_listener, router).await.unwrap();
    info!("Listening HTTP requests on: {}", http_addr);

    Server::builder()
        .add_service(service::session_manager_server::SessionManagerServer::new(
            grpc_server::Server {
                app_state: state.clone(),
            },
        ))
        .serve(grpc_addr.to_socket_addrs().unwrap().next().unwrap())
        .await?;

    info!("Listening gRPC requests on: {}", grpc_addr);

    Ok(())
}
