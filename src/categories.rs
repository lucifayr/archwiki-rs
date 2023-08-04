use itertools::Itertools;
use scraper::{Node, Selector};
use std::collections::HashMap;

use crate::{
    error::WikiError,
    utils::{extract_tag_attr, fetch_html, fetch_page, HtmlTag},
};

pub fn list_categories(categories: &HashMap<String, Vec<String>>, flatten: bool) -> String {
    if flatten {
        return categories.values().flatten().sorted().join("\n");
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

pub async fn fetch_all_page_names() -> Result<HashMap<String, Vec<String>>, WikiError> {
    let document = fetch_page("Table_of_contents").await?;
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

pub async fn fetch_page_names_from_categoriy(category: &str) -> Option<Vec<String>> {
    let selector = Selector::parse("#mw-pages").expect("#mw-pages to be a valid css selector");
    let document = fetch_html(&format!(
        "https://wiki.archlinux.org/title/Category:{category}"
    ))
    .await
    .ok()?;

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
