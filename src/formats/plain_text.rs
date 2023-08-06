use colored::Colorize;
use ego_tree::NodeRef;
use scraper::Node;

use crate::{
    error::WikiError,
    utils::{extract_tag_attr, fetch_page, get_page_content, get_top_pages, HtmlTag},
};

/// Reads the body of the ArchWiki page as a plain text string, removing all tags and only leaving
/// the text node content. URLs can be shown in a markdown like syntax.
///
/// If the ArchWiki returns a 404 for the page being searched for the top 5 pages that are most
/// like the page that was given as an argument are returned as a `NoPageFound` error.
///
/// Errors:
/// - If it fails to fetch the page
pub async fn read_page_as_plain_text(
    page: &str,
    pages: &[&str],
    show_urls: bool,
) -> Result<String, WikiError> {
    let document = fetch_page(page).await?;
    let content = match get_page_content(&document) {
        Some(content) => content,
        None => {
            let recommendations = get_top_pages(page, 5, pages);
            return Err(WikiError::NoPageFound(recommendations.join("\n")));
        }
    };

    let res = content
        .children()
        .map(|node| format_children(node, show_urls))
        .collect::<Vec<String>>()
        .join("");

    Ok(res)
}

fn format_children(node: NodeRef<Node>, show_urls: bool) -> String {
    match node.value() {
        Node::Text(text) => text.to_string(),
        Node::Element(e) => match e.name() {
            "a" => {
                let child_text = node
                    .children()
                    .map(|node| format_children(node, show_urls))
                    .collect::<Vec<String>>()
                    .join("");

                if show_urls {
                    wrap_text_in_url(
                        &child_text,
                        &extract_tag_attr(e, &HtmlTag::A, "href").unwrap_or("".to_string()),
                    )
                } else {
                    child_text
                }
            }
            "tbody" | "tr" | "td" | "th" => node
                .children()
                .map(|node| format_table(node, show_urls))
                .collect::<Vec<String>>()
                .join(""),
            _ => node
                .children()
                .map(|node| format_children(node, show_urls))
                .collect::<Vec<String>>()
                .join(""),
        },
        _ => node
            .children()
            .map(|node| format_children(node, show_urls))
            .collect::<Vec<String>>()
            .join(""),
    }
}

fn format_table(node: NodeRef<Node>, show_urls: bool) -> String {
    match node.value() {
        Node::Text(text) => text.to_string().trim_end().to_owned(),
        Node::Element(e) => match e.name() {
            "tr" => {
                node.children()
                    .filter_map(|node| {
                        let str = format_table(node, show_urls);
                        if str.is_empty() {
                            None
                        } else {
                            Some(format!("{str:<25}"))
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" | ")
                    + "\n"
            }
            _ => format_children(node, show_urls),
        },
        _ => format_children(node, show_urls),
    }
}

fn wrap_text_in_url(text: &str, url: &str) -> String {
    format!("{text}[{url}]", url = url.cyan())
}
