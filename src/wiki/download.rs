use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use futures::future;

use crate::{
    args::internal::{WikiMetadataArgs, WikiMetadataFmtArgs},
    error::WikiError,
    formats::{
        convert_page_to_html, convert_page_to_markdown, convert_page_to_plain_text, PageFormat,
    },
};

use super::api::{fetch_all_pages, fetch_page_without_recommendations};

pub async fn fetch_metadata(
    WikiMetadataArgs { hide_progress, fmt }: WikiMetadataArgs,
) -> Result<String, WikiError> {
    #[cfg(feature = "cli")]
    let _spin_task = progress_spinner(hide_progress);

    let wiki_tree = fetch_all_pages().await?;
    let out = match fmt {
        WikiMetadataFmtArgs::Yaml => serde_yaml::to_string(&wiki_tree)?,
        WikiMetadataFmtArgs::JsonRaw => serde_json::to_string(&wiki_tree)?,
        WikiMetadataFmtArgs::JsonPretty => serde_json::to_string_pretty(&wiki_tree)?,
    };

    Ok(out)
}

#[cfg(feature = "cli")]
fn progress_spinner(hide_progress: bool) -> Option<std::thread::JoinHandle<()>> {
    let spinner = indicatif::ProgressBar::new_spinner();
    let mut spin_task = None;

    if hide_progress {
        spinner.finish_and_clear();
    } else {
        spin_task = Some(std::thread::spawn(move || loop {
            spinner.tick();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }));
    };

    spin_task
}

#[cfg(all(
    not(feature = "cli"),
    any(
        feature = "wasm-web",
        feature = "wasm-nodejs",
        feature = "wasm-bundler"
    )
))]
fn progress_spinner(hide_progress: bool) -> Option<std::thread::JoinHandle<()>> {
    None
}

#[cfg(feature = "cli")]
pub use local_wiki::copy_wiki_to_fs;

#[cfg(feature = "cli")]
mod local_wiki {
    use super::{
        convert_page_to_html, convert_page_to_markdown, convert_page_to_plain_text,
        fetch_page_without_recommendations, fs, future, Arc, HashMap, PageFormat, Path, PathBuf,
        WikiError,
    };

    use crate::io::{create_dir_if_not_exists, page_path, to_save_file_name};
    use clap::{builder::PossibleValue, ValueEnum};
    use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

    #[allow(clippy::too_many_arguments)]
    pub async fn copy_wiki_to_fs(
        wiki_tree: HashMap<String, Vec<String>>,
        format: PageFormat,
        location: PathBuf,
        log_dir: &Path,
        thread_count: usize,
        override_exisiting_files: bool,
        hide_progress: bool,
        show_urls: bool,
    ) -> Result<(), WikiError> {
        create_dir_if_not_exists(&location)?;

        let total_page_count = wiki_tree.values().map(Vec::len).sum::<usize>();

        if !hide_progress {
            if let Some(format) = format
                .to_possible_value()
                .as_ref()
                .map(PossibleValue::get_name)
            {
                println!("downloading {total_page_count} pages as {format}\n");
            }
        }

        let multibar = MultiProgress::new();

        let category_count = wiki_tree.values().filter(|v| !v.is_empty()).count();
        let category_bar = multibar.add(
            ProgressBar::new(category_count.try_into().unwrap_or(0))
                .with_prefix("---FETCHING CATEGORIES---")
                .with_style(
                    ProgressStyle::with_template("[{prefix:^40}]\t {pos:>4}/{len:4}")
                        .unwrap()
                        .progress_chars("##-"),
                ),
        );

        if hide_progress {
            category_bar.finish_and_clear();
        }

        let wiki_tree_without_empty_cats = wiki_tree
            .into_iter()
            .filter(|(_, p)| !p.is_empty())
            .collect_vec();

        let format = Arc::new(format);
        let location = Arc::new(location);
        let multibar = Arc::new(multibar);
        let catbar = Arc::new(category_bar);

        let wiki_tree_chunks =
            chunk_wiki_with_even_page_distribution(wiki_tree_without_empty_cats, thread_count);

        let tasks = wiki_tree_chunks
            .into_iter()
            .map(|chunk| {
                let format_ref = Arc::clone(&format);
                let location_ref = Arc::clone(&location);
                let multibar_ref = Arc::clone(&multibar);
                let catbar_ref = Arc::clone(&catbar);

                tokio::spawn(async move {
                    download_wiki_chunk(
                        &chunk,
                        &format_ref,
                        &location_ref,
                        hide_progress,
                        show_urls,
                        override_exisiting_files,
                        &multibar_ref,
                        &catbar_ref,
                    )
                    .await
                })
            })
            .collect_vec();

        let results = future::join_all(tasks).await;
        let mut all_failed_fetches = vec![];

        for result in results {
            match result {
                Ok(Ok(mut failed_fetchs)) => all_failed_fetches.append(&mut failed_fetchs),
                Ok(Err(thread_err)) => {
                    eprintln!(
                    "ERROR: a thread paniced, some pages might be missing\nREASON: {thread_err}"
                );
                }
                Err(_) => {
                    eprintln!("ERROR: failed to join threads, some pages might be missing");
                }
            }
        }

        if !hide_progress {
            let successfuly_fetched_pages = total_page_count - all_failed_fetches.len();
            println!("downloaded {successfuly_fetched_pages} pages successfully");
        }

        if !all_failed_fetches.is_empty() {
            if !hide_progress {
                println!("failed to download {} pages", all_failed_fetches.len());
            }

            let failed_fetches_str = all_failed_fetches
                .into_iter()
                .map(|(page, err)| format!("failed to page '{page}'\nREASON: {err}"))
                .collect_vec()
                .join("\n\n");

            let path = log_dir.join("local-wiki-download-err.log");
            let write = fs::write(&path, failed_fetches_str);

            if write.is_ok() && !hide_progress {
                println!("error log written to '{}'", path.to_string_lossy());
            }
        }

        if !hide_progress {
            println!(
                "saved local copy of the ArchWiki to '{}'",
                location.to_string_lossy()
            );
        }

        Ok(())
    }
    use itertools::Itertools;

