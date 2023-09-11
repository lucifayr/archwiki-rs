use std::{
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use ego_tree::NodeRef;
use regex::Regex;
use scraper::{node::Element, ElementRef, Html, Node, Selector};

use crate::{error::WikiError, formats::PageFormat};

pub const PAGE_CONTENT_CLASS: &str = "mw-parser-output";

pub enum HtmlTag {
    A,
    Ul,
    Li,
}

impl HtmlTag {
    pub fn name(&self) -> String {
        match *self {
            HtmlTag::A => "a".to_owned(),
            HtmlTag::Ul => "ul".to_owned(),
            HtmlTag::Li => "li".to_owned(),
        }
    }
}

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

/// Selects the body of an ArchWiki page
pub fn get_page_content(document: &Html) -> Option<ElementRef<'_>> {
    let class = format!(".{PAGE_CONTENT_CLASS}");
    let selector =
        Selector::parse(&class).unwrap_or_else(|_| panic!("{class} should be valid selector"));
    document.select(&selector).next()
}

pub fn get_elements_by_tag<'a>(root: NodeRef<'a, Node>, tag: &HtmlTag) -> Vec<NodeRef<'a, Node>> {
    root.children()
        .flat_map(|n| {
            if let Node::Element(e) = n.value() {
                if e.name() == tag.name() {
                    Some(n)
                } else {
                    None
                }
            } else {
                None
            }
        })
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
pub fn update_relative_urls(html: &str, base_url: &str) -> String {
    html.replace("href=\"/", &format!("href=\"{base_url}/"))
        .replace("src=\"/", &format!("src=\"{base_url}/"))
        .replace("data=\"/", &format!("data=\"{base_url}/"))
        .replace("manifest=\"/", &format!("manifest=\"{base_url}/"))
        .replace("ping=\"/", &format!("ping=\"{base_url}/"))
        .replace("poster=\"/", &format!("poster=\"{base_url}/"))
}

pub fn read_pages_file_as_str(path: PathBuf) -> Result<String, WikiError> {
    fs::read_to_string(&path).map_err(|err| {
        match err.kind() {
            ErrorKind::NotFound => WikiError::IO(io::Error::new(ErrorKind::NotFound,  format!("Could not find pages file at '{}'. Try running 'archwiki-rs sync-wiki' to create the missing file.", path.to_string_lossy()))),
            _ => err.into()
        }
    })
}

fn to_save_file_name(page: &str) -> String {
    let regex = Regex::new("[^-0-9A-Za-z_]").expect("'[^0-9A-Za-z_]' should be a valid regex");
    regex.replace_all(page, "_").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_to_save_file_name() {
        let cases = [
            ("Neovim", "Neovim"),
            ("3D Mouse", "3D_Mouse"),
            ("/etc/fstab", "_etc_fstab"),
            (".NET", "_NET"),
            ("ASUS MeMO Pad 7 (ME176C(X))", "ASUS_MeMO_Pad_7__ME176C_X__"),
        ];

        for (input, output) in cases {
            assert_eq!(output, to_save_file_name(input));
        }
    }
}
