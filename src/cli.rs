use clap::{Parser, Subcommand};

use crate::formats::PageFormat;

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Read a page from the ArchWiki",
        long_about = "Read a page from the ArchWiki, if the page is not found similar page names are recommended. A list of page names is in the pages.yml file which can be updated with the 'update-all' and 'update-category' commands."
    )]
    ReadPage {
        #[arg(short, long)]
        /// Don't cache the read page locally
        no_cache_write: bool,
        #[arg(short, long)]
        /// Don't read the page from cache even if an entry for it is cached
        ignore_cache: bool,
        #[arg(short, long)]
        /// Don't invalidate the cache even if it is considered stale. A cache is considered stale
        /// after it hasn't been updated in more then 14 days.
        disable_cache_invalidation: bool,
        #[arg(short, long)]
        /// Show URLs for plain-text output
        show_urls: bool,
        #[arg(short, long, value_enum, default_value_t = PageFormat::PlainText)]
        /// The format that the page should be displayed in
        format: PageFormat,
        page: String,
    },
    #[command(
        about = "List all pages from the ArchWiki that have been downloaded",
        long_about = "List all pages from the ArchWiki that have been downloaded. See 'update-all' or 'update-category' for information on downloading."
    )]
    ListPages {
        #[arg(short, long)]
        /// Flatten all pages and don't show their category names
        flatten: bool,
    },
    #[command(
        about = "Download all pages from a category",
        long_about = "Download all pages from a category. Categories are stored in the pages.yml file."
    )]
    UpdateCategory { category: String },
    #[command(
        about = "Download all pages from the ArchWiki",
        long_about = "Download all pages from the archwiki. Only the English pages are stored."
    )]
    UpdateAll,
}

#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}
