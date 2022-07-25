use std::{fs, time::UNIX_EPOCH};

use convert_case::{Case, Casing};
use serde::Deserialize;

use crate::template::{Template, TEMPLATES};

use super::apply_links::apply_links_config;
use super::apply_projects::apply_projects_config;
use super::common::{rewrite_with_template, with_static_resource};

const BUILD_IN_ROUTE: [&'static str; 3] = ["post", "not_found", "root"];
const FIRST_PAGE_ROUTE_ENUM_NAME: &'static str = "Home";
const POST_SOURCE_SUFFIX: &'static str = "PostsSource";

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct PageConfig {
    pub name: String,
    pub route: Option<String>,
    pub template: String,
    pub source: String,
    // Posts optional config
    pub banner: Option<String>,
    // projects & links optional config
    pub banner_link: Option<String>,
    pub banner_text: Option<String>,
}

pub fn parse_page_route(page: &PageConfig) -> String {
    match &page.route {
        Some(route) => route,
        None => &page.name,
    }
    .clone()
}

pub fn get_route_enum_name(idx: usize, page: &PageConfig) -> String {
    if idx == 0 {
        FIRST_PAGE_ROUTE_ENUM_NAME.to_string()
    } else {
        parse_page_route(page).to_case(Case::Pascal)
    }
}

pub fn verify_pages_name_and_route(pages: &Vec<PageConfig>) {
    let mut route_vec: Vec<String> = vec![];

    for page in pages {
        let route = parse_page_route(page);

        if BUILD_IN_ROUTE.contains(&route.as_str()) {
            panic!("The {} has already declare", route);
        }

        if route_vec.contains(&route) {
            panic!("Cannot redeclare {}", route);
        }

        route_vec.push(route)
    }
}

pub fn verify_template(template: &str) {
    let is_legal_template = TEMPLATES.contains(&template);

    if !is_legal_template {
        panic!(
            "Please make sure the template({}) is one of [{}]",
            template,
            TEMPLATES.join(", ")
        );
    }
}

pub fn apply_router(pages: &Vec<PageConfig>) {
    let routes = pages
        .iter()
        .enumerate()
        .map(|(idx, page)| {
            verify_template(&page.template);

            let route = parse_page_route(&page);
            let route_enum_name = get_route_enum_name(idx, page);

            format!("#[at(\"/{}\")]\n{}", route, route_enum_name)
        })
        .collect::<Vec<String>>();
    let routes_config = format!(
        r#"
    use yew_router::prelude::*;

    #[derive(Clone, Routable, PartialEq, Debug)]
    pub enum RootRoutes {{
        {},
        #[at("/")]
        Root,
        #[at("/posts/:filename")]
        Post {{ filename: String }},
        #[not_found]
        #[at("/404")]
        NotFound,
    }} 
"#,
        routes.join(",\n")
    );

    // Write back to routes
    fs::write(".zzhack/router/src/lib.rs", routes_config)
        .expect("Please make sure the .zzhack folder does exist.");
}

pub fn apply_routes_switch(pages: &Vec<PageConfig>, static_resource_path: &Option<String>) {
    let mut post_file_template: Vec<String> = vec![];
    let switch_cases = pages
        .iter()
        .enumerate()
        .map(|(idx, page)| {
            let template = page.template.as_str();
            let template: Template = template.into();
            let route_enum_name = get_route_enum_name(idx, page);
            let posts_source_key_name = format!("{}{}", route_enum_name, POST_SOURCE_SUFFIX);
            let properties = match template {
                Template::Post => format!("filename=\"{}\"", page.source),
                Template::Posts => format!("posts_key=\"{}\"", posts_source_key_name),
                _ => String::from(""),
            };

            match template {
                Template::Links => {
                    let banner_text = page
                        .banner_text
                        .clone()
                        .expect("Missing the banner_text field in links page");

                    rewrite_with_template(
                        format!(
                            "
                            pub const LINKS_BANNER_TEXT: &'static str = \"{}\";
                    ",
                            banner_text
                        ),
                        "pages/links/src/links_config.rs",
                    );

                    apply_links_config(&page.source)
                }
                Template::Projects => {
                    let banner_link = page.banner_link.clone().unwrap_or(String::from("#"));
                    let banner_text = page.banner_text.clone().unwrap_or(String::from(""));

                    rewrite_with_template(
                        format!(
                            "
                        pub const PROJECTS_BANNER_TEXT: &'static str = \"{}\";
                        pub const PROJECTS_BANNER_LINK: &'static str = \"{}\";
                    ",
                            banner_text, banner_link
                        ),
                        "pages/projects/src/projects_config.rs",
                    );

                    apply_projects_config(&page.source);
                }
                Template::Posts => {
                    let banner = page.banner.clone().unwrap_or(String::from(""));
                    let banner_source = with_static_resource(static_resource_path, &banner);
                    // let banner Text
                    rewrite_with_template(
                        format!(
                            "
                                pub const BANNER_LINK: &'static str = \"{}\";
                            ",
                            banner_source
                        ),
                        "pages/home/src/posts_config.rs",
                    );

                    // Construct routes
                    let dir = fs::read_dir(&page.source).expect(
                        format!("Please make sure the {} does exits.", &page.source).as_str(),
                    );
                    let posts_source = dir
                        .map(|entry| {
                            let entry = entry.unwrap();
                            let path = entry.path();
                            let stemname = &path.file_stem().unwrap().to_str().unwrap();
                            let metadata = entry.metadata().unwrap();
                            let modified_time = metadata
                                .modified()
                                .unwrap()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis();
                            let filepath = fs::canonicalize(&path).unwrap();

                            format!(
                                "
                                PostFile {{
                                    content: include_str!(\"{}\"),
                                    modified_time: {},
                                    filename: \"{}\"
                                }}",
                                filepath.to_str().unwrap(),
                                modified_time,
                                stemname
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(",\n");

                    post_file_template.push(format!(
                        "
                        (String::from(\"{}\"), vec![{}])
                    ",
                        posts_source_key_name, posts_source
                    ));
                }
                _ => (),
            };

            format!(
                "RootRoutes::{} => html! {{ <{} {} />}}",
                route_enum_name,
                template.into_component_name(),
                properties
            )
        })
        .collect::<Vec<String>>();

    let posts_source_template = format!(
        "
        use std::collections::HashMap;

        #[derive(Clone)]
        pub struct PostFile {{
            pub content: &'static str,
            pub modified_time: u128,
            pub filename: &'static str,
        }}

        pub fn get_posts() -> HashMap<String, Vec<PostFile>> {{
            HashMap::from([{}])
        }}
        ",
        post_file_template.join(",\n")
    );

    let switch_fn_template = format!(
        r#"
    use router::RootRoutes;
    use yew::prelude::*;
    use yew_router::prelude::*;

    use home::Home;
    use links::Links;
    use projects::Projects;
    use post::Post;
    use not_found::NotFound;

    pub fn switch(routes: &RootRoutes) -> Html {{
        match routes {{
            {},
            RootRoutes::Root => html! {{<Redirect<RootRoutes> to={{RootRoutes::Home}}/>}},
            RootRoutes::Post {{ filename }} => html! {{<Post filename={{filename.clone()}} />}},
            RootRoutes::NotFound => html! {{ <NotFound />}},
        }}
    }}
    "#,
        switch_cases.join(",\n")
    );

    fs::write(".zzhack/app/src/routes_switch.rs", switch_fn_template)
        .expect("Please make sure the .zzhack folder does exist.");
    fs::write(".zzhack/services/src/posts.rs", posts_source_template).unwrap();
}

pub fn apply_navigator(pages: &Vec<PageConfig>) {
    let navigator_pages = pages
        .iter()
        .enumerate()
        .map(|(idx, page)| {
            format!(
                "Page {{
            route: RootRoutes::{},
            name: \"{}\"
        }}",
                get_route_enum_name(idx, page),
                page.name
            )
        })
        .collect::<Vec<String>>();

    let navigator_pages = format!(
        "
    use router::RootRoutes;

    pub struct Page {{
        pub route: RootRoutes,
        pub name: &'static str,
    }}
    
    pub const PAGES: [Page; {}] = [
        {}
    ]; 
    ",
        pages.len(),
        navigator_pages.join(",\n"),
    );

    println!("{}", navigator_pages);

    fs::write(".zzhack/ui/src/common/header/pages.rs", navigator_pages)
        .expect("Please make sure the .zzhack folder does exist.");
}

pub fn apply_pages_config(pages: Vec<PageConfig>, static_resource_path: &Option<String>) {
    verify_pages_name_and_route(&pages);
    apply_router(&pages);
    apply_routes_switch(&pages, static_resource_path);
    apply_navigator(&pages);
}
