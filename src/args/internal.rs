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
    pub json: bool,
    pub json_raw: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListLanguagesArgs {
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
