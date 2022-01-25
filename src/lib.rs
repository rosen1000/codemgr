extern crate serde;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use serde_derive::{Deserialize, Serialize};

// use toml::{Value, value::Array, macros::Deserialize};
// use serde_derive::Deserialize;

pub fn read_manifest() -> Result<Manifest, Error> {
    let file = File::open("./example.toml");
    let mut manifest_text = "".to_string();
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                manifest_text += &line.unwrap().to_string();
                manifest_text += &"\n".to_string();
            }
        }
        Err(_) => {
            println!("Error occured")
        }
    }

    let manifest: Manifest = toml::from_str(&manifest_text).unwrap();
    Ok(manifest)
}

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    pub version: u8,
    pub app: Vec<App>
}

#[derive(Deserialize, Serialize)]
pub struct App {
    pub version: u8,
}
