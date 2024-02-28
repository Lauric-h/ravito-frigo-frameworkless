pub struct Food {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) ingestion: IngestionType,
    pub(crate) carbs: i32,
    pub(crate) calories: i32,
    pub(crate) proteins: i32,
    pub(crate) electrolytes: bool,
    pub(crate) comment: String
}

pub(crate) enum IngestionType {
    EAT,
    DRINK
}