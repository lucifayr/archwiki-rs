use futures::TryFutureExt;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::wasm::{InfoWasmArgs, ReadPageWasmArgs, SearchWasmArgs, WikiMetadataWasmArgs},
    info, search, wiki,
};

// TODO replace with functions
pub(crate) const LOCATION_OF_CACHE_IN_BROWSER: &str = "";
pub(crate) const LOCATION_OF_DATA_IN_BROWSER: &str = "";

pub(crate) const LOCATION_OF_CACHE_IN_WASM_WITH_FS: &str = "";
pub(crate) const LOCATION_OF_DATA_IN_WASM_WITH_FS: &str = "";

// TODO add docs

#[wasm_bindgen(js_name = fetchWikiPage)]
pub async fn fetch_wiki_page(args: ReadPageWasmArgs) -> Result<String, String> {
    wiki::fetch_and_format_page(args.into())
        .await
        .map_err(|err| err.to_string())
}

#[wasm_bindgen(js_name = searchWikiPages)]
pub async fn search_wiki_pages(args: SearchWasmArgs) -> Result<String, String> {
    search::fetch(args.into())
        .await
        .map_err(|err| err.to_string())
}

#[wasm_bindgen(js_name = fetchWikiMetadata)]
pub async fn fetch_wiki_metadata(args: WikiMetadataWasmArgs) -> Result<String, String> {
    wiki::fetch_metadata(args.into())
        .await
        .map_err(|err| err.to_string())
}

#[wasm_bindgen(js_name = appInfo)]
pub fn app_info(args: InfoWasmArgs) -> Result<String, String> {
    let cache_dir;
    let data_dir;
    if todo!("create function that used feature flags to get env") {
        cache_dir = LOCATION_OF_CACHE_IN_WASM_WITH_FS;
        data_dir = LOCATION_OF_DATA_IN_WASM_WITH_FS;
    } else {
        cache_dir = LOCATION_OF_CACHE_IN_BROWSER;
        data_dir = LOCATION_OF_DATA_IN_BROWSER;
    }

    info::fmt(args.into(), &cache_dir, &data_dir).map_err(|err| err.to_string())
}
