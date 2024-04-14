use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::wasm::{InfoWasmArgs, SearchWasmArgs, WikiMetadataWasmArgs},
    info, search, wiki,
};

// TODO replace with functions
pub(crate) const LOCATION_OF_CACHE_IN_BROWSER: &str = "";
pub(crate) const LOCATION_OF_DATA_IN_BROWSER: &str = "";

pub(crate) const LOCATION_OF_CACHE_IN_WASM_WITH_FS: &str = "";
pub(crate) const LOCATION_OF_DATA_IN_WASM_WITH_FS: &str = "";

/// TODO add docs
#[wasm_bindgen]
#[allow(non_snake_case)]
pub async fn fetchWikiMetadata(args: WikiMetadataWasmArgs) -> Result<String, String> {
    wiki::fetch_metadata(args.into())
        .await
        .map_err(|err| err.to_string())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub async fn searchWikiPages(args: SearchWasmArgs) -> Result<String, String> {
    search::fetch(args.into())
        .await
        .map_err(|err| err.to_string())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn appInfo(args: InfoWasmArgs, env_is_browser: Option<bool>) -> Result<String, String> {
    let cache_dir;
    let data_dir;
    if let Some(false) = env_is_browser {
        cache_dir = LOCATION_OF_CACHE_IN_WASM_WITH_FS;
        data_dir = LOCATION_OF_DATA_IN_WASM_WITH_FS;
    } else {
        cache_dir = LOCATION_OF_CACHE_IN_BROWSER;
        data_dir = LOCATION_OF_DATA_IN_BROWSER;
    }

    info::fmt(args.into(), &cache_dir, &data_dir).map_err(|err| err.to_string())
}
