pub(crate) const CACHE_LOCATION_BROWSER: &str = "";
pub(crate) const DATA_LOCATION_BROWSER: &str = "";

pub(crate) const CACHE_LOCATION_WASM_WITH_FS: &str = "";
pub(crate) const DATA_LOCATION_WASM_WITH_FS: &str = "";

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

    use crate::{args::wasm::InfoWasmArgs, info};

    #[wasm_bindgen]
    pub fn fmt(args: InfoWasmArgs, env_is_browser: Option<bool>) -> Result<String, String> {
        // TODO
        let cache_dir;
        let data_dir;
        if let Some(false) = env_is_browser {
            cache_dir = "fs";
            data_dir = "fs";
        } else {
            cache_dir = "cache";
            data_dir = "local storage";
        }

        info::fmt(args.into(), &cache_dir, &data_dir).map_err(|err| err.to_string())
    }
}
