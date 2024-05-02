use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikipediaPageSummary {
    #[serde(rename = "type")]
    pub wikipedia_page_summary_type: String,
    pub title: String,
    pub displaytitle: String,
    pub namespace: Namespace,
    pub wikibase_item: String,
    pub titles: Titles,
    pub pageid: i64,
    pub thumbnail: Option<Originalimage>,
    pub originalimage: Option<Originalimage>,
    pub lang: String,
    pub dir: String,
    pub revision: String,
    pub tid: String,
    pub timestamp: String,
    pub description: String,
    pub description_source: String,
    pub content_urls: ContentUrls,
    pub extract: String,
    pub extract_html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentUrls {
    pub desktop: Desktop,
    pub mobile: Desktop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Desktop {
    pub page: String,
    pub revisions: String,
    pub edit: String,
    pub talk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Namespace {
    pub id: i64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Originalimage {
    pub source: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Titles {
    pub canonical: String,
    pub normalized: String,
    pub display: String,
}
