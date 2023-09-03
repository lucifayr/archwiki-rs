use scraper::Html;

use crate::{
    error::WikiError,
    utils::{get_page_content, get_top_pages},
};

/// Converts the body of the ArchWiki page to a Markdown string.
///
/// If the ArchWiki page doesn't have content the top 5 pages that are most
/// like the page that was given as an argument are returned as a `NoPageFound` error.
///
/// Errors:
/// - If it fails to fetch the page
pub async fn convert_page_to_markdown(
    document: &Html,
    page: &str,
    pages: &[&str],
) -> Result<String, WikiError> {
    let content = match get_page_content(document) {
        Some(content) => content,
        None => {
            let recommendations = get_top_pages(page, 5, pages);
            return Err(WikiError::NoPageFound(recommendations.join("\n")));
        }
    };

    let md = html2md::parse_html(&content.html());
    let res = format!("# {heading}\n\n{body}", heading = page, body = md);
    Ok(res)
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
        let output = convert_page_to_markdown(&document, page, &[])
            .await
            .unwrap();

        assert_eq!(output, expected_output);
    }
}
