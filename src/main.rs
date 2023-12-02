use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn respond_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn day1_part1(Path(packet_ids): Path<String>) -> impl IntoResponse {
    let packet_ids: Vec<i64> = packet_ids
        .split('/')
        .map(|id| id.parse::<i64>().unwrap())
        .collect();
    if packet_ids.len() > 20 {
        return (StatusCode::BAD_REQUEST, "").into_response();
    }
    (
        StatusCode::OK,
        packet_ids
            .into_iter()
            .reduce(|acc, id| acc ^ id)
            .unwrap()
            .pow(3)
            .to_string(),
    )
        .into_response()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(respond_error))
        .route("/1/*key", get(day1_part1));

    Ok(router.into())
}
