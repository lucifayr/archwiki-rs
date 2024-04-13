#[derive(Debug, Clone)]
pub struct SearchArgs {
    pub search: String,
    pub lang: String,
    pub limit: u16,
    pub text_search: bool,
    pub args_json: SearchJsonArgs,
}

impl Default for SearchArgs {
    fn default() -> Self {
        Self {
            search: String::default(),
            lang: String::from("en"),
            limit: 5,
            text_search: false,
            args_json: SearchJsonArgs::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchJsonArgs {
    pub json: bool,
    pub json_raw: bool,
}

#[derive(Debug, Clone, Default)]
pub struct WikiMetadataArgs {
    pub hide_progress: bool,
    pub args_json: Option<WikiMetadataJsonArgs>,
    pub args_yaml: Option<WikiMetadataYamlArgs>,
}

#[derive(Debug, Clone, Default)]
pub struct WikiMetadataJsonArgs {
    pub json: bool,
    pub json_raw: bool,
}

#[derive(Debug, Clone, Default)]
pub struct WikiMetadataYamlArgs {
    pub yaml: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ListPagesArgs {
    pub args_plain: Option<ListPagesPlainArgs>,
    pub args_json: Option<ListPagesJsonArgs>,
}

#[derive(Debug, Clone, Default)]
pub struct ListPagesPlainArgs {
    pub flatten: bool,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListPagesJsonArgs {
    pub json: bool,
    pub json_raw: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListCategoriesArgs {
    pub args_json: Option<ListCategoriesJsonArgs>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListCategoriesJsonArgs {
    pub json: bool,
    pub json_raw: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListLanguagesArgs {
    pub args_json: Option<ListLanguagesJsonArgs>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListLanguagesJsonArgs {
    pub json: bool,
    pub json_raw: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InfoArgs {
    pub args_plain: Option<InfoPlainArgs>,
    pub args_json: Option<InfoJsonArgs>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InfoPlainArgs {
    pub show_cache_dir: bool,
    pub show_data_dir: bool,
    pub only_values: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InfoJsonArgs {
    pub json: bool,
    pub json_raw: bool,
}
