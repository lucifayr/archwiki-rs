use scraper::Html;

/// Converts the body of the ArchWiki page to a Markdown string
pub fn convert_page_to_markdown(document: &Html, page: &str) -> String {
    let md = html2md::parse_html(&document.html());
    format!("# {heading}\n\n{body}", heading = page, body = md)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_convert_page_to_markdown() {
        let page = "test page";
        let input = r#"<div>
            <h3>Hello, world!</h3>
            </div>"#;

        let expected_output = format!(
            r#"# {page}

### Hello, world! ###"#
        );

        let document = Html::parse_document(input);
        let output = convert_page_to_markdown(&document, page);

        assert_eq!(output, expected_output);
    }
}
