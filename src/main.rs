#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]

use std::{fs, path::Path};

use args::cli::{CliArgs, Commands};
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use directories::BaseDirs;
use error::WikiError;

use crate::{
    args::cli::{CompletionsCliArgs, LocalWikiCliArgs, ReadPageCliArgs},
    formats::format_page,
    io::{page_cache_exists, page_path},
    utils::read_pages_as_tree,
    wiki::{copy_wiki_to_fs, fetch_page},
};

mod args;
mod error;
mod formats;
mod info;
mod io;
mod langs;
mod list;
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
            let out = search::fetch(args.into()).await?;
            println!("{out}");
        }
        Commands::ListPages(args) => {
            let (path, is_default) = args
                .page_file
                .clone()
                .map_or((default_page_file_path, true), |path| (path, false));
            let wiki_tree = read_pages_as_tree(&path, is_default)?;

            let out = list::fmt_pages(args.into(), &wiki_tree)?;
            println!("{out}");
        }
        Commands::ListCategories(args) => {
            let (path, is_default) = args
                .page_file
                .clone()
                .map_or((default_page_file_path, true), |path| (path, false));
            let wiki_tree = read_pages_as_tree(&path, is_default)?;

            let out = list::fmt_categories(args.into(), &wiki_tree)?;
            println!("{out}");
        }
        Commands::ListLanguages(args) => {
            let langs = langs::fetch_all().await?;
            let out = langs::fmt(args.into(), &langs)?;
            println!("{out}");
        }
        Commands::SyncWiki(args) => {
            let path = args.out_file.clone().unwrap_or(default_page_file_path);
            let print = args.print;
            let hide_progress = args.hide_progress;

            let out = wiki::fetch_metadata(args.into()).await?;

            if print {
                println!("{out}");
            } else {
                fs::write(&path, out)?;

                if !hide_progress {
                    println!("data saved to {}", path.to_string_lossy());
                }
            }
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

            let wiki_tree = read_pages_as_tree(&path, is_default)?;

            copy_wiki_to_fs(
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
            let out = info::fmt(args.into(), &cache_dir, &data_dir)?;
            println!("{out}");
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
        match fetch_page(&page, &lang).await {
            Ok(document) => format_page(&format, &document, &page, show_urls),
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

fn generate_shell_completion(shell: Shell) {
    let mut command = CliArgs::command();
    generate(
        shell,
        &mut command,
        std::env!("CARGO_BIN_NAME"),
        &mut std::io::stdout(),
    );
}
