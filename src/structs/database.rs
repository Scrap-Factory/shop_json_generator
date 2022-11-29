use std::ops::Index;

use serde::Deserialize;

use crate::util::FromFile;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    pub shape_set_list: Option<Vec<String>>,
    pub tool_set_list: Option<Vec<String>>,
}

impl FromFile<Database> for Database {}

impl Index<&str> for Database {
    type Output = Option<Vec<String>>;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "shape_set_list" => &self.shape_set_list,
            "tool_set_list" => &self.tool_set_list,
            _ => &None,
        }
    }
}
