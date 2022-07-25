use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct AppInfo {
    pub name: String,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub logo: Option<String>,
}

impl AppInfo {
    pub fn default() -> AppInfo {
        AppInfo {
            name: String::from("zzhack"),
            description: None,
            keywords: None,
            logo: Some(String::from("zzhack")),
        }
    }
}

pub fn apply_app(app_info_config: Option<AppInfo>) -> String {
    let default_app_info = AppInfo::default();
    let app_info = app_info_config.unwrap_or(default_app_info.clone());
    let logo = app_info.logo.unwrap_or(default_app_info.logo.unwrap());

    format!(
        "<meta name=\"description\" content=\"{}\">
    <meta name=\"keywords\" content=\"{}\">
    <link rel=\"Shortcut Icon\" href=\"{}\">
    <link rel=\"bookmark\" href=\"{}\" type=\"image/x-icon\" />
    <title>{}</title>
    ",
        app_info.description.unwrap_or_default(),
        app_info.keywords.unwrap_or_default(),
        logo,
        logo,
        app_info.name,
    )
}
