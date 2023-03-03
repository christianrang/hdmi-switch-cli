use indexmap::IndexMap;

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
    pub fn command_build(self, input: &str, output: &str) -> Result<String, String> {
        let mut inputs: IndexMap<String, String> = IndexMap::new();
        let mut outputs: IndexMap<String, String> = IndexMap::new();

        inputs.extend(self.input_defaults);
        inputs.extend(self.input_aliases);

        let input = match inputs.get(input) {
            Some(value) => value,
            _ => {
                return Err(format!("Input {} not supported", input));
            }
        };

        outputs.extend(self.output_defaults);
        outputs.extend(self.output_aliases);

        let output = match outputs.get(output) {
            Some(value) => value,
            _ => {
                return Err(format!("Output {} not supported", output));
            }
        };

        let command: String = format!("SET SW {} {}\n\r", input, output);
        return Ok(command);
    }
}

// NOTE: should this return an error?
fn is_valid_input(input: &str) -> bool {
    match input {
        "hdmiin1" => return true,
        "hdmiin2" => return true,
        "hdmiin3" => return true,
        "hdmiin4" => return true,
        _v => return false,
    }
}

// NOTE: should this return an error?
fn is_valid_output(output: &str) -> bool {
    match output {
        "hdmiout1" => return true,
        "hdmiout2" => return true,
        "hdmiout3" => return true,
        "hdmiout4" => return true,
        _v => return false,
    }
}
