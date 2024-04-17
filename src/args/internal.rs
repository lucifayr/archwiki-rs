use crate::formats::PageFormat;

#[derive(Debug, Clone)]
pub struct ReadPageArgs {
    pub page: String,
    pub format: PageFormat,
    pub lang: String,
    pub show_urls: bool,
}

impl Default for ReadPageArgs {
    fn default() -> Self {
        Self {
            page: String::default(),
            format: PageFormat::default(),
            lang: String::from("en"),
            show_urls: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchArgs {
    pub search: String,
    pub lang: String,
    pub limit: u16,
    pub text_search: bool,
    pub fmt: SearchFmtArgs,
    pub text_snippet_fmt: SearchSnippetFmtArgs,
    pub no_highlight_snippet: bool,
}

impl Default for SearchArgs {
    fn default() -> Self {
        Self {
            search: String::default(),
            lang: String::from("en"),
            limit: 5,
            text_search: false,
            fmt: SearchFmtArgs::Plain,
            text_snippet_fmt: SearchSnippetFmtArgs::Plain,
            no_highlight_snippet: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SearchFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchSnippetFmtArgs {
    Plain,
    Markdown,
    Html,
}

#[derive(Debug, Clone)]
pub struct WikiMetadataArgs {
    pub hide_progress: bool,
    pub fmt: WikiMetadataFmtArgs,
}

#[derive(Debug, Clone)]
pub enum WikiMetadataFmtArgs {
    JsonPretty,
    JsonRaw,
    Yaml,
}

#[derive(Debug, Clone)]
pub struct ListPagesArgs {
    pub fmt: ListPagesFmtArgs,
    pub args_plain: Option<ListPagesPlainArgs>,
}

#[derive(Debug, Clone, Default)]
pub struct ListPagesPlainArgs {
    pub flatten: bool,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ListPagesFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

#[derive(Debug, Clone)]
pub struct ListCategoriesArgs {
    pub fmt: ListCategoriesFmtArgs,
}

#[derive(Debug, Clone)]
pub enum ListCategoriesFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

#[derive(Debug, Clone)]
pub struct ListLanguagesArgs {
    pub fmt: ListLanguagesFmtArgs,
}

#[derive(Debug, Clone)]
pub enum ListLanguagesFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

#[derive(Debug, Clone)]
pub struct InfoArgs {
    pub fmt: InfoFmtArgs,
    pub args_plain: Option<InfoPlainArgs>,
}

#[derive(Debug, Clone, Default)]
pub struct InfoPlainArgs {
    pub show_cache_dir: bool,
    pub show_data_dir: bool,
    pub only_values: bool,
}

#[derive(Debug, Clone)]
pub enum InfoFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}
