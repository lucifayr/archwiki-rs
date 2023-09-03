use std::{
    fs,
    path::{Path, PathBuf},
};

use itertools::Itertools;
use regex::Regex;
use scraper::{node::Element, ElementRef, Html, Selector};

use crate::{
    error::{InvalidApiResponseError, WikiError},
    formats::PageFormat,
    wiki_api::{fetch_open_search, OpenSearchItem},
};

pub const PAGE_CONTENT_CLASS: &str = "mw-parser-output";

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
    let class = format!(".{PAGE_CONTENT_CLASS}");
    let selector = Selector::parse(&class).expect(&format!("{class} should be valid selector"));
    document.select(&selector).next()
}

pub fn open_search_to_page_names(
    search_result: &[OpenSearchItem],
) -> Result<Vec<String>, WikiError> {
    let page_names = search_result.get(1).ok_or(WikiError::InvalidApiResponse(
        InvalidApiResponseError::OpenSearchMissingNthElement(1),
    ))?;

    if let OpenSearchItem::Array(names) = page_names {
        Ok(names.to_owned())
    } else {
        Err(WikiError::InvalidApiResponse(
            InvalidApiResponseError::OpenSearchNthElementShouldBeArray(1),
        ))
    }
}

/// Convert an open search response into a list of name and URL pairs
///
/// Errors:
/// - If the search results don't have an array as the 1. and 3. elements in the list
/// - If the arrays in the search results have different lengths
pub fn open_search_to_page_url_tupel(
    search_result: &[OpenSearchItem],
) -> Result<Vec<(String, String)>, WikiError> {
    let page_names = search_result.get(1).ok_or(WikiError::InvalidApiResponse(
        InvalidApiResponseError::OpenSearchMissingNthElement(1),
    ))?;

    let page_urls = search_result.get(3).ok_or(WikiError::InvalidApiResponse(
        InvalidApiResponseError::OpenSearchMissingNthElement(3),
    ))?;

    if let OpenSearchItem::Array(names) = page_names {
        if let OpenSearchItem::Array(urls) = page_urls {
            if names.len() != urls.len() {
                return Err(WikiError::InvalidApiResponse(
                    InvalidApiResponseError::OpenSearchArraysLengthMismatch,
                ));
            }

            Ok(names
                .iter()
                .zip(urls)
                .map(|(a, b)| (a.to_owned(), b.to_owned()))
                .collect_vec())
        } else {
            Err(WikiError::InvalidApiResponse(
                InvalidApiResponseError::OpenSearchNthElementShouldBeArray(3),
            ))
        }
    } else {
        Err(WikiError::InvalidApiResponse(
            InvalidApiResponseError::OpenSearchNthElementShouldBeArray(1),
        ))
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

pub async fn search_for_similar_pages(
    search: &str,
    lang: Option<&str>,
    limit: Option<u16>,
) -> Result<Vec<String>, WikiError> {
    let lang = lang.unwrap_or("en");
    let limit = limit.unwrap_or(5);

    let search_res = fetch_open_search(search, lang, limit).await?;
    open_search_to_page_names(&search_res)
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
pub fn update_relative_urls(html: &str) -> String {
    html.replace("href=\"/", "href=\"https://wiki.archlinux.org/")
        .replace("src=\"/", "src=\"https://wiki.archlinux.org/")
        .replace("data=\"/", "data=\"https://wiki.archlinux.org/")
        .replace("manifest=\"/", "manifest=\"https://wiki.archlinux.org/")
        .replace("ping=\"/", "ping=\"https://wiki.archlinux.org/")
        .replace("poster=\"/", "poster=\"https://wiki.archlinux.org/")
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

    #[test]
    fn test_process_open_search() {
        let valid_input = vec![
            OpenSearchItem::Single("test".to_owned()),
            OpenSearchItem::Array(vec!["name 1".to_owned(), "name 2".to_owned()]),
            OpenSearchItem::Array(vec![]),
            OpenSearchItem::Array(vec!["url 1".to_owned(), "url 2".to_owned()]),
        ];

        let missing_elements = vec![OpenSearchItem::Single("test".to_owned())];
        let not_arrays = vec![
            OpenSearchItem::Single("test".to_owned()),
            OpenSearchItem::Array(vec!["name 1".to_owned(), "name 2".to_owned()]),
            OpenSearchItem::Array(vec![]),
            OpenSearchItem::Single("invalid".to_owned()),
        ];
        let different_lengths = vec![
            OpenSearchItem::Single("test".to_owned()),
            OpenSearchItem::Array(vec!["name 1".to_owned()]),
            OpenSearchItem::Array(vec![]),
            OpenSearchItem::Array(vec!["url 1".to_owned(), "url 2".to_owned()]),
        ];

        assert_eq!(
            open_search_to_page_url_tupel(&valid_input).unwrap(),
            vec![
                ("name 1".to_owned(), "url 1".to_owned()),
                ("name 2".to_owned(), "url 2".to_owned())
            ]
        );

        match open_search_to_page_url_tupel(&missing_elements).unwrap_err() {
            WikiError::InvalidApiResponse(res) => {
                assert_eq!(res, InvalidApiResponseError::OpenSearchMissingNthElement(1))
            }
            _ => panic!("expected error to be of type 'InvalidApiResponse'"),
        }

        match open_search_to_page_url_tupel(&not_arrays).unwrap_err() {
            WikiError::InvalidApiResponse(res) => {
                assert_eq!(
                    res,
                    InvalidApiResponseError::OpenSearchNthElementShouldBeArray(3)
                )
            }
            _ => panic!("expected error to be of type 'InvalidApiResponse'"),
        }

        match open_search_to_page_url_tupel(&different_lengths).unwrap_err() {
            WikiError::InvalidApiResponse(res) => {
                assert_eq!(res, InvalidApiResponseError::OpenSearchArraysLengthMismatch)
            }
            _ => panic!("expected error to be of type 'InvalidApiResponse'"),
        }
    }
}
