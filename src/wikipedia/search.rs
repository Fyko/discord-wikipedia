use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikipediaSearchResponse {
    pub pages: Vec<Page>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: i64,
    pub key: String,
    pub title: String,
    pub excerpt: String,
    pub matched_title: Option<serde_json::Value>,
    pub description: Option<String>,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub mimetype: String,
    pub width: i64,
    pub height: i64,
    pub duration: Option<serde_json::Value>,
    pub url: String,
}
