use scraper::Html;

use crate::utils::get_page_content;

/// Converts the body of the ArchWiki page to a Markdown string
pub fn convert_page_to_markdown(document: &Html, page: &str) -> String {
    let content = get_page_content(document).expect("page should have content");

    let md = html2md::parse_html(&content.html());
    format!("# {heading}\n\n{body}", heading = page, body = md)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::PAGE_CONTENT_CLASS;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_convert_page_to_markdown() {
        let page = "test page";
        let input = format!(
            r#"<div class="{PAGE_CONTENT_CLASS}">
    <h3>Hello, world!</h3>
</div>"#
        );

        let expected_output = format!(
            r#"# {page}

### Hello, world! ###"#
        );

        let document = Html::parse_document(&input);
        let output = convert_page_to_markdown(&document, page);

        assert_eq!(output, expected_output);
    }
}
