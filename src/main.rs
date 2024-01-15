use std::{collections::HashMap, fs};

use clap::Parser;
use cli::{CliArgs, Commands};
use directories::BaseDirs;
use error::WikiError;
use formats::plain_text::convert_page_to_plain_text;
use itertools::Itertools;
use scraper::Html;
use url::Url;
use wiki_api::fetch_page_by_url;

use crate::{
    categories::{fetch_all_pages, list_pages},
    formats::{html::convert_page_to_html, markdown::convert_page_to_markdown, PageFormat},
    languages::{fetch_all_langs, format_lang_table},
    search::{format_open_search_table, format_text_search_table, open_search_to_page_url_tupel},
    utils::{create_cache_page_path, page_cache_exists, read_pages_file_as_str},
    wiki_api::{fetch_open_search, fetch_page, fetch_text_search},
};

mod categories;
mod cli;
mod error;
mod formats;
mod languages;
mod search;
mod utils;
mod wiki_api;

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
                "failed to get valid home directory".to_owned(),
            ))
        }
    };

    let cache_dir = base_dir.cache_dir().join("archwiki-rs");
    let data_dir = base_dir.data_local_dir().join("archwiki-rs");
    fs::create_dir_all(&cache_dir)?;
    fs::create_dir_all(&data_dir)?;

    let default_page_file_path = data_dir.join(PAGE_FILE_NAME);

    match args.command {
        Commands::ReadPage {
            page,
            no_cache_write,
            ignore_cache,
            disable_cache_invalidation,
            show_urls,
            lang,
            format,
        } => {
            let page_cache_path = create_cache_page_path(&page, &format, &cache_dir);
            let use_cached_page = !ignore_cache
                && page_cache_exists(&page_cache_path, disable_cache_invalidation).unwrap_or(false);

            let out = if use_cached_page {
                fs::read_to_string(&page_cache_path)?
            } else {
                match fetch_document(&page, lang.as_deref()).await {
                    Ok(document) => match format {
                        PageFormat::PlainText => convert_page_to_plain_text(&document, show_urls),
                        PageFormat::Markdown => convert_page_to_markdown(&document, &page),
                        PageFormat::Html => convert_page_to_html(&document, &page),
                    },
                    Err(err)
                        if !ignore_cache
                            && page_cache_exists(&page_cache_path, true).unwrap_or(false) =>
                    {
                        eprintln!("failed to fetch fresh page content, using possibly outdated cache instead\nERROR: {err}");
                        fs::read_to_string(&page_cache_path)?
                    }
                    Err(err) => return Err(err),
                }
            };

            let mut caching_failed_warning = String::new();

            if !no_cache_write {
                match fs::write(&page_cache_path, out.as_bytes()) {
                    Ok(_) => {}
                    Err(_) => {
                        caching_failed_warning =
                            format!("\n\n! failed to cache page with name {page}");
                    }
                }
            }

            println!("{out}{caching_failed_warning}");
        }
        Commands::Search {
            search,
            limit,
            lang,
            text_search,
        } => {
            let out = if !text_search {
                let search_res = fetch_open_search(&search, &lang, limit).await?;
                let name_url_pairs = open_search_to_page_url_tupel(&search_res)?;
                format_open_search_table(&name_url_pairs)
            } else {
                let search_res = fetch_text_search(&search, &lang, limit).await?;
                format_text_search_table(&search_res)
            };

            println!("{out}");
        }
        Commands::ListPages {
            flatten,
            category,
            page_file,
        } => {
            let path = page_file.unwrap_or(default_page_file_path);
            let file = read_pages_file_as_str(path)?;

            let pages_map: HashMap<String, Vec<String>> = serde_yaml::from_str(&file)?;

            let out = if let Some(category) = category {
                pages_map
                    .get(&category)
                    .ok_or(WikiError::NoCategoryFound(category))?
                    .iter()
                    .sorted()
                    .join("\n")
            } else {
                list_pages(&pages_map, flatten)
            };

            println!("{out}");
        }
        Commands::ListCategories { page_file } => {
            let path = page_file.unwrap_or(default_page_file_path);
            let file = read_pages_file_as_str(path)?;

            let pages_map: HashMap<String, Vec<String>> = serde_yaml::from_str(&file)?;

            let out = pages_map.keys().unique().sorted().join("\n");
            println!("{out}");
        }
        Commands::ListLanguages => {
            let langs = fetch_all_langs().await?;
            let out = format_lang_table(&langs);

            println!("{out}");
        }
        Commands::SyncWiki {
            hide_progress,
            thread_count,
            max_categories,
            start_at,
            print,
        } => {
            let thread_count = thread_count.unwrap_or(num_cpus::get_physical());
            let res = fetch_all_pages(
                hide_progress,
                thread_count,
                max_categories,
                start_at.as_deref(),
            )
            .await?;

            let out = serde_yaml::to_string(&res)?;

            if !print {
                fs::write(&default_page_file_path, out)?;

                if !hide_progress {
                    println!("data saved to {}", default_page_file_path.to_string_lossy());
                }
            } else {
                println!("{out}");
            }
        }
        Commands::Info {
            show_cache_dir,
            show_data_dir,
            only_values,
        } => {
            let no_flags_provided = !show_data_dir && !show_cache_dir;
            let info = [
                (
                    !only_values,
                    "VALUE".into(),
                    "NAME",
                    "DESCRIPTION",
                ),
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

async fn fetch_document(page: &str, lang: Option<&str>) -> Result<Html, WikiError> {
    match Url::parse(page) {
        Ok(url) => fetch_page_by_url(url).await,
        Err(_) => fetch_page(page, lang).await,
    }
}
