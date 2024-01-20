use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_complete::Shell;

use crate::formats::PageFormat;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[allow(clippy::module_name_repetitions)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Read a page from the ArchWiki",
        long_about = "Read a page from the ArchWiki, if the page is not found similar page names are recommended"
    )]
    ReadPage(ReadPageCliArgs),
    #[command(
        about = "Search the ArchWiki for pages",
        long_about = "Search the ArchWiki for pages"
    )]
    Search(SearchCliArgs),
    #[command(
        about = "List all pages from the ArchWiki that have been downloaded",
        long_about = "List all pages from the ArchWiki that have been downloaded. See 'sync-wiki' for information on downloading"
    )]
    ListPages(ListPagesCliArgs),
    #[command(
        about = "List all categories from the ArchWiki that have been downloaded",
        long_about = "List categories  from the ArchWiki that have been downloaded. See 'sync-wiki' for information on downloading"
    )]
    ListCategories(ListCategoriesCliArgs),
    #[command(
        about = "List all languages that the ArchWiki supports",
        long_about = "List all languages that the ArchWiki supports"
    )]
    ListLanguages,
    #[command(
        about = "Download information about the pages and categories on the ArchWiki",
        long_about = "Download information about the pages and categories on the ArchWiki. Page and category names are used for the 'list-pages' and 'list-categories' sub-commands"
    )]
    SyncWiki(SyncWikiCliArgs),
    #[command(
        about = "Download a copy of the ArchWiki. Will take a long time :)",
        long_about = "Download a copy of the ArchWiki. Will take a long time :). The exact hierarchy of the wiki is not mainted, sub-categories are put at the top level of the wiki directory"
    )]
    LocalWiki(LocalWikiCliArgs),
    #[command(
        about = "Retrive information related to this tool",
        long_about = "Retrive information related to this tool"
    )]
    Info(InfoCliArgs),
    #[command(
        about = "Generate scripts for shell autocompletion",
        long_about = "Generate scripts for shell autocompletion"
    )]
    Completions(CompletionsCliArgs),
}

#[derive(Parser)]
#[allow(clippy::struct_excessive_bools)]
pub struct ReadPageCliArgs {
    #[arg(short, long)]
    /// Don't cache the read page locally
    pub no_cache_write: bool,
    #[arg(short, long)]
    /// Don't read the page from cache even if an entry for it is cached
    pub ignore_cache: bool,
    #[arg(short, long)]
    /// Don't invalidate the cache even if it is considered stale. A cache is considered stale
    /// after it hasn't been updated in more then 14 days
    pub disable_cache_invalidation: bool,
    #[arg(short, long)]
    /// Show URLs for plain-text output
    pub show_urls: bool,
    #[arg(short, long)]
    /// Preferred page language
    pub lang: Option<String>,
    #[arg(short, long, value_enum, default_value_t = PageFormat::PlainText)]
    /// The format that the page should be displayed in
    pub format: PageFormat,
    /// The name of the page to read or an absolute URL to the page
    pub page: String,
}

#[derive(Parser)]
pub struct SearchCliArgs {
    pub search: String,
    #[arg(short, long, default_value_t = String::from("en"))]
    /// Preferred language of the content to search for
    pub lang: String,
    #[arg(short = 'L', long, default_value_t = 5)]
    /// Maximum number of results
    pub limit: u16,
    #[arg(short, long)]
    /// Search for pages by text content instead of title
    pub text_search: bool,
}

#[derive(Parser)]
pub struct ListPagesCliArgs {
    #[arg(short, long)]
    /// Flatten all pages and don't show their category names
    pub flatten: bool,
    #[arg(short, long, value_delimiter = ',')]
    /// Only show pages in these categories
    pub categories: Vec<String>,
    #[arg(short, long)]
    /// Use a different file to read pages from
    pub page_file: Option<PathBuf>,
}

#[derive(Parser)]
pub struct ListCategoriesCliArgs {
    #[arg(short, long)]
    /// Use a different file to read pages from
    pub page_file: Option<PathBuf>,
}

#[derive(Parser)]
pub struct SyncWikiCliArgs {
    #[arg(short = 'H', long)]
    /// Hide progress indicators
    pub hide_progress: bool,
    #[arg(short, long)]
    /// Print result to stdout instead of writing to a file. Output is formatted as YAML
    pub print: bool,
    #[arg(short, long)]
    /// Use custom output file location
    pub out_file: Option<PathBuf>,
}

#[derive(Parser)]
pub struct LocalWikiCliArgs {
    #[arg(short, long)]
    /// Amount of threads to use for fetching pages from the ArchWiki. If not provided the
    /// number of physical cores is used
    pub thread_count: Option<usize>,
    #[arg(short, long)]
    /// Use a different file to read pages from
    pub page_file: Option<PathBuf>,
    #[arg(short = 'H', long)]
    /// Hide progress indicators
    pub hide_progress: bool,
    #[arg(short, long)]
    /// Show URLs in plain-text files
    pub show_urls: bool,
    #[arg(short, long)]
    /// Override already downloaded files
    pub override_existing_files: bool,
    #[arg(short, long, value_enum, default_value_t = PageFormat::PlainText)]
    /// The format that the page should be displayed in
    pub format: PageFormat,
    /// Location to store the local copy of the wiki at
    pub location: PathBuf,
}

#[derive(Parser)]
pub struct InfoCliArgs {
    #[arg(short = 'c', long)]
    pub show_cache_dir: bool,
    #[arg(short = 'd', long)]
    pub show_data_dir: bool,
    #[arg(short, long)]
    pub only_values: bool,
}

#[derive(Parser)]
pub struct CompletionsCliArgs {
    /// Shell type that completion scripts will be generated for
    pub shell: Option<Shell>,
}
