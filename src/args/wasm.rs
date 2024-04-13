use wasm_bindgen::prelude::wasm_bindgen;

use super::internal::{
    InfoArgs, InfoJsonArgs, InfoPlainArgs, ListCategoriesArgs, ListCategoriesJsonArgs,
    ListCategoriesPlainArgs, ListLanguagesArgs, ListLanguagesJsonArgs, ListLanguagesPlainArgs,
    ListPagesArgs, ListPagesJsonArgs, ListPagesPlainArgs, SearchArgs, SearchJsonArgs,
    SearchPlainArgs, WikiMetadataArgs, WikiMetadataJsonArgs, WikiMetadataYamlArgs,
};

#[derive(Debug)]
#[wasm_bindgen]
pub struct SearchWasmArgs {
    search: Option<String>,
    lang: Option<String>,
    limit: Option<u16>,
    text_search: Option<bool>,
    args_plain: Option<SearchPlainWasmArgs>,
    args_json: Option<SearchJsonWasmArgs>,
}

impl From<SearchWasmArgs> for SearchArgs {
    fn from(
        SearchWasmArgs {
            search,
            lang,
            limit,
            text_search,
            args_plain,
            args_json,
        }: SearchWasmArgs,
    ) -> Self {
        Self {
            search: search.unwrap_or_else(|| Self::default().search),
            lang: lang.unwrap_or_else(|| Self::default().lang),
            limit: limit.unwrap_or_else(|| Self::default().limit),
            text_search: text_search.unwrap_or_else(|| Self::default().text_search),
            args_plain: args_plain.map(Into::into),
            args_json: Some(args_json.unwrap_or_default().into()),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct SearchPlainWasmArgs {
    plain: Option<bool>,
}

impl From<SearchPlainWasmArgs> for SearchPlainArgs {
    fn from(SearchPlainWasmArgs { plain }: SearchPlainWasmArgs) -> Self {
        Self {
            plain: plain.unwrap_or_else(|| Self::default().plain),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct SearchJsonWasmArgs {
    json: Option<bool>,
    json_raw: Option<bool>,
}

impl Default for SearchJsonWasmArgs {
    fn default() -> Self {
        Self {
            json: None,
            json_raw: Some(true),
        }
    }
}

impl From<SearchJsonWasmArgs> for SearchJsonArgs {
    fn from(SearchJsonWasmArgs { json, json_raw }: SearchJsonWasmArgs) -> Self {
        Self {
            json: json.unwrap_or_else(|| Self::default().json),
            json_raw: json_raw.unwrap_or_else(|| Self::default().json_raw),
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WikiMetadataWasmArgs {
    args_yaml: Option<WikiMetadataYamlWasmArgs>,
    args_json: Option<WikiMetadataJsonWasmArgs>,
}

impl From<WikiMetadataWasmArgs> for WikiMetadataArgs {
    fn from(
        WikiMetadataWasmArgs {
            args_yaml,
            args_json,
        }: WikiMetadataWasmArgs,
    ) -> Self {
        Self {
            hide_progress: true,
            args_json: Some(args_json.unwrap_or_default().into()),
            args_yaml: args_yaml.map(Into::into),
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WikiMetadataYamlWasmArgs {
    yaml: Option<bool>,
}

impl From<WikiMetadataYamlWasmArgs> for WikiMetadataYamlArgs {
    fn from(WikiMetadataYamlWasmArgs { yaml }: WikiMetadataYamlWasmArgs) -> Self {
        Self {
            yaml: yaml.unwrap_or_else(|| Self::default().yaml),
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WikiMetadataJsonWasmArgs {
    json: Option<bool>,
    json_raw: Option<bool>,
}

impl Default for WikiMetadataJsonWasmArgs {
    fn default() -> Self {
        Self {
            json: None,
            json_raw: Some(true),
        }
    }
}

impl From<WikiMetadataJsonWasmArgs> for WikiMetadataJsonArgs {
    fn from(WikiMetadataJsonWasmArgs { json, json_raw }: WikiMetadataJsonWasmArgs) -> Self {
        Self {
            json: json.unwrap_or_else(|| Self::default().json),
            json_raw: json_raw.unwrap_or_else(|| Self::default().json_raw),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListPagesWasmArgs {
    args_plain: Option<ListPagesPlainWasmArgs>,
    args_json: Option<ListPagesJsonWasmArgs>,
}

impl From<ListPagesWasmArgs> for ListPagesArgs {
    fn from(
        ListPagesWasmArgs {
            args_plain,
            args_json,
        }: ListPagesWasmArgs,
    ) -> Self {
        Self {
            args_plain: args_plain.map(Into::into),
            args_json: Some(args_json.unwrap_or_default().into()),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListPagesPlainWasmArgs {
    flatten: Option<bool>,
    categories: Option<Vec<String>>,
}

impl From<ListPagesPlainWasmArgs> for ListPagesPlainArgs {
    fn from(
        ListPagesPlainWasmArgs {
            flatten,
            categories,
        }: ListPagesPlainWasmArgs,
    ) -> Self {
        Self {
            flatten: flatten.unwrap_or_else(|| Self::default().flatten),
            categories: categories.unwrap_or_else(|| Self::default().categories),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListPagesJsonWasmArgs {
    json: Option<bool>,
    json_raw: Option<bool>,
}

impl Default for ListPagesJsonWasmArgs {
    fn default() -> Self {
        Self {
            json: None,
            json_raw: Some(true),
        }
    }
}

impl From<ListPagesJsonWasmArgs> for ListPagesJsonArgs {
    fn from(ListPagesJsonWasmArgs { json, json_raw }: ListPagesJsonWasmArgs) -> Self {
        Self {
            json: json.unwrap_or_else(|| Self::default().json),
            json_raw: json_raw.unwrap_or_else(|| Self::default().json_raw),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListCategoriesWasmArgs {
    args_plain: Option<ListCategoriesPlainWasmArgs>,
    args_json: Option<ListCategoriesJsonWasmArgs>,
}

impl From<ListCategoriesWasmArgs> for ListCategoriesArgs {
    fn from(
        ListCategoriesWasmArgs {
            args_json,
            args_plain,
        }: ListCategoriesWasmArgs,
    ) -> Self {
        Self {
            args_plain: args_plain.map(Into::into),
            args_json: Some(args_json.unwrap_or_default().into()),
        }
    }
}
#[derive(Debug)]
#[wasm_bindgen]
pub struct ListCategoriesPlainWasmArgs {
    plain: Option<bool>,
}

impl From<ListCategoriesPlainWasmArgs> for ListCategoriesPlainArgs {
    fn from(ListCategoriesPlainWasmArgs { plain }: ListCategoriesPlainWasmArgs) -> Self {
        Self {
            plain: plain.unwrap_or_else(|| Self::default().plain),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListCategoriesJsonWasmArgs {
    json: Option<bool>,
    json_raw: Option<bool>,
}

impl Default for ListCategoriesJsonWasmArgs {
    fn default() -> Self {
        Self {
            json: None,
            json_raw: Some(true),
        }
    }
}

impl From<ListCategoriesJsonWasmArgs> for ListCategoriesJsonArgs {
    fn from(ListCategoriesJsonWasmArgs { json, json_raw }: ListCategoriesJsonWasmArgs) -> Self {
        Self {
            json: json.unwrap_or_else(|| Self::default().json),
            json_raw: json_raw.unwrap_or_else(|| Self::default().json_raw),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListLanguagesWasmArgs {
    args_plain: Option<ListLanguagesPlainWasmArgs>,
    args_json: Option<ListLanguagesJsonWasmArgs>,
}

impl From<ListLanguagesWasmArgs> for ListLanguagesArgs {
    fn from(
        ListLanguagesWasmArgs {
            args_json,
            args_plain,
        }: ListLanguagesWasmArgs,
    ) -> Self {
        Self {
            args_plain: args_plain.map(Into::into),
            args_json: Some(args_json.unwrap_or_default().into()),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListLanguagesPlainWasmArgs {
    plain: Option<bool>,
}

impl From<ListLanguagesPlainWasmArgs> for ListLanguagesPlainArgs {
    fn from(ListLanguagesPlainWasmArgs { plain }: ListLanguagesPlainWasmArgs) -> Self {
        Self {
            plain: plain.unwrap_or_else(|| Self::default().plain),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListLanguagesJsonWasmArgs {
    json: Option<bool>,
    json_raw: Option<bool>,
}

impl Default for ListLanguagesJsonWasmArgs {
    fn default() -> Self {
        Self {
            json: None,
            json_raw: Some(true),
        }
    }
}

impl From<ListLanguagesJsonWasmArgs> for ListLanguagesJsonArgs {
    fn from(ListLanguagesJsonWasmArgs { json, json_raw }: ListLanguagesJsonWasmArgs) -> Self {
        Self {
            json: json.unwrap_or_else(|| Self::default().json),
            json_raw: json_raw.unwrap_or_else(|| Self::default().json_raw),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct InfoWasmArgs {
    args_plain: Option<InfoPlainWasmArgs>,
    args_json: Option<InfoJsonWasmArgs>,
}

impl From<InfoWasmArgs> for InfoArgs {
    fn from(
        InfoWasmArgs {
            args_plain,
            args_json,
        }: InfoWasmArgs,
    ) -> Self {
        Self {
            args_plain: args_plain.map(Into::into),
            args_json: Some(args_json.unwrap_or_default().into()),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct InfoPlainWasmArgs {
    show_cache_dir: Option<bool>,
    show_data_dir: Option<bool>,
    only_values: Option<bool>,
}

impl From<InfoPlainWasmArgs> for InfoPlainArgs {
    fn from(
        InfoPlainWasmArgs {
            show_cache_dir,
            show_data_dir,
            only_values,
        }: InfoPlainWasmArgs,
    ) -> Self {
        Self {
            show_cache_dir: show_cache_dir.unwrap_or_else(|| Self::default().show_cache_dir),
            show_data_dir: show_data_dir.unwrap_or_else(|| Self::default().show_data_dir),
            only_values: only_values.unwrap_or_else(|| Self::default().only_values),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct InfoJsonWasmArgs {
    json: Option<bool>,
    json_raw: Option<bool>,
}

impl Default for InfoJsonWasmArgs {
    fn default() -> Self {
        Self {
            json: None,
            json_raw: Some(true),
        }
    }
}

impl From<InfoJsonWasmArgs> for InfoJsonArgs {
    fn from(InfoJsonWasmArgs { json, json_raw }: InfoJsonWasmArgs) -> Self {
        Self {
            json: json.unwrap_or_else(|| Self::default().json),
            json_raw: json_raw.unwrap_or_else(|| Self::default().json_raw),
        }
    }
}
