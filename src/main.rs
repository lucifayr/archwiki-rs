use std::{collections::HashMap, fs};

use categories::{fetch_all_page_names, fetch_page_names_from_categoriy, list_categories};
use clap::Parser;
use cli::{CliArgs, Commands};
use directories::BaseDirs;
use error::WikiError;
use formats::plain_text::read_page;
use itertools::Itertools;
use utils::{create_data_dir, get_data_dir_path};

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

    let dir_path = get_data_dir_path(base_dir)?;
    create_data_dir(&dir_path)?;

    let pages_path = dir_path
        + if cfg!(windows) {
            "\\pages.yml"
        } else {
            "/pages.yml"
        };

    let page_map: HashMap<String, Vec<String>> = match fs::read_to_string(&pages_path) {
        Ok(file) => serde_yaml::from_str(&file)?,
        Err(_e) => HashMap::default(),
    };

    match args.command {
        Commands::ReadPage { page, show_urls } => {
            let out = read_page(
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
                show_urls,
            )
            .await?;

            println!("{out}");
        }
        Commands::ListCategories { flatten } => {
            let out = list_categories(&page_map, flatten);
            println!("{out}");
        }
        Commands::UpdateCategory { category } => {
            match fetch_page_names_from_categoriy(&category).await {
                Some(pages) => {
                    let mut content = page_map.clone();
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
