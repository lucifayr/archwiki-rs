use std::collections::HashMap;

use futures::TryFutureExt;
use wasm_bindgen::{convert::IntoWasmAbi, prelude::wasm_bindgen, JsValue};

use crate::{
    args::wasm::{
        ListCategoriesWasmArgs, ListLanguagesWasmArgs, ListPagesWasmArgs, ReadPageWasmArgs,
        SearchWasmArgs, WikiMetadataWasmArgs,
    },
    langs, list, search,
    utils::flip_page_tree,
    wiki,
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

#[wasm_bindgen(js_name = listWikiPages)]
pub fn list_wiki_pages(args: ListPagesWasmArgs, metadata: JsValue) -> Result<String, String> {
    let page_to_category_map: HashMap<String, Vec<String>> =
        serde_wasm_bindgen::from_value(metadata).map_err(|err| err.to_string())?;
    let wiki_tree = flip_page_tree(page_to_category_map);

    list::fmt_pages(args.into(), &wiki_tree).map_err(|err| err.to_string())
}

#[wasm_bindgen(js_name = listWikiCategories)]
pub fn list_wiki_categories(
    args: ListCategoriesWasmArgs,
    metadata: JsValue,
) -> Result<String, String> {
    let page_to_category_map: HashMap<String, Vec<String>> =
        serde_wasm_bindgen::from_value(metadata).map_err(|err| err.to_string())?;
    let wiki_tree = flip_page_tree(page_to_category_map);

    list::fmt_categories(args.into(), &wiki_tree).map_err(|err| err.to_string())
}

#[wasm_bindgen(js_name = listWikiLanguages)]
pub async fn list_wiki_languages(args: ListLanguagesWasmArgs) -> Result<String, String> {
    let langs = langs::fetch_all().await.map_err(|err| err.to_string())?;
    langs::fmt(args.into(), &langs).map_err(|err| err.to_string())
}

#[wasm_bindgen(js_name = setupConsoleError)]
pub fn setup_console_error() {
    console_error_panic_hook::set_once();
}
