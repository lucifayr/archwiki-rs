use futures::TryFutureExt;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::wasm::{ReadPageWasmArgs, SearchWasmArgs, WikiMetadataWasmArgs},
    info, search, wiki,
};

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
