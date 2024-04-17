use colored::Colorize;
use itertools::Itertools;
use regex::Regex;
use scraper::Html;
use serde::{Deserialize, Serialize};

use crate::{
    args::internal::{SearchArgs, SearchFmtArgs, SearchSnippetFmtArgs},
    error::WikiError,
    wiki::{fetch_open_search, fetch_text_search},
};

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum OpenSearchItem {
    Single(String),
    Array(Vec<String>),
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct OpenSearchItemParsed {
    pub title: String,
    pub url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TextSearchApiResponse {
    pub search: Vec<TextSearchItem>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSearchItem {
    pub title: String,
    pub snippet: String,
}

impl TextSearchItem {
    pub fn prettify_snippet(&mut self, fmt: SearchSnippetFmtArgs, no_highlight: bool) {
        let snip = if let Ok(rgx) =
            regex::RegexBuilder::new("<span class=\\\"searchmatch\\\">(.*?)</span>")
                .case_insensitive(true)
                .build()
        {
            fmt_match(&rgx, &self.snippet, fmt, no_highlight)
        } else if fmt != SearchSnippetFmtArgs::Html {
            Html::parse_fragment(&self.snippet)
                .root_element()
                .inner_html()
        } else {
            return;
        };

        self.snippet = snip.replace(['\n', '\r'], " ");
    }
}

fn fmt_match(rgx: &Regex, snippet: &str, fmt: SearchSnippetFmtArgs, no_highlight: bool) -> String {
    let snippet = match fmt {
        SearchSnippetFmtArgs::Plain
        | SearchSnippetFmtArgs::Markdown
        | SearchSnippetFmtArgs::Html
            if no_highlight =>
        {
            rgx.replace_all(snippet, "$1").to_string()
        }
        SearchSnippetFmtArgs::Plain => rgx
            .replace_all(snippet, format!("{}", "$1".cyan()))
            .to_string(),
        SearchSnippetFmtArgs::Markdown => rgx.replace_all(snippet, "**$1**").to_string(),
        SearchSnippetFmtArgs::Html => snippet.to_owned(),
    };

    Html::parse_fragment(&snippet).root_element().inner_html()
}

pub async fn fetch(
    SearchArgs {
        search,
        lang,
        limit,
        text_search,
        fmt,
        text_snippet_fmt,
        no_highlight_snippet,
    }: SearchArgs,
) -> Result<String, WikiError> {
    let out = if text_search {
        let mut search_res = fetch_text_search(&search, &lang, limit).await?;

        for item in &mut search_res {
            item.prettify_snippet(text_snippet_fmt, no_highlight_snippet);
        }

        match fmt {
            SearchFmtArgs::Plain => fmt_text_search_plain(&mut search_res),
            SearchFmtArgs::JsonRaw => serde_json::to_string(&search_res)?,
            SearchFmtArgs::JsonPretty => serde_json::to_string_pretty(&search_res)?,
        }
    } else {
        let search_res = fetch_open_search(&search, &lang, limit).await?;
        let name_url_pairs = open_search_to_page_url_pairs(&search_res)?;

        match fmt {
            SearchFmtArgs::Plain => fmt_open_search_plain(&name_url_pairs),
            SearchFmtArgs::JsonRaw => serde_json::to_string(&name_url_pairs)?,
            SearchFmtArgs::JsonPretty => serde_json::to_string_pretty(&name_url_pairs)?,
        }
    };

    Ok(out)
}

fn fmt_text_search_plain(search_result: &mut [TextSearchItem]) -> String {
    let mut table = format!("{c1:20} | {c2:90}\n", c1 = "PAGE", c2 = "SNIPPET");
    let body = search_result
        .iter()
        .map(|item| format!("{:20} | {:90}", item.title, item.snippet))
        .collect_vec()
        .join("\n");

    table += &body;
    table
}

fn fmt_open_search_plain(name_url_pairs: &[OpenSearchItemParsed]) -> String {
    let mut table = format!("{c1:20} | {c2:90}\n", c1 = "PAGE", c2 = "URL");
    let body = name_url_pairs
        .iter()
        .map(|OpenSearchItemParsed { title, url }| format!("{title:20} | {url:90}"))
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
pub fn open_search_to_page_url_pairs(
    search_result: &[OpenSearchItem],
) -> Result<Vec<OpenSearchItemParsed>, WikiError> {
    use crate::error::InvalidApiResponse as IAR;

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
                .map(|(title, url)| OpenSearchItemParsed {
                    title: title.clone(),
                    url: url.clone(),
                })
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
    use crate::error::InvalidApiResponse as IAR;

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
    use crate::error::InvalidApiResponse as IAR;

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
    use crate::error::InvalidApiResponse as IAR;
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
            open_search_to_page_url_pairs(&valid_input).unwrap(),
            vec![
                OpenSearchItemParsed {
                    title: "name 1".to_owned(),
                    url: "url 1".to_owned()
                },
                OpenSearchItemParsed {
                    title: "name 2".to_owned(),
                    url: "url 2".to_owned()
                },
            ]
        );

        match open_search_to_page_url_pairs(&missing_elements).unwrap_err() {
            WikiError::InvalidApiResponse(res) => {
                assert_eq!(res, IAR::OpenSearchMissingNthElement(1));
            }
            _ => panic!("expected error to be of type 'InvalidApiResponse'"),
        }

        match open_search_to_page_url_pairs(&not_arrays).unwrap_err() {
            WikiError::InvalidApiResponse(res) => {
                assert_eq!(res, IAR::OpenSearchNthElementShouldBeArray(3));
            }
            _ => panic!("expected error to be of type 'InvalidApiResponse'"),
        }

        match open_search_to_page_url_pairs(&different_lengths).unwrap_err() {
            WikiError::InvalidApiResponse(res) => {
                assert_eq!(res, IAR::OpenSearchArraysLengthMismatch);
            }
            _ => panic!("expected error to be of type 'InvalidApiResponse'"),
        }
    }

    #[test]
    fn test_format_open_search_table() {
        let pairs = vec![
            OpenSearchItemParsed {
                title: "page 1".to_owned(),
                url: "url 1".to_owned(),
            },
            OpenSearchItemParsed {
                title: "page 2".to_owned(),
                url: "url 2".to_owned(),
            },
            OpenSearchItemParsed {
                title: "page 3".to_owned(),
                url: "url 3".to_owned(),
            },
        ];

        let res = fmt_open_search_plain(&pairs);
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
        let mut items = vec![
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

        let res = fmt_text_search_plain(&mut items);
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
