use colored::Colorize;
use ego_tree::NodeRef;
use scraper::{Html, Node};

use crate::utils::extract_tag_attr;

/// Converts the body of the ArchWiki page to a plain text string, removing all tags and
/// only leaving the text node content. URLs can be shown in a markdown like syntax.
#[allow(clippy::module_name_repetitions)]
pub fn convert_page_to_plain_text(document: &Html, show_urls: bool) -> String {
    document
        .root_element()
        .children()
        .map(|node| format_children_as_plain_text(node, show_urls))
        .collect::<String>()
}

#[allow(clippy::module_name_repetitions)]
pub fn format_children_as_plain_text(node: NodeRef<Node>, show_urls: bool) -> String {
    match node.value() {
        Node::Text(text) => text.to_string(),
        Node::Element(e) => match e.name() {
            "a" => {
                let child_text = node
                    .children()
                    .map(|node| format_children_as_plain_text(node, show_urls))
                    .collect::<String>();

                if show_urls {
                    wrap_text_in_url(
                        &child_text,
                        &extract_tag_attr(e, "a", "href").unwrap_or_default(),
                    )
                } else {
                    child_text
                }
            }
            "tbody" | "tr" | "td" | "th" => node
                .children()
                .map(|node| format_table(node, show_urls))
                .collect::<String>(),
            _ => node
                .children()
                .map(|node| format_children_as_plain_text(node, show_urls))
                .collect::<String>(),
        },
        _ => node
            .children()
            .map(|node| format_children_as_plain_text(node, show_urls))
            .collect::<String>(),
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
            _ => format_children_as_plain_text(node, show_urls),
        },
        _ => format_children_as_plain_text(node, show_urls),
    }
}

fn wrap_text_in_url(text: &str, url: &str) -> String {
    format!("{text}[{url}]", url = url.cyan())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_convert_page_to_plain_text() {
        {
            let input = r#"<div">
                <h3>Hello, world!</h3>
                <div>how <span><bold>are</bold></span> you</div>
                I'm great
                </div>"#;

            let expected_output = r"
                Hello, world!
                how are you
                I'm great
                ";

            let document = Html::parse_document(input);
            let output = convert_page_to_plain_text(&document, false);

            assert_eq!(output, expected_output);
        }

        {
            let input = r#"<div>
    <h3>Hello, world!</h3>
    <a href="example.com">example</a>
</div>"#;

            let expected_output = format!(
                r#"
    Hello, world!
    example[{url}]
"#,
                url = "example.com".cyan()
            );

            let document = Html::parse_document(input);
            let output = convert_page_to_plain_text(&document, true);

            dbg!(&output);
            assert_eq!(output, expected_output);
        }
    }
}
