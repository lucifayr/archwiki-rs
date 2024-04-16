use std::collections::HashMap;

use futures::TryFutureExt;
use wasm_bindgen::{convert::IntoWasmAbi, prelude::wasm_bindgen, JsValue};

use crate::{
    args::{
        internal,
        wasm::{
            ListCategoriesArgs, ListLanguagesArgs, ListPagesArgs, ReadPageArgs, SearchArgs,
            WikiMetadataArgs,
        },
    },
    error, langs, list, search,
    utils::{flip_page_tree, is_archwiki_url},
    wiki,
};

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
}

/// Fetch a single article page from the ArchWiki. Provide either the name of the page or an
/// absolute URL of the format 'https://wiki.archlinux.org/title/{page}'.
///
/// # Returns
///
/// The fetched page in the specified format. Defaults HTML if no format is provided.
///
/// # Errors
///
/// - On network errors
/// - When no page is found
#[wasm_bindgen(js_name = fetchWikiPage)]
pub async fn fetch_wiki_page(args: ReadPageArgs) -> Result<String, error::WasmWikiError> {
    let mut args: internal::ReadPageArgs = args.into();
    if let Some(page) = is_archwiki_url(&args.page) {
        args.page = page.to_owned();
    };

    wiki::fetch_and_format_page(args).await.map_err(Into::into)
}

/// Search content on the ArchWiki for the specified query. See `SearchArgs` for more details
/// on how to define a search.
///
/// # Returns
///
/// A string contain a list of search results. The Structure of the string varies depending on the
/// provided `args`.
///
/// # Errors
///
/// - On network errors
/// - On serialization/deserialization errors
#[wasm_bindgen(js_name = searchWikiPages)]
pub async fn search_wiki_pages(args: SearchArgs) -> Result<String, error::WasmWikiError> {
    search::fetch(args.into()).await.map_err(Into::into)
}

/// Fetch page and category metadata from the ArchWiki. This takes a few seconds depending on
/// network speed, so the result should be stored in a browser cache or to disk for future use.
///
/// # Returns
///
/// A JSON string of the following format:
///
/// ```json
/// {
///   "page-name": [
///     "category-name-1",
///     "category-name-2"
///   ]
/// }
/// ```
///
/// Example:
///
/// ```json
/// {
///   "OpenSSH": [
///     "OpenBSD",
///     "Secure Shell",
///     "Servers"
///   ]
/// }
/// ```
///
/// # Errors
///
/// - On network errors
/// - On serialization/deserialization errors
#[wasm_bindgen(js_name = fetchWikiMetadata)]
pub async fn fetch_wiki_metadata(args: WikiMetadataArgs) -> Result<String, error::WasmWikiError> {
    wiki::fetch_metadata(args.into()).await.map_err(Into::into)
}

/// Format the provided `metadata` as a list of pages. See `fetchWikiMetadata` on how to fetch this metadata.
///
/// # Returns
///
/// A string a string formatted based on the provided `args`. Defaults to raw JSON if no format is
/// provided.
///
/// # Errors
///
/// - On network errors
/// - On serialization/deserialization errors
#[wasm_bindgen(js_name = listWikiPages)]
pub fn list_wiki_pages(
    args: ListPagesArgs,
    metadata: JsValue,
) -> Result<String, error::WasmWikiError> {
    let page_to_category_map: HashMap<String, Vec<String>> =
        serde_wasm_bindgen::from_value(metadata)?;
    let wiki_tree = flip_page_tree(page_to_category_map);

    list::fmt_pages(args.into(), &wiki_tree).map_err(Into::into)
}

/// Format the provided `metadata` as a list of categories. See `fetchWikiMetadata` on how to fetch this metadata.
///
/// # Returns
///
/// A string a string formatted based on the provided `args`. Defaults to raw JSON if no format is
/// provided.
///
/// # Errors
///
/// - On network errors
/// - On serialization/deserialization errors
#[wasm_bindgen(js_name = listWikiCategories)]
pub fn list_wiki_categories(
    args: ListCategoriesArgs,
    metadata: JsValue,
) -> Result<String, error::WasmWikiError> {
    let page_to_category_map: HashMap<String, Vec<String>> =
        serde_wasm_bindgen::from_value(metadata)?;
    let wiki_tree = flip_page_tree(page_to_category_map);

    list::fmt_categories(args.into(), &wiki_tree).map_err(Into::into)
}

/// Fetch the list of supported languages from the ArchWiki.
///
/// # Returns
///
/// A string contain a list language names and codes formatted depending on the provided `args`.
/// Defaults to raw JSON if no format is provided.
///
/// # Errors
///
/// - On network errors
/// - On serialization/deserialization errors
#[wasm_bindgen(js_name = listWikiLanguages)]
pub async fn list_wiki_languages(args: ListLanguagesArgs) -> Result<String, error::WasmWikiError> {
    let langs = langs::fetch_all().await?;
    langs::fmt(args.into(), &langs).map_err(Into::into)
}
