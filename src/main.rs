use std::{fs, io, path::Path};

use clap::{builder::PossibleValue, Parser, ValueEnum};
use cli::{CliArgs, Commands};
use directories::BaseDirs;
use error::WikiError;
use formats::plain_text::convert_page_to_plain_text;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use itertools::Itertools;
use wiki_api::fetch_page_without_recommendations;

use crate::{
    categories::list_pages,
    formats::{html::convert_page_to_html, markdown::convert_page_to_markdown, PageFormat},
    languages::{fetch_all_langs, format_lang_table},
    search::{format_open_search_table, format_text_search_table, open_search_to_page_url_tupel},
    utils::{
        create_cache_page_path, page_cache_exists, read_pages_file_as_category_tree,
        to_save_file_name, UNCATEGORIZED_KEY,
    },
    wiki_api::{fetch_all_pages, fetch_open_search, fetch_page, fetch_text_search},
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
                match fetch_page(&page, lang.as_deref()).await {
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
            let (path, is_default) = page_file
                .map(|path| (path, false))
                .unwrap_or((default_page_file_path, true));

            let wiki_tree = read_pages_file_as_category_tree(&path, is_default)?;
            let out = if let Some(category) = category {
                wiki_tree
                    .get(&category)
                    .ok_or(WikiError::NoCategoryFound(category))?
                    .iter()
                    .sorted()
                    .join("\n")
            } else {
                list_pages(&wiki_tree, flatten)
            };

            println!("{out}");
        }
        Commands::ListCategories { page_file } => {
            let (path, is_default) = page_file
                .map(|path| (path, false))
                .unwrap_or((default_page_file_path, true));

            let wiki_tree = read_pages_file_as_category_tree(&path, is_default)?;
            let out = wiki_tree
                .keys()
                .unique()
                .sorted()
                .filter(|cat| cat.as_str() != UNCATEGORIZED_KEY)
                .join("\n");

            println!("{out}");
        }
        Commands::ListLanguages => {
            let langs = fetch_all_langs().await?;
            let out = format_lang_table(&langs);

            println!("{out}");
        }
        Commands::SyncWiki {
            hide_progress,
            print,
            out_file,
        } => {
            let spinner = ProgressBar::new_spinner();
            if hide_progress {
                spinner.finish_and_clear();
            }

            let _spin_task = std::thread::spawn(move || loop {
                spinner.tick();
                std::thread::sleep(std::time::Duration::from_millis(100));
            });

            let wiki_tree = fetch_all_pages().await?;
            let out = serde_yaml::to_string(&wiki_tree)?;

            if !print {
                let path = out_file.unwrap_or(default_page_file_path);
                fs::write(&path, out)?;

                if !hide_progress {
                    println!("data saved to {}", path.to_string_lossy());
                }
            } else {
                println!("{out}");
            }
        }
        Commands::LocalWiki {
            location,
            format,
            page_file,
            override_wiki_directory,
            hide_progress,
        } => {
            let (path, is_default) = page_file
                .map(|path| (path, false))
                .unwrap_or((default_page_file_path, true));

            let wiki_tree = read_pages_file_as_category_tree(&path, is_default)?;

            create_dir_if_not_exists(&location, !override_wiki_directory)?;

            if !hide_progress {
                if let Some(format) = format
                    .to_possible_value()
                    .as_ref()
                    .map(PossibleValue::get_name)
                {
                    println!("downloading pages as {format}\n",)
                }
            }

            let all_bars = MultiProgress::new();

            let category_count = wiki_tree.values().filter(|v| !v.is_empty()).count();
            let category_bar = all_bars.add(
                ProgressBar::new(category_count.try_into().unwrap_or(0))
                    .with_prefix("fetching categories")
                    .with_style(
                        ProgressStyle::with_template("[{prefix:^22}]\t {pos:>4}/{len:4}")
                            .unwrap()
                            .progress_chars("##-"),
                    ),
            );

            if hide_progress {
                category_bar.finish_and_clear();
            }

            for (cat, pages) in wiki_tree {
                if pages.is_empty() {
                    continue;
                }

                let cat_dir = location.join(to_save_file_name(&cat));
                create_dir_if_not_exists(&cat_dir, !override_wiki_directory)?;

                let bar = all_bars.add(
                    ProgressBar::new(pages.len().try_into().unwrap_or(0))
                        .with_prefix("fetching sub-pages")
                        .with_style(
                            ProgressStyle::with_template(
                                "[{prefix:^22}]\t {bar:40.cyan/blue} {pos:>4}/{len:4}",
                            )
                            .unwrap()
                            .progress_chars("##-"),
                        ),
                );

                if hide_progress {
                    bar.finish_and_clear();
                }

                category_bar.inc(1);
                for page in pages {
                    bar.inc(1);

                    match write_page_to_local_wiki(&page, &cat_dir, &format).await {
                        Ok(()) => {}
                        Err(err) => {
                            eprintln!("[WARNING] FAILED TO FETCH PAGE '{page}'\nERROR: {err}")
                        }
                    }
                }
            }

            if !hide_progress {
                println!(
                    "saved local copy of the ArchWiki to '{}'",
                    location.to_string_lossy()
                )
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

async fn write_page_to_local_wiki(
    page: &str,
    parent_dir: &Path,
    format: &PageFormat,
) -> Result<(), WikiError> {
    let document = fetch_page_without_recommendations(page).await?;

    let (content, ext) = match format {
        PageFormat::PlainText => (convert_page_to_plain_text(&document, false), ""),
        PageFormat::Markdown => (convert_page_to_markdown(&document, page), "md"),
        PageFormat::Html => (convert_page_to_html(&document, page), "html"),
    };

    let file_path = parent_dir.join(to_save_file_name(page)).with_extension(ext);

    fs::write(file_path, content)?;
    Ok(())
}

fn create_dir_if_not_exists(dir: &Path, err_when_exists: bool) -> Result<(), WikiError> {
    match fs::create_dir(dir) {
        Ok(_) => {}
        Err(err) => {
            if err.kind() != io::ErrorKind::AlreadyExists {
                return Err(err.into());
            } else if err_when_exists {
                return Err(WikiError::Path(format!(
                    "ERROR: directory '{}' already exists",
                    dir.to_string_lossy()
                )));
            }
        }
    }

    Ok(())
}
