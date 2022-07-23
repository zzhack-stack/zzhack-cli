use std::{fs, path::Path};

pub fn apply_config(config_path: &String, target_config_path: &'static str) {
    let config = Path::new(&config_path);
    let config_content =
        fs::read_to_string(config).expect("Please make sure your source path is exist");

    fs::write(target_config_path, config_content)
        .expect("Please make sure the .zzhack folder does exist");
}
