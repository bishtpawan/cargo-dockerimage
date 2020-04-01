use cargo_toml::{Manifest, Value};
use docker::DOCKER_FILE_PATH;
use docker::{build_docker_image, create_docker_file};
use log::{error, info};
use std::env::args;
use std::fs;

pub static TOML_PATH: &str = "./Cargo.toml";
pub static DOCKER_COMMAND: &str = "docker";
pub static TOML_FILE_ERROR: &str = "Could Not Find `Cargo.toml`";
pub static TOML_CONTENT_ERROR: &str = "Error while retrieving values from Toml's content";
pub static COMMAND_LINE_ERROR: &str =
    "Please provide docker file's name as a command line argument";
pub static RUST_LOG: &str = "RUST_LOG";
pub static LOG_LEVEL: &str = "cargo_dockerimage=info";

// This project is in the form of Cargo plugin which is used to create and build docker image.
#[macro_use]
pub mod docker;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    std::env::set_var(RUST_LOG, LOG_LEVEL);
    env_logger::init();
    let docker_image_name: String = args().nth(2).expect(COMMAND_LINE_ERROR);
    match fs::read(TOML_PATH) {
        Ok(toml_content) => {
            let toml_values: Manifest<Value> =
                Manifest::from_slice(&toml_content).expect(TOML_CONTENT_ERROR);
            if fs::File::open(DOCKER_FILE_PATH).is_err() {
                match create_docker_file(&toml_values.package.unwrap().name) {
                    Ok(success) => info!("{}", success),
                    Err(error) => {
                        error!("{}", error);
                        return;
                    }
                }
            }
            match build_docker_image(&docker_image_name, DOCKER_COMMAND) {
                Ok(success) => info!("{}", success),
                Err(error) => {
                    error!("{}", &error);
                    return;
                }
            }
        }
        Err(_error) => error!("{}", TOML_FILE_ERROR),
    }
}
