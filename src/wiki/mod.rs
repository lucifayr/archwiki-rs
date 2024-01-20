mod api;
mod download;

pub use api::{fetch_open_search, fetch_page, fetch_text_search, Response};

#[allow(clippy::module_name_repetitions)]
pub use download::{download_wiki, sync_wiki_info};
