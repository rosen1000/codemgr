extern crate serde;

use ansi_term::{Color, Color::*};
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::ops::Add;

const MANIFEST_PATH: &str = "./manifest.toml";

fn warn_ansi() -> String {
    ansi("!", Blue, Yellow)
}

// fn ask_ansi() -> String {
//     ansi("?", Blue, Cyan)
// }

// fn error_ansi() -> String {
//     ansi("!!", Blue, Red)
// }

fn ansi(sym: &str, bracket_color: Color, sym_color: Color) -> String {
    bracket_color
        .bold()
        .paint("[")
        .to_owned()
        .to_string()
        .add(&sym_color.paint(sym).to_owned().to_string())
        .add(&bracket_color.bold().paint("]").to_owned().to_string())
}

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
            println!("{} manifest.toml not found!", warn_ansi());
            println!("{} probably you are not in initialized folder", warn_ansi());
            return None;
        }
    }

    Some(toml::from_str(&manifest_text).unwrap())
}

pub fn init_manifest() {
    let file = File::create(MANIFEST_PATH);
    match file {
        Ok(mut file) => {
            file.write(r#"version = 1"#.as_bytes()).unwrap();
        },
        Err(_) => {}
    }
}

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    pub version: Option<u8>,
    pub app: Option<Vec<App>>,
}

#[derive(Deserialize, Serialize)]
pub struct App {
    pub version: Option<u8>,
}

impl Manifest {
    pub fn save(&self) {
        let mut file = File::create(MANIFEST_PATH).unwrap();
        file.write(toml::to_string(&self).unwrap().as_bytes()).unwrap();
    }
}
