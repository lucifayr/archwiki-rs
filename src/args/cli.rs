#![cfg(feature = "cli")]

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

use crate::formats::PageFormat;

use super::internal::{
    InfoArgs, InfoFmtArgs, InfoPlainArgs, ListCategoriesArgs, ListCategoriesFmtArgs,
    ListLanguagesArgs, ListLanguagesFmtArgs, ListPagesArgs, ListPagesFmtArgs, ListPagesPlainArgs,
    ReadPageArgs, SearchArgs, SearchFmtArgs, WikiMetadataArgs, WikiMetadataFmtArgs,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[allow(clippy::module_name_repetitions)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        about = "Read a page from the ArchWiki",
        long_about = "Read a page from the ArchWiki. If the page is not found similar page names are recommended."
    )]
    ReadPage(ReadPageCliArgs),
    #[command(
        about = "Search the ArchWiki for pages",
        long_about = "Search the ArchWiki for pages"
    )]
    Search(SearchCliArgs),
    #[command(
        about = "List all pages from the ArchWiki (requires metadata to be synced)",
        long_about = "List metadata information of pages from the ArchWiki that has been downloaded. See 'sync-wiki' for information on downloading metadata."
    )]
    ListPages(ListPagesCliArgs),
    #[command(
        about = "List all categories from the ArchWiki (requires metadata to be synced)",
        long_about = "List metadata information of categories from the ArchWiki that has been downloaded. See 'sync-wiki' for information on downloading metadata."
    )]
    ListCategories(ListCategoriesCliArgs),
    #[command(
        about = "List all languages that the ArchWiki supports",
        long_about = "List all languages that the ArchWiki supports"
    )]
    ListLanguages(ListLanguagesCliArgs),
    #[command(
        about = "Download metadata information about the pages and categories on the ArchWiki",
        long_about = "Download metadata information about the pages and categories on the ArchWiki. Page and category names are used for the 'list-pages' and 'list-categories' sub-commands."
    )]
    SyncWiki(WikiMetadataCliArgs),
    #[command(
        about = "Download a copy of the ArchWiki. Will take a long time :)",
        long_about = "Download a copy of the ArchWiki. Will take a long time :). The exact hierarchy of the wiki is not mainted, sub-categories are put at the top level of the wiki directory."
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

#[derive(Parser, Debug)]
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
    #[arg(short, long, default_value_t = ReadPageArgs::default().lang)]
    /// Preferred page language
    pub lang: String,
    #[arg(short, long, value_enum, default_value_t = ReadPageArgs::default().format)]
    /// The format that the page should be displayed in
    pub format: PageFormat,
    /// The name of the page to read or an absolute URL to the page
    pub page: String,
}

#[derive(Parser, Debug)]
pub struct SearchCliArgs {
    pub search: String,
    #[arg(short, long, default_value_t = SearchArgs::default().lang)]
    /// Preferred language of the content to search for
    pub lang: String,
    #[arg(short = 'L', long, default_value_t = SearchArgs::default().limit)]
    /// Maximum number of results
    pub limit: u16,
    #[arg(short, long, default_value_t = SearchArgs::default().text_search)]
    /// Search for pages by text content instead of title
    pub text_search: bool,

    #[command(flatten)]
    pub args_json: Option<SearchJsonCliArgs>,
}

impl From<SearchCliArgs> for SearchArgs {
    fn from(
        SearchCliArgs {
            search,
            lang,
            limit,
            text_search,
            args_json,
        }: SearchCliArgs,
    ) -> Self {
        Self {
            search,
            lang,
            limit,
            text_search,
            fmt: args_json.into(),
        }
    }
}

impl From<Option<SearchJsonCliArgs>> for SearchFmtArgs {
    fn from(value: Option<SearchJsonCliArgs>) -> Self {
        match value {
            Some(args) if args.json_raw => Self::JsonRaw,
            Some(args) if args.json => Self::JsonPretty,
            Some(_) => Self::JsonPretty,
            None => Self::Plain,
        }
    }
}

#[derive(Args, Debug, Default)]
pub struct SearchJsonCliArgs {
    #[arg(short, long)]
    /// Display data as pretty-printed JSON
    pub json: bool,
    #[arg(short = 'J', long)]
    /// Display data as raw JSON
    pub json_raw: bool,
}

#[derive(Parser, Debug)]
pub struct ListPagesCliArgs {
    #[arg(short, long)]
    /// Use a different file to read pages from
    pub page_file: Option<PathBuf>,

    #[command(flatten)]
    pub args_plain: Option<ListPagesPlainCliArgs>,
    #[command(flatten)]
    pub args_json: Option<ListPagesJsonCliArgs>,
}

