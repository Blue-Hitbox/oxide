use axum::{
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/files", get(list_files))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Backend listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello from Oxide Backend!"
}

#[derive(Serialize)]
struct FileInfo {
    name: String,
    is_dir: bool,
}

async fn list_files() -> Json<Vec<FileInfo>> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                files.push(FileInfo {
                    name: entry.file_name().to_string_lossy().into_owned(),
                    is_dir: metadata.is_dir(),
                });
            }
        }
    }
    Json(files)
}