    type FailedPageFetches = Vec<(String, WikiError)>;

    #[allow(clippy::too_many_arguments)]
    async fn download_wiki_chunk(
        chunk: &[(String, Vec<String>)],
        format: &PageFormat,
        location: &Path,
        hide_progress: bool,
        show_urls: bool,
        override_exisiting_files: bool,
        multibar: &MultiProgress,
        catbar: &ProgressBar,
    ) -> Result<FailedPageFetches, WikiError> {
        let mut failed_fetches = vec![];

        for (cat, pages) in chunk {
            let cat_dir = location.join(to_save_file_name(cat));
            create_dir_if_not_exists(&cat_dir)?;

            let width = unicode_width::UnicodeWidthStr::width(cat.as_str());

            let leak_str: &'static str = Box::leak(
                format!(
                    " fetching pages in \"{}\"",
                    if width <= 18 {
                        truncate_unicode_str(18, cat)
                    } else {
                        truncate_unicode_str(15, cat) + "..."
                    }
                )
                .into_boxed_str(),
            );

            let bar = multibar.add(
                ProgressBar::new(pages.len().try_into().unwrap_or(0))
                    .with_prefix(leak_str)
                    .with_style(
                        ProgressStyle::with_template(
                            "[{prefix:<40}]\t {bar:40.cyan/blue} {pos:>4}/{len:4}",
                        )
                        .unwrap()
                        .progress_chars("##-"),
                    ),
            );

            if hide_progress {
                bar.finish_and_clear();
            }

            catbar.inc(1);
            for page in pages {
                bar.inc(1);

                let path = page_path(page, format, &cat_dir);
                if override_exisiting_files || !path.exists() {
                    match write_page_to_local_wiki(page, &path, format, show_urls).await {
                        Ok(()) => {}
                        Err(err) => failed_fetches.push((page.to_owned(), err)),
                    }
                }
            }
        }

        Ok(failed_fetches)
    }

    async fn write_page_to_local_wiki(
        page: &str,
        page_path: &Path,
        format: &PageFormat,
        show_urls: bool,
    ) -> Result<(), WikiError> {
        let document = fetch_page_without_recommendations(page).await?;
        let content = match format {
            PageFormat::PlainText => convert_page_to_plain_text(&document, show_urls),
            PageFormat::Markdown => convert_page_to_markdown(&document, page),
            PageFormat::Html => convert_page_to_html(&document, page),
        };

        fs::write(page_path, content)?;
        Ok(())
    }

    fn chunk_wiki_with_even_page_distribution(
        wiki_tree: Vec<(String, Vec<String>)>,
        chunk_count: usize,
    ) -> Vec<Vec<(String, Vec<String>)>> {
        let mut chunks: Vec<Vec<(String, Vec<String>)>> =
            (0..chunk_count).map(|_| vec![]).collect();

        for entry in wiki_tree {
            if let Some(chunk) = chunks.iter_mut().min_by(|a, b| {
                let count_a = a.iter().map(|(_, pages)| pages.len()).sum::<usize>();
                let count_b = b.iter().map(|(_, pages)| pages.len()).sum::<usize>();

                count_a.cmp(&count_b)
            }) {
                chunk.push(entry);
            }
        }

        chunks
    }

    fn truncate_unicode_str(n: usize, text: &str) -> String {
        let mut count = 0;
        let mut res = vec![];
        let mut chars = text.chars();

        while count < n {
            if let Some(char) = chars.next() {
                count += unicode_width::UnicodeWidthChar::width(char).unwrap_or(0);
                res.push(char);
            } else {
                break;
            }
        }

        res.into_iter().collect::<String>()
    }
}