impl From<ListPagesCliArgs> for ListPagesArgs {
    fn from(
        ListPagesCliArgs {
            args_plain,
            args_json,
            ..
        }: ListPagesCliArgs,
    ) -> Self {
        Self {
            args_plain: args_plain.clone().map(Into::into),
            fmt: (args_plain, args_json).into(),
        }
    }
}

#[derive(Args, Debug, Clone)]
#[group(id = "plain-list-pages", conflicts_with_all = ["json-list-pages"])]
pub struct ListPagesPlainCliArgs {
    #[arg(short, long)]
    /// Flatten all pages and don't show their category names
    pub flatten: bool,
    #[arg(short, long, value_delimiter = ',')]
    /// Only show pages in these categories
    pub categories: Vec<String>,
}

impl From<ListPagesPlainCliArgs> for ListPagesPlainArgs {
    fn from(
        ListPagesPlainCliArgs {
            flatten,
            categories,
        }: ListPagesPlainCliArgs,
    ) -> Self {
        Self {
            flatten,
            categories,
        }
    }
}

#[derive(Args, Debug)]
#[group(id = "json-list-pages" , conflicts_with_all = ["plain-list-pages"])]
pub struct ListPagesJsonCliArgs {
    #[arg(short, long)]
    /// Display data as pretty-printed JSON
    pub json: bool,
    #[arg(short = 'J', long)]
    /// Display data as raw JSON
    pub json_raw: bool,
}

impl From<(Option<ListPagesPlainCliArgs>, Option<ListPagesJsonCliArgs>)> for ListPagesFmtArgs {
    fn from(value: (Option<ListPagesPlainCliArgs>, Option<ListPagesJsonCliArgs>)) -> Self {
        match value {
            (Some(_plain_args), _) => Self::Plain,
            (_, Some(args)) if args.json_raw => Self::JsonRaw,
            (_, Some(args)) if args.json => Self::JsonPretty,
            _ => Self::Plain,
        }
    }
}

#[derive(Parser, Debug)]
pub struct ListCategoriesCliArgs {
    #[arg(short, long)]
    /// Use a different file to read pages from
    pub page_file: Option<PathBuf>,

    #[command(flatten)]
    pub args_json: Option<ListCategoriesJsonCliArgs>,
}

impl From<ListCategoriesCliArgs> for ListCategoriesArgs {
    fn from(ListCategoriesCliArgs { args_json, .. }: ListCategoriesCliArgs) -> Self {
        Self {
            fmt: args_json.into(),
        }
    }
}

#[derive(Args, Debug)]
pub struct ListCategoriesJsonCliArgs {
    #[arg(short, long)]
    /// Display data as pretty-printed JSON
    pub json: bool,
    #[arg(short = 'J', long)]
    /// Display data as raw JSON
    pub json_raw: bool,
}

impl From<Option<ListCategoriesJsonCliArgs>> for ListCategoriesFmtArgs {
    fn from(value: Option<ListCategoriesJsonCliArgs>) -> Self {
        match value {
            Some(args) if args.json_raw => Self::JsonRaw,
            Some(args) if args.json => Self::JsonPretty,
            _ => Self::Plain,
        }
    }
}

#[derive(Parser, Debug)]
pub struct ListLanguagesCliArgs {
    #[command(flatten)]
    pub args_json: Option<ListLanguagesJsonCliArgs>,
}

impl From<ListLanguagesCliArgs> for ListLanguagesArgs {
    fn from(ListLanguagesCliArgs { args_json }: ListLanguagesCliArgs) -> Self {
        Self {
            fmt: args_json.into(),
        }
    }
}

#[derive(Args, Debug)]
pub struct ListLanguagesJsonCliArgs {
    #[arg(short, long)]
    /// Display data as pretty-printed JSON
    pub json: bool,
    #[arg(short = 'J', long)]
    /// Display data as raw JSON
    pub json_raw: bool,
}

impl From<Option<ListLanguagesJsonCliArgs>> for ListLanguagesFmtArgs {
    fn from(value: Option<ListLanguagesJsonCliArgs>) -> Self {
        match value {
            Some(args) if args.json_raw => Self::JsonRaw,
            Some(args) if args.json => Self::JsonPretty,
            _ => Self::Plain,
        }
    }
}

