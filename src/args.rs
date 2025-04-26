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
            if self.required { " [wequiwed >w<]" } else { "" }
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

    pub fn parse_from_iter<I>(&self, args_iter: I) -> HashMap<String, String>
    where
        I: Iterator<Item = String>,
    {
        let args: Vec<String> = args_iter.skip(1).collect();
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
                            "(｡•́︿•̀｡) oopsies! wequiwed awgument '{}' pwovided with empty vawue!!",
                            arg_def.long
                        );
                    } else {
                        parsed_args.insert(arg_def.name(), value.clone());
                    }
                    i += 2;
                } else {
                    if arg_def.required {
                        println!(
                            "(Ｔ▽Ｔ) e-ep!! wequiwed awgument '{}' pwovided but no vawue given!!!",
                            arg_def.long
                        );
                    }
                    i += 1;
                }
            } else {
                println!("( ｡ •̀ ᴖ •́ ｡) wawnin': unwecognized awgument '{}'", current);
                i += 1;
            }
        }

        for arg in &self.args {
            if arg.required && !processed_arg_names.contains(&arg.name()) {
                println!(
                    "(ó﹏ò｡) nooo~ wequiwed awgument '{}' was not pwovided!!",
                    arg.long
                );
            }
        }

        parsed_args
    }

    pub fn parse(&self) -> HashMap<String, String> {
        self.parse_from_iter(env::args())
    }

    pub fn parse_with_writer<I, W>(&self, args_iter: I, writer: &mut W) -> HashMap<String, String>
    where
        I: Iterator<Item = String>,
        W: std::io::Write,
    {
        let args: Vec<String> = args_iter.skip(1).collect();
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
                        writeln!(
                            writer,
                            "(｡•́︿•̀｡) oopsies! wequiwed awgument '{}' pwovided with empty vawue!!",
                            arg_def.long
                        )
                        .unwrap();
                    } else {
                        parsed_args.insert(arg_def.name(), value.clone());
                    }
                    i += 2;
                } else {
                    if arg_def.required {
                        writeln!(
                            writer,
                            "(Ｔ▽Ｔ) e-ep!! wequiwed awgument '{}' pwovided but no vawue given!!!",
                            arg_def.long
                        )
                        .unwrap();
                    }
                    i += 1;
                }
            } else {
                writeln!(
                    writer,
                    "( ｡ •̀ ᴖ •́ ｡) wawnin': unwecognized awgument '{}'",
                    current
                )
                .unwrap();
                i += 1;
            }
        }

        for arg in &self.args {
            if arg.required && !processed_arg_names.contains(&arg.name()) {
                writeln!(
                    writer,
                    "(ó﹏ò｡) nooo~ wequiwed awgument '{}' was not pwovided!!",
                    arg.long
                )
                .unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten_arg() {
        assert_eq!(shorten_arg("help"), "h");
        assert_eq!(shorten_arg("version"), "v");
        assert_eq!(shorten_arg(""), "");
    }

    #[test]
    fn test_arg_new() {
        let arg = Arg::new("help", "Shows help message", false);
        assert_eq!(arg.long, "--help");
        assert_eq!(arg.short, "-h");
        assert_eq!(arg.desc, "Shows help message");
        assert_eq!(arg.required, false);
    }

    #[test]
    fn test_arg_matches() {
        let arg = Arg::new("help", "Shows help message", false);
        assert!(arg.matches("--help"));
        assert!(arg.matches("-h"));
        assert!(!arg.matches("help"));
        assert!(!arg.matches("-help"));
    }

    #[test]
    fn test_arg_name() {
        let arg = Arg::new("config", "Config file path", true);
        assert_eq!(arg.name(), "config");
    }

    #[test]
    fn test_arg_display() {
        let required_arg = Arg::new("input", "Input file", true);
        let optional_arg = Arg::new("output", "Output file", false);

        assert_eq!(
            format!("{}", required_arg),
            "--input (-i) — Input file [wequiwed >w<]"
        );
        assert_eq!(format!("{}", optional_arg), "--output (-o) — Output file");
    }

    #[test]
    fn test_parse_from_iter() {
        let mut parser = ArgsParser::new();
        parser.add_argument(Arg::new("input", "Input file", true));
        parser.add_argument(Arg::new("output", "Output file", false));

        let mock_args = vec![
            "program".to_string(),
            "--input".to_string(),
            "file.txt".to_string(),
            "--output".to_string(),
            "out.txt".to_string(),
        ];

        let parsed = parser.parse_from_iter(mock_args.into_iter());

        assert_eq!(parsed.get("input"), Some(&"file.txt".to_string()));
        assert_eq!(parsed.get("output"), Some(&"out.txt".to_string()));
    }

    #[test]
    fn test_parse_with_missing_required_arg() {
        let mut parser = ArgsParser::new();
        parser.add_argument(Arg::new("input", "Input file", true));

        // missing required argument
        let mock_args = vec!["program".to_string()];

        let mut output = Vec::new();
        let parsed = parser.parse_with_writer(mock_args.into_iter(), &mut output);

        assert!(parsed.is_empty());
        assert!(!output.is_empty());

        let error_message = String::from_utf8(output).unwrap();
        assert!(error_message.contains("wequiwed awgument '--input'"));
    }

    #[test]
    fn test_parse_with_unknown_arg() {
        let mut parser = ArgsParser::new();
        parser.add_argument(Arg::new("input", "Input file", true));

        let mock_args = vec![
            "program".to_string(),
            "--unknown".to_string(),
            "--input".to_string(),
            "file.txt".to_string(),
        ];

        let mut output = Vec::new();
        let parsed = parser.parse_with_writer(mock_args.into_iter(), &mut output);

        assert_eq!(parsed.get("input"), Some(&"file.txt".to_string()));

        let error_message = String::from_utf8(output).unwrap();
        assert!(error_message.contains("unwecognized awgument '--unknown'"));
    }

    #[test]
    fn test_validate_with_required_args() {
        let mut parser = ArgsParser::new();
        parser.add_argument(Arg::new("input", "Input file", true));
        parser.add_argument(Arg::new("output", "Output file", false));

        let mut parsed_args = HashMap::new();
        assert!(!parser.validate(&parsed_args));

        parsed_args.insert("input".to_string(), "".to_string());
        assert!(!parser.validate(&parsed_args));

        parsed_args.insert("input".to_string(), "file.txt".to_string());
        assert!(parser.validate(&parsed_args));

        parsed_args.insert("output".to_string(), "out.txt".to_string());
        assert!(parser.validate(&parsed_args));
    }
}
