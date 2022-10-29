use serde::Deserialize;

use crate::util::FromFile;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Databse {
    pub shape_set_list: Vec<String>,
}

impl FromFile<Databse> for Databse {}
