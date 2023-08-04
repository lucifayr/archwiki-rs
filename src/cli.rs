use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Read a page from the Archwiki",
        long_about = "Read a page from the Archwiki, if the page is not found similar page names are recommended. A list of page names is in the pages.yml file which can be updated with the 'update-all' and 'update-category' commands."
    )]
    ReadPage {
        #[arg(short, long)]
        /// Show URLs in output
        show_urls: bool,
        page: String,
    },
    #[command(
        about = "List all pages from the Archwiki that have been downloaded.",
        long_about = "List all pages from the Archwiki that have been downloaded. See 'update-all' or 'update-category' for information on downloading."
    )]
    ListCategories {
        #[arg(short, long)]
        /// Flatten all pages and don't show category names
        flatten: bool,
    },
    #[command(
        about = "Download all pages from a category",
        long_about = "Download all pages from a category. Categories are stored in the pages.yml file."
    )]
    UpdateCategory { category: String },
    #[command(
        about = "Download all pages from the Archwiki",
        long_about = "Download all pages from the archwiki. Only the English pages are stored."
    )]
    UpdateAll,
}

#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}
