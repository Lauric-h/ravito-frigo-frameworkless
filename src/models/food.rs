use mysql::prelude::FromValue;
use mysql::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum IngestionType {
    EAT,
    DRINK
}

