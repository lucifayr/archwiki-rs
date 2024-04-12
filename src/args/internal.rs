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
