use std::{
    collections::HashMap,
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use itertools::Itertools;
use scraper::node::Element;

use crate::{error::WikiError, formats::PageFormat};

pub const UNCATEGORIZED_KEY: &str = "Uncategorized";

/// Construct a path to cache a page. Different page formats are cached separately.
/// All none word characters are escaped with an '_'
pub fn create_cache_page_path(page: &str, format: &PageFormat, cache_dir: &Path) -> PathBuf {
    let ext = match format {
        PageFormat::PlainText => "",
        PageFormat::Markdown => "md",
        PageFormat::Html => "html",
    };

    cache_dir.join(to_save_file_name(page)).with_extension(ext)
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

pub fn extract_tag_attr(element: &Element, tag: &str, attr: &str) -> Option<String> {
    if element.name() == tag {
        element.attr(attr).map(|attr| attr.to_owned())
    } else {
        None
    }
}

/// Replaces relative URLs in certain HTML attributes with absolute URLs.
/// The list of attributes is taken from https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes
pub fn update_relative_urls(html: &str, base_url: &str) -> String {
    html.replace("href=\"/", &format!("href=\"{base_url}/"))
        .replace("src=\"/", &format!("src=\"{base_url}/"))
        .replace("data=\"/", &format!("data=\"{base_url}/"))
        .replace("manifest=\"/", &format!("manifest=\"{base_url}/"))
        .replace("ping=\"/", &format!("ping=\"{base_url}/"))
        .replace("poster=\"/", &format!("poster=\"{base_url}/"))
}

pub fn read_pages_file_as_category_tree(
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

    let page_to_category_map: HashMap<String, Vec<String>> = serde_yaml::from_str(&content)?;

    let mut category_to_page_map = HashMap::new();
    let mut uncategorized_pages = vec![];

    for (page, cats) in page_to_category_map.into_iter().collect_vec() {
        if cats.is_empty() {
            uncategorized_pages.push(page)
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

    Ok(category_to_page_map)
}

pub fn to_save_file_name(page: &str) -> String {
    sanitize_filename::sanitize(page)
}

pub fn truncate_unicode_str(n: usize, text: &str) -> String {
    let mut count = 0;
    let mut res = vec![];
    let mut chars = text.chars();

    while count < n {
        if let Some(char) = chars.next() {
            count += unicode_width::UnicodeWidthChar::width(char).unwrap_or(0);
            res.push(char);
        } else {
            break;
        }
    }

    res.into_iter().collect::<String>()
}

pub fn page_path(page: &str, format: &PageFormat, parent_dir: &Path) -> PathBuf {
    let ext = match format {
        PageFormat::PlainText => "",
        PageFormat::Markdown => "md",
        PageFormat::Html => "html",
    };

    parent_dir.join(to_save_file_name(page)).with_extension(ext)
}

pub fn create_dir_if_not_exists(dir: &Path) -> Result<(), WikiError> {
    match fs::create_dir(dir) {
        Ok(_) => {}
        Err(err) => {
            if err.kind() != io::ErrorKind::AlreadyExists {
                return Err(err.into());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_to_save_file_name() {
        let cases = [
            ("Neovim", "Neovim"),
            ("3D Mouse", "3D Mouse"),
            ("/etc/fstab", "etcfstab"),
            (".NET", ".NET"),
            ("ASUS MeMO Pad 7 (ME176C(X))", "ASUS MeMO Pad 7 (ME176C(X))"),
        ];

        for (input, output) in cases {
            assert_eq!(output, to_save_file_name(input));
        }
    }
}
