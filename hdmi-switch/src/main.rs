use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process;
use structopt::StructOpt;
use telnet::Telnet;

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
    fn get_file_path(&self) -> Result<String, String> {
        let mut configuration: String = self.configuration.clone();
        if configuration == "" {
            let home = match env::var("HOME") {
                Ok(home) => home,
                Err(error) => return Err(error.to_string()),
            };
            configuration = String::from(format!("{}/.config/hdmi-switch/configuration.yaml", home))
        }

        return Ok(configuration);
    }
}

#[derive(Serialize, Deserialize)]
struct Configuration {
    host: String,
}

fn main() {
    let opt = Opt::from_args();

    let configuration_file_path: String = opt
        .get_file_path()
        .expect("unable to find configuration file");
    let configuration = match get_configuration(configuration_file_path) {
        Ok(configuration) => configuration,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let mut telnet =
        Telnet::connect((configuration.host, 23), 256).expect("Couldn't connect to the server...");

    let _event = telnet
        .read()
        .expect("Error reading connection response from HDMI switch");

    let buffer: String = match command_build(opt.input, opt.output) {
        Ok(command) => command,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
    telnet
        .write(&buffer.as_bytes())
        .expect("Error sending command to HDMI switch");
}

fn command_build(input: String, output: String) -> Result<String, String> {
    let input = match input.as_str() {
        "pc" => "hdmiin1",
        "ps" => "hdmiin2",
        "switch" => "hdmiin3",
        "work" => "hdmiin4",
        v => {
            return Err(String::from(format!("Input {} not supported", v)));
        }
    };
    let output = match output.as_str() {
        "pc" => "hdmiout1",
        "tv" => "hdmiout2",
        "all" => "all",
        v => {
            return Err(String::from(format!("Output {} not supported", v)));
        }
    };

    let command: String = String::from(format!("SET SW {} {}\n\r", input, output));
    return Ok(command);
}

fn get_configuration(file_path: String) -> Result<Configuration, String> {
    let contents = match fs::read_to_string(file_path.as_str()) {
        Ok(contents) => contents,
        Err(error) => return Err(format!("{}: {}", error, file_path)),
    };

    let configuration: Configuration = match serde_yaml::from_str(contents.as_str()) {
        Ok(configuration) => configuration,
        Err(error) => return Err(error.to_string()),
    };

    return Ok(configuration);
}
