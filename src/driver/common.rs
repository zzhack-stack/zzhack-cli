use fs_extra::dir::{copy, remove, CopyOptions};
use std::{fs, path::Path};

pub const APPLY_ZZHACK_FOLDER_ERROR_MESSAGE: &'static str =
    "Please make sure the .zzhack folder does exist";

pub fn apply_config(config_path: &String, target_config_path: &'static str) {
    let config = Path::new(&config_path);
    let config_content =
        fs::read_to_string(config).expect("Please make sure your source path is exist");

    fs::write(target_config_path, config_content).expect(APPLY_ZZHACK_FOLDER_ERROR_MESSAGE);
}

pub fn rewrite_dir(from: &str, to: &str) {
    let cp_err_msg = format!("No directory such as {}", from);
    let mut options = CopyOptions::default();

    options.copy_inside = true;
    options.content_only = true;

    remove(to).unwrap();
    copy(
        fs::canonicalize(from).unwrap(),
        ".zzhack/app/assets/sources/",
        &options,
    )
    .expect(cp_err_msg.as_str());
}
