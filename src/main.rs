use codemgr::{init_manifest, read_manifest};
// use term_inquiry::List;

pub mod lib;

fn main() {
    let manifest = read_manifest();
    if manifest.is_none() {
        init_manifest();
        return;
    }

    let manifest = manifest.unwrap();

    println!("{:?}", manifest);
    return;

    // let value = List::new("pizza?".to_string())
    //     .add_item("option1", "yes")
    //     .add_item("option2", "no")
    //     .inquire();
    // match value {
    //     Ok(value) => match value {
    //         _ => {}
    //     },
    //     _ => {}
    // }

    // let value = toml! {
    //     [meta]
    //     version = 1

    //     [[app]]
    //     languages = ["rust"]
    //     name = "codemgr"
    //     tags = ["cli-tool"]
    // };
    // let arr = value.as_table().unwrap()["app"].as_array().unwrap();
    // println!("array:\n{:?}\n", arr);

    // let config = read_manifest();
    // if config.is_none() {
    //     // TODO: handle
    // }
    // let mut config = config.unwrap();
    // config.version = Option::Some(3);
    // config.save();
    // // config.app.push("asd".to_string());
    // let manifest = toml::to_string(&config).expect("cringe");
    // println!("manifest:\n{}\n", manifest);
}
