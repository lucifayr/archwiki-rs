use reqwest::Error;
use scraper::{Html, Node, Selector};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let body = reqwest::get("https://wiki.archlinux.org/title/Installation_guide")
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);
    let content_selector = Selector::parse(".mw-parser-output").unwrap();
    let content = document.select(&content_selector).next().unwrap();

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
