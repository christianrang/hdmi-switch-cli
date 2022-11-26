use indexmap::IndexMap;
use std::error::Error;

pub struct Switch {
    input_defaults: IndexMap<String, String>,
    input_aliases: IndexMap<String, String>,

    output_defaults: IndexMap<String, String>,
    output_aliases: IndexMap<String, String>,
}

impl Switch {
    pub fn new() -> Self {
        let switch = Switch {
            input_defaults: IndexMap::from([
                ("hdmiin1".to_string(), "hdmiin1".to_string()),
                ("hdmiin2".to_string(), "hdmiin2".to_string()),
                ("hdmiin3".to_string(), "hdmiin3".to_string()),
                ("hdmiin4".to_string(), "hdmiin4".to_string()),
                ("all".to_string(), "all".to_string()),
            ]),
            input_aliases: IndexMap::new(),
            output_defaults: IndexMap::from([
                ("hdmiout1".to_string(), "hdmiout1".to_string()),
                ("hdmiout2".to_string(), "hdmiout2".to_string()),
                ("hdmiout3".to_string(), "hdmiout3".to_string()),
                ("hdmiout4".to_string(), "hdmiout4".to_string()),
                ("all".to_string(), "all".to_string()),
            ]),
            output_aliases: IndexMap::new(),
        };
        return switch;
    }

    pub fn list_input_defaults(&self) {
        println!("Input Defaults:");
        for (key, value) in self.input_defaults.iter() {
            println!("  {}: {}", key, value);
        }
    }
    pub fn list_input_aliases(&self) {
        println!("Input aliases:");
        for (key, value) in self.input_aliases.iter() {
            println!("  {}: {}", key, value);
        }
    }

    pub fn list_output_defaults(&self) {
        println!("Output Defaults:");
        for (key, value) in self.output_defaults.iter() {
            println!("  {}: {}", key, value);
        }
    }
    pub fn list_output_aliases(&self) {
        println!("Output aliases:");
        for (key, value) in self.output_aliases.iter() {
            println!("  {}: {}", key, value);
        }
    }

    pub fn load_input_alias(&mut self, alias: &str, default: &str) -> Result<(), String> {
        match is_valid_input(default) {
            true => {
                self.input_aliases
                    .insert(alias.to_string(), default.to_string());
                return Ok(());
            }
            false => {
                return Err(format!("Input {} is not a supported input.", default));
            }
        };
    }
    /// Accepts a vector of aliases. Vector is expected to be a tuple of (alias, default)
    pub fn load_input_aliases(&mut self, aliases: Vec<(&str, &str)>) -> Result<(), Box<dyn Error>> {
        for (alias, default) in aliases.iter() {
            self.load_input_alias(alias, default)?;
        }
        return Ok(());
    }

    pub fn load_output_alias(&mut self, alias: &str, default: &str) -> Result<(), String> {
        match is_valid_output(default) {
            true => {
        self.output_aliases
            .insert(alias.to_string(), default.to_string());
                return Ok(());
            }
            false => {
                return Err(format!("Output {} is not a supported output.", default));
            }
        };
    }
    /// Accepts a vector of aliases. Vector is expected to be a tuple of (alias, default)
    pub fn load_output_aliases(&mut self, aliases: Vec<(&str, &str)>) -> Result<(), Box<dyn Error>>{
        for (alias, default) in aliases.iter() {
            self.load_output_alias(alias, default)?;
        }
        return Ok(());
    }
    pub fn command_build(self, input: String, output: String) -> Result<String, String> {
        let mut inputs: IndexMap<String, String> = IndexMap::new();
        let mut outputs: IndexMap<String, String> = IndexMap::new();

        inputs.extend(self.input_defaults);
        inputs.extend(self.input_aliases);

        let input = match inputs.get(&input) {
            Some(value) => value,
            _ => {
                return Err(format!("Input {} not supported", input));
            }
        };

        outputs.extend(self.output_defaults);
        outputs.extend(self.output_aliases);

        let output = match outputs.get(&output) {
            Some(value) => value,
            _ => {
                return Err(format!("Output {} not supported", output));
            }
        };

        let command: String = format!("SET SW {} {}\n\r", input, output);
        return Ok(command);
    }
}

fn is_valid_input(input: &str) -> bool {
    match input {
        "hdmiin1" => return true,
        "hdmiin2" => return true,
        "hdmiin3" => return true,
        "hdmiin4" => return true,
        _v => return false,
    }
}

fn is_valid_output(output: &str) -> bool {
    match output {
        "hdmiout1" => return true,
        "hdmiout2" => return true,
        "hdmiout3" => return true,
        "hdmiout4" => return true,
        _v => return false,
    }
}

#[allow(dead_code)]
pub fn list_default_inputs() {
    let inputs: Switch = Switch::new();
    inputs.list_input_defaults();
}

#[allow(dead_code)]
pub fn list_input_aliases() {
    let switch: Switch = Switch::new();

    // Set aliases
    switch.list_input_aliases();
}

#[allow(dead_code)]
pub fn list_default_outputs() {
    let inputs: Switch = Switch::new();
    inputs.list_output_defaults();
}
