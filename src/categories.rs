use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct CategoryListItem {
    name: String,
    url: String,
}

use crate::error::WikiError;

/// Returns a print ready list of the provided page names in
/// 1. A tree format if `flatten` is `false`:
/// Xfce:
/// ───┤Thunar
/// ───┤Xfce
/// ───┤Xfwm
///
/// Xiaomi:
/// ───┤Xiaomi Mi Notebook Air 13.3
/// ───┤Xiaomi Mi Notebook Pro 15.6
///
/// 2. A newline separated list if `flatten` is `true`:
/// Xsettingsd
/// Xsettingsd
/// Xterm
/// Xtrabackup
///
/// Sorting behavior depends on if the list is flattened or not.
///
/// If it is not flattened the list is first ordered by category names and then by page names withing those
/// categories.
/// If it is flattened then it will by sorted by page names.
pub fn list_pages(categories: &HashMap<String, Vec<String>>, flatten: bool) -> String {
    if flatten {
        return categories.values().flatten().unique().sorted().join("\n");
    }

    categories
        .iter()
        .sorted()
        .map(|(cat, pages)| {
            let list = pages.iter().map(|p| format!("───┤{p}")).join("\n");

            format!("{cat}:\n{list}",)
        })
        .join("\n\n")
}

/// TODO replace with api call
/// Scrapes the ArchWiki for all page names and their immediate parent category. Category nesting
/// is ignored as a category can be a sub category of multiple other categories.
///
/// Caution this function will most likely take several minutes to finish (-, – )…zzzZZ
#[allow(unused)]
pub async fn fetch_all_pages(
    hide_progress: bool,
    thread_count: usize,
    max_categories: Option<u32>,
    start_at: Option<&str>,
) -> Result<HashMap<String, Vec<String>>, WikiError> {
    todo!()
}
