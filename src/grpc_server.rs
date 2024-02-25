use crate::{
    service::{GetSessionRequest, GetSessionResponse},
    AppState,
};
use tonic::{Request, Response, Status};

use crate::service::session_manager_server::SessionManager;

pub struct Server {
    pub app_state: AppState,
}

#[tonic::async_trait]
impl SessionManager for Server {
    async fn get_session(
        &self,
        request: Request<GetSessionRequest>,
    ) -> Result<Response<GetSessionResponse>, Status> {
        let request = request.into_inner();
        let stream_key = request.stream_key;
        let redis_client = &self.app_state.redis_client;

        let option_stream_session = match redis_client.get_stream_session(&stream_key).await {
            Ok(session) => session,
            Err(_) => return Err(Status::internal("Internal Server Error")),
        };

        let stream_session = match option_stream_session {
            Some(session) => session,
            None => return Ok(Response::new(GetSessionResponse { status: 1, stream_path: "".to_string() })),
        };

        Ok(Response::new(GetSessionResponse { status: 0, stream_path: stream_session.get_publish_url().to_string()}))
    }
}
