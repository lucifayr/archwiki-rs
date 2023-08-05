use std::{collections::HashMap, fs};

use categories::{fetch_all_page_names, fetch_page_names_from_categoriy, list_pages};
use clap::Parser;
use cli::{CliArgs, Commands};
use directories::BaseDirs;
use error::WikiError;
use formats::plain_text::read_page_as_plain_text;
use itertools::Itertools;

use crate::{
    formats::{html::read_page_as_html, markdown::read_page_as_markdown, PageFormat},
    utils::{create_page_path_path, page_cache_exists},
};

mod categories;
mod cli;
mod error;
mod formats;
mod utils;

#[tokio::main]
async fn main() -> Result<(), WikiError> {
    let args = CliArgs::parse();
    let base_dir = match BaseDirs::new() {
        Some(base_dir) => base_dir,
        None => {
            return Err(WikiError::Path(
                "Failed to get valid home directory".to_owned(),
            ))
        }
    };

    let dir_path = base_dir.data_local_dir().join("archwiki-rs");
    fs::create_dir_all(&dir_path)?;

    let pages_path = dir_path.join("pages.yml");
    let pages_map: HashMap<String, Vec<String>> = match fs::read_to_string(&pages_path) {
        Ok(file) => serde_yaml::from_str(&file)?,
        Err(_e) => HashMap::default(),
    };

    match args.command {
        Commands::ReadPage {
            page,
            no_cache_write,
            ignore_cache,
            disable_cache_invalidation,
            show_urls,
            format,
        } => {
            let pages = pages_map
                .values()
                .map(|pages| pages.iter().map(|p| p.as_str()).collect())
                .reduce(|acc: Vec<&str>, pages| acc.into_iter().chain(pages).collect())
                .unwrap_or(Vec::new())
                .into_iter()
                .unique()
                .collect::<Vec<&str>>();

            let cache_dir = base_dir.cache_dir().join("archwiki-rs");

            let page = pages
                .iter()
                .find(|p| p.eq_ignore_ascii_case(&page))
                .map(|p| p.to_owned().to_owned())
                .unwrap_or(page);

            let page_cache_path = create_page_path_path(&page, &format, &cache_dir);
            let out = if !ignore_cache
                && page_cache_exists(&page_cache_path, disable_cache_invalidation).unwrap_or(false)
            {
                fs::read_to_string(&page_cache_path)?
            } else {
                match format {
                    PageFormat::PlainText => {
                        read_page_as_plain_text(&page, &pages, show_urls).await?
                    }
                    PageFormat::Markdown => read_page_as_markdown(&page, &pages).await?,
                    PageFormat::Html => read_page_as_html(&page, &pages).await?,
                }
            };

            if !no_cache_write {
                fs::write(&page_cache_path, out.as_bytes())?;
            }

            println!("{out}");
        }
        Commands::ListPages { flatten } => {
            let out = list_pages(&pages_map, flatten);
            println!("{out}");
        }
        Commands::ListCategories => {
            let out = pages_map.keys().unique().sorted().join("\n");
            println!("{out}");
        }
        Commands::UpdateCategory { category } => {
            match fetch_page_names_from_categoriy(&category).await {
                Some(pages) => {
                    let mut content = pages_map.clone();
                    content.insert(category, pages);

                    let yaml = serde_yaml::to_string(&content)?;
                    fs::write(&pages_path, yaml)?;
                }
                None => println!("Found no pages for category {category}"),
            }
        }
        Commands::UpdateAll => {
            let pages = fetch_all_page_names().await?;
            let yaml = serde_yaml::to_string(&pages)?;
            fs::write(&pages_path, yaml)?;
        }
    }

    Ok(())
}
