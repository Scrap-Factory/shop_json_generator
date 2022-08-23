#![feature(path_try_exists)]

mod shapeset;

use shapeset::Shapeset;

use colored::Colorize;
use json_comments::StripComments;

use serde_json::{from_reader, to_string};
use std::fs;

fn main() {
    let mut json = String::from("{\n");
    let files = [
        "./Objects/Database/ShapeSets/belts.shapeset",
        "./Objects/Database/ShapeSets/droppers.shapeset",
        "./Objects/Database/ShapeSets/furnaces.shapeset",
        "./Objects/Database/ShapeSets/generators.shapeset",
        "./Objects/Database/ShapeSets/storage.shapeset",
        "./Objects/Database/ShapeSets/upgraders.shapeset",
        "./Objects/Database/ShapeSets/decor.shapeset",
    ];
    for file in files.iter() {
        if let Ok(false) = fs::try_exists(file) {
            println!(
                "{} | path {} doesn't exist",
                "Warning".bright_yellow(),
                file
            );
            continue;
        }

        let file = fs::read_to_string(file).unwrap();
        let stripped = StripComments::new(file.as_bytes());
        let file: Shapeset = from_reader(stripped).unwrap();
        for part in file.partList.iter() {
            if part.shop.is_none() {
                println!(
                    "{} | Part {} doesn't have a shop property",
                    "Warning".bright_yellow(),
                    part.uuid
                );
                continue;
            }
            if part.name.is_none() {
                println!(
                    "{} | Part {} doesn't have a name property",
                    "Warning".bright_yellow(),
                    part.uuid
                )
            } else {
                json += &format!("  // {}\n", part.name.as_ref().unwrap());
            }
            json += &format!(r#"    "{}": "#, part.uuid);
            json += &to_string(part.shop.as_ref().unwrap()).unwrap();
            json += ",\n";
        }
    }
    json.pop();
    json.pop();
    json += "\n";
    json += "}";

    fs::write("./Scripts/shop.json", json).unwrap();
}
