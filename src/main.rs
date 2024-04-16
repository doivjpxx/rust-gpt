use axum::{routing::get, Router};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenvy::dotenv;
use rust_gpt::routes::messages_route::{create_message, get_messages};
use std::env;
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gpt_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/messages", get(get_messages).post(create_message))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(tracing::Level::DEBUG)),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], env::var("PORT").unwrap().parse().unwrap()));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::debug!("Listening on: {}", addr);
    axum::serve(listener, app).await.unwrap();
}
