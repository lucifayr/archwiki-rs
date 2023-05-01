use clap::Parser;
use scraper::{ElementRef, Html, Node, Selector};
use thiserror::Error;

#[derive(Parser)]
struct CliArgs {
    // The title of the article to retrieve from the Archwiki
    article: String,
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
    let document = fetch_article(&args.article).await?;
    let content = match get_article_content(&document) {
        Some(content) => content,
        None => {
            return Err(WikiError::HtmlError(
                "Failed to find article content".to_owned(),
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

async fn fetch_article(article: &str) -> Result<Html, reqwest::Error> {
    let body = reqwest::get(format!(
        "https://wiki.archlinux.org/title/{title}",
        title = article
    ))
    .await?
    .text()
    .await?;

    Ok(Html::parse_document(&body))
}

fn get_article_content<'a>(document: &'a Html) -> Option<ElementRef<'a>> {
    let selector =
        Selector::parse(".mw-parser-output").expect(".mw-parser-output should be valid selector");
    document.select(&selector).next()
}
