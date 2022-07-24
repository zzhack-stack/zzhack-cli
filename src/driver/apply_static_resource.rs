use super::common::rewrite_dir;

const RESOURCE_DIR: &'static str = ".zzhack/app/assets/sources/";

pub fn apply_static_resource(resource_dir: String) {
    rewrite_dir(&resource_dir, RESOURCE_DIR);
}
