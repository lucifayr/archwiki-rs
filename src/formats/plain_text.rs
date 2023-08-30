use colored::Colorize;
use ego_tree::NodeRef;
use scraper::{Html, Node};

use crate::{
    error::WikiError,
    utils::{extract_tag_attr, get_page_content, get_top_pages, HtmlTag},
};

/// Converts the body of the ArchWiki page to a plain text string, removing all tags and
/// only leaving the text node content. URLs can be shown in a markdown like syntax.
///
/// the body of the ArchWiki page to a Markdown string.
///
/// If the ArchWiki page doesn't have content the top 5 pages that are most
/// like the page that was given as an argument are returned as a `NoPageFound` error.
///
/// Errors:
/// - If it fails to fetch the page
pub async fn convert_page_to_plain_text(
    document: &Html,
    page: &str,
    pages: &[&str],
    show_urls: bool,
) -> Result<String, WikiError> {
    let content = match get_page_content(document) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::PAGE_CONTENT_CLASS;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_convert_page_to_markdown() {
        {
            let page = "plain page";
            let input = format!(
                r#"<div class="{PAGE_CONTENT_CLASS}">
    <h3>Hello, world!</h3>
    <div>how <span><bold>are</bold></span> you</div>
    I'm great
</div>"#
            );

            let expected_output = format!(
                r#"
    Hello, world!
    how are you
    I'm great
"#
            );

            let document = Html::parse_document(&input);
            let output = convert_page_to_plain_text(&document, page, &[], false)
                .await
                .unwrap();

            assert_eq!(output, expected_output);
        }

        {
            let page = "page with links";
            let input = format!(
                r#"<div class="{PAGE_CONTENT_CLASS}">
    <h3>Hello, world!</h3>
    <a href="example.com">example</a>
</div>"#
            );

            let expected_output = format!(
                r#"
    Hello, world!
    example[{url}]
"#,
                url = "example.com".cyan()
            );

            let document = Html::parse_document(&input);
            let output = convert_page_to_plain_text(&document, page, &[], true)
                .await
                .unwrap();

            dbg!(&output);
            assert_eq!(output, expected_output);
        }
    }

    #[tokio::test]
    #[allow(unreachable_code)]
    async fn test_recommendations() {
        return;
        todo!("make fuzzy matcher better");

        let page = "noexistent page";
        let input = format!("<div>nothing here</div>");

        let recommendations = [
            "noexistent pa",
            "noexist",
            "existent page",
            "page",
            "existent p",
            "gdjaskfjslsvcsf",
            "jkjzcffsdfsf",
            "fjffjfsdfds",
        ];

        let document = Html::parse_document(&input);
        let output = convert_page_to_plain_text(&document, page, &recommendations, false)
            .await
            .unwrap_err();

        dbg!(&output);

        match output {
            WikiError::NoPageFound(pages) => {
                assert_eq!(
                    pages,
                    [
                        "noexistent pa",
                        "noexist",
                        "existent page",
                        "page",
                        "existent p",
                    ]
                    .join("\n")
                )
            }
            _ => panic!("expected ouput to be WikiError::NoPageFound"),
        }
    }
}
