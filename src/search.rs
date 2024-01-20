use colored::Colorize;
use itertools::Itertools;
use scraper::Html;

use crate::{error::WikiError, formats::format_children_as_plain_text};

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(untagged)]
pub enum OpenSearchItem {
    Single(String),
    Array(Vec<String>),
}

#[derive(Debug, serde::Deserialize)]
pub struct TextSearchApiResponse {
    pub search: Vec<TextSearchItem>,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
pub struct TextSearchItem {
    pub title: String,
    pub snippet: String,
}

impl TextSearchItem {
    pub fn prettify_snippet(&mut self, search: &str) {
        let frag = Html::parse_fragment(&self.snippet);
        let new_snip = frag
            .root_element()
            .children()
            .map(|node| format_children_as_plain_text(node, false))
            .collect::<Vec<String>>()
            .join("")
            .replace('\n', " ");

        if let Ok(rgx) = regex::RegexBuilder::new(&format!("({search})"))
            .case_insensitive(true)
            .build()
        {
            self.snippet = rgx
                .replace_all(&new_snip, format!("{}", "$1".cyan()))
                .to_string();
        } else {
            self.snippet = new_snip;
        }
    }
}

pub fn format_text_search_table(search_result: &[TextSearchItem]) -> String {
    let mut table = format!("{c1:20} | {c2:90}\n", c1 = "PAGE", c2 = "SNIPPET");
    let body = search_result
        .iter()
        .map(|item| format!("{:20} | {:90}", item.title, item.snippet))
        .collect_vec()
        .join("\n");

    table += &body;
    table
}

pub fn format_open_search_table(name_url_pairs: &[(String, String)]) -> String {
    let mut table = format!("{c1:20} | {c2:90}\n", c1 = "PAGE", c2 = "URL");
    let body = name_url_pairs
        .iter()
        .map(|(name, url)| format!("{name:20} | {url:90}"))
        .collect_vec()
        .join("\n");

    table += &body;
    table
}

/// Convert an open search response into a list of name and URL pairs
///
/// Errors:
/// - If the search results don't have an array as the 1. and 3. elements in the list
/// - If the arrays in the search results have different lengths
pub fn open_search_to_page_url_tupel(
    search_result: &[OpenSearchItem],
) -> Result<Vec<(String, String)>, WikiError> {
    use crate::error::InvalidApiResponseError as IAR;

    let page_names = search_result.get(1).ok_or(WikiError::InvalidApiResponse(
        IAR::OpenSearchMissingNthElement(1),
    ))?;

    let page_urls = search_result.get(3).ok_or(WikiError::InvalidApiResponse(
        IAR::OpenSearchMissingNthElement(3),
    ))?;

    if let OpenSearchItem::Array(names) = page_names {
        if let OpenSearchItem::Array(urls) = page_urls {
            if names.len() != urls.len() {
                return Err(WikiError::InvalidApiResponse(
                    IAR::OpenSearchArraysLengthMismatch,
                ));
            }

            Ok(names
                .iter()
                .zip(urls)
                .map(|(a, b)| (a.to_owned(), b.to_owned()))
                .collect_vec())
        } else {
            Err(WikiError::InvalidApiResponse(
                IAR::OpenSearchNthElementShouldBeArray(3),
            ))
        }
    } else {
        Err(WikiError::InvalidApiResponse(
            IAR::OpenSearchNthElementShouldBeArray(1),
        ))
    }
}

pub fn open_search_to_page_names(
    search_result: &[OpenSearchItem],
) -> Result<Vec<String>, WikiError> {
    use crate::error::InvalidApiResponseError as IAR;

    let page_names = search_result.get(1).ok_or(WikiError::InvalidApiResponse(
        IAR::OpenSearchMissingNthElement(1),
    ))?;

    if let OpenSearchItem::Array(names) = page_names {
        Ok(names.to_owned())
    } else {
        Err(WikiError::InvalidApiResponse(
            IAR::OpenSearchNthElementShouldBeArray(1),
        ))
    }
}

/// Return provided page name if the top search result exactly matches it
pub fn open_search_is_page_exact_match<'a>(
    page: &'a str,
    search_result: &[OpenSearchItem],
) -> Result<Option<&'a str>, WikiError> {
    use crate::error::InvalidApiResponseError as IAR;

    let page_names = search_result.get(1).ok_or(WikiError::InvalidApiResponse(
        IAR::OpenSearchMissingNthElement(1),
    ))?;

    let OpenSearchItem::Array(names) = page_names else {
        return Err(WikiError::InvalidApiResponse(
            IAR::OpenSearchNthElementShouldBeArray(1),
        ));
    };

    Ok(names
        .first()
        .and_then(|name| (name == page).then_some(page)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::InvalidApiResponseError as IAR;
    use pretty_assertions::assert_eq;

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
                assert_eq!(res, IAR::OpenSearchMissingNthElement(1))
            }
            _ => panic!("expected error to be of type 'InvalidApiResponse'"),
        }

        match open_search_to_page_url_tupel(&not_arrays).unwrap_err() {
            WikiError::InvalidApiResponse(res) => {
                assert_eq!(res, IAR::OpenSearchNthElementShouldBeArray(3))
            }
            _ => panic!("expected error to be of type 'InvalidApiResponse'"),
        }

        match open_search_to_page_url_tupel(&different_lengths).unwrap_err() {
            WikiError::InvalidApiResponse(res) => {
                assert_eq!(res, IAR::OpenSearchArraysLengthMismatch)
            }
            _ => panic!("expected error to be of type 'InvalidApiResponse'"),
        }
    }

    #[test]
    fn test_format_open_search_table() {
        let pairs = vec![
            ("page 1".to_owned(), "url 1".to_owned()),
            ("page 2".to_owned(), "url 2".to_owned()),
            ("page 3".to_owned(), "url 3".to_owned()),
        ];

        let res = format_open_search_table(&pairs);
        let res_row_count = res.split('\n').collect_vec().len();
        let third_page = res
            .split('\n')
            .nth(3)
            .unwrap()
            .split('|')
            .next()
            .unwrap()
            .trim();

        assert_eq!(res_row_count, 4);
        assert_eq!(third_page, "page 3");
    }

    #[test]
    fn test_format_text_search_table() {
        let items = vec![
            TextSearchItem {
                title: "page 1".to_owned(),
                snippet: "snippet 1".to_owned(),
            },
            TextSearchItem {
                title: "page 2".to_owned(),
                snippet: "snippet 2".to_owned(),
            },
            TextSearchItem {
                title: "page 3".to_owned(),
                snippet: "snippet 3".to_owned(),
            },
            TextSearchItem {
                title: "page 4".to_owned(),
                snippet: "snippet 4".to_owned(),
            },
        ];

        let res = format_text_search_table(&items);
        let res_row_count = res.split('\n').collect_vec().len();
        let third_page = res
            .split('\n')
            .nth(3)
            .unwrap()
            .split('|')
            .nth(1)
            .unwrap()
            .trim();

        assert_eq!(res_row_count, 5);
        assert_eq!(third_page, "snippet 3");
    }
}
