use std::{
    collections::HashMap,
    fs,
    io::{self, ErrorKind},
    path::Path,
};

use itertools::Itertools;
use scraper::node::Element;

use crate::error::WikiError;

pub const UNCATEGORIZED_KEY: &str = "Uncategorized";

pub fn archwiki_url_page_title(str: &str) -> Option<&str> {
    Some(str.split_once("https://wiki.archlinux.org/title/")?.1)
}

pub fn extract_tag_attr(element: &Element, tag: &str, attr: &str) -> Option<String> {
    if element.name() == tag {
        element.attr(attr).map(ToOwned::to_owned)
    } else {
        None
    }
}

/// Replaces relative URLs in certain HTML attributes with absolute URLs.
/// The list of attributes is taken from <https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes>
/// Note:
/// - `/` matter, check implementation
pub fn update_relative_urls(html: &str, root_url: &str, path: &str) -> String {
    html.replace("href=\"/", &format!("href=\"{root_url}/"))
        .replace("src=\"/", &format!("src=\"{root_url}/"))
        .replace("data=\"/", &format!("data=\"{root_url}/"))
        .replace("manifest=\"/", &format!("manifest=\"{root_url}/"))
        .replace("ping=\"/", &format!("ping=\"{root_url}/"))
        .replace("poster=\"/", &format!("poster=\"{root_url}/"))
        .replace("href=\"./", &format!("href=\"{root_url}/{path}/"))
        .replace("src=\"./", &format!("src=\"{root_url}/{path}/"))
        .replace("data=\"./", &format!("data=\"{root_url}/{path}/"))
        .replace("manifest=\"./", &format!("manifest=\"{root_url}/{path}/"))
        .replace("ping=\"./", &format!("ping=\"{root_url}/{path}/"))
        .replace("poster=\"./", &format!("poster=\"{root_url}{path}/"))
}

#[cfg(feature = "cli")]
pub fn read_pages_as_tree(
    path: &Path,
    is_default_path: bool,
) -> Result<HashMap<String, Vec<String>>, WikiError> {
    let content = fs::read_to_string(path).map_err(|err| {
        match err.kind() {
            ErrorKind::NotFound =>  {
                let path_str = path.to_string_lossy();
                let extra_path_arg = if is_default_path {
                    String::new()
                } else {
                    format!(" --out-file {path_str}")
                };

                WikiError::IO(io::Error::new(ErrorKind::NotFound,  format!("Could not find pages file at '{path_str}'. Try running 'archwiki-rs sync-wiki{extra_path_arg}' to create the missing file." )))
            }
            _ => err.into()
        }
    })?;

    let page_to_category_map: HashMap<String, Vec<String>> = serde_yaml::from_str(&content)?; // also parses JSON files
    Ok(flip_page_tree(page_to_category_map))
}

pub fn flip_page_tree(
    page_to_category_map: HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut category_to_page_map = HashMap::new();
    let mut uncategorized_pages = vec![];

    for (page, cats) in page_to_category_map.into_iter().collect_vec() {
        if cats.is_empty() {
            uncategorized_pages.push(page);
        } else {
            for cat in cats {
                let mut pages: Vec<String> =
                    category_to_page_map.get(&cat).cloned().unwrap_or_default();
                pages.push(page.clone());

                category_to_page_map.insert(cat, pages);
            }
        }
    }

    if !uncategorized_pages.is_empty() {
        for (i, uncategoriesed_chunk) in uncategorized_pages
            .into_iter()
            .sorted()
            .chunks(500)
            .into_iter()
            .enumerate()
        {
            let key = format!("{UNCATEGORIZED_KEY} #{n}", n = i + 1);
            category_to_page_map.insert(key, uncategoriesed_chunk.collect_vec());
        }
    }

    category_to_page_map
}
