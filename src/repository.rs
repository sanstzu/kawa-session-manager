use std::sync::Arc;

use axum::BoxError;
use redis::{aio::Connection, AsyncCommands, Client};
use tokio::sync::Mutex;

use crate::models::StreamSession;

static STREAM_KEY_HASH: &'static str = "stream:stream_keys";
static STREAM_PATH_HASH: &'static str = "stream:stream_paths";

pub struct StreamSessionRepository {
    connection: Arc<Mutex<Connection>>,
}

impl StreamSessionRepository {
    pub async fn new(connection_url: String) -> Result<StreamSessionRepository, BoxError> {
        let redis_client = Client::open(connection_url).unwrap();
        let inner_connection = redis_client.get_async_connection().await?;

        let connection = Arc::new(Mutex::new(inner_connection));

        Ok(StreamSessionRepository { connection })
    }

    pub async fn set_stream_session(&self, stream_session: &StreamSession) -> Result<(), BoxError> {
        let StreamSessionRepository { connection } = self;

        let stream_key = stream_session.get_stream_key();
        let stream_path = stream_session.get_publish_url();

        let command: &'static str = r"local val = redis.call('HSETNX', KEYS[2], ARGV[2], ARGV[1])

        if val == 0 then
            return 0
        end

        redis.call('HSET', KEYS[1], ARGV[1], ARGV[2])

        return 1";

        let cmd = redis::Script::new(command);

        let mut inner_connection = connection.lock().await;

        let res: i32 = cmd
            .key(STREAM_KEY_HASH)
            .key(STREAM_PATH_HASH)
            .arg(stream_key)
            .arg(stream_path)
            .invoke_async(&mut *inner_connection)
            .await?;

        if res == 0 {
            return Err("Stream key already exists".into());
        }

        Ok(())
    }

    pub async fn get_stream_session(
        &self,
        stream_key: &str,
    ) -> Result<Option<StreamSession>, BoxError> {
        let StreamSessionRepository { connection } = self;

        let mut inner_connection = connection.lock().await;

        let result: Option<String> = inner_connection.hget(STREAM_KEY_HASH, stream_key).await?;

        match result {
            Some(publish_url) => Ok(Some(StreamSession::new(publish_url))),
            None => Ok(None),
        }
    }

    pub async fn delete_stream_key(&self, stream_key: &str) -> Result<(), BoxError> {
        let StreamSessionRepository { connection } = self;

        let mut inner_connection = connection.lock().await;

        let command = r"local val = redis.call('HGET', KEYS[1], ARGV[1])

        if val == false then
            return 0
        end

        redis.call('HDEL', KEYS[1], ARGV[1])
        redis.call('HDEL', KEYS[2], val)

        return 1";

        let cmd = redis::Script::new(command);

        let res: i32 = cmd
            .key(STREAM_KEY_HASH)
            .key(STREAM_PATH_HASH)
            .arg(stream_key)
            .invoke_async(&mut *inner_connection)
            .await?;

        if res == 0 {
            return Err("Stream key does not exist".into());
        }

        Ok(())
    }
}
