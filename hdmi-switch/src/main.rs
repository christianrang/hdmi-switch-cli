mod configuration;
mod utils;

use std::env;
use std::error::Error;
use structopt::StructOpt;
use telnet::Telnet;

#[derive(Debug, StructOpt)]
#[structopt(name = "hdmi-switch", about = "Cli client for 4KMX44-H2")]
struct Opt {
    #[structopt(short, long, default_value = "")]
    configuration: String,

    #[structopt(subcommand)]
    cmd: Option<SubCommand>,
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

    fn execute(self, configuration: configuration::Configuration) -> Result<(), String> {
        let port = configuration.get_port();

        let mut telnet = Telnet::connect((configuration.server.host, port), 256)
            .expect("Couldn't connect to the server...");

        let _event = telnet
            .read()
            .expect("Error reading connection response from HDMI switch");

        let mut switch = utils::Switch::new();
        for (alias, default) in configuration.input.aliases.iter() {
            switch.load_input_alias(&alias, &default)?;
        }
        for (alias, default) in configuration.output.aliases.iter() {
            switch.load_output_alias(&alias, &default)?;
        }

        match self.cmd {
            Some(SubCommand::Switch(switch_opts)) => {
                let buffer: String = switch.command_build(&switch_opts.input, &switch_opts.output)?;

                telnet
                    .write(&buffer.as_bytes())
                    .expect("Error sending command to HDMI switch");
            }
            Some(SubCommand::Ls {}) => {
                println!("Aliases:");
                switch.list_input_aliases();
                println!();
                switch.list_output_aliases();
                println!();
                println!();
                println!("Defaults:");
                switch.list_input_defaults();
                println!();
                switch.list_output_defaults();
            }
            None => {
                return Err("No subcommand found. Please use -h for available subcommands".to_string());
            }
        }
        return Ok(());
    }
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    Switch(SwitchOptions),
    Ls {},
}

#[derive(Debug, StructOpt)]
struct SwitchOptions {
    #[structopt(short, long)]
    input: String,
    #[structopt(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let configuration_file_path: String = opt
        .get_file_path()
        .expect("unable to find configuration file");

    let configuration = configuration::get_configuration(configuration_file_path)?;

    opt.execute(configuration)?;

    Ok(())
}
