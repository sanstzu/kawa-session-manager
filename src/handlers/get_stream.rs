use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
pub struct GetStreamQuery {
    stream_key: String,
}

#[derive(Serialize)]
struct GetStreamResponse {
    stream_key: String,
    publish_path: String,
}

pub async fn handler(
    State(state): State<AppState>,
    Query(query): Query<GetStreamQuery>,
) -> Response {
    let GetStreamQuery { stream_key } = query;
    let redis_client = &state.redis_client;

    let option_stream_session = match redis_client.get_stream_session(&stream_key).await {
        Ok(session) => session,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let stream_session = match option_stream_session {
        Some(session) => session,
        None => return (StatusCode::NOT_FOUND).into_response(),
    };

    return (
        StatusCode::OK,
        Json(GetStreamResponse {
            stream_key: stream_session.get_stream_key().to_string(),
            publish_path: stream_session.get_publish_url().to_string(),
        }),
    )
        .into_response();
}
