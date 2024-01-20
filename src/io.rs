use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{error::WikiError, formats::PageFormat};

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
