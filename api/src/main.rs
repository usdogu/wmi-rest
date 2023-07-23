use std::{net::SocketAddr, sync::{Arc, Mutex}};

use axum::{extract::{State, Path}, routing::get, Router};
use tokio::signal;

struct AppState {
    pwsh: powershell_rs::Shell,
}

impl AppState {
    fn new() -> Self {
        Self {
            pwsh: powershell_rs::Shell::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::new()));
    let app = Router::new()
    .route("/vms", get(get_vms))
    .route("/vms/:id/memory", get(get_memory))
    .route("/vms/:id/network", get(get_network))
    .route("/vms/:id/processor", get(get_processor))
    .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn get_vms(State(state): State<Arc<Mutex<AppState>>>) -> String {
    let pwsh = &mut state.lock().unwrap().pwsh;
    let vms = hyperv::vm::get_vms(pwsh).unwrap();
    vms
}

async fn get_memory(State(state): State<Arc<Mutex<AppState>>>, Path(id): Path<String>) -> String {
    let pwsh = &mut state.lock().unwrap().pwsh;
    let memory = hyperv::memory::get_memory(id, pwsh).unwrap();
    memory
}

async fn get_processor(State(state): State<Arc<Mutex<AppState>>>, Path(id): Path<String>) -> String {
    let pwsh = &mut state.lock().unwrap().pwsh;
    let processor = hyperv::processor::get_processor(id, pwsh).unwrap();
    processor
}

async fn get_network(State(state): State<Arc<Mutex<AppState>>>, Path(id): Path<String>) -> String {
    let pwsh = &mut state.lock().unwrap().pwsh;
    let network = hyperv::network::get_network(id, pwsh).unwrap();
    network
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