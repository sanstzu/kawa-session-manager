use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{models::StreamSession, AppState};

#[derive(Deserialize)]
pub struct CreateStreamBody {
    publish_path: String,
}

#[derive(Serialize)]
struct CreateStreamResponse {
    stream_key: String,
    publish_path: String,
}

pub async fn handler(
    State(state): State<AppState>,
    Json(query): Json<CreateStreamBody>,
) -> Response {
    let CreateStreamBody { publish_path } = query;
    let connection = &state.redis_client;

    let stream_session = StreamSession::new(publish_path);

    match connection.set_stream_session(&stream_session).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(CreateStreamResponse {
                stream_key: stream_session.get_stream_key().to_string(),
                publish_path: stream_session.get_publish_url().to_string(),
            }),
        )
            .into_response(),
        Err(_) => (StatusCode::CONFLICT).into_response(),
    }
}
