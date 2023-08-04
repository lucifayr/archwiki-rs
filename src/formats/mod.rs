use clap::ValueEnum;

pub mod plain_text;

#[derive(Debug, Clone, ValueEnum)]
pub enum PageFormat {
    PlainText,
    Markdown,
    Html,
}
