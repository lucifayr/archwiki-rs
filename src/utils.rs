#![allow(dead_code)]

use std::{fs, path::Path};

use directories::BaseDirs;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use scraper::{node::Element, ElementRef, Html, Selector};

use crate::{error::WikiError, formats::PageFormat};

pub enum HtmlTag {
    A,
}

impl HtmlTag {
    pub fn name(&self) -> String {
        match *self {
            HtmlTag::A => "a".to_owned(),
        }
    }
}

/// Selects the body of an ArchWiki page
pub fn get_page_content(document: &Html) -> Option<ElementRef<'_>> {
    let selector =
        Selector::parse(".mw-parser-output").expect(".mw-parser-output should be valid selector");
    document.select(&selector).next()
}

/// Gets an ArchWiki pages entire content. Also updates all relative URLs to absolute URLs.
/// `/title/Neovim` -> `https://wiki.archlinux.org/title/Neovim`
pub async fn fetch_page(page: &str) -> Result<Html, reqwest::Error> {
    let url = format!("https://wiki.archlinux.org/title/{page}");

    let body = reqwest::get(&url).await?.text().await?;
    let body_with_abs_urls = update_relative_urls(&body);

    Ok(Html::parse_document(&body_with_abs_urls))
}

/// Check if a page has been cached. Different page formats are cached separately.
/// If a page has existed for more then 14 days and `disable_cache_invalidation` is false
/// this function will return false even if a cache file exists.
pub fn page_cache_exists(
    page: &str,
    format: &PageFormat,
    cache_dir_path: &Path,
    disable_cache_invalidation: bool,
) -> Result<bool, WikiError> {
    let ext = match format {
        PageFormat::PlainText => "",
        PageFormat::Markdown => "md",
        PageFormat::Html => "html",
    };

    let page_cache_path = cache_dir_path.join(page).with_extension(ext);
    if !page_cache_path.exists() {
        return Ok(false);
    } else if disable_cache_invalidation {
        return Ok(true);
    }

    let fourteen_days = 1209600;
    let secs_since_modified = fs::File::open(page_cache_path)?
        .metadata()?
        .modified()?
        .elapsed()?
        .as_secs();

    Ok(secs_since_modified < fourteen_days)
}

/// Read a page from the page cache. Different page formats are cached separately.
pub fn read_page_from_cache(
    page: &str,
    format: &PageFormat,
    cache_dir_path: &Path,
) -> Result<String, WikiError> {
    let ext = match format {
        PageFormat::PlainText => "",
        PageFormat::Markdown => "md",
        PageFormat::Html => "html",
    };

    let page_cache_path = cache_dir_path.join(page).with_extension(ext);
    Ok(fs::read_to_string(page_cache_path)?)
}

/// Write a page to the page cache. Different page formats are cached separately.
pub fn write_page_to_cache(
    data: String,
    page: &str,
    format: &PageFormat,
    cache_dir_path: &Path,
) -> Result<(), WikiError> {
    fs::create_dir_all(cache_dir_path)?;

    let ext = match format {
        PageFormat::PlainText => "",
        PageFormat::Markdown => "md",
        PageFormat::Html => "html",
    };

    let page_cache_path = cache_dir_path.join(page).with_extension(ext);
    fs::write(page_cache_path, data.as_bytes())?;
    Ok(())
}

pub fn get_top_pages<'a>(search: &str, amount: usize, pages: &[&'a str]) -> Vec<&'a str> {
    let matcher = SkimMatcherV2::default();
    let mut ranked_pages = pages
        .iter()
        .map(|page| (matcher.fuzzy_match(page, search).unwrap_or(0), *page))
        .collect::<Vec<(i64, &str)>>();

    ranked_pages.sort_by(|a, b| a.0.cmp(&b.0));
    ranked_pages
        .into_iter()
        .rev()
        .take(amount)
        .map(|e| e.1)
        .collect()
}

pub fn extract_tag_attr(element: &Element, tag: &HtmlTag, attr: &str) -> Option<String> {
    if element.name() == tag.name() {
        element.attr(attr).map(|attr| attr.to_owned())
    } else {
        None
    }
}

pub fn create_data_dir(path: &str) -> Result<(), WikiError> {
    fs::create_dir_all(path)?;
    Ok(())
}

// these functions should use path buffers instead of strings

pub fn get_data_dir_path(base_dir: &BaseDirs) -> Result<String, WikiError> {
    let postfix = if cfg!(windows) {
        "\\archwiki-rs"
    } else {
        "/archwiki-rs"
    };

    match base_dir.data_local_dir().to_str() {
        Some(path) => Ok(path.to_owned() + postfix),
        None => Err(WikiError::Path(
            "Failed to convert path to string".to_owned(),
        )),
    }
}

pub fn get_cache_dir_path(base_dir: BaseDirs) -> Result<String, WikiError> {
    let postfix = if cfg!(windows) {
        "\\archwiki-rs"
    } else {
        "/archwiki-rs"
    };

    match base_dir.cache_dir().to_str() {
        Some(path) => Ok(path.to_owned() + postfix),
        None => Err(WikiError::Path(
            "Failed to convert path to string".to_owned(),
        )),
    }
}

fn update_relative_urls(html: &str) -> String {
    html.replace("href=\"/", "href=\"https://wiki.archlinux.org/")
}
