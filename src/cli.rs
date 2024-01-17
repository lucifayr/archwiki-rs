use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::formats::PageFormat;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Read a page from the ArchWiki",
        long_about = "Read a page from the ArchWiki, if the page is not found similar page names are recommended. A list of page names is in the pages.yml file which can be updated with the 'sync-wiki' command."
    )]
    ReadPage {
        #[arg(short, long)]
        /// Don't cache the read page locally.
        no_cache_write: bool,
        #[arg(short, long)]
        /// Don't read the page from cache even if an entry for it is cached.
        ignore_cache: bool,
        #[arg(short, long)]
        /// Don't invalidate the cache even if it is considered stale. A cache is considered stale
        /// after it hasn't been updated in more then 14 days.
        disable_cache_invalidation: bool,
        #[arg(short, long)]
        /// Show URLs for plain-text output.
        show_urls: bool,
        #[arg(short, long)]
        /// Preferred page language
        lang: Option<String>,
        #[arg(short, long, value_enum, default_value_t = PageFormat::PlainText)]
        /// The format that the page should be displayed in.
        format: PageFormat,
        /// The name of the page to read or an absolute URL to the page.
        page: String,
    },
    #[command(
        about = "Search the ArchWiki for pages",
        long_about = "Search the ArchWiki for pages by title. Uses the 'opensearch' API action to perform queries."
    )]
    Search {
        search: String,
        #[arg(short, long, default_value_t = String::from("en"))]
        /// Preferred language of the content to search for.
        lang: String,
        #[arg(short = 'L', long, default_value_t = 5)]
        /// Maximum number of results.
        limit: u16,
        #[arg(short, long)]
        /// Search for pages by text content instead of title. Uses the 'query' API action instead
        /// of 'opensearch'.
        text_search: bool,
    },
    #[command(
        about = "List all pages from the ArchWiki that have been downloaded",
        long_about = "List all pages from the ArchWiki that have been downloaded. See 'sync-wiki' for information on downloading."
    )]
    ListPages {
        #[arg(short, long)]
        /// Flatten all pages and don't show their category names.
        flatten: bool,
        #[arg(short, long)]
        /// Only show pages in this category.
        category: Option<String>,
        #[arg(short, long)]
        /// Use a different file to read pages from.
        page_file: Option<PathBuf>,
    },
    #[command(
        about = "List all categories from the ArchWiki that have been downloaded",
        long_about = "List categories  from the ArchWiki that have been downloaded. See 'sync-wiki' for information on downloading."
    )]
    ListCategories {
        #[arg(short, long)]
        /// Use a different file to read pages from.
        page_file: Option<PathBuf>,
    },
    #[command(
        about = "List all languages that the ArchWiki supports",
        long_about = "List all languages that the ArchWiki supports."
    )]
    ListLanguages,
    #[command(
        about = "Download information about the pages and categories on the ArchWiki",
        long_about = "Download information about the pages and categories on the ArchWiki. Page and category names are used for the 'list-pages' and 'list-categories' commands"
    )]
    SyncWiki {
        #[arg(short = 'H', long)]
        /// Hide progress indicators.
        hide_progress: bool,
        #[arg(short, long)]
        /// Print result to stdout instead of writing to a file. Output is formatted as YAML.
        print: bool,
        #[arg(short, long)]
        /// Use custom output file location.
        out_file: Option<PathBuf>,
    },
    #[command(
        about = "Download a copy of the ArchWiki. Will take a long time :)",
        long_about = "Download a copy of the ArchWiki. Will take a long time :). The exact hierarchy of the wiki is not mainted, sub categories are put at the top level of the directory."
    )]
    LocalWiki {
        #[arg(short, long)]
        /// Use a different file to read pages from.
        page_file: Option<PathBuf>,
        #[arg(short = 'H', long)]
        /// Hide progress indicators.
        hide_progress: bool,
        #[arg(short, long)]
        /// Override directory at 'location' if it already exists.
        override_wiki_directory: bool,
        #[arg(short, long, value_enum, default_value_t = PageFormat::PlainText)]
        /// The format that the page should be displayed in.
        format: PageFormat,
        /// Location to store the local copy of the wiki at.
        location: PathBuf,
    },
    #[command(
        about = "Retrive information related to this tool",
        long_about = "Retrive information related to this tool. All Info is shown by default."
    )]
    Info {
        #[arg(short = 'c', long)]
        /// Location of the cache directory.
        show_cache_dir: bool,
        #[arg(short = 'd', long)]
        /// Location of the data directory.
        show_data_dir: bool,
        #[arg(short, long)]
        /// Only show values and not the properties they belong to or their descriptions.
        only_values: bool,
    },
}
