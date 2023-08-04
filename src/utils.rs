#![allow(dead_code)]

use std::fs;

use directories::BaseDirs;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use scraper::{node::Element, ElementRef, Html, Selector};

use crate::error::WikiError;

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

pub fn get_data_dir_path(base_dir: BaseDirs) -> Result<String, WikiError> {
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

fn update_relative_urls(html: &str) -> String {
    html.replace("href=\"/", "href=\"https://wiki.archlinux.org/")
}
