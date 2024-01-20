use itertools::Itertools;
use serde::Deserialize;

use crate::{error::WikiError, wiki::Response};

const LANGUAGE_API_URL: &str =
    "https://wiki.archlinux.org/api.php?action=query&meta=siteinfo&siprop=languages&format=json";

#[derive(Debug, Deserialize)]
struct LanguageApiResponse {
    languages: Vec<Language>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Language {
    code: String,
    #[serde(rename = "*")]
    name: String,
}

pub async fn fetch_all_langs() -> Result<Vec<Language>, WikiError> {
    let body = reqwest::get(LANGUAGE_API_URL).await?.text().await?;
    let json: Response<LanguageApiResponse> = serde_json::from_str(&body)?;

    Ok(json.query.languages)
}

pub fn format_lang_table(langs: &[Language]) -> String {
    let mut table = format!("{c1:20} | {c2:90}\n", c1 = "CODE", c2 = "NAME");
    let body = langs
        .iter()
        .sorted_by(|a, b| a.code.cmp(&b.code))
        .map(|l| format!("{code:20} | {name:90}", code = l.code, name = l.name))
        .collect_vec()
        .join("\n");

    table += &body;
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_format_lang_table() {
        let langs = vec![
            Language {
                code: "a".into(),
                name: "aaa".into(),
            },
            Language {
                code: "b".into(),
                name: "abc".into(),
            },
            Language {
                code: "c".into(),
                name: "123".into(),
            },
            Language {
                code: "2".into(),
                name: "fdsal".into(),
            },
            Language {
                code: "1".into(),
                name: "hi".into(),
            },
        ];

        let res = format_lang_table(&langs);
        let res_row_count = res.split('\n').collect_vec().len();
        let second_code = res
            .split('\n')
            .nth(2)
            .unwrap()
            .split('|')
            .next()
            .unwrap()
            .trim();

        assert_eq!(res_row_count, 6);
        assert_eq!(second_code, "2");
    }
}
