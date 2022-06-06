use codemgr::{init_manifest, read_manifest, META_DATA_VERSION};
// use question::Question;
use term_inquiry::List;

pub mod lib;

fn main() {
    let manifest = read_manifest();
    if manifest.is_none() {
        init_manifest();
        return;
    }

    let manifest = manifest.unwrap();

    if manifest.meta.as_ref().unwrap().version != META_DATA_VERSION {
        println!("This folder is different version from this program!");
        return;
    }

    let answer = List::new("Operation".to_string())
        .add_item("Open project", 1)
        .add_item("New project", 2)
        .add_item("Delete project", 3)
        .inquire();

    match answer {
        Ok(select) => match select {
            1 => open_project(),
            2 => new_project(),
            3 => delete_project(),
            _ => println!("lmao cringe"),
        },
        Err(_) => println!("lmao cringe"),
    }

    println!("{:?}", manifest);

    for app in manifest.app.unwrap() {
        println!("{}: {:?}", app.name, app.languages);
    }
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

fn open_project() {
    println!("open");
}

fn new_project() {
    println!("new");
}

fn delete_project() {
    println!("delete");
}
