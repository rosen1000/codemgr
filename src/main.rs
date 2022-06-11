use std::fs;

use inquire::{Confirm, Select, Text};
use util::{create_sym_link, init_manifest, read_manifest, App, Manifest, ReadManifestError, Meta};

use crate::util::{print, LoggingLevel, META_DATA_VERSION};

pub mod logger;
pub mod util;

fn main() {
    let manifest = read_manifest();
    let manifest = match manifest {
        Ok(m) => {
            Some(m)
        }
        Err(ReadManifestError::NotFound) => {
            init_manifest();
            None
        }
        Err(ReadManifestError::NoApps) => {
            Some(Manifest {meta: Meta {version : META_DATA_VERSION}, apps : vec![]})
        }
    };
    if manifest.is_none() {
        return;
    }

    let manifest = manifest.unwrap();
    if manifest.meta.version != META_DATA_VERSION {
        println!("This folder is different version from this program!");
        return;
    }

    let answer = Select::new(
        "Operation",
        vec![
            "Open project",
            "New project",
            "Delete project",
            "Remove code folder",
        ],
    )
    .prompt();

    match answer {
        Ok(select) => match select {
            "Open project" => open_project(),
            "New project" => new_project(&mut manifest.to_owned()),
            "Delete project" => delete_project(),
            "Remove code folder" => delete_code_folder(),
            _ => println!("lmao cringe"),
        },
        Err(_) => println!("lmao cringe"),
    }

    // for app in manifest.app.as_ref().unwrap() {
    //     println!("{}: {:?}", app.name, app.languages);
    // }
}

fn open_project() {
    println!("open");
}

fn new_project(manifest: &mut Manifest) {
    let name = Text::new("Name of new project?").prompt().unwrap();
    let languages = Text::new("Languages split by commas")
        .prompt()
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let tags = Text::new("Tags split by commas")
        .prompt()
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let show = Confirm::new("Show new project in folder?")
        .prompt()
        .unwrap();

    let app = App {
        name,
        languages,
        tags,
    };

    let res = fs::create_dir_all(format!(".apps/{}", app.name));
    if let Err(e) = res {
        print(e, LoggingLevel::Error);
        return;
    }

    if show {
        create_sym_link(app.clone().name);
    }

    manifest.apps.push(app);
    manifest.save();

    print("Done!", LoggingLevel::Info);
}

fn delete_project() {
    println!("delete");
}

fn delete_code_folder() {
    fs::remove_dir_all(".apps").unwrap();
    fs::remove_file("manifest.toml").unwrap();
}
