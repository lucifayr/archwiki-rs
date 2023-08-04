use std::process::exit;

use crate::{
    error::WikiError,
    utils::{fetch_page, get_page_content, get_top_pages},
};

/// Reads the body of the Archwiki page as a HTML string.
///
/// If the Archwiki returns a 404 for the page being searched for the top 5 pages that are most
/// like the page that was given as an argument are printed to stderr and the program is forced to
/// exit with status 2.
///
/// Errors:
/// - If it fails to fetch the page
pub async fn read_page_as_html(page: &str, pages: &[&str]) -> Result<String, WikiError> {
    let document = fetch_page(page).await?;

    let content = match get_page_content(&document) {
        Some(content) => content,
        None => {
            let recommendations = get_top_pages(page, 5, pages);
            eprintln!("{}", recommendations.join("\n"));
            exit(2);
        }
    };

    let res = format!(
        "<h1>{heading}</h1>\n{body}",
        heading = page,
        body = content.html()
    );
    Ok(res)
}
