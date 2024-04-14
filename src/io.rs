#![cfg(feature = "cli")]

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{error::WikiError, formats::PageFormat};

pub struct AppDirs {
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub log_dir: PathBuf,
}
pub fn app_dirs() -> Result<AppDirs, WikiError> {
    let Some(base_dir) = directories::BaseDirs::new() else {
        return Err(WikiError::Path(
            "failed to get valid home directory".to_owned(),
        ));
    };

    let cache_dir = base_dir.cache_dir().join("archwiki-rs");
    let data_dir = base_dir.data_local_dir().join("archwiki-rs");
    let log_dir = data_dir.join("logs");

    Ok(AppDirs {
        data_dir,
        cache_dir,
        log_dir,
    })
}

pub fn page_path(page: &str, format: &PageFormat, parent_dir: &Path) -> PathBuf {
    let ext = match format {
        PageFormat::PlainText => "",
        PageFormat::Markdown => "md",
        PageFormat::Html => "html",
    };

    parent_dir.join(to_save_file_name(page)).with_extension(ext)
}

/// Check if a page has been cached.
/// If a page has existed for more then 14 days and `disable_cache_invalidation` is false
/// this function will return false even if a cache file exists.
pub fn page_cache_exists(
    cache_location: &Path,
    disable_cache_invalidation: bool,
) -> Result<bool, WikiError> {
    if !cache_location.exists() {
        return Ok(false);
    } else if disable_cache_invalidation {
        return Ok(true);
    }

    let fourteen_days = 1_209_600;
    let secs_since_modified = fs::File::open(cache_location)?
        .metadata()?
        .modified()?
        .elapsed()?
        .as_secs();

    Ok(secs_since_modified < fourteen_days)
}

pub fn create_dir_if_not_exists(dir: &Path) -> Result<(), WikiError> {
    match fs::create_dir(dir) {
        Ok(()) => {}
        Err(err) => {
            if err.kind() != io::ErrorKind::AlreadyExists {
                return Err(err.into());
            }
        }
    }

    Ok(())
}

pub fn to_save_file_name(page: &str) -> String {
    sanitize_filename::sanitize(page)
}
