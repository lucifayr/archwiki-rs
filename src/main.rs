use clap::Parser;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use pages::PAGES;
use scraper::{ElementRef, Html, Node, Selector};
use thiserror::Error;

mod pages;

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

    let page = if !PAGES.contains(&args.page.as_str()) {
        recommend_pages(&args.page)
    } else {
        &args.page
    };

    let document = fetch_page(page).await?;
    let content = match get_page_content(&document) {
        Some(content) => content,
        None => {
            return Err(WikiError::HtmlError(
                "Failed to find page content".to_owned(),
            ))
        }
    };

    let _res = content
        .descendants()
        .map(|node| match node.value() {
            Node::Text(text) => text.to_string(),
            _ => "".to_owned(),
        })
        .collect::<Vec<String>>()
        .join("");

    Ok(())
}

fn recommend_pages<'a>(search: &str) -> &'a str {
    let top_pages = get_top_pages(search, 10);
    println!("{top_pages:?}");
    todo!()
}

fn get_top_pages<'a>(search: &str, amount: usize) -> Vec<&'a str> {
    let matcher = SkimMatcherV2::default();
    let mut ranked_pages = PAGES
        .iter()
        .map(|page| (matcher.fuzzy_match(page, search).unwrap(), *page))
        .collect::<Vec<(i64, &str)>>();

    println!("{search:?}");
    ranked_pages.sort_by(|a, b| a.0.cmp(&b.0));
    ranked_pages.into_iter().take(amount).map(|e| e.1).collect()
}

async fn fetch_all_page_names() -> Result<Vec<String>, WikiError> {
    let document = fetch_page("Table_of_contents").await?;
    let selector = Selector::parse(".mw-parser-output").unwrap();

    let cat_hrefs = document
        .select(&selector)
        .next()
        .unwrap()
        .descendants()
        .filter_map(|node| extract_a_tag_attr(node.value(), "href"))
        .skip(1)
        .collect::<Vec<String>>();

    let mut pages = Vec::with_capacity(cat_hrefs.len());
    for cat in cat_hrefs {
        let res = fetch_page_names_from_categoriy(&cat).await;
        pages.append(&mut res.unwrap_or(Vec::new()));
    }

    Ok(pages)
}

async fn fetch_page_names_from_categoriy(category: &str) -> Option<Vec<String>> {
    let selector = Selector::parse("#mw-pages").unwrap();
    let document = fetch_html(&format!("https://wiki.archlinux.org{category}"))
        .await
        .unwrap();

    Some(
        document
            .select(&selector)
            .next()?
            .descendants()
            .filter_map(|node| extract_a_tag_attr(node.value(), "title"))
            .collect::<Vec<String>>(),
    )
}

fn extract_a_tag_attr(node: &Node, attr: &str) -> Option<String> {
    if let Node::Element(e) = node {
        if e.name() == "a" {
            if let Some(attr) = e.attr(attr) {
                Some(attr.to_owned())
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
