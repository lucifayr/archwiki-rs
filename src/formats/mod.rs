mod html;
mod markdown;
mod plain_text;

pub use html::convert_page_to_html;
pub use markdown::convert_page_to_markdown;
pub use plain_text::{convert_page_to_plain_text, format_children_as_plain_text};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PageFormat {
    PlainText,
    Markdown,
    Html,
}
