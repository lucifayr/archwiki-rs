#![warn(clippy::pedantic)]
#![allow(dead_code)]
#![allow(unused)]
#![allow(clippy::doc_markdown)]

mod args;
mod error;
mod formats;
mod info;
mod io;
mod langs;
mod list;
mod search;
mod utils;
mod wasm;
mod wiki;

pub use wasm::*;
