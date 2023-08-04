use clap::ValueEnum;

pub mod html;
pub mod markdown;
pub mod plain_text;

#[derive(Debug, Clone, ValueEnum)]
pub enum PageFormat {
    PlainText,
    Markdown,
    Html,
}
