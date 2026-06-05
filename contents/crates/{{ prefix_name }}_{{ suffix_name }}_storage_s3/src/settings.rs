use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StorageS3Settings {
    pub endpoint: String,
    pub bucket: String,
    pub prefix: Option<String>,
    pub access_key: String,
    pub secret_key: String,
}

impl Default for StorageS3Settings {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:9000".to_string(),
            bucket: String::new(),
            prefix: None,
            access_key: String::new(),
            secret_key: String::new(),
        }
    }
}
