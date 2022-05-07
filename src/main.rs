use std::future::Future;
use std::io::Error;
use std::net::SocketAddr;
use axum::{
    Router,
    routing::{get, get_service},
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    response::IntoResponse,
    http::StatusCode
};
use tower_http::{
    trace::{DefaultMakeSpan, TraceLayer},
};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};



#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let logger = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().include_headers(true));
    let app = Router::new()
        .route("/", get_service(
        ServeDir::new("assets").append_index_html_on_directories(true),
        ).handle_error(handle_unexpected_error))
        .route("/ws", get(ws_handler))
        .layer(
        logger,
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn handle_unexpected_error(error: Error) -> (StatusCode, String) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    println!("client sent str: {:?}", t);
                }
                Message::Binary(_) => {
                    println!("client sent binary data");
                }
                Message::Ping(_) => {
                    println!("socket ping");
                }
                Message::Pong(_) => {
                    println!("socket pong");
                }
                Message::Close(_) => {
                    println!("client disconnected");
                    return;
                }
            }
        } else {
            println!("client disconnected");
            return;
        }
    }

    loop {
        if socket
            .send(Message::Text(String::from("Hi!")))
            .await
            .is_err()
        {
            println!("client disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}


async fn handler() -> String {
    String::from("Hello, world!")
}