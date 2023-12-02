use axum::{extract::Path, http::StatusCode, response::IntoResponse};

pub(crate) async fn part1(Path(packet_ids): Path<String>) -> impl IntoResponse {
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
