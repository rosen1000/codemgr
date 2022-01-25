use codemgr::{read_manifest, Manifest};
use term_inquiry::List;

pub mod lib;

fn main() {
    let value = List::new("pizza?".to_string())
        .add_item("option1", "yes")
        .add_item("option2", "no")
        .inquire();
    match value {
        Ok(value) => match value {
            _ => {}
        },
        _ => {}
    }

    let value = "[[app]]\nversion = 1".parse::<toml::Value>().unwrap();
    let arr = value.as_table().unwrap()["app"].as_array().unwrap();
    println!("array:\n{:?}\n", arr);

    let mut config: Manifest = read_manifest().unwrap();
    config.version = 3;
    // config.app.push("asd".to_string());
    let manifest = toml::to_string(&config).expect("cringe");
    println!("manifest:\n{}\n", manifest);

}
