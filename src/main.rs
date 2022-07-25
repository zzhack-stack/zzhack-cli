use clap::{Parser, Subcommand};
use config::apply_config;
use std::fs::create_dir;
use std::io::ErrorKind;
use std::process::Command;
use utils::exec::exec_sync_with_spinner;

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
}

const TEMPLATE_DIR: &'static str = ".zzhack";
const TEMPLATE_REMOTE_ADDR: &'static str = "https://github.com/zzhack-stack/zzhack";

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
