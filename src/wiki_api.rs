use scraper::Html;

use crate::{error::WikiError, utils::update_relative_urls};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub query: T,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(untagged)]
pub enum OpenSearchItem {
    Single(String),
    Array(Vec<String>),
}

pub async fn fetch_open_search(
    search: &str,
    lang: &str,
    limit: u16,
) -> Result<Vec<OpenSearchItem>, WikiError> {
    let url = format!("https://wiki.archlinux.org/api.php?action=opensearch&format=json&uselang={lang}&limit={limit}&search={search}");
    let body = reqwest::get(url).await?.text().await?;
    let res: Vec<OpenSearchItem> = serde_json::from_str(&body)?;

    // the first item in the response should be the search term
    debug_assert_eq!(res.get(0), Some(&OpenSearchItem::Single(search.to_owned())));

    return Ok(res);
}

/// Gets an ArchWiki pages entire content. Also updates all relative URLs to absolute URLs.
/// `/title/Neovim` -> `https://wiki.archlinux.org/title/Neovim`
pub async fn fetch_page(page: &str) -> Result<Html, reqwest::Error> {
    let url = format!("https://wiki.archlinux.org/title/{page}");

    let body = reqwest::get(&url).await?.text().await?;
    let body_with_abs_urls = update_relative_urls(&body);

    Ok(Html::parse_document(&body_with_abs_urls))
}
