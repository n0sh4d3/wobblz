use std::{collections::HashMap, env, fmt};

#[derive(Debug)]
pub struct Arg {
    pub long: String,
    pub short: String,
    pub desc: String,
    pub required: bool,
}

impl Arg {
    pub fn new(long: &str, desc: &str, required: bool) -> Self {
        let short = shorten_arg(long);
        Self {
            long: format!("--{}", long),
            short: format!("-{}", short),
            desc: desc.to_string(),
            required,
        }
    }

    pub fn matches(&self, arg: &str) -> bool {
        arg == self.short || arg == self.long
    }

    // Helper to get the name without the dashes for use as hash keys
    pub fn name(&self) -> String {
        self.long.trim_start_matches("--").to_string()
    }
}

fn shorten_arg(long: &str) -> String {
    long.chars()
        .next()
        .map(|c| c.to_string())
        .unwrap_or_default()
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) — {}{}",
            self.long,
            self.short,
            self.desc,
            if self.required { " [required]" } else { "" }
        )
    }
}

pub struct ArgsParser {
    args: Vec<Arg>,
}

impl ArgsParser {
    pub fn new() -> Self {
        ArgsParser { args: Vec::new() }
    }

    pub fn add_argument(&mut self, arg: Arg) {
        self.args.push(arg);
    }

    pub fn parse(&self) -> HashMap<String, String> {
        let args: Vec<String> = env::args().skip(1).collect();
        let mut parsed_args = HashMap::new();
        let mut processed_arg_names = Vec::new();
        let mut i = 0;

        while i < args.len() {
            let current = &args[i];

            if let Some(arg_def) = self.args.iter().find(|arg| arg.matches(current)) {
                processed_arg_names.push(arg_def.name());

                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    let value = &args[i + 1];

                    if value.is_empty() && arg_def.required {
                        println!(
                            "Error: Required argument '{}' provided with empty value",
                            arg_def.long
                        );
                    } else {
                        parsed_args.insert(arg_def.name(), value.clone());
                    }
                    i += 2;
                } else {
                    if arg_def.required {
                        println!(
                            "Error: Required argument '{}' provided but no value given",
                            arg_def.long
                        );
                    }
                    i += 1;
                }
            } else {
                println!("Warning: Unrecognized argument '{}'", current);
                i += 1;
            }
        }

        for arg in &self.args {
            if arg.required && !processed_arg_names.contains(&arg.name()) {
                println!("Error: Required argument '{}' not provided", arg.long);
            }
        }

        parsed_args
    }

    pub fn validate(&self, parsed_args: &HashMap<String, String>) -> bool {
        let mut is_valid = true;

        for arg in &self.args {
            if arg.required {
                let name = arg.name();
                if !parsed_args.contains_key(&name) {
                    is_valid = false;
                } else if parsed_args[&name].is_empty() {
                    is_valid = false;
                }
            }
        }

        is_valid
    }
}

fn main() {
    let mut parser = ArgsParser::new();
    parser.add_argument(Arg::new("target", "person to attack!", true));
    parser.add_argument(Arg::new("wordlist", "words to use", true));

    let parsed_args = parser.parse();

    if parser.validate(&parsed_args) {
        println!("\nAll required arguments provided correctly!");
        println!("Target: {}", parsed_args.get("target").unwrap());
        println!("Wordlist: {}", parsed_args.get("wordlist").unwrap());
    }
}
