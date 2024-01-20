use scraper::{Html, Selector};

/// Converts the body of the ArchWiki page to a HTML string
#[allow(clippy::module_name_repetitions)]
pub fn convert_page_to_html(document: &Html, page: &str) -> String {
    let body_selector = Selector::parse("body").expect("body should be a valid css selector");
    format!(
        "<h1>{heading}</h1>\n{body}",
        heading = page,
        body = document
            .select(&body_selector)
            .next()
            .map(|body| body.inner_html())
            .unwrap_or_default()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_convert_page_to_html() {
        let page = "test page";
        let input = r#"<div>
    <title>Hello, world!</title>
</div>"#;

        let expected_output = format!(
            r#"<h1>{page}</h1>
<div>
    <title>Hello, world!</title>
</div>"#
        );

        let document = Html::parse_document(input);
        let output = convert_page_to_html(&document, page);

        assert_eq!(output, expected_output);
    }
}