#[derive(Parser, Debug)]
pub struct WikiMetadataCliArgs {
    #[arg(short = 'H', long)]
    /// Hide progress indicators
    pub hide_progress: bool,
    #[arg(short, long)]
    /// Print result to stdout instead of writing to a file. By default output is formatted as YAML
    pub print: bool,
    #[arg(short, long)]
    /// Use custom output file location
    pub out_file: Option<PathBuf>,
    #[command(flatten)]
    pub args_yaml: Option<WikiMetdataYamlCliArgs>,
    #[command(flatten)]
    pub args_json: Option<WikiMetadtaJsonCliArgs>,
}

impl From<WikiMetadataCliArgs> for WikiMetadataArgs {
    fn from(
        WikiMetadataCliArgs {
            hide_progress,
            args_yaml,
            args_json,
            ..
        }: WikiMetadataCliArgs,
    ) -> Self {
        Self {
            hide_progress,
            fmt: (args_yaml, args_json).into(),
        }
    }
}

impl
    From<(
        Option<WikiMetdataYamlCliArgs>,
        Option<WikiMetadtaJsonCliArgs>,
    )> for WikiMetadataFmtArgs
{
    fn from(
        value: (
            Option<WikiMetdataYamlCliArgs>,
            Option<WikiMetadtaJsonCliArgs>,
        ),
    ) -> Self {
        match value {
            (Some(args), _) if args.yaml => Self::Yaml,
            (_, Some(args)) if args.json_raw => Self::JsonRaw,
            (_, Some(args)) if args.json => Self::JsonPretty,
            _ => Self::Yaml,
        }
    }
}

#[derive(Args, Debug, Clone, Copy)]
#[group(id = "yaml-sync-wiki", conflicts_with_all = ["json-sync-wiki"])]
pub struct WikiMetdataYamlCliArgs {
    #[arg(short, long)]
    /// Format data as YAML
    pub yaml: bool,
}

#[derive(Args, Debug, Clone, Copy)]
#[group(id = "json-sync-wiki", conflicts_with_all = ["yaml-sync-wiki"])]
pub struct WikiMetadtaJsonCliArgs {
    #[arg(short, long)]
    /// Format data as pretty-printed JSON
    pub json: bool,
    #[arg(short = 'J', long)]
    /// Format data as raw JSON
    pub json_raw: bool,
}

#[derive(Parser, Debug)]
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

#[derive(Parser, Debug)]
pub struct InfoCliArgs {
    #[command(flatten)]
    pub args_plain: Option<InfoPlainCliArgs>,
    #[command(flatten)]
    pub args_json: Option<InfoJsonCliArgs>,
}

impl From<InfoCliArgs> for InfoArgs {
    fn from(
        InfoCliArgs {
            args_plain,
            args_json,
            ..
        }: InfoCliArgs,
    ) -> Self {
        Self {
            args_plain: args_plain.clone().map(Into::into),
            fmt: (args_plain, args_json).into(),
        }
    }
}

#[derive(Args, Debug, Clone)]
#[group(id = "plain-info", conflicts_with_all = ["json-info"])]
pub struct InfoPlainCliArgs {
    #[arg(short = 'c', long)]
    /// Show entry for cache directory
    pub show_cache_dir: bool,
    #[arg(short = 'd', long)]
    /// Show entry for data directory
    pub show_data_dir: bool,
    #[arg(short, long)]
    /// Only display values, hide names and descriptions
    pub only_values: bool,
}

impl From<InfoPlainCliArgs> for InfoPlainArgs {
    fn from(
        InfoPlainCliArgs {
            show_cache_dir,
            show_data_dir,
            only_values,
        }: InfoPlainCliArgs,
    ) -> Self {
        Self {
            show_cache_dir,
            show_data_dir,
            only_values,
        }
    }
}

#[derive(Args, Debug)]
#[group(id = "json-info", conflicts_with_all = ["plain-info"])]
pub struct InfoJsonCliArgs {
    #[arg(short, long)]
    /// Display data as pretty-printed JSON
    pub json: bool,
    #[arg(short = 'J', long)]
    /// Display data as raw JSON
    pub json_raw: bool,
}

impl From<(Option<InfoPlainCliArgs>, Option<InfoJsonCliArgs>)> for InfoFmtArgs {
    fn from(value: (Option<InfoPlainCliArgs>, Option<InfoJsonCliArgs>)) -> Self {
        match value {
            (Some(_plain_args), _) => Self::Plain,
            (_, Some(args)) if args.json_raw => Self::JsonRaw,
            (_, Some(args)) if args.json => Self::JsonPretty,
            _ => Self::Plain,
        }
    }
}

#[derive(Parser, Debug)]
pub struct CompletionsCliArgs {
    /// Shell type that completion scripts will be generated for
    pub shell: Option<Shell>,
}
