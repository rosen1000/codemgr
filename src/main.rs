use std::fs;

use inquire::{Confirm, MultiSelect, Select, Text};
use util::{init_manifest, read_manifest, App, Manifest};

use crate::util::{print, LoggingLevel, META_DATA_VERSION};

pub mod logger;
pub mod util;

fn main() {
    let manifest = read_manifest();
    if manifest.is_none() {
        init_manifest();
        return;
    }

    let manifest = manifest.unwrap();
    println!("using v{}", manifest.meta.version);
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
            "Open project" => open_project(&mut manifest.to_owned()),
            "New project" => new_project(&mut manifest.to_owned()),
            "Delete project" => delete_project(&mut manifest.to_owned()),
            "Remove code folder" => delete_code_folder(),
            _ => println!("lmao cringe"),
        },
        Err(_) => println!("lmao cringe"),
    }
}

fn open_project(manifest: &mut Manifest) {
    let select = Select::new("Select mode", vec!["by language", "by tags"]).prompt();
    match select {
        Ok(mode) => match mode {
            "by language" => {
                let langs_selection =
                    MultiSelect::new("Select languages", manifest.to_languages()).prompt();
                if let Ok(langs) = langs_selection {
                    manifest.apps.iter().for_each(|app| app.dematerialize());
                    manifest
                        .search_apps_by_langs(langs)
                        .iter()
                        .for_each(|a| a.materialize());
                }
            }
            "by tags" => {
                let tag_selection = MultiSelect::new("Select tags", manifest.to_tags()).prompt();
                if let Ok(tags) = tag_selection {
                    manifest.apps.iter().for_each(|app| app.dematerialize());
                    manifest
                        .search_apps_by_tags(tags)
                        .iter()
                        .for_each(|a| a.materialize());
                }
            }
            _ => {}
        },
        _ => {}
    }
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
        app.materialize();
    }

    manifest.apps.push(app);
    manifest.save();

    print("Done!", LoggingLevel::Info);
}

fn delete_project(manifest: &mut Manifest) {
    let select = Select::new(
        "Which project to delete?",
        manifest
            .apps
            .clone()
            .iter()
            .map(|app| app.clone().name)
            .collect(),
    )
    .prompt();

    let name = select.unwrap();

    let app = manifest
        .apps
        .swap_remove(manifest.apps.iter().position(|a| a.name == name).unwrap());
    if let Err(e) = app.nuke() {
        print(e.to_string(), LoggingLevel::Error);
    }
    println!("{:?}", manifest);
    manifest.save();
}

fn delete_code_folder() {
    fs::remove_dir_all(".apps").unwrap();
    fs::remove_file("manifest.toml").unwrap();
}
