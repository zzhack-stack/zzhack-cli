use crate::driver::common::apply_config;

pub fn apply_projects_config(projects_source: &String) {
    apply_config(
        projects_source,
        ".zzhack/services/src/projects_service/projects.json",
    );
}
