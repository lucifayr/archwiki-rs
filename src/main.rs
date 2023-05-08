use std::{collections::HashMap, fs, io, process::exit};

use clap::{Parser, Subcommand};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use scraper::{ElementRef, Html, Node, Selector};
use thiserror::Error;

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Read a page from the Archwiki",
        long_about = "Read a page from the Archwiki, if the page is not found similar page names are recommended. A list of page names is in the pages.yml file which can be updated with the 'update-all' and 'update-category' commands."
    )]
    ReadPage { page: String },
    #[command(
        about = "Download all pages from a category",
        long_about = "Download all pages from a category. Categories are stored in the pages.yml file."
    )]
    UpdateCategory { category: String },
    #[command(
        about = "Download all pages from the Archwiki",
        long_about = "Download all pages from the archwiki. Only the English pages are stored."
    )]
    UpdateAll,
}

#[derive(Parser)]
struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Error, Debug)]
enum WikiError {
    #[error("A network error occurred")]
    Network(#[from] reqwest::Error),
    #[error("A yaml parsing error occurred")]
    YamlParsing(#[from] serde_yaml::Error),
    #[error("An IO error occurred")]
    IO(#[from] io::Error),
}

#[tokio::main]
async fn main() -> Result<(), WikiError> {
    let args = CliArgs::parse();
    let page_map: HashMap<String, Vec<String>> = match fs::read_to_string("pages.yml") {
        Ok(file) => serde_yaml::from_str(&file)?,
        Err(_e) => HashMap::default(),
    };

    match args.command {
        Commands::ReadPage { page } => {
            read_page(
                &page,
                page_map
                    .values()
                    .map(|pages| pages.iter().map(|p| p.as_str()).collect())
                    .reduce(|acc: Vec<&str>, pages| acc.into_iter().chain(pages).collect())
                    .unwrap_or(Vec::new())
                    .into_iter()
                    .unique()
                    .collect::<Vec<&str>>()
                    .as_slice(),
            )
            .await?;
        }
        Commands::UpdateCategory { category } => {
            match fetch_page_names_from_categoriy(&category).await {
                Some(pages) => {
                    let mut content = page_map.clone();
                    content.insert(category, pages);
                    let yaml = serde_yaml::to_string(&content)?;
                    fs::write("pages.yml", yaml)?;
                }
                None => println!("Found no pages for category {category}"),
            }
        }
        Commands::UpdateAll => {
            let pages = fetch_all_page_names().await?;
            let yaml = serde_yaml::to_string(&pages)?;
            fs::write("pages.yml", yaml)?;
        }
    }

    Ok(())
}

async fn read_page(page: &str, pages: &[&str]) -> Result<(), WikiError> {
    let document = fetch_page(page).await?;
    let content = match get_page_content(&document) {
        Some(content) => content,
        None => {
            let recommendations = get_top_pages(page, 5, pages);
            eprintln!("{}", recommendations.join("\n"));
            exit(2);
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

fn get_top_pages<'a>(search: &str, amount: usize, pages: &[&'a str]) -> Vec<&'a str> {
    let matcher = SkimMatcherV2::default();
    let mut ranked_pages = pages
        .iter()
        .map(|page| (matcher.fuzzy_match(page, search).unwrap_or(0), *page))
        .collect::<Vec<(i64, &str)>>();

    ranked_pages.sort_by(|a, b| a.0.cmp(&b.0));
    ranked_pages
        .into_iter()
        .rev()
        .take(amount)
        .map(|e| e.1)
        .collect()
}

// TODO: fix duplicate pages being found
async fn fetch_all_page_names() -> Result<HashMap<String, Vec<String>>, WikiError> {
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

    let mut pages = HashMap::new();
    for cat in cat_hrefs {
        let cat_name = cat.split(':').last().unwrap_or("");
        let res = fetch_page_names_from_categoriy(cat_name).await;
        pages.insert(cat_name.to_owned(), res.unwrap_or(Vec::new()));
    }

    Ok(pages)
}

async fn fetch_page_names_from_categoriy(category: &str) -> Option<Vec<String>> {
    let selector = Selector::parse("#mw-pages").unwrap();
    let document = fetch_html(&format!(
        "https://wiki.archlinux.org/title/Category:{category}"
    ))
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
            e.attr(attr).map(|attr| attr.to_owned())
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
