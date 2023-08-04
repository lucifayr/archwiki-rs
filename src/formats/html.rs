use std::process::exit;

use crate::{
    error::WikiError,
    utils::{fetch_page, get_page_content, get_top_pages},
};

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
