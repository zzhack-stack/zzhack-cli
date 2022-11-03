use clap::{Parser, Subcommand};
use config::apply_config;
use fs_extra::dir::remove;
use std::fs::{self, create_dir};
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;
use utils::exec::exec_sync_with_spinner;

use crate::driver::common::copy_inside;

mod config;
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
    Build,
}

const TEMPLATE_DIR: &'static str = ".zzhack";
const TEMPLATE_REMOTE_ADDR: &'static str = "https://github.com/zzhack-stack/zzhack";
const CLI_CONFIG_TEMPLATE_REMOTE_ADDR: &'static str =
    "https://github.com/zzhack-stack/zzhack-init-template-zh";

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

            // TODO: disgusting language

            if !Path::new("./zzhack.config.json").exists() {
                exec_sync_with_spinner("Sync zzhack zh template", || {
                    Command::new("git")
                        .arg("clone")
                        .arg(CLI_CONFIG_TEMPLATE_REMOTE_ADDR)
                        .arg(".template")
                        .output()
                        .unwrap();
                });

                copy_inside(".template", ".");

                if Path::new("./.template").exists() {
                    remove(".template").unwrap();
                }
            }
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
        Action::Build => {
            Command::new("trunk")
                .arg("build")
                .arg("-d")
                .arg("../../dist")
                .current_dir(".zzhack/app")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    }
}
