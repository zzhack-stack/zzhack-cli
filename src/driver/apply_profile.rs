use std::fs;

use serde::Deserialize;

use super::common::{with_static_resource, APPLY_ZZHACK_FOLDER_ERROR_MESSAGE};

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ProfileConfig {
    pub name: String,
    pub avatar: String,
}

impl ProfileConfig {
    pub fn default() -> ProfileConfig {
        ProfileConfig {
            name: String::from("zzhack"),
            avatar: String::from("/images/zzhack_favicon.svg"),
        }
    }
}

pub fn apply_profile(config: Option<ProfileConfig>, static_resource_path: &Option<String>) {
    let config = match config {
        Some(config) => ProfileConfig {
            avatar: with_static_resource(static_resource_path, &config.avatar),
            ..config
        },
        None => ProfileConfig::default(),
    };

    let template = format!(
        "
    pub const AUTHOR_NAME: &'static str = \"{}\";
    pub const AUTHOR_AVATAR: &'static str = \"{}\";
    ",
        config.name, config.avatar
    );

    fs::write(".zzhack/ui/src/post_card/author.rs", template)
        .expect(APPLY_ZZHACK_FOLDER_ERROR_MESSAGE);
}
