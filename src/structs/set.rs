use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::util::FromFile;

use super::shop::Shop;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Data {
    pub uuid: String,
    pub shop: Option<Shop>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub part_list: Option<Vec<Data>>,
    pub block_list: Option<Vec<Data>>,
    pub tool_list: Option<Vec<Data>>,
}

impl FromFile<Set> for Set {}

impl Index<&str> for Set {
    type Output = Option<Vec<Data>>;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "part_list" => &self.part_list,
            "block_list" => &self.block_list,
            "tool_list" => &self.tool_list,
            _ => &None,
        }
    }
}
