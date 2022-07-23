pub const TEMPLATES: [&'static str; 4] = ["posts", "post", "projects", "links"];

pub enum Template {
    Posts,
    Post,
    Projects,
    Links,
}

impl From<&str> for Template {
    fn from(template: &str) -> Self {
        match template {
            "post" => Template::Post,
            "posts" => Template::Posts,
            "links" => Template::Links,
            "projects" => Template::Projects,
            _ => panic!(
                "Please make sure the template({}) is one of [{}]",
                template,
                TEMPLATES.join(", ")
            ),
        }
    }
}

impl Template {
    pub fn into_component_name(&self) -> &'static str {
        match &self {
            Template::Links => "Links",
            Template::Post => "Post",
            Template::Posts => "Home",
            Template::Projects => "Projects",
        }
    }
}
