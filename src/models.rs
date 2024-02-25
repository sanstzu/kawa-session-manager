use uuid::Uuid;

#[derive(Clone)]
pub struct StreamSession {
    stream_key: String, // Private key that is use
    publish_path: String,
}

impl StreamSession {
    pub fn new(publish_path: String) -> StreamSession {
        // TODO: Use hash for stream key generation instead

        let stream_key = Uuid::new_v4().to_string();
        StreamSession {
            stream_key,
            publish_path,
        }
    }

    pub fn get_stream_key(&self) -> &str {
        &self.stream_key
    }

    pub fn get_publish_url(&self) -> &str {
        &self.publish_path
    }
}
