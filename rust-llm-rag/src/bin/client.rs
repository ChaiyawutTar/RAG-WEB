use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::Router;
use ollama_rs::Ollama;
use rust_llm_rag::infrastructure::vector_db::{init_client, QdrantDb};
use rust_llm_rag::llm::{handlers, usecases};
use rust_llm_rag::setting::setting::Setting;
use socketioxide::extract::Data;
use socketioxide::{extract::SocketRef, SocketIo};
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;
use rust_llm_rag::infrastructure::mongo::repository::MongoDb;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let setting = Setting::new();
    let model = Arc::new(&setting).llm.model.clone();

    let vector_db_client = init_client(Arc::clone(&setting));
    let qdrant_db = QdrantDb::new(vector_db_client);

    let ollama = Arc::new(Ollama::default());
    let mongo_db = MongoDb::new(&Arc::clone(&setting).mongodb.uri).await;

    let llm_usecases = usecases::UsecasesImpl::new(Arc::clone(&qdrant_db), Arc::clone(&ollama));
    let llm_handlers = handlers::Handlers::new(Arc::clone(&llm_usecases), Arc::clone(&mongo_db));

    let (socket_layer, io) = SocketIo::builder()
        .max_payload(Arc::clone(&setting).server.max_payload)
        .max_buffer_size(Arc::clone(&setting).server.max_buffer_size)
        .build_layer();

    // Register a handler for the default namespace
    io.ns("/", |s: SocketRef| {
        // For each "message" event received, send a "message-back" event with the "Hello World!" event
        s.on(
            "prompt",
            |s: SocketRef, Data::<String>(prompt)| async move {
                let result = llm_handlers.chatting(prompt, model).await;

                s.emit("result", result).ok();
            },
        );
    });

    // let app = Router::new()
    //     .layer(TraceLayer::new_for_http())
    //     .layer(socket_layer);
    let app = Router::new()
    .layer(
        CorsLayer::new()
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([CONTENT_TYPE])
            .allow_credentials(true),
    )
    .layer(TraceLayer::new_for_http())
    .layer(socket_layer);

    let uri = format!("0.0.0.0:{}", Arc::clone(&setting).server.port);
    let listener = tokio::net::TcpListener::bind(&uri).await.unwrap();

    info!(
        "🦀 Server is starting on: :{}",
        Arc::clone(&setting).server.port
    );

    axum::serve(listener, app).await.unwrap();
}
