use core::panic;
use std::collections::HashMap;

use futures::future;
use indicatif::{MultiProgress, ProgressBar};
use itertools::Itertools;
use scraper::Html;
use serde::Deserialize;
use url::Url;

use crate::{
    error::WikiError,
    search::{
        open_search_is_page_exact_match, open_search_to_page_names, OpenSearchItem,
        TextSearchApiResponse, TextSearchItem,
    },
    utils::update_relative_urls,
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub query: T,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiResponseWithContinue<T, V> {
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
    let mut res: ApiResponse<TextSearchApiResponse> = serde_json::from_str(&body)?;

    for item in res.query.search.as_mut_slice() {
        item.prettify_snippet(search);
    }

    Ok(res.query.search)
}

/// Gets the HTML content of an ArchWiki page.
///
/// If the ArchWiki page doesn't exists the top 5 pages that are most
/// like the page that was given as an argument are returned as a `NoPageFound` error.
pub async fn fetch_page(page: &str, lang: Option<&str>) -> Result<Html, WikiError> {
    let lang = lang.unwrap_or("en");
    let search_res = fetch_open_search(page, lang, 5).await?;

    let Some(page_title) = open_search_is_page_exact_match(page, &search_res)? else {
        let similar_pages = open_search_to_page_names(&search_res)?;
        return Err(WikiError::NoPageFound(similar_pages.join("\n")));
    };

    let raw_url = format!(
        "https://wiki.archlinux.org/rest.php/v1/page/{title}/html",
        title = urlencoding::encode(page_title)
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
pub async fn fetch_page_by_url(url: Url) -> Result<Html, WikiError> {
    let base_url = format!(
        "{schema}://{host}",
        schema = url.scheme(),
        host = url.host_str().unwrap_or("")
    );

    let body = reqwest::get(url).await?.text().await?;
    let body_with_abs_urls = update_relative_urls(&body, &base_url);

    Ok(Html::parse_document(&body_with_abs_urls))
}

/// Gets a list of all ArchWiki categories and the pages inside them.
/// All categories are treated as top-level and sub categories are ignored.
pub async fn fetch_wiki_tree(
    thread_count: usize,
    delay: u64,
    hide_progress: bool,
) -> Result<HashMap<String, Vec<String>>, WikiError> {
    let categories = fetch_all_categories().await?;

    let multi_bar = MultiProgress::new();
    let chunk_count = categories.len() / thread_count;

    let tasks = categories
        .chunks(chunk_count)
        .map(|chunk| {
            let chunk = chunk.to_vec();

            let bar = ProgressBar::new(chunk.len().try_into().unwrap_or(0));
            let bar = multi_bar.add(bar);
            if hide_progress {
                bar.finish_and_clear();
            }

            tokio::spawn(async move {
                let mut wiki_sectoin = HashMap::new();
                for category in chunk {
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;

                    let pages = match fetch_pages_in_category(&category).await {
                        Ok(pages) => pages,
                        Err(_) => {
                            // wait if rate limited
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            fetch_pages_in_category(&category)
                                .await
                                .unwrap_or_else(|err| {
                                    eprintln!(
                                        "failed to fetch pages in category {}\n ERROR {err}",
                                        category
                                    );
                                    vec![]
                                })
                        }
                    };

                    if !pages.is_empty() {
                        wiki_sectoin.insert(category.to_owned(), pages);
                    }
                    bar.inc(1);
                }

                wiki_sectoin
            })
        })
        .collect_vec();

    let mut wiki = HashMap::new();
    let sections = future::join_all(tasks).await;

    for section in sections {
        match section {
            Ok(data) => {
                wiki.extend(data);
            }
            Err(err) => panic!("failed to sync wiki\nERROR: {err}"),
        }
    }

    Ok(wiki)
}

pub async fn fetch_all_pages() -> Result<Vec<String>, WikiError> {
    #[derive(Debug, Deserialize)]
    struct ApiAllPagesQuery {
        allpages: Vec<Page>,
    }

    #[derive(Debug, Deserialize)]
    struct Page {
        title: String,
    }

    impl From<Page> for String {
        fn from(value: Page) -> Self {
            value.title
        }
    }

    #[derive(Debug, Deserialize)]
    struct ApiAllPageContinueParams {
        apcontinue: String,
    }

    let api_url =
        "https://wiki.archlinux.org/api.php?action=query&list=allpages&format=json&aplimit=500";

    let mut pages: Vec<String> = vec![];

    let body = reqwest::get(api_url).await?.text().await?;
    let mut api_resp: ApiResponseWithContinue<ApiAllPagesQuery, ApiAllPageContinueParams> =
        serde_json::from_str(&body)?;

    pages.append(
        &mut api_resp
            .query
            .allpages
            .into_iter()
            .map(Into::into)
            .collect(),
    );

    while let Some(continue_params) = api_resp.r#continue {
        let next_api_url = format!("{api_url}&apcontinue={}", continue_params.apcontinue);

        let body = reqwest::get(&next_api_url).await?.text().await?;
        api_resp = serde_json::from_str(&body)?;

        pages.append(
            &mut api_resp
                .query
                .allpages
                .into_iter()
                .map(Into::into)
                .collect(),
        );
    }

    Ok(pages)
}

async fn fetch_all_categories() -> Result<Vec<String>, WikiError> {
    #[derive(Debug, Deserialize)]
    struct ApiAllCategoriesQuery {
        allcategories: Vec<Category>,
    }

    #[derive(Debug, Deserialize)]
    struct Category {
        #[serde[rename = "*"]]
        name: String,
    }

    impl From<Category> for String {
        fn from(value: Category) -> Self {
            value.name
        }
    }

    #[derive(Debug, Deserialize)]
    struct ApiAllCategoryContinueParams {
        accontinue: String,
    }

    let api_url = "https://wiki.archlinux.org/api.php?action=query&list=allcategories&format=json&aclimit=500";

    let mut categories: Vec<String> = vec![];

    let body = reqwest::get(api_url).await?.text().await?;
    let mut api_resp: ApiResponseWithContinue<ApiAllCategoriesQuery, ApiAllCategoryContinueParams> =
        serde_json::from_str(&body)?;

    categories.append(
        &mut api_resp
            .query
            .allcategories
            .into_iter()
            .map(Into::into)
            .collect(),
    );

    while let Some(continue_params) = api_resp.r#continue {
        let next_api_url = format!("{api_url}&accontinue={}", continue_params.accontinue);

        let body = reqwest::get(&next_api_url).await?.text().await?;
        api_resp = serde_json::from_str(&body)?;

        categories.append(
            &mut api_resp
                .query
                .allcategories
                .into_iter()
                .map(Into::into)
                .collect(),
        );
    }

    Ok(categories)
}

async fn fetch_pages_in_category(category: &str) -> Result<Vec<String>, WikiError> {
    #[derive(Debug, Deserialize)]
    struct ApiCategoryMembersQuery {
        categorymembers: Vec<Page>,
    }

    #[derive(Debug, Deserialize)]
    struct Page {
        title: String,
    }

    impl From<Page> for String {
        fn from(value: Page) -> Self {
            value.title
        }
    }

    let api_url = format!("https://wiki.archlinux.org/api.php?action=query&list=categorymembers&format=json&cmtype=page&cmlimit=500&cmtitle=Category:{title}", title = urlencoding::encode(category));

    let body = reqwest::get(api_url).await?.text().await?;
    let api_resp: ApiResponse<ApiCategoryMembersQuery> = serde_json::from_str(&body)?;

    Ok(api_resp
        .query
        .categorymembers
        .into_iter()
        .map(Into::into)
        .collect())
}
