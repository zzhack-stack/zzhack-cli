use crate::driver::common::apply_config;

pub fn apply_links_config(links_source: &String) {
    apply_config(
        links_source,
        ".zzhack/services/src/links_service/links.json",
    );
}
