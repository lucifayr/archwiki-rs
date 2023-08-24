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
    utils::{create_page_path, fetch_page_langs, format_page_lang, page_cache_exists},
};

mod categories;
mod cli;
mod error;
mod formats;
mod utils;

const PAGE_FILE_NAME: &str = "pages.yml";

#[tokio::main]
#[termination::display]
async fn main() -> Result<(), WikiError> {
    human_panic::setup_panic!();

    let args = CliArgs::parse();
    let base_dir = match BaseDirs::new() {
        Some(base_dir) => base_dir,
        None => {
            return Err(WikiError::Path(
                "Failed to get valid home directory".to_owned(),
            ))
        }
    };

    let cache_dir = base_dir.cache_dir().join("archwiki-rs");
    let data_dir = base_dir.data_local_dir().join("archwiki-rs");
    fs::create_dir_all(&cache_dir)?;
    fs::create_dir_all(&data_dir)?;

    let pages_path = data_dir.join(PAGE_FILE_NAME);
    let pages_map: HashMap<String, Vec<String>> = match fs::read_to_string(&pages_path) {
        Ok(file) => serde_yaml::from_str(&file)?,
        Err(_) => HashMap::default(),
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

            let page = pages
                .iter()
                .find(|p| p.eq_ignore_ascii_case(&page))
                .map(|p| p.to_owned().to_owned())
                .unwrap_or(page);

            let page_cache_path = create_page_path(&page, &format, &cache_dir);
            let use_cached_page = !ignore_cache
                && page_cache_exists(&page_cache_path, disable_cache_invalidation).unwrap_or(false);

            let out = if use_cached_page {
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
        Commands::ReadPageLanguages { show_urls, page } => {
            let pages = pages_map
                .values()
                .map(|pages| pages.iter().map(|p| p.as_str()).collect())
                .reduce(|acc: Vec<&str>, pages| acc.into_iter().chain(pages).collect())
                .unwrap_or(Vec::new())
                .into_iter()
                .unique()
                .collect::<Vec<&str>>();

            let page = pages
                .iter()
                .find(|p| p.eq_ignore_ascii_case(&page))
                .map(|p| p.to_owned().to_owned())
                .unwrap_or(page);

            let langs = fetch_page_langs(&page, &pages).await?;

            println!(
                "{}",
                langs
                    .into_iter()
                    .map(|l| format_page_lang(&l, show_urls))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
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
        Commands::Info {
            show_cache_dir,
            show_data_dir,
            only_values,
        } => {
            let no_flags_provided = !show_data_dir && !show_cache_dir;
            let info = [
                (
                    show_cache_dir || no_flags_provided,
                    cache_dir,
                    "cache directory",
                    "stores caches of ArchWiki pages after download to speed up future requests",
                ),
                (
                    show_data_dir || no_flags_provided,
                    data_dir,
                    "data directory",  
                    "stores the 'pages.yml' file that is used for suggestions about what ArchWiki pages exist"
                ),
            ];

            let out = info
                .iter()
                .filter_map(|entry| {
                    entry.0.then_some(if only_values {
                        format!("{val}", val = entry.1.to_string_lossy())
                    } else {
                        format!(
                            "{name:20} | {desc:90} | {val}",
                            name = entry.2,
                            desc = entry.3,
                            val = entry.1.to_string_lossy()
                        )
                    })
                })
                .join("\n");

            println!("{out}");
        }
    }

    Ok(())
}
