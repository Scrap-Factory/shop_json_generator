mod errors;
mod structs;
mod util;

use colored::Colorize;
use error_stack::{IntoReport, Result, ResultExt};
use errors::{GenDbError, GenSetError};
use log::{error, info};
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::Write,
    process,
};
use structs::{database::Database, folder::Folder, set::Set, shop::Shop};
use util::FromFile;

const DBS: [Folder; 2] = [
    Folder {
        path: "./Objects/Database/shapesets.shapedb",
        db_entry: "shape_set_list",
        set_entries: ["block_list", "part_list"],
    },
    Folder {
        path: "./Tools/Database/toolsets.tooldb",
        db_entry: "tool_set_list",
        set_entries: ["tool_list", ""],
    },
];

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

    let mut json: HashMap<String, Shop> = HashMap::new();

    for db in DBS {
        match gen_db(&db, &mut json) {
            Ok(_) => info!("Generated {} succsefully", db.path),
            Err(err) => {
                error!("Failed to generate {}", db.path);
                println!("{err:#?}")
            }
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

fn gen_db(folder: &Folder, json: &mut HashMap<String, Shop>) -> Result<(), GenDbError> {
    let db = Database::from_file(folder.path)
        .change_context(GenDbError)
        .attach_printable(format!("Failed to parse {}", folder.path))?;

    let sets = db[folder.db_entry]
        .as_ref()
        .ok_or(GenDbError)
        .into_report()
        .attach_printable(format!(
            "Failed to find {} in file {}",
            folder.db_entry, folder.path
        ))?;

    for set in sets {
        match gen_set(&set.replace("$CONTENT_DATA", "."), folder, json)
            .change_context(GenDbError)
            .attach_printable(format!("Failed to generate set {}", set))
        {
            Ok(_) => {}
            Err(err) => {
                println!("{err}");
                continue;
            }
        }
    }

    Ok(())
}

fn gen_set(
    path: &str,
    folder: &Folder,
    json: &mut HashMap<String, Shop>,
) -> Result<(), GenSetError> {
    let set = Set::from_file(path)
        .change_context(GenSetError)
        .attach_printable(format!("Failed to parse {path}"))?;

    for entry in folder.set_entries {
        if entry.is_empty() {
            continue;
        }

        if let Some(data) = set[entry].as_ref() {
            for part in data {
                if let Some(shop) = part.shop.clone() {
                    json.insert(part.uuid.clone(), shop);
                }
            }
        }
    }

    Ok(())
}
