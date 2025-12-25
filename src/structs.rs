#[derive(Default, Debug)]
pub struct SelectData {
    pub items: Vec<SelectField>,
    pub table: Option<String>,
    pub limit: Option<u64>,
    pub only: bool,
}

#[derive(Debug)]
pub struct SelectField {
    pub name: String,
    pub alias: Option<String>,
}
