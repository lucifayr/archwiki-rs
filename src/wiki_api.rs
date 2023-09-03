use scraper::Html;

use crate::{
    error::WikiError,
    utils::{open_search_get_exact_match_url, open_search_to_page_names, update_relative_urls},
};

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
///
/// If the ArchWiki page doesn't have exists the top 5 pages that are most
/// like the page that was given as an argument are returned as a `NoPageFound` error.
pub async fn fetch_page(page: &str, lang: Option<&str>) -> Result<Html, WikiError> {
    let lang = lang.unwrap_or("en");

    let search_res = fetch_open_search(page, lang, 5).await?;

    let Some(url) = open_search_get_exact_match_url(page, &search_res)? else {
        let similar_pages = open_search_to_page_names(&search_res)?;
        return Err(WikiError::NoPageFound(similar_pages.join("\n")));
    };

    let body = reqwest::get(&url).await?.text().await?;
    let body_with_abs_urls = update_relative_urls(&body);

    Ok(Html::parse_document(&body_with_abs_urls))
}
