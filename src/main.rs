use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use axum::{
    http::uri::Uri,
    response::Html,
    routing::{get, post_service},
    Router,
};
use dotenvy::dotenv;
use sea_orm::Database;
use seaography::{async_graphql, lazy_static};
use std::env;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

lazy_static::lazy_static! {
    static ref URI: Uri = env::var("URL").unwrap_or("http://localhost:8301".into()).parse().expect("URL must be a valid HTTP URL");
    static ref GRAPHQL_ENDPOINT: String = env::var("GRAPHQL_ENDPOINT").unwrap_or("/graphql".into());
    static ref PLAYGROUND_ENDPOINT: String = env::var("PLAYGROUND_ENDPOINT").unwrap_or("/playground".into());
    static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://app.db?mode=rwc".into());
    static ref GRAPH_DEPTH_LIMIT: usize = {
        let value = env::var("GRAPH_DEPTH_LIMIT").map_or(16, |data| data.parse().unwrap_or(16));
        assert!(
            value >= 16 && value <= 64,
            "GRAPH_DEPTH_LIMIT must be between 16 and 64"
        );
        value
    };
    static ref GRAPH_COMPLEXITY_LIMIT: usize = {
        let value = env::var("GRAPH_COMPLEXITY_LIMIT").map_or(4096, |data| data.parse().unwrap_or(4096));
        assert!(
            value >= 128 && value <= 8192,
            "GRAPH_COMPLEXITY_LIMIT must be between 128 and 8192"
        );
        value
    };
    static ref WWWROOT_PATH: String = env::var("WWWROOT_PATH").unwrap_or("./wwwroot".into());
}

async fn graphql_playground() -> Html<String> {
    Html(
        GraphiQLSource::build()
            .endpoint(&*GRAPHQL_ENDPOINT)
            .finish(),
    )
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(tracing::Level::INFO.to_string())),
        )
        .init();
    let database = Database::connect(&*DATABASE_URL)
        .await
        .expect("Fail to initialize database connection");
    let schema = rust_graphql_service::query_root::schema(
        database,
        *GRAPH_DEPTH_LIMIT,
        *GRAPH_COMPLEXITY_LIMIT,
    )
    .unwrap();
    let app = Router::new()
        .route(&*GRAPHQL_ENDPOINT, post_service(GraphQL::new(schema)))
        .route(&*PLAYGROUND_ENDPOINT, get(graphql_playground))
        .fallback_service(ServeDir::new(&*WWWROOT_PATH).append_index_html_on_directories(true));

    let listener = TcpListener::bind((&*URI.host().unwrap(), URI.port_u16().unwrap()))
        .await
        .unwrap();
    tracing::info!("GraphQL service listening on {}", &*URI);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
