use itertools::Itertools;
use scraper::{Html, Node, Selector};
use std::collections::HashMap;

use crate::{
    error::WikiError,
    utils::{extract_tag_attr, HtmlTag},
    wiki_api::fetch_page,
};

/// Returns a print ready list of the provided page names in
/// 1. A tree format if `flatten` is `false`:
/// Xfce:
/// ───┤Thunar
/// ───┤Xfce
/// ───┤Xfwm
///
/// Xiaomi:
/// ───┤Xiaomi Mi Notebook Air 13.3
/// ───┤Xiaomi Mi Notebook Pro 15.6
///
/// 2. A newline separated list if `flatten` is `true`:
/// Xsettingsd
/// Xsettingsd
/// Xterm
/// Xtrabackup
///
/// Sorting behavior depends on if the list is flattened or not.
///
/// If it is not flattened the list is first ordered by category names and then by page names withing those
/// categories.
/// If it is flattened then it will by sorted by page names.
pub fn list_pages(categories: &HashMap<String, Vec<String>>, flatten: bool) -> String {
    if flatten {
        return categories.values().flatten().unique().sorted().join("\n");
    }

    categories
        .iter()
        .sorted()
        .map(|(cat, pages)| {
            let list = pages.iter().map(|p| format!("───┤{p}")).join("\n");

            format!("{cat}:\n{list}",)
        })
        .join("\n\n")
}

/// Scrapes the ArchWiki for all page names and their immediate parent category. Category nesting
/// is ignored as a category can be a sub category of multiple other categories.
///
/// Caution this function will most likely take several minutes to finish (-, – )…zzzZZ
pub async fn fetch_all_page_names() -> Result<HashMap<String, Vec<String>>, WikiError> {
    let document = fetch_page("Table_of_contents", None).await?;
    let selector =
        Selector::parse(".mw-parser-output").expect(".mw-parser-output to be a valid css selector");

    let categories = match document.select(&selector).next() {
        Some(next) => next,
        None => return Err(WikiError::Html("No categories found".to_owned())),
    };

    let cat_hrefs = categories
        .descendants()
        .filter_map(|node| {
            if let Node::Element(e) = node.value() {
                extract_tag_attr(e, &HtmlTag::A, "href")
            } else {
                None
            }
        })
        .skip(1)
        .collect::<Vec<String>>();

    let mut pages = HashMap::new();
    for cat in cat_hrefs {
        let cat_name = cat.split(':').last().unwrap_or("");
        let res = fetch_page_names_from_categoriy(cat_name).await;
        pages.insert(cat_name.to_owned(), res.unwrap_or(Vec::new()));
    }

    Ok(pages)
}

/// Scrape the ArchWiki for a list of all page names that belong to a specific category
pub async fn fetch_page_names_from_categoriy(category: &str) -> Option<Vec<String>> {
    let selector = Selector::parse("#mw-pages").expect("#mw-pages to be a valid css selector");

    let url = format!("https://wiki.archlinux.org/title/Category:{category}");
    let body = reqwest::get(&url).await.ok()?.text().await.ok()?;
    let document = Html::parse_document(&body);

    Some(
        document
            .select(&selector)
            .next()?
            .descendants()
            .filter_map(|node| {
                if let Node::Element(e) = node.value() {
                    extract_tag_attr(e, &HtmlTag::A, "title")
                } else {
                    None
                }
            })
            .collect::<Vec<String>>(),
    )
}
