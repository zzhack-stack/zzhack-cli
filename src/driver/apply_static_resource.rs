use std::path::Path;

pub fn apply_static_resource(resource_dir: String) -> String {
    let dir_path = Path::new("../..").join(&resource_dir);

    if !Path::new(&resource_dir).exists() {
        panic!("Please make sure the resource_dir does exist");
    }

    format!(
        "<link data-trunk rel=\"copy-dir\" href=\"{}\">",
        dir_path.to_str().unwrap()
    )
}
