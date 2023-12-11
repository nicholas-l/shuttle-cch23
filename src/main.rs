use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

mod day01;
mod day04;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn respond_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(respond_error))
        .route("/1/*key", get(day01::part1))
        .merge(day04::routes());

    Ok(router.into())
}
