use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    args::internal::{ListCategoriesArgs, ListPagesArgs, ListPagesPlainArgs},
    error::WikiError,
    utils::UNCATEGORIZED_KEY,
};

pub fn fmt_pages(
    ListPagesArgs {
        args_plain,
        args_json,
    }: ListPagesArgs,
    wiki_tree: &HashMap<String, Vec<String>>,
) -> Result<String, WikiError> {
    let out = match (args_plain, args_json) {
        (Some(args_plain), _) => fmt_page_tree(wiki_tree, args_plain),

        (_, Some(args_json)) => {
            if args_json.json_raw {
                serde_json::to_string(wiki_tree)?
            } else {
                serde_json::to_string_pretty(wiki_tree)?
            }
        }
        _ => fmt_page_tree(wiki_tree, ListPagesPlainArgs::default()),
    };

    Ok(out)
}

pub fn fmt_categories(
    args: ListCategoriesArgs,
    wiki_tree: &HashMap<String, Vec<String>>,
) -> Result<String, WikiError> {
    let out = match args.args_json {
        Some(args_json) if args_json.json_raw => serde_json::to_string(wiki_tree)?,
        Some(_) => serde_json::to_string_pretty(wiki_tree)?,
        None => wiki_tree
            .keys()
            .unique()
            .sorted()
            .filter(|cat| cat.as_str() != UNCATEGORIZED_KEY)
            .join("\n"),
    };

    Ok(out)
}

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
fn fmt_page_tree(
    wiki_tree: &HashMap<String, Vec<String>>,
    ListPagesPlainArgs {
        flatten,
        categories,
    }: ListPagesPlainArgs,
) -> String {
    let categories = (!categories.is_empty()).then_some(&categories);

    if flatten {
        return wiki_tree
            .iter()
            .filter_map(|(cat, pages)| {
                categories.map_or(Some(pages), |filter| {
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
            categories.map_or(Some((cat, pages)), |filter| {
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
