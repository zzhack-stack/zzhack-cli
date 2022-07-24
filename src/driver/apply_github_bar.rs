use std::fs;

use crate::template;

use super::common::APPLY_ZZHACK_FOLDER_ERROR_MESSAGE;

const HEADER_CONFIG_PATH: &'static str = ".zzhack/ui/src/common/header/header_config.rs";

fn rewrite_github_bar(template: &str) {
    fs::write(HEADER_CONFIG_PATH, template).expect(APPLY_ZZHACK_FOLDER_ERROR_MESSAGE);
}

pub fn apply_github_bar(github_link: String) {
    rewrite_github_bar(&format!(
        "
        pub const IS_GITHUB_BAR_VISIBLE: bool = true;
        pub const GITHUB_BAR_LINK: &'static str = \"{}\";
    ",
        github_link
    ));
}

pub fn reset_github_bar() {
    rewrite_github_bar(
        "
    pub const IS_GITHUB_BAR_VISIBLE: bool = false;
    pub const GITHUB_BAR_LINK: &'static str = \"\";
",
    );
}
