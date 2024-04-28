mod api;
mod download;

#[allow(unused)]
pub use api::{fetch_and_format_page, fetch_open_search, fetch_page, fetch_text_search, Response};

#[allow(unused)]
pub use download::fetch_metadata;

#[cfg(feature = "cli")]
#[allow(unused)]
pub use download::copy_wiki_to_fs;
