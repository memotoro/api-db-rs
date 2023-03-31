use api_db_rs::config::database::Config;
use api_db_rs::handler::processor::{
    failure_processor, request_processor, response_processor, span_processor,
};
use api_db_rs::handler::server::{
    all_records, create_record, delete_record, record_by_id, update_record,
};
use api_db_rs::repository::database::Postgres;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let db_config = Config::new_from_env().unwrap();

    let pg = Postgres::new(&db_config).await.unwrap();

    let repo = Arc::new(pg);

    let app = Router::new()
        .route("/records", get(all_records).post(create_record))
        .route(
            "/records/:id",
            get(record_by_id).patch(update_record).delete(delete_record),
        )
        .with_state(repo)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(span_processor)
                .on_request(request_processor)
                .on_response(response_processor)
                .on_failure(failure_processor),
        );

    let port = 8080;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
