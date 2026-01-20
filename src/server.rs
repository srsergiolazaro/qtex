use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::sync::broadcast;
use tower_http::services::ServeFile;
use tower_http::cors::CorsLayer;

static BROADCAST_CHANNEL: std::sync::OnceLock<broadcast::Sender<String>> = std::sync::OnceLock::new();

pub async fn start_server(port: u16, output_path: PathBuf) -> Result<u16, Box<dyn std::error::Error>> {
    let (tx, _) = broadcast::channel(16);
    BROADCAST_CHANNEL.set(tx).map_err(|_| "Failed to set broadcast channel")?;

    let app = Router::new()
        .route("/view", get(view_pdf))
        .route("/ws", get(ws_handler))
        .route_service("/pdf", ServeFile::new(output_path))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let actual_port = listener.local_addr()?.port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    Ok(actual_port)
}

pub async fn notify_clients(path: &Path) {
    if let Some(tx) = BROADCAST_CHANNEL.get() {
        let _ = tx.send(path.to_string_lossy().to_string());
    }
}

async fn view_pdf() -> Html<&'static str> {
    Html(include_str!("view_template.html"))
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut rx = BROADCAST_CHANNEL.get().unwrap().subscribe();

    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}
