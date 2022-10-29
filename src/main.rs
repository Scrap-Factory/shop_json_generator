mod errors;
mod structs;
mod util;

use colored::Colorize;
use error_stack::{Result, ResultExt};
use errors::{GenDbError, GenSetError};
use log::{error, info};
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::Write,
    process,
};
use structs::{database::Databse, set::Set, shop::Shop};
use util::FromFile;

const DATABASE: &str = "./Objects/Database/shapesets.shapedb";
fn main() {
    simple_logger::init().expect("Failed to init a logger");

    let output_dir = String::from("./Scripts");
    let json_name = String::from("shop.json");

    create_dir_all(output_dir.clone()).unwrap_or_else(|_| {
        println!(
            "{} | No permission to create {}",
            "Error".bright_red(),
            output_dir
        );
        process::exit(1)
    });

    let mut file = File::create(&format!("{output_dir}/{json_name}")).unwrap_or_else(|_| {
        println!(
            "{} | No permission to create {}",
            "Error".bright_red(),
            json_name
        );
        process::exit(1)
    });

    let mut json = HashMap::new();

    match gen_db(DATABASE, &mut json) {
        Ok(_) => info!("Generated everything succsefully "),
        Err(err) => {
            error!("Failed to generate {DATABASE}");
            println!("{err:#?}")
        }
    }

    match serde_json::to_string_pretty(&json) {
        Ok(json) => match file.write_all(json.as_bytes()) {
            Ok(_) => {}
            Err(_) => error!("Failed to write to file {output_dir}/{json_name}"),
        },
        Err(err) => error!("{err}"),
    }
}

fn gen_db(path: &str, json: &mut HashMap<String, Shop>) -> Result<(), GenDbError> {
    let db = Databse::from_file(path)
        .change_context(GenDbError)
        .attach_printable(format!("Failed to parse {path}"))?;

    for path in db.shape_set_list {
        gen_set(&path.replace("$CONTENT_DATA", "."), json)
            .change_context(GenDbError)
            .attach_printable(format!("Failed to generate set {path}"))?;
    }

    Ok(())
}

fn gen_set(path: &str, json: &mut HashMap<String, Shop>) -> Result<(), GenSetError> {
    let set = Set::from_file(path)
        .change_context(GenSetError)
        .attach_printable(format!("Failed to parse {path}"))?;

    if let Some(parts) = set.part_list {
        for part in parts {
            if let Some(shop) = part.shop {
                json.insert(part.uuid, shop);
            }
        }
    }

    if let Some(blocks) = set.block_list {
        for block in blocks {
            if let Some(shop) = block.shop {
                json.insert(block.uuid, shop);
            }
        }
    }

    Ok(())
}
