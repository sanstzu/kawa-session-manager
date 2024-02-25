use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct RemoveStreamQuery {
    stream_key: String,
}

pub async fn handler(
    State(state): State<AppState>,
    Query(query): Query<RemoveStreamQuery>,
) -> Response {
    let RemoveStreamQuery { stream_key } = query;
    let inner_connection = &state.redis_client;

    match inner_connection.delete_stream_key(&stream_key).await {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
