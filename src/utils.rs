#![allow(dead_code)]

use std::{
    fs,
    path::{Path, PathBuf},
};

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

/// Construct a path to cache a page. Different page formats are cached separately.
/// Slashes are escaped with the unicode character ∕.
pub fn create_page_path_path(page: &str, format: &PageFormat, cache_dir: &Path) -> PathBuf {
    let ext = match format {
        PageFormat::PlainText => "",
        PageFormat::Markdown => "md",
        PageFormat::Html => "html",
    };

    cache_dir.join(&page.replace("/", "∕")).with_extension(ext)
}

/// Check if a page has been cached.
/// If a page has existed for more then 14 days and `disable_cache_invalidation` is false
/// this function will return false even if a cache file exists.
pub fn page_cache_exists(
    cache_location: &Path,
    disable_cache_invalidation: bool,
) -> Result<bool, WikiError> {
    if !cache_location.exists() {
        return Ok(false);
    } else if disable_cache_invalidation {
        return Ok(true);
    }

    let fourteen_days = 1209600;
    let secs_since_modified = fs::File::open(cache_location)?
        .metadata()?
        .modified()?
        .elapsed()?
        .as_secs();

    Ok(secs_since_modified < fourteen_days)
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

/// Replaces relative URLs in certain HTML attributes with absolute URLs.
/// The list of attributes is taken from https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes
fn update_relative_urls(html: &str) -> String {
    html.replace("href=\"/", "href=\"https://wiki.archlinux.org/")
        .replace("src=\"/", "src=\"https://wiki.archlinux.org/")
        .replace("data=\"/", "data=\"https://wiki.archlinux.org/")
        .replace("manifest=\"/", "manifest=\"https://wiki.archlinux.org/")
        .replace("ping=\"/", "ping=\"https://wiki.archlinux.org/")
        .replace("poster=\"/", "poster=\"https://wiki.archlinux.org/")
}
