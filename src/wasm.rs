// TODO
pub(crate) const LOCATION_OF_CACHE_IN_BROWSER: &str = "";
pub(crate) const LOCATION_OF_DATA_IN_BROWSER: &str = "";

pub(crate) const LOCATION_OF_CACHE_IN_WASM_WITH_FS: &str = "";
pub(crate) const LOCATION_OF_DATA_IN_WASM_WITH_FS: &str = "";

pub mod wiki {
    use wasm_bindgen::prelude::wasm_bindgen;

    use crate::{args::wasm::WikiMetadataWasmArgs, wiki};

    #[wasm_bindgen]
    pub async fn fetch_wiki_metadata(args: WikiMetadataWasmArgs) -> Result<String, String> {
        wiki::fetch_metadata(args.into())
            .await
            .map_err(|err| err.to_string())
    }
}

pub mod search {
    use crate::{args::wasm::SearchWasmArgs, search};
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub async fn fetch(args: SearchWasmArgs) -> Result<String, String> {
        search::fetch(args.into())
            .await
            .map_err(|err| err.to_string())
    }
}

pub mod info {
    use wasm_bindgen::prelude::wasm_bindgen;

    use crate::{
        args::wasm::InfoWasmArgs, info, LOCATION_OF_CACHE_IN_BROWSER,
        LOCATION_OF_CACHE_IN_WASM_WITH_FS, LOCATION_OF_DATA_IN_BROWSER,
        LOCATION_OF_DATA_IN_WASM_WITH_FS,
    };

    #[wasm_bindgen]
    pub fn fmt(args: InfoWasmArgs, env_is_browser: Option<bool>) -> Result<String, String> {
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
}
