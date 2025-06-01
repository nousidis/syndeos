use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub id: Option<i64>,
    pub key: String,
    pub value: String,
}
