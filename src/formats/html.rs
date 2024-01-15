use scraper::Html;

/// Converts the body of the ArchWiki page to a HTML string
pub fn convert_page_to_html(document: &Html, page: &str) -> String {
    format!(
        "<h1>{heading}</h1>\n{body}",
        heading = page,
        body = document.html()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_convert_page_to_html() {
        let page = "test page";
        let input = format!(
            r#"<div>
    <title>Hello, world!</title>
</div>"#
        );

        let expected_output = format!(
            r#"<h1>{page}</h1>
<div>
    <title>Hello, world!</title>
</div>"#
        );

        let document = Html::parse_document(&input);
        let output = convert_page_to_html(&document, page);

        assert_eq!(output, expected_output);
    }
}
