mod api;
mod download;

#[allow(unused)]
pub use api::{fetch_and_format_page, fetch_open_search, fetch_page, fetch_text_search, Response};

pub use download::fetch_metadata;

#[cfg(feature = "cli")]
pub use download::copy_wiki_to_fs;
