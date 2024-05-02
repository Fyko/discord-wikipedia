use anyhow::Result;

use self::{page_summary::WikipediaPageSummary, search::WikipediaSearchResponse};

pub mod page_summary;
pub mod search;

pub async fn fetch_page_summary(title: &str) -> Result<WikipediaPageSummary> {
    let url = format!(
        "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
        title
    );

    let response = reqwest::get(&url).await?;
    if response.status().as_u16() > 299 {
        let reason = response.text().await?;

        return Err(anyhow::anyhow!("Failed to fetch page summary: {reason}"));
    }

    let json = response.json::<WikipediaPageSummary>().await?;

    Ok(json)
}

pub async fn fetch_search_results(query: &str) -> Result<WikipediaSearchResponse> {
    let url = format!("https://en.wikipedia.org/w/rest.php/v1/search/title?q={query}&limit=10",);

    let response = reqwest::get(&url).await?;
    if response.status().as_u16() > 299 {
        let reason = response.text().await?;

        return Err(anyhow::anyhow!("Failed to fetch search results: {reason}"));
    }

    let json = response.json::<WikipediaSearchResponse>().await?;

    Ok(json)
}
