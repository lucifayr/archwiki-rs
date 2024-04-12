pub mod search {
    use crate::{error::WikiError, search};
    use clap::CommandFactory;
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen]
    pub async fn fetch() -> Result<String, JsValue> {
        search::fetch(args).await
    }
}
