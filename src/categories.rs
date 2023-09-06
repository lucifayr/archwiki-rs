use indicatif::ProgressBar;
use itertools::Itertools;
use scraper::{Html, Node, Selector};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Clone)]
struct CategoryListItem {
    name: String,
    url: String,
}

use crate::{
    error::WikiError,
    utils::{extract_tag_attr, get_elements_by_tag, HtmlTag},
    wiki_api::fetch_page_by_url,
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
pub async fn fetch_all_pages(
    hide_progress: bool,
) -> Result<HashMap<String, Vec<String>>, WikiError> {
    let url = "https://wiki.archlinux.org/index.php?title=Special:Categories&offset=&limit=10000";
    let document = fetch_page_by_url(
        Url::parse(url).unwrap_or_else(|_| panic!("{url} should be a valid url")),
    )
    .await?;

    let body_class = ".mw-spcontent";
    let selector = Selector::parse(body_class)
        .unwrap_or_else(|_| panic!("{body_class} should be valid selector"));

    let body = document.select(&selector).next().unwrap();

    let category_list_element = get_elements_by_tag(*body, &HtmlTag::Ul)
        .into_iter()
        .next()
        .unwrap();

    let items = parse_category_list(category_list_element)
        .into_iter()
        .collect_vec();

    let mut wiki_map = HashMap::new();

    let bar = ProgressBar::new(items.len().try_into().unwrap_or(0));

    if hide_progress {
        bar.finish_and_clear();
    }

    for item in items {
        let pages = fetch_page_names_from_categoriy(&item.url).await?;
        wiki_map.insert(item.name, pages);

        bar.inc(1);
    }

    Ok(wiki_map)
}

fn parse_category_list(list_node: ego_tree::NodeRef<'_, scraper::Node>) -> Vec<CategoryListItem> {
    let list_items = get_elements_by_tag(list_node, &HtmlTag::Li);
    list_items
        .into_iter()
        .flat_map(|li| {
            let a_tag = li.first_child()?;
            let a_tag_element = a_tag.value().as_element()?;

            let name = a_tag.first_child()?.value().as_text()?.to_string();
            let url = extract_tag_attr(&a_tag_element, &HtmlTag::A, "href")?;

            Some(CategoryListItem { name, url })
        })
        .collect()
}

/// Scrape the ArchWiki for a list of all page names that belong to a specific category
async fn fetch_page_names_from_categoriy(url_str: &str) -> Result<Vec<String>, WikiError> {
    let selector = Selector::parse("#mw-pages").expect("#mw-pages to be a valid css selector");

    let body = reqwest::get(url_str).await?.text().await?;
    let document = Html::parse_document(&body);

    let Some(page_container) =  document.select(&selector).next() else {
        return Ok(vec![])
    };

    Ok(page_container
        .descendants()
        .filter_map(|node| {
            if let Node::Element(e) = node.value() {
                extract_tag_attr(e, &HtmlTag::A, "title")
            } else {
                None
            }
        })
        .collect())
}
