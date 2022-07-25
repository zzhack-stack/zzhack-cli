use std::fs;

use serde::Deserialize;

use crate::driver::apply_app::{apply_app, AppInfo};
use crate::driver::apply_footer::{apply_footer, FooterConfig};
use crate::driver::apply_github_bar::{apply_github_bar, reset_github_bar};
use crate::driver::apply_profile::{apply_profile, ProfileConfig};
use crate::driver::apply_router::{apply_pages_config, PageConfig};
use crate::driver::apply_static_resource::apply_static_resource;
use crate::driver::html::HtmlTemplate;

const CONFIG_NAME: &'static str = "zzhack.config.json";

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ContactConfig {
    pub kind: String,
    pub icon: Option<String>,
    pub link: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ZZHACKConfig {
    pub pages: Vec<PageConfig>,
    pub resource_dir: Option<String>,
    pub contacts: Option<Vec<ContactConfig>>,
    pub github_bar: Option<String>,
    pub profile: Option<ProfileConfig>,
    pub app: Option<AppInfo>,
    pub footer: Option<FooterConfig>,
}

fn optional<T, F>(config: Option<T>, cb: F) -> String
where
    F: Fn(T) -> String,
{
    match config {
        Some(value) => cb(value),
        None => String::from(""),
    }
}

pub fn apply_config() {
    let config = fs::read_to_string(CONFIG_NAME)
        .expect("Cannot find zzhack.config.json, use zzhack init to create a template config.");
    let config: ZZHACKConfig = serde_json::from_str(&config)
        .expect("Cannot parse the config of zzhack, please make sure you are suit the bounds of zzhack config");

    apply_pages_config(config.pages, &config.resource_dir);
    apply_profile(config.profile, &config.resource_dir);
    apply_footer(config.footer, &config.resource_dir);

    HtmlTemplate::from(optional(config.resource_dir, |resource_dir_path| {
        apply_static_resource(resource_dir_path)
    }))
    .append(apply_app(config.app))
    .write();

    match config.github_bar {
        Some(github_bar_config) => apply_github_bar(github_bar_config),
        None => reset_github_bar(),
    }
}
