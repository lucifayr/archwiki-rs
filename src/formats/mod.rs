mod html;
mod markdown;
mod plain_text;

pub use html::convert_page_to_html;
pub use markdown::convert_page_to_markdown;
pub use plain_text::{convert_page_to_plain_text, format_children_as_plain_text};
use scraper::Html;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[cfg_attr(
    any(feature = "wasm-web", feature = "wasm-nodejs"),
    wasm_bindgen::prelude::wasm_bindgen(js_name = PageFmtArgs)
)]
pub enum PageFormat {
    PlainText,
    Markdown,
    Html,
}

impl Default for PageFormat {
    fn default() -> Self {
        Self::PlainText
    }
}

pub fn format_page(
    format: &PageFormat,
    page: &Html,
    page_title: &str,
    show_urls_for_plain: bool,
) -> String {
    match format {
        PageFormat::PlainText => convert_page_to_plain_text(page, show_urls_for_plain),
        PageFormat::Markdown => convert_page_to_markdown(page, page_title),
        PageFormat::Html => convert_page_to_html(page, page_title),
    }
}
