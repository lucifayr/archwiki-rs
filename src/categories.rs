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
