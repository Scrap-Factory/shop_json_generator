use serde::{Deserialize, Serialize};

use crate::util::FromFile;

use super::shop::Shop;

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub uuid: String,
    pub shop: Option<Shop>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub part_list: Option<Vec<Data>>,
    pub block_list: Option<Vec<Data>>,
}

impl FromFile<Set> for Set {}
