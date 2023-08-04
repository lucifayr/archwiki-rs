#![allow(dead_code)]

use std::fs;

use directories::BaseDirs;
use ego_tree::NodeRef;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use scraper::{node::Element, ElementRef, Html, Node, Selector};

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

pub fn extract_tag_attr(element: &Element, tag: &HtmlTag, attr: &str) -> Option<String> {
    if element.name() == tag.name() {
        element.attr(attr).map(|attr| attr.to_owned())
    } else {
        None
    }
}

pub fn find_tag<'a>(node: NodeRef<'a, Node>, tag: &'a HtmlTag) -> Option<&'a Element> {
    for child in node.children() {
        if let Node::Element(e) = child.value() {
            println!("{}", e.name());
        }
        match child.value() {
            Node::Element(e) if e.name() == tag.name() => return Some(e),
            _ => {
                if let Some(element) = find_tag(child, tag) {
                    return Some(element);
                }
            }
        }
    }

    None
}

pub fn get_page_content(document: &Html) -> Option<ElementRef<'_>> {
    let selector =
        Selector::parse(".mw-parser-output").expect(".mw-parser-output should be valid selector");
    document.select(&selector).next()
}

pub fn update_relative_urls(html: &str) -> String {
    html.replace("href=\"/", "href=\"https://wiki.archlinux.org/")
}

pub async fn fetch_page(page: &str) -> Result<Html, reqwest::Error> {
    fetch_html(&format!("https://wiki.archlinux.org/title/{page}",)).await
}

pub async fn fetch_html(url: &str) -> Result<Html, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
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
