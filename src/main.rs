use clap::{Parser, Subcommand};
use driver::apply_github_bar::{apply_github_bar, reset_github_bar};
use driver::apply_router::{apply_pages_config, PageConfig};
use driver::apply_static_resource::{apply_static_resource, reset_static_resource};
use serde::Deserialize;
use std::fs::{self, create_dir};
use std::io::ErrorKind;
use std::process::Command;
use utils::exec::exec_sync_with_spinner;

mod driver;
mod template;
mod utils;

#[derive(Parser, Debug)]
struct CLI {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Init,
    Serve,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ContactConfig {
    pub kind: String,
    pub icon: Option<String>,
    pub link: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ZZHACKConfig {
    pub app_name: String,
    pub logo: Option<String>,
    pub pages: Vec<PageConfig>,
    pub resource_dir: Option<String>,
    pub contacts: Option<Vec<ContactConfig>>,
    pub github_bar: Option<String>,
}

const TEMPLATE_DIR: &'static str = ".zzhack";
const TEMPLATE_REMOTE_ADDR: &'static str = "https://github.com/zzhack-stack/zzhack";
const CONFIG_NAME: &'static str = "zzhack.config.json";

pub fn apply_config() {
    let config = fs::read_to_string(CONFIG_NAME)
        .expect("Cannot find zzhack.config.json, use zzhack init to create a template config.");
    let config: ZZHACKConfig = serde_json::from_str(&config)
        .expect("Cannot parse the config of zzhack, please make sure you are suit the bounds of zzhack config");

    apply_pages_config(config.pages);

    match config.resource_dir {
        Some(resource_dir_path) => apply_static_resource(resource_dir_path),
        None => reset_static_resource(),
    };

    match config.github_bar {
        Some(github_bar_config) => apply_github_bar(github_bar_config),
        None => reset_github_bar(),
    }
}

pub fn main() {
    let args = CLI::parse();

    match args.action {
        Action::Init => {
            exec_sync_with_spinner("Create template from network", || {
                match create_dir(".zzhack") {
                    Err(err) => match err.kind() {
                        ErrorKind::AlreadyExists => (),
                        _ => panic!("{}", err),
                    },
                    _ => (),
                }
                Command::new("git")
                    .arg("clone")
                    .arg("-b")
                    .arg("feature/cli")
                    .arg(TEMPLATE_REMOTE_ADDR)
                    .arg(TEMPLATE_DIR)
                    .output()
                    .unwrap();
            });
            exec_sync_with_spinner("Generate zzhack init template config", move || {
                // Command::new("git")
                //     .arg("clone")
                //     .arg("-b")
                //     .arg("feature/cli")
                //     .arg(TEMPLATE_REMOTE_ADDR)
                //     .arg(TEMPLATE_DIR)
                //     .output()
                //     .unwrap();
            });
            // ::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
            //     .unwrap()
            //     .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
            // create_dir(TEMPLATE_DIR).unwrap();
        }
        Action::Serve => {
            exec_sync_with_spinner("Apply config", || {
                apply_config();
            });

            Command::new("trunk")
                .arg("serve")
                .current_dir(".zzhack/app")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    }
}
