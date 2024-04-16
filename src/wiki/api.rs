use std::collections::HashMap;

use scraper::Html;
use serde::Deserialize;
use url::Url;

use crate::{
    args::internal::ReadPageArgs,
    error::WikiError,
    formats::format_page,
    search::{
        open_search_is_page_exact_match, open_search_to_page_names, OpenSearchItem,
        TextSearchApiResponse, TextSearchItem,
    },
    utils::update_relative_urls,
};

const BLOCK_LISTED_CATEGORY_PREFIXES: &[&str] = &[
    "Pages flagged with",
    "Sections flagged with",
    "Pages or sections flagged with",
    "Pages where template include size is exceeded",
    "Pages with broken package links",
    "Pages with broken section links",
    "Pages with missing package links",
    "Pages with missing section links",
    "Pages with dead links",
];

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Response<T> {
    pub query: T,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ResponseWithContinue<T, V> {
    pub query: T,
    pub r#continue: Option<V>,
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
    debug_assert_eq!(
        res.first(),
        Some(&OpenSearchItem::Single(search.to_owned()))
    );

    Ok(res)
}

pub async fn fetch_text_search(
    search: &str,
    lang: &str,
    limit: u16,
) -> Result<Vec<TextSearchItem>, WikiError> {
    let url = format!("https://wiki.archlinux.org/api.php?action=query&list=search&format=json&srwhat=text&uselang={lang}&srlimit={limit}&srsearch={search}");
    let body = reqwest::get(url).await?.text().await?;
    let res: Response<TextSearchApiResponse> = serde_json::from_str(&body)?;

    Ok(res.query.search)
}

#[allow(unused)]
pub async fn fetch_and_format_page(
    ReadPageArgs {
        page,
        format,
        lang,
        show_urls,
    }: ReadPageArgs,
) -> Result<String, WikiError> {
    let doc = fetch_page(&page, &lang).await?;
    Ok(format_page(&format, &doc, &page, show_urls))
}

/// Gets the HTML content of an ArchWiki page.
///
/// If the ArchWiki page doesn't exists the top 5 pages that are most
/// like the page that was given as an argument are returned as a `NoPageFound` error.
pub async fn fetch_page(page: &str, lang: &str) -> Result<Html, WikiError> {
    let search_res = fetch_open_search(page, lang, 5).await?;

    let Some(page_title) = open_search_is_page_exact_match(page, &search_res)? else {
        let similar_pages = open_search_to_page_names(&search_res)?;
        return Err(WikiError::NoPageFound(similar_pages.join("\n")));
    };

    fetch_page_without_recommendations(page_title).await
}

/// Gets the HTML content of an ArchWiki page.
pub async fn fetch_page_without_recommendations(page: &str) -> Result<Html, WikiError> {
    let raw_url = format!(
        "https://wiki.archlinux.org/rest.php/v1/page/{title}/html",
        title = urlencoding::encode(page)
    );

    let url = Url::parse(&raw_url)?;
    let document = fetch_page_by_url(url).await?;
    Ok(document)
}

/// Gets an ArchWiki pages entire content. Also updates all relative URLs to absolute URLs.
/// `/title/Neovim` -> `https://wiki.archlinux.org/title/Neovim`.
/// A different base URL is used for pages that aren't hosted directly on `wiki.archlinux.org`
///
/// If the page has no content a `NoPageFound` Error is returned.
async fn fetch_page_by_url(url: Url) -> Result<Html, WikiError> {
    let base_url = format!(
        "{schema}://{host}",
        schema = url.scheme(),
        host = url.host_str().unwrap_or("")
    );

    let body = reqwest::get(url).await?.text().await?;
    let body_with_abs_urls = update_relative_urls(&body, &base_url);

    Ok(Html::parse_document(&body_with_abs_urls))
}

/// Gets the names of all pages on the ArchWiki and the categories that they belong to.
///
/// ### Example
///
/// ```sh
/// Wine        # page name
/// - Emulation # category
/// - Gaming    # category
/// ```
pub async fn fetch_all_pages() -> Result<HashMap<String, Vec<String>>, WikiError> {
    #[derive(Debug, Deserialize)]
    struct ApiAllPagesQuery {
        pages: HashMap<String, Page>,
    }

    #[derive(Debug, Deserialize)]
    struct Page {
        title: String,
        categories: Option<Vec<Category>>,
    }

    #[derive(Debug, Deserialize)]
    struct Category {
        title: String,
    }

    impl From<Category> for String {
        fn from(value: Category) -> Self {
            value
                .title
                .split_once("Category:")
                .map(|(_, title)| title.to_owned())
                .unwrap_or(value.title)
        }
    }

    #[derive(Debug, Deserialize)]
    struct ApiAllPageContinueParams {
        gapcontinue: Option<String>,
        clcontinue: Option<String>,
    }

    let api_url =
        "https://wiki.archlinux.org/api.php?action=query&generator=allpages&prop=categories&format=json&gaplimit=max&cllimit=max";

    let mut pages: Vec<Page> = vec![];

    let body = reqwest::get(api_url).await?.text().await?;
    let mut api_resp: ResponseWithContinue<ApiAllPagesQuery, ApiAllPageContinueParams> =
        serde_json::from_str(&body)?;

    pages.append(&mut api_resp.query.pages.into_values().collect());

    while let Some(continue_params) = api_resp.r#continue {
        let next_api_url = if let Some(gapcontinue) = continue_params.gapcontinue {
            format!("{api_url}&gapcontinue={gapcontinue}")
        } else if let Some(clcontinue) = continue_params.clcontinue {
            format!("{api_url}&clcontinue={clcontinue}")
        } else {
            break;
        };

        let body = reqwest::get(&next_api_url).await?.text().await?;
        api_resp = serde_json::from_str(&body)?;

        pages.append(&mut api_resp.query.pages.into_values().collect());
    }

    let page_category_tree = pages.into_iter().map(|page| {
        (
            page.title,
            page.categories
                .map(|cats| {
                    cats.into_iter()
                        .map::<String, _>(Into::into)
                        .filter(|cat| !is_blocked_category(cat))
                        .collect()
                })
                .unwrap_or_default(),
        )
    });

    Ok(page_category_tree.collect())
}

fn is_blocked_category(category: &str) -> bool {
    BLOCK_LISTED_CATEGORY_PREFIXES
        .iter()
        .any(|blocked_prefix| category.starts_with(blocked_prefix))
}
