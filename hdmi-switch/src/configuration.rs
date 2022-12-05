use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub server: ServerConfiguration,
    pub input: InputConfiguration,
    pub output: OutputConfiguration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfiguration {
    pub host: String,
    pub port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputConfiguration {
    pub aliases: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputConfiguration {
    pub aliases: HashMap<String, String>,
}

impl Configuration {
    pub fn get_port(&self) -> u16 {
        let port = match self.server.port {
            Some(port) => port,
            _ => 23,
        };

        return port;
    }
}

pub fn get_configuration(file_path: String) -> Result<Configuration, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path.as_str())?;
    let configuration: Configuration = serde_yaml::from_str(contents.as_str())?;

    return Ok(configuration);
}
