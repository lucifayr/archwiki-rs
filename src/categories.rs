#![allow(unused)]

use itertools::Itertools;
use std::collections::HashMap;

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
pub fn list_pages(
    wiki_tree: &HashMap<String, Vec<String>>,
    categories_filter: Option<&[String]>,
    flatten: bool,
) -> String {
    if flatten {
        return wiki_tree
            .iter()
            .filter_map(|(cat, pages)| {
                categories_filter.map_or(Some(pages), |filter| {
                    filter.iter().contains(cat).then_some(pages)
                })
            })
            .flatten()
            .unique()
            .sorted()
            .join("\n");
    }

    wiki_tree
        .iter()
        .filter_map(|(cat, pages)| {
            categories_filter.map_or(Some((cat, pages)), |filter| {
                filter.iter().contains(cat).then_some((cat, pages))
            })
        })
        .sorted()
        .map(|(cat, pages)| {
            let list = pages.iter().map(|p| format!("───┤{p}")).join("\n");

            format!("{cat}:\n{list}",)
        })
        .join("\n\n")
}
