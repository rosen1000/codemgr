extern crate serde;

use inquire::Select;
use question::{Answer::*, Question};
use serde_derive::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use toml::toml;

pub use crate::logger::{print, LoggingLevel};

const MANIFEST_PATH: &str = "./manifest.toml";
pub const META_DATA_VERSION: u8 = 1;

#[derive(Debug)]
pub enum ReadManifestError {
    NotFound,
    NoApps
}

pub fn read_manifest() -> Result<Manifest, ReadManifestError> {
    let file = File::open(MANIFEST_PATH);
    let mut manifest_text = String::new();
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                manifest_text += &line.unwrap().to_string();
                manifest_text += &"\n".to_string();
            }
        }
        Err(_) => {
            print("Manifest.toml not found!", LoggingLevel::Warn);
            return Err(ReadManifestError::NotFound);
        }
    }

    let parsed = toml::from_str(&manifest_text);
    if parsed.is_err() {
        return Err(ReadManifestError::NoApps);
    }

    let parsed = parsed.unwrap();
    Ok(parsed)
}

pub fn init_manifest() {
    let answear = Question::new(
        "No manifest.toml was found!\nDo you want to turn this folder into a \"code\" folder?",
    )
    .yes_no()
    .default(YES)
    .show_defaults()
    .confirm();

    if answear == YES {
        println!("Creating new code management dir...");
        let file = File::create(MANIFEST_PATH);
        match file {
            Ok(mut file) => {
                let initial_data = toml! {
                    [meta]
                    version = 1
                };
                file.write(initial_data.to_string().as_bytes()).unwrap();
                fs::create_dir(".apps").unwrap();
                println!("Done!");
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
            }
        }
    } else {
        println!("Good bye!");
    }
}

#[cfg(target_family = "unix")]
pub fn create_sym_link(app: String) {
    std::os::unix::fs::symlink(format!(".apps/{}", app), app).unwrap();
}

#[cfg(target_family = "windows")]
pub fn create_sym_link(app: String) {
    std::os::windows::fs::symlink_dir(format!(".apps/{}", app), app).unwrap();
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Manifest {
    pub meta: Meta,
    pub apps: Vec<App>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Meta {
    pub version: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct App {
    pub name: String,
    pub languages: Vec<String>,
    pub tags: Vec<String>,
}

impl Manifest {
    pub fn save(&self) {
        let mut file = File::create(MANIFEST_PATH).unwrap();
        file.write(toml::to_string(&self).unwrap().as_bytes())
            .unwrap();
    }

    pub fn search_app_by_name(&self) -> Option<&App> {
        let apps = self.apps.iter().map(|app| app.clone().name).collect();
        let query = Select::new("Select project", apps).prompt();

        if let Err(e) = &query {
            print(e, LoggingLevel::Error);
            return None;
        }

        let selected = query.unwrap();

        for app in &self.apps {
            if app.name == selected {
                return Some(app);
            }
        }

        return None;
    }
}
