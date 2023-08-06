use crate::{
    error::WikiError,
    utils::{fetch_page, get_page_content, get_top_pages},
};

/// Reads the body of the ArchWiki page as a HTML string.
///
/// If the ArchWiki returns a 404 for the page being searched for the top 5 pages that are most
/// like the page that was given as an argument are returned as a `NoPageFound` error.
///
/// Errors:
/// - If it fails to fetch the page
pub async fn read_page_as_html(page: &str, pages: &[&str]) -> Result<String, WikiError> {
    let document = fetch_page(page).await?;

    let content = match get_page_content(&document) {
        Some(content) => content,
        None => {
            let recommendations = get_top_pages(page, 5, pages);
            return Err(WikiError::NoPageFound(recommendations.join("\n")));
        }
    };

    let res = format!(
        "<h1>{heading}</h1>\n{body}",
        heading = page,
        body = content.html()
    );
    Ok(res)
}
