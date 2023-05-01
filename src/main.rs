use clap::Parser;
use scraper::{ElementRef, Html, Node, Selector};

#[derive(Parser)]
struct CliArgs {
    // The title of the article to retrieve from the Archwiki
    article: String,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let document = fetch_article(&args.article).await.unwrap();
    let content = get_article_content(&document).unwrap();

    let res = content
        .descendants()
        .map(|node| match node.value() {
            Node::Text(text) => text.to_string(),
            _ => "".to_owned(),
        })
        .collect::<Vec<String>>()
        .join("");

    println!("{res}");
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
