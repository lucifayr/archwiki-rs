use clap::Parser;
use scraper::{ElementRef, Html, Node, Selector};
use thiserror::Error;

#[derive(Parser)]
struct CliArgs {
    // The title of the page to retrieve from the Archwiki
    page: String,
}

#[derive(Error, Debug)]
enum WikiError {
    #[error("A network error occurred")]
    NetworkError(#[from] reqwest::Error),
    #[error("A HTML parsing error occurred")]
    HtmlError(String),
}

#[tokio::main]
async fn main() -> Result<(), WikiError> {
    let args = CliArgs::parse();
    let document = fetch_page(&args.page).await?;
    let content = match get_page_content(&document) {
        Some(content) => content,
        None => {
            return Err(WikiError::HtmlError(
                "Failed to find page content".to_owned(),
            ))
        }
    };

    let res = content
        .descendants()
        .map(|node| match node.value() {
            Node::Text(text) => text.to_string(),
            _ => "".to_owned(),
        })
        .collect::<Vec<String>>()
        .join("");

    println!("{res}");
    Ok(())
}

async fn fetch_all_page_names() -> Result<Vec<String>, WikiError> {
    // The holy page
    // https://wiki.archlinux.org/api.php
    todo!()
}

async fn fetch_page(page: &str) -> Result<Html, reqwest::Error> {
    let body = reqwest::get(format!(
        "https://wiki.archlinux.org/title/{title}",
        title = page
    ))
    .await?
    .text()
    .await?;

    Ok(Html::parse_document(&body))
}

fn get_page_content(document: &Html) -> Option<ElementRef<'_>> {
    let selector =
        Selector::parse(".mw-parser-output").expect(".mw-parser-output should be valid selector");
    document.select(&selector).next()
}
