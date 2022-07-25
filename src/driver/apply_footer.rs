use serde::Deserialize;

use super::common::{rewrite_with_template, with_static_resource};

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct FooterConfig {
    pub copyright: String,
    pub contacts: Vec<Contact>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Contact {
    pub icon: String,
    pub link: String,
    pub icon_size: Option<i32>,
}

impl FooterConfig {
    pub fn default() -> FooterConfig {
        FooterConfig {
            copyright: String::from(""),
            contacts: vec![],
        }
    }
}

pub fn apply_footer(config: Option<FooterConfig>, static_resource_path: &Option<String>) {
    let config = config.unwrap_or(FooterConfig::default());
    let contacts_template = config
        .contacts
        .iter()
        .map(|contact| {
            format!(
                "Contact {{
        link: \"{}\",
        icon: \"{}\",
        icon_size: {},
    }}",
                contact.link,
                with_static_resource(static_resource_path, &contact.icon),
                contact.icon_size.unwrap_or(30)
            )
        })
        .collect::<Vec<String>>();

    rewrite_with_template(
        format!(
            "
        #[derive(PartialEq, Clone)]
        pub struct Contact {{
            pub link: &'static str,
            pub icon: &'static str,
            pub icon_size: i32,
        }}

        pub const CONTACTS: [Contact; {}] = [{}];
        pub const FOOTER_TEXT: &'static str = \"{}\";
    ",
            contacts_template.len(),
            contacts_template.join(",\n"),
            config.copyright
        ),
        "ui/src/common/footer_source.rs",
    );
}
