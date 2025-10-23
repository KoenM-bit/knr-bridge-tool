use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DonePayload {
    pub id: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}