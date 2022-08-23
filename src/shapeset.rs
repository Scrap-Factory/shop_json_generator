use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Part {
    pub uuid: String,
    pub shop: Option<Shop>,
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Shop {
    pub tier: i32,
    pub price: String,
    pub category: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Shapeset {
    pub partList: Vec<Part>,
}
