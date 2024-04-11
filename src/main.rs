#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use cli::{CliArgs, Commands};
use directories::BaseDirs;
use error::WikiError;

use itertools::Itertools;

use crate::{
    categories::list_pages,
    cli::{
        CompletionsCliArgs, ListCategoriesCliArgs, ListPagesCliArgs, LocalWikiCliArgs,
        ReadPageCliArgs, SearchCliArgs, SyncWikiCliArgs,
    },
    formats::{
        convert_page_to_html, convert_page_to_markdown, convert_page_to_plain_text, PageFormat,
    },
    io::{page_cache_exists, page_path},
    languages::{fetch_all_langs, format_lang_table},
    search::{format_open_search_table, format_text_search_table, open_search_to_page_url_tupel},
    utils::{read_pages_file_as_category_tree, UNCATEGORIZED_KEY},
    wiki::{download_wiki, fetch_open_search, fetch_page, fetch_text_search, sync_wiki_info},
};

mod categories;
mod cli;
mod error;
mod formats;
mod info;
mod io;
mod languages;
mod search;
mod utils;
mod wiki;

const PAGE_FILE_NAME: &str = "pages.yml";

#[tokio::main]
#[termination::display]
async fn main() -> Result<(), WikiError> {
    human_panic::setup_panic!();

    let args = CliArgs::parse();
    let Some(base_dir) = BaseDirs::new() else {
        return Err(WikiError::Path(
            "failed to get valid home directory".to_owned(),
        ));
    };

    let cache_dir = base_dir.cache_dir().join("archwiki-rs");
    let data_dir = base_dir.data_local_dir().join("archwiki-rs");
    let log_dir = data_dir.join("logs");
    fs::create_dir_all(&cache_dir)?;
    fs::create_dir_all(&data_dir)?;
    fs::create_dir_all(&log_dir)?;

    let default_page_file_path = data_dir.join(PAGE_FILE_NAME);

    match args.command {
        Commands::ReadPage(args) => {
            read_page(args, &cache_dir).await?;
        }
        Commands::Search(args) => {
            search_wiki(args).await?;
        }
        Commands::ListPages(args) => {
            list_wiki_pages(args, default_page_file_path)?;
        }
        Commands::ListCategories(args) => {
            list_wiki_categories(args, default_page_file_path)?;
        }
        Commands::ListLanguages => {
            let langs = fetch_all_langs().await?;
            let out = format_lang_table(&langs);

            println!("{out}");
        }
        Commands::SyncWiki(SyncWikiCliArgs {
            hide_progress,
            print,
            out_file,
        }) => {
            let path = out_file.unwrap_or(default_page_file_path);
            sync_wiki_info(&path, print, hide_progress).await?;
        }
        Commands::LocalWiki(LocalWikiCliArgs {
            location,
            format,
            page_file,
            thread_count,
            show_urls,
            override_existing_files,
            hide_progress,
        }) => {
            let thread_count = thread_count.unwrap_or(num_cpus::get_physical()).max(1);

            let (path, is_default) =
                page_file.map_or((default_page_file_path, true), |path| (path, false));

            let wiki_tree = read_pages_file_as_category_tree(&path, is_default)?;

            download_wiki(
                wiki_tree,
                format,
                location,
                &log_dir,
                thread_count,
                override_existing_files,
                hide_progress,
                show_urls,
            )
            .await?;
        }
        Commands::Info(args) => {
            info::display(args, cache_dir, data_dir)?;
        }
        Commands::Completions(CompletionsCliArgs { shell }) => {
            generate_shell_completion(shell.unwrap_or(Shell::from_env().expect(
                "failed to automatically detect shell, please provided it as an explict argument",
            )));
        }
    };

    Ok(())
}

async fn read_page(
    ReadPageCliArgs {
        no_cache_write,
        ignore_cache,
        disable_cache_invalidation,
        show_urls,
        lang,
        format,
        page,
    }: ReadPageCliArgs,
    cache_dir: &Path,
) -> Result<(), WikiError> {
    let page_cache_path = page_path(&page, &format, cache_dir);
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
                if !ignore_cache && page_cache_exists(&page_cache_path, true).unwrap_or(false) =>
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
            Ok(()) => {}
            Err(_) => {
                caching_failed_warning = format!("\n\n! failed to cache page with name {page}");
            }
        }
    }

    println!("{out}{caching_failed_warning}");
    Ok(())
}

async fn search_wiki(
    SearchCliArgs {
        search,
        lang,
        limit,
        text_search,
    }: SearchCliArgs,
) -> Result<(), WikiError> {
    let out = if text_search {
        let search_res = fetch_text_search(&search, &lang, limit).await?;
        format_text_search_table(&search_res)
    } else {
        let search_res = fetch_open_search(&search, &lang, limit).await?;
        let name_url_pairs = open_search_to_page_url_tupel(&search_res)?;
        format_open_search_table(&name_url_pairs)
    };

    println!("{out}");
    Ok(())
}

fn list_wiki_pages(
    ListPagesCliArgs {
        flatten,
        categories,
        page_file,
    }: ListPagesCliArgs,
    default_page_file_path: PathBuf,
) -> Result<(), WikiError> {
    let (path, is_default) = page_file.map_or((default_page_file_path, true), |path| (path, false));

    let wiki_tree = read_pages_file_as_category_tree(&path, is_default)?;
    let out = list_pages(
        &wiki_tree,
        (!categories.is_empty()).then_some(&categories),
        flatten,
    );

    println!("{out}");
    Ok(())
}

fn list_wiki_categories(
    ListCategoriesCliArgs {
        page_file,
        json,
        json_raw,
    }: ListCategoriesCliArgs,
    default_page_file_path: PathBuf,
) -> Result<(), WikiError> {
    let (path, is_default) = page_file.map_or((default_page_file_path, true), |path| (path, false));

    let wiki_tree = read_pages_file_as_category_tree(&path, is_default)?;

    let out = if json {
        serde_json::to_string_pretty(&wiki_tree)?
    } else if json_raw {
        serde_json::to_string(&wiki_tree)?
    } else {
        wiki_tree
            .keys()
            .unique()
            .sorted()
            .filter(|cat| cat.as_str() != UNCATEGORIZED_KEY)
            .join("\n")
    };

    println!("{out}");
    Ok(())
}

fn generate_shell_completion(shell: Shell) {
    let mut command = CliArgs::command();
    generate(
        shell,
        &mut command,
        std::env!("CARGO_BIN_NAME"),
        &mut std::io::stdout(),
    );
}
