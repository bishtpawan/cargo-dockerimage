use log::info;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

pub static DOCKER_FILE_PATH: &str = "./Dockerfile";
pub static DOCKER_FILE_LOCATION: &str = ".";
pub static BASE_IMAGE: &str = "ubuntu";
pub static UPDATE: &str = "apt-get update";
pub static UPGRADE: &str = "apt-get -y upgrade";
pub static OPENSSL: &str = "apt-get install -y openssl";
pub static BINARY_PATH: &str = "./target/debug/";
pub static CONTAINER_PATH: &str = "./";
pub static DOCKER_BUILD: &str = "build";
pub static DOCKER_TAG: &str = "-t";
pub static SUCCESS_FILE: &str = "Docker File Successfully Created";
pub static FAILURE_FILE: &str = "Unable To Create Docker File";
pub static SUCCESS_IMAGE: &str = "Docker Image Successfully Build";
pub static FAILURE_IMAGE: &str = "Unable To Build Docker Image";
pub static WRITER_ERROR: &str = "Unable to write to docker file";

/// Create docker file for Rust Application and store it into local machine inside the project.
///
/// # Arguments
///
/// * `binary_name` - Executable name of the application
///
/// # Return
///
/// This function returns the response in Result enum of &str and &str(Success or failure message)
///
pub fn create_docker_file(binary_name: &str) -> Result<&str, &str> {
    match OpenOptions::new()
        .create(true)
        .append(true)
        .open(DOCKER_FILE_PATH)
    {
        Ok(docker_file) => {
            writeln!(&docker_file, "FROM {}", BASE_IMAGE).expect(WRITER_ERROR);
            writeln!(&docker_file, "RUN {}", UPDATE).expect(WRITER_ERROR);
            writeln!(&docker_file, "RUN {}", UPGRADE).expect(WRITER_ERROR);
            writeln!(&docker_file, "RUN {}", OPENSSL).expect(WRITER_ERROR);
            writeln!(
                &docker_file,
                "COPY  {}{} {}",
                BINARY_PATH, binary_name, CONTAINER_PATH
            )
            .expect(WRITER_ERROR);
            writeln!(&docker_file, "CMD {}{}", CONTAINER_PATH, binary_name).expect(WRITER_ERROR);
            Ok(SUCCESS_FILE)
        }
        Err(_file_error) => Err(FAILURE_FILE),
    }
}

/// Build docker image for Rust Application.
///
/// # Arguments
///
/// * `docker_image_name` - Name of the docker image
/// * `docker_file_location` - Location of the docker file
///
/// # Return
///
/// This function returns the response in Result enum of &str(Status of building docker image)
///
pub fn build_docker_image(docker_image_name: &str, docker_command: &str) -> Result<String, String> {
    info!("Processing...");
    let mut command = Command::new(docker_command);
    command.args(&[
        DOCKER_BUILD,
        DOCKER_TAG,
        docker_image_name,
        DOCKER_FILE_LOCATION,
    ]);
    match &command.output() {
        Ok(output) if output.status.success() => Ok(SUCCESS_IMAGE.into()),
        Ok(output) => Err(format!(
            "{}\nCommand: {:?}\n{:?}",
            FAILURE_IMAGE, command, output
        )),
        Err(command_error) => Err(format!(
            "{}\nCommand: {:?}\n{:?}",
            FAILURE_IMAGE, command, command_error
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::docker::{build_docker_image, create_docker_file};
    use cargo_toml::{Manifest, Value};
    use std::fs;

    #[test]
    fn test_create_docker_file() {
        let toml_content = fs::read("Cargo.toml").unwrap();
        let toml_values: Manifest<Value> = Manifest::from_slice(&toml_content)
            .expect("Error while retrieving values from Toml's content");
        assert_eq!(
            create_docker_file(&toml_values.package.unwrap().name).unwrap(),
            "Docker File Successfully Created"
        );
    }

    #[test]
    fn test_build_docker_image_success() {
        assert_eq!(
            build_docker_image("test", "docker"),
            Ok(String::from("Docker Image Successfully Build"))
        );
    }

    #[test]
    fn test_build_docker_image_fail() {
        assert_eq!(
            build_docker_image("test", "./"),
            Err(String::from("Unable To Build Docker Image\nCommand: \"./\" \"build\" \"-t\" \"test\" \".\"\nOs { code: 13, kind: PermissionDenied, message: \"Permission denied\" }"))
        );
    }
}
