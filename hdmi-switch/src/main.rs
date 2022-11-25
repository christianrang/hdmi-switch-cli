use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use structopt::StructOpt;
use telnet::Telnet;
use std::error::Error;

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
    let input = match input.as_str() {
        "pc" => "hdmiin1",
        "ps" => "hdmiin2",
        "switch" => "hdmiin3",
        "work" => "hdmiin4",
        "hdmiin1" => "hdmiin1",
        "hdmiin2" => "hdmiin2",
        "hdmiin3" => "hdmiin3",
        "hdmiin4" => "hdmiin4",
        v => {
            return Err(format!("Input {} not supported", v));
        }
    };
    let output = match output.as_str() {
        "pc" => "hdmiout1",
        "tv" => "hdmiout2",
        "hdmiout1" => "hdmiout1",
        "hdmiout2" => "hdmiout2",
        "hdmiout3" => "hdmiout3",
        "hdmiout4" => "hdmiout4",
        "all" => "all",
        v => {
            return Err(format!("Output {} not supported", v));
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
