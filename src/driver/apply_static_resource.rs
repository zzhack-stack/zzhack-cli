use std::path::Path;

use fs_extra::dir::{copy, CopyOptions};

use super::common::{rewrite_with_template, APPLY_ZZHACK_FOLDER_ERROR_MESSAGE};

fn apply_post_parser_resource_dir(resource_dir: &str) {
    let folder_name = Path::new(resource_dir)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    rewrite_with_template(
        format!(
            "pub const SOURCES_FOLDER_NAME: &'static str = \"{}\";
    ",
            folder_name
        ),
        "services/src/markdown_service/sources_config.rs",
    )
}

fn move_static_dir(resource_dir: &str) {
    let mut options = CopyOptions::default();

    options.overwrite = true;

    copy(resource_dir, ".zzhack/app/assets/", &options).expect(APPLY_ZZHACK_FOLDER_ERROR_MESSAGE);
}

pub fn apply_static_resource(resource_dir: String) -> String {
    apply_post_parser_resource_dir(&resource_dir);
    move_static_dir(&resource_dir);

    let dir_path = Path::new("../..").join(&resource_dir);

    if !Path::new(&resource_dir).exists() {
        panic!("Please make sure the resource_dir does exist");
    }

    format!(
        "<link data-trunk rel=\"copy-dir\" href=\"{}\">",
        dir_path.to_str().unwrap()
    )
}
