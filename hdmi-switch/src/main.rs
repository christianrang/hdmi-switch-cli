mod configuration;
mod utils;

use std::env;
use std::error::Error;
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
    fn get_file_path(&self) -> Result<String, Box<dyn Error>> {
        let mut configuration: String = self.configuration.clone();
        if configuration == "" {
            let home = env::var("HOME")?;
            configuration = format!("{}/.config/hdmi-switch/configuration.yaml", home)
        }

        return Ok(configuration);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut switch = utils::Switch::new();
    switch.load_input_aliases(vec![
        ("pc", "hdmiin1"),
        ("ps", "hdmiin2"),
        ("switch", "hdmiin3"),
        ("work", "hdmiin4"),
    ])?;

    switch.load_output_alias("pc", "hdmiout1");
    switch.load_output_alias("tv", "hdmiout2");
    switch.load_output_aliases(vec![
        ("pc", "hdmiout1"),
        ("tv", "hdmiout2"),
    ]);

    let configuration_file_path: String = opt
        .get_file_path()
        .expect("unable to find configuration file");

    let configuration = configuration::get_configuration(configuration_file_path)?;

    let port = configuration.get_port()?;

    let mut telnet = Telnet::connect((configuration.server.host, port), 256)
        .expect("Couldn't connect to the server...");

    let _event = telnet
        .read()
        .expect("Error reading connection response from HDMI switch");

    let buffer: String = switch.command_build(opt.input, opt.output)?;
    telnet
        .write(&buffer.as_bytes())
        .expect("Error sending command to HDMI switch");

    Ok(())
}
