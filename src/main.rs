use util::{init_manifest, read_manifest};
use inquire::Select;

use crate::util::{print, LoggingLevel};

pub mod util;
pub mod logger;

fn main() {
    let manifest = read_manifest();
    if manifest.is_none() {
        init_manifest();
        return;
    }

    let manifest = manifest.unwrap();

    let apps = manifest
        .app
        .unwrap()
        .iter()
        .map(|app| app.clone().name)
        .collect();
    let query = Select::new("Select project", apps).prompt();

    if let Err(e) = &query {
        print(e, LoggingLevel::ERROR);
        return;
    }

    print(query.unwrap(), LoggingLevel::INFO);
}
