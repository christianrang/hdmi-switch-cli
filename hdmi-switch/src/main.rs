use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use structopt::StructOpt;
use telnet::Telnet;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, StructOpt)]
#[structopt(name = "hdmi-switch", about = "Cli client for 4KMX44-H2")]
struct Opt {
    #[structopt(short, long)]
    input: String,
    #[structopt(short, long)]
    output: String,
    #[structopt(short, long, default_value = "")]
    configuration: String,
}

impl Opt {
    fn get_file_path(&self) -> Result<String, Box<dyn Error>> {
        let mut configuration: String = self.configuration.clone();
        if configuration == "" {
            let home = env::var("HOME")?;
            configuration = format!("{}/.config/hdmi-switch/configuration.yaml", home)
        }

        return Ok(configuration);
    }
}

#[derive(Serialize, Deserialize)]
struct Configuration {
    host: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let configuration_file_path: String = opt
        .get_file_path()
        .expect("unable to find configuration file");

    let configuration = get_configuration(configuration_file_path)?;

    let mut telnet =
        Telnet::connect((configuration.host, 23), 256).expect("Couldn't connect to the server...");

    let _event = telnet
        .read()
        .expect("Error reading connection response from HDMI switch");

    let buffer: String = command_build(opt.input, opt.output)?;
    telnet
        .write(&buffer.as_bytes())
        .expect("Error sending command to HDMI switch");

    Ok(())
}

fn command_build(input: String, output: String) -> Result<String, String> {
    let mut configured_inputs: HashMap<String, String> = HashMap::new();
    let mut default_inputs: HashMap<String, String> = HashMap::new();

    // Set defaults
    default_inputs.insert("hdmiin1".to_string(), "hdmiin1".to_string());
    default_inputs.insert("hdmiin2".to_string(), "hdmiin2".to_string());
    default_inputs.insert("hdmiin3".to_string(), "hdmiin3".to_string());
    default_inputs.insert("hdmiin4".to_string(), "hdmiin4".to_string());

    // Set aliases
    configured_inputs.insert("pc".to_string(), "hdmiin1".to_string());
    configured_inputs.insert("ps".to_string(), "hdmiin2".to_string());
    configured_inputs.insert("switch".to_string(), "hdmiin3".to_string());
    configured_inputs.insert("work".to_string(), "hdmiin4".to_string());
    configured_inputs.extend(default_inputs);

    let input = match configured_inputs.get(&input){ 
        Some(value) => value,
        _ => {
            return Err(format!("Input {} not supported", input));
        }
    };

    let mut configured_outputs: HashMap<String, String> = HashMap::new();
    let mut default_outputs: HashMap<String, String> = HashMap::new();

    // Set defaults
    default_outputs.insert("hdmiout1".to_string(), "hdmiout1".to_string());
    default_outputs.insert("hdmiout2".to_string(), "hdmiout2".to_string());
    default_outputs.insert("hdmiout3".to_string(), "hdmiout3".to_string());
    default_outputs.insert("hdmiout4".to_string(), "hdmiout4".to_string());

    configured_outputs.insert("pc".to_string(), "hdmiout1".to_string());
    configured_outputs.insert("tv".to_string(), "hdmiout2".to_string());
    configured_outputs.extend(default_outputs);

    let output = match configured_outputs.get(&output){ 
        Some(value) => value,
        _ => {
            return Err(format!("Output {} not supported", output));
        }
    };

    // let command: String = String::from(format!("SET SW {} {}\n\r", input, output));
    let command: String = format!("SET SW {} {}\n\r", input, output);
    return Ok(command);
}

fn get_configuration(file_path: String) -> Result<Configuration, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path.as_str())?;
    let configuration: Configuration = serde_yaml::from_str(contents.as_str())?;

    return Ok(configuration);
}
