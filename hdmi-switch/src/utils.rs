use indexmap::IndexMap;
use anyhow::{Result, anyhow};

type HdmiIn<'a> = &'a str;

pub const HDMIIN1: HdmiIn = "hdmiin1";
pub const HDMIIN2: HdmiIn = "hdmiin2";
pub const HDMIIN3: HdmiIn = "hdmiin3";
pub const HDMIIN4: HdmiIn = "hdmiin4";

type HdmiOut<'a> = &'a str;

pub const HDMIOUT1: HdmiOut = "hdmiout1";
pub const HDMIOUT2: HdmiOut = "hdmiout2";
pub const HDMIOUT3: HdmiOut = "hdmiout3";
pub const HDMIOUT4: HdmiOut = "hdmiout4";
pub const HDMIOUTALL: HdmiOut = "all";

pub struct Switch {
    input_defaults: IndexMap<String, String>,
    input_aliases: IndexMap<String, String>,

    output_defaults: IndexMap<String, String>,
    output_aliases: IndexMap<String, String>,

    longest_str: usize
}

impl Switch {
    pub fn new() -> Self {
        let switch = Switch {
            input_defaults: IndexMap::from([
                (HDMIIN1.to_string(), HDMIIN1.to_string()),
                (HDMIIN2.to_string(), HDMIIN2.to_string()),
                (HDMIIN3.to_string(), HDMIIN3.to_string()),
                (HDMIIN4.to_string(), HDMIIN4.to_string()),
            ]),
            input_aliases: IndexMap::new(),
            output_defaults: IndexMap::from([
                (HDMIOUT1.to_string(), HDMIOUT1.to_string()),
                (HDMIOUT2.to_string(), HDMIOUT2.to_string()),
                (HDMIOUT3.to_string(), HDMIOUT3.to_string()),
                (HDMIOUT4.to_string(), HDMIOUT4.to_string()),
                (HDMIOUTALL.to_string(), HDMIOUTALL.to_string()),
            ]),
            output_aliases: IndexMap::new(),
            longest_str: 0,
        };
        return switch;
    }

    pub fn longest_input_output_key(&mut self) -> usize {
        for (key, _value) in self.input_defaults.iter() {
            if key.len() > self.longest_str {
                self.longest_str = key.len()
            }
        }

        for (key, _value) in self.input_aliases.iter() {
            if key.len() > self.longest_str {
                self.longest_str = key.len()
            }
        }

        for (key, _value) in self.output_defaults.iter() {
            if key.len() > self.longest_str {
                self.longest_str = key.len()
            }
        }

        for (key, _value) in self.output_aliases.iter() {
            if key.len() > self.longest_str {
                self.longest_str = key.len()
            }
        }

        return self.longest_str;
    }

    pub fn list_input_defaults(&mut self) {
        if self.longest_str == 0 {
            self.longest_input_output_key();
        }

        println!("  Input Defaults:");
        for (key, value) in self.input_defaults.iter() {
            let spacer_len = self.longest_str - key.len();
            let mut spacer_string: String = String::new();
            for _ in 0..spacer_len {
                spacer_string.push_str(" ")
            }
            println!("    {key}: {spacer_string}{value}");
        }
    }
    pub fn list_input_aliases(&mut self) {
        if self.longest_str == 0 {
            self.longest_input_output_key();
        }

        println!("  Input aliases:");
        for (key, value) in self.input_aliases.iter() {
            let spacer_len = self.longest_str - key.len();
            let mut spacer_string: String = String::new();
            for _ in 0..spacer_len {
                spacer_string.push_str(" ")
            }
            println!("    {key}: {spacer_string}{value}");
        }
    }
    pub fn list_output_defaults(&mut self) {
        if self.longest_str == 0 {
            self.longest_input_output_key();
        }

        println!("  Output Defaults:");
        for (key, value) in self.output_defaults.iter() {
            let spacer_len = self.longest_str - key.len();
            let mut spacer_string: String = String::new();
            for _ in 0..spacer_len {
                spacer_string.push_str(" ")
            }
            println!("    {key}: {spacer_string}{value}");
        }
    }
    pub fn list_output_aliases(&mut self) {
        if self.longest_str == 0 {
            self.longest_input_output_key();
        }

        println!("  Output aliases:");
        for (key, value) in self.output_aliases.iter() {
            let spacer_len = self.longest_str - key.len();
            let mut spacer_string: String = String::new();
            for _ in 0..spacer_len {
                spacer_string.push_str(" ")
            }
            println!("    {key}: {spacer_string}{value}");
        }
    }

    pub fn load_input_alias(&mut self, alias: &str, default: &str) -> Result<()> {
        match is_valid_input(default) {
            true => {
                self.input_aliases
                    .insert(alias.to_string(), default.to_string());
                return Ok(());
            }
            false => {
                return Err(anyhow!("Loading aliases: Input {} is not a supported input.", default));
            }
        };
    }

    pub fn load_output_alias(&mut self, alias: &str, default: &str) -> Result<()> {
        match is_valid_output(default) {
            true => {
                self.output_aliases
                    .insert(alias.to_string(), default.to_string());
                return Ok(());
            }
            false => {
                return Err(anyhow!("Loading aliases: Output {} is not a supported output.", default));
            }
        };
    }
    pub fn command_build(self, input: &str, output: &str) -> Result<String> {
        let mut inputs: IndexMap<String, String> = IndexMap::new();
        let mut outputs: IndexMap<String, String> = IndexMap::new();

        inputs.extend(self.input_defaults);
        inputs.extend(self.input_aliases);

        let input = match inputs.get(input) {
            Some(value) => value,
            _ => {
                return Err(anyhow!("Input {} not supported", input));
            }
        };

        outputs.extend(self.output_defaults);
        outputs.extend(self.output_aliases);

        let output = match outputs.get(output) {
            Some(value) => value,
            _ => {
                return Err(anyhow!("Output {} not supported", output));
            }
        };

        let command: String = format!("SET SW {} {}\n\r", input, output);
        return Ok(command);
    }
}

// NOTE: should this return an error?
fn is_valid_input(input: &str) -> bool {
    match to_hdmi_in(input) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

pub fn to_hdmi_in(input: &str) -> Result<HdmiIn> {
    match input {
        HDMIIN1 => return Ok(HDMIIN1),
        HDMIIN2 => return Ok(HDMIIN2),
        HDMIIN3 => return Ok(HDMIIN3),
        HDMIIN4 => return Ok(HDMIIN4),
        _v => return Err(anyhow!("{} is not a supported HDMI input", input)),
    }
}

// NOTE: should this return an error?
fn is_valid_output(output: &str) -> bool {
    match to_hdmi_out(output) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

pub fn to_hdmi_out(output: &str) -> Result<HdmiOut> {
    match output {
        HDMIOUT1 => return Ok(HDMIOUT1),
        HDMIOUT2 => return Ok(HDMIOUT2),
        HDMIOUT3 => return Ok(HDMIOUT3),
        HDMIOUT4 => return Ok(HDMIOUT4),
        _v => return Err(anyhow!("{} is not a suppported HDMI output", output)),
    }
}
