use std::path::PathBuf;

use itertools::Itertools;
use serde::Serialize;

use crate::{
    args::internal::{InfoArgs, InfoPlainArgs},
    error::WikiError,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AppInfo {
    cache_dir: String,
    data_dir: String,
}

pub fn fmt(args: InfoArgs, cache_dir: &PathBuf, data_dir: &PathBuf) -> Result<String, WikiError> {
    let info = AppInfo {
        cache_dir: cache_dir.to_string_lossy().to_string(),
        data_dir: data_dir.to_string_lossy().to_string(),
    };

    let out = match (args.args_plain, args.args_json) {
        (Some(args_plain), _) => fmt_plain(info, args_plain),
        (_, Some(args_json)) => {
            if args_json.json_raw {
                serde_json::to_string(&info)?
            } else {
                serde_json::to_string_pretty(&info)?
            }
        }
        _ => fmt_plain(info, InfoPlainArgs::default()),
    };

    Ok(out)
}

fn fmt_plain(
    info: AppInfo,
    InfoPlainArgs {
        show_cache_dir,
        show_data_dir,
        only_values,
    }: InfoPlainArgs,
) -> String {
    let no_flags_provided = !show_data_dir && !show_cache_dir;
    let info = [
        (!only_values, "VALUE".into(), "NAME", "DESCRIPTION"),
        (
            show_cache_dir || no_flags_provided,
            info.cache_dir,
            "cache directory",
            "stores caches of ArchWiki pages after download to speed up future requests",
        ),
        (
            show_data_dir || no_flags_provided,
            info.data_dir,
            "data directory",
            "stores log files and ArchWiki metadata",
        ),
    ];

    let out = info
        .iter()
        .filter_map(|entry| {
            entry.0.then_some(if only_values {
                entry.1.clone()
            } else {
                format!(
                    "{name:20} | {desc:90} | {val}",
                    name = entry.2,
                    desc = entry.3,
                    val = entry.1
                )
            })
        })
        .join("\n");

    out
}
