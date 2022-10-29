use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Shop {
    pub tier: i32,
    pub price: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prestige: Option<bool>,
}
