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

pub fn with_static_resource(static_resource_path: &Option<String>, source_name: &str) -> String {
    let static_resource_path = static_resource_path.clone().expect(
        "Please make sure you has already declare resource_dir field in your zzhack config",
    );
    let static_resource_path = Path::new(&static_resource_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    format!("/{}/{}", static_resource_path, source_name)
}

pub fn rewrite_with_template(template: String, rewrite_path: &'static str) {
    fs::write(format!(".zzhack/{}", rewrite_path), template)
        .expect(APPLY_ZZHACK_FOLDER_ERROR_MESSAGE)
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
