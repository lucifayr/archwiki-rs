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
    fetch_all_page_names().await.unwrap();
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
    let document = fetch_page("Table_of_contents").await.unwrap();
    let selector = Selector::parse(".mw-parser-output").unwrap();

    let hrefs = document
        .select(&selector)
        .next()
        .unwrap()
        .descendants()
        .filter_map(|node| extract_href(node.value()))
        .collect::<Vec<String>>();

    println!("{}", hrefs.join("\n"));
    todo!()
}

fn extract_href(node: &Node) -> Option<String> {
    if let Node::Element(e) = node {
        if e.name() == "a" {
            if let Some(href) = e.attr("href") {
                Some(href.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

async fn fetch_html(url: &str) -> Result<Html, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(Html::parse_document(&body))
}

async fn fetch_page(page: &str) -> Result<Html, reqwest::Error> {
    fetch_html(&format!(
        "https://wiki.archlinux.org/title/{title}",
        title = page
    ))
    .await
}

fn get_page_content(document: &Html) -> Option<ElementRef<'_>> {
    let selector =
        Selector::parse(".mw-parser-output").expect(".mw-parser-output should be valid selector");
    document.select(&selector).next()
}
