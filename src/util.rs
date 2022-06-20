extern crate serde;

use inquire::Select;
use question::{Answer::*, Question};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use toml::toml;

pub use crate::logger::{print, LoggingLevel};

const MANIFEST_PATH: &str = "./manifest.toml";
pub const META_DATA_VERSION: u8 = 1;

pub fn read_manifest() -> Option<Manifest> {
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
            // return Err(ReadManifestError::NotFound);
            return None;
        }
    }

    let parsed = toml::from_str(&manifest_text);
    if parsed.is_err() {
        let manifest = Manifest {
            meta: Meta {
                version: META_DATA_VERSION,
            },
            apps: vec![],
        };
        manifest.save();
        return Some(manifest);
    }

    Some(parsed.unwrap())
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
fn create_sym_link(app: String) -> Result<(), std::io::Error> {
    std::os::unix::fs::symlink(format!(".apps/{}", app), app)
}

#[cfg(target_family = "windows")]
fn create_sym_link(app: String) -> Result<(), std::io::Error> {
    std::os::windows::fs::symlink_dir(format!(".apps/{}", app), app);
}

fn delete_sym_link(app: String) -> Result<(), std::io::Error> {
    std::fs::remove_file(app)
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

impl App {
    pub fn materialize(&self) {
        if let Err(e) = create_sym_link(self.clone().name) {
            if e.raw_os_error().unwrap() != 17 {
                print(e.to_string(), LoggingLevel::Error);
            }
        }
    }

    pub fn dematerialize(&self) {
        if let Err(e) = delete_sym_link(self.clone().name) {
            if e.raw_os_error().unwrap() != 2 {
                print(e.to_string(), LoggingLevel::Error);
            }
        }
    }

    pub fn nuke(&self) -> Result<(), std::io::Error> {
        let name = self.clone().name;
        delete_sym_link(name.clone())?;
        fs::remove_dir_all(format!(".apps/{}", name))?;
        Ok(())
    }
}

impl Manifest {
    pub fn save(&self) {
        let mut file = File::create(MANIFEST_PATH).unwrap();
        let manifest_toml = toml::to_string(&self);
        println!("{:?}", manifest_toml);
        if let Ok(to_write) = manifest_toml {
            if let Err(e) = file.write(to_write.as_bytes()) {
                print(e.to_string(), LoggingLevel::Error);
            }
        } else if let Err(e) = manifest_toml {
            print(e.to_string(), LoggingLevel::Error);
        }
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

    pub fn search_apps_by_tags(&self, tags: Vec<String>) -> Vec<&App> {
        let mut apps: Vec<&App> = vec![];

        self.apps.iter().for_each(|app| {
            app.tags.iter().for_each(|tag| {
                if tags.contains(tag) {
                    apps.push(app);
                }
            })
        });

        apps
    }

    pub fn search_apps_by_langs(&self, langs: Vec<String>) -> Vec<&App> {
        let mut apps: Vec<&App> = vec![];

        self.apps.iter().for_each(|app| {
            app.languages.iter().for_each(|lang| {
                if langs.contains(lang) {
                    apps.push(app);
                }
            })
        });

        apps
    }

    pub fn to_tags(&self) -> Vec<String> {
        let mut tag: HashSet<String> = HashSet::new();

        self.apps.clone().iter().for_each(|app| {
            app.tags.iter().for_each(|t| {
                tag.insert(t.to_string());
            });
        });

        tag.into_iter().collect()
    }

    pub fn to_languages(&self) -> Vec<String> {
        let mut langs: HashSet<String> = HashSet::new();

        self.apps.clone().iter().for_each(|app| {
            app.languages.iter().for_each(|l| {
                langs.insert(l.to_string());
            });
        });

        langs.into_iter().collect()
    }
}
