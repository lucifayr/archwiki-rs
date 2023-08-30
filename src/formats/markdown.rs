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
