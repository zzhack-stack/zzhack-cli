use std::fs::{self, read_dir};

use fs_extra::dir::remove;

use super::common::rewrite_dir;
use std::path::Path;

const RESOURCE_DIR: &'static str = ".zzhack/app/assets/sources/";

pub fn apply_static_resource(resource_dir: String) {
    rewrite_dir(&resource_dir, RESOURCE_DIR);
}

pub fn reset_static_resource() {
    if Path::new(RESOURCE_DIR).exists() {
        remove(RESOURCE_DIR).unwrap();
    } else {
        fs::create_dir(RESOURCE_DIR).unwrap();
    }
}
