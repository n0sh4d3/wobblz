use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct Huggies {
    fuzz_word: String,
    wordlist: String,
    threads: i32,
}

impl Default for Huggies {
    fn default() -> Self {
        Self {
            fuzz_word: "".to_string(),
            wordlist: "".to_string(),
            threads: 0,
        }
    }
}

impl Huggies {
    pub fn fuzz_word(&self) -> &str {
        &self.fuzz_word
    }

    pub fn wordlist(&self) -> &str {
        &self.wordlist
    }

    pub fn threads(&self) -> i32 {
        self.threads
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidThreads(String),
    MissingWordlist,
}

pub struct HuggiesParser<W: Write = std::io::Stdout> {
    writer: W,
}

impl Default for HuggiesParser<std::io::Stdout> {
    fn default() -> Self {
        Self {
            writer: std::io::stdout(),
        }
    }
}

impl<W: Write> HuggiesParser<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn parse(
        &mut self,
        content: &[String],
        args: &HashMap<String, String>,
    ) -> Result<Huggies, ParseError> {
        const FUZZWORD: &str = "fuzzword";
        const WORDLIST: &str = "wordlist";
        const THREADS: &str = "threads";

        let mut huggies = Huggies::default();
        let mut config = HashMap::new();

        for line in content {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            let parts: Vec<_> = trimmed.splitn(2, '=').collect();
            if parts.len() != 2 {
                continue;
            }

            let key = parts[0].trim();
            let value = parts[1].trim();

            if key.is_empty() || key.contains(' ') {
                continue;
            }

            match key {
                FUZZWORD => {
                    if config.contains_key(key) {
                        writeln!(
                            self.writer,
                            "(｡•́︿•̀｡) oopsie woopsie!! '{}' was decwawed mowe than once!!",
                            key
                        )
                        .ok();
                    } else {
                        config.insert(key.to_string(), value.to_string());
                        huggies.fuzz_word = value.to_string();
                    }
                }
                WORDLIST => {
                    if config.contains_key(key) {
                        writeln!(self.writer, "(ó﹏ò｡) oh nuu!! '{}' is awweady thewe!!", key).ok();
                    } else {
                        config.insert(key.to_string(), value.to_string());
                        huggies.wordlist = value.to_string();
                    }
                }
                THREADS => {
                    if config.contains_key(key) {
                        writeln!(self.writer, "(ノ_<。) uh-oh!! '{}' is duplicated!!", key).ok();
                    } else {
                        config.insert(key.to_string(), value.to_string());
                        match value.parse::<i32>() {
                            Ok(threads) => huggies.threads = threads,
                            Err(_) => return Err(ParseError::InvalidThreads(value.to_string())),
                        }
                    }
                }
                _ => {}
            }
        }

        if !config.contains_key(FUZZWORD) {
            writeln!(
                self.writer,
                "(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄) no fuzzwowd~ using defauwt 'HUGGIE'~"
            )
            .ok();
            huggies.fuzz_word = "HUGGIE".to_string();
        }

        if !config.contains_key(WORDLIST) {
            writeln!(
                self.writer,
                "(｡>﹏<｡) no wowdlist p-pwovided~ using awgs one~!"
            )
            .ok();

            if let Some(wordlist) = args.get(WORDLIST) {
                huggies.wordlist = wordlist.to_owned();
            } else {
                return Err(ParseError::MissingWordlist);
            }
        }

        if !config.contains_key(THREADS) {
            writeln!(self.writer, "(✿^‿^) no thweads specified, using 10 nya~").ok();
            huggies.threads = 10;
        }

        Ok(huggies)
    }
}

pub fn parse(content: &[String], args: &HashMap<String, String>) -> Huggies {
    let mut parser = HuggiesParser::default();
    parser.parse(content, args).unwrap_or_else(|err| match err {
        ParseError::InvalidThreads(value) => {
            panic!("✧･ﾟ: *✧･ﾟ:* invawid thweads count >w< *:･ﾟ✧*:･ﾟ✧: {}", value);
        }
        ParseError::MissingWordlist => {
            panic!("(｡>﹏<｡) no wowdlist p-pwovided anywhewe~!");
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_config() {
        let mut output = Vec::new();
        let mut parser = HuggiesParser::new(&mut output);

        let args = HashMap::from([("wordlist".to_string(), "words.txt".to_string())]);

        let result = parser.parse(&[], &args).unwrap();

        assert_eq!(result.fuzz_word(), "HUGGIE");
        assert_eq!(result.wordlist(), "words.txt");
        assert_eq!(result.threads(), 10);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("no fuzzwowd"));
        assert!(output_str.contains("no wowdlist"));
        assert!(output_str.contains("no thweads"));
    }

    #[test]
    fn test_full_config() {
        let mut output = Vec::new();
        let mut parser = HuggiesParser::new(&mut output);

        let config = vec![
            "fuzzword = TEST".to_string(),
            "wordlist = dictionary.txt".to_string(),
            "threads = 5".to_string(),
        ];

        let args = HashMap::new();

        let result = parser.parse(&config, &args).unwrap();

        assert_eq!(result.fuzz_word(), "TEST");
        assert_eq!(result.wordlist(), "dictionary.txt");
        assert_eq!(result.threads(), 5);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.is_empty());
    }

    #[test]
    fn test_duplicate_keys() {
        let mut output = Vec::new();
        let mut parser = HuggiesParser::new(&mut output);

        let config = vec![
            "fuzzword = FIRST".to_string(),
            "fuzzword = SECOND".to_string(),
            "wordlist = dict1.txt".to_string(),
            "wordlist = dict2.txt".to_string(),
            "threads = 3".to_string(),
            "threads = 8".to_string(),
        ];

        let args = HashMap::new();

        let result = parser.parse(&config, &args).unwrap();

        assert_eq!(result.fuzz_word(), "FIRST");
        assert_eq!(result.wordlist(), "dict1.txt");
        assert_eq!(result.threads(), 3);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("decwawed mowe than once"));
        assert!(output_str.contains("awweady thewe"));
        assert!(output_str.contains("duplicated"));
    }

    #[test]
    fn test_invalid_threads() {
        let mut output = Vec::new();
        let mut parser = HuggiesParser::new(&mut output);

        let config = vec![
            "fuzzword = TEST".to_string(),
            "wordlist = dictionary.txt".to_string(),
            "threads = not_a_number".to_string(),
        ];

        let args = HashMap::new();

        let result = parser.parse(&config, &args);

        assert!(matches!(result, Err(ParseError::InvalidThreads(_))));
    }

    #[test]
    fn test_missing_wordlist() {
        let mut output = Vec::new();
        let mut parser = HuggiesParser::new(&mut output);

        let config = vec!["fuzzword = TEST".to_string(), "threads = 5".to_string()];

        let args = HashMap::new();

        let result = parser.parse(&config, &args);

        assert!(matches!(result, Err(ParseError::MissingWordlist)));
    }

    #[test]
    fn test_comments_and_whitespace() {
        let mut output = Vec::new();
        let mut parser = HuggiesParser::new(&mut output);

        let config = vec![
            "// This is a comment".to_string(),
            "".to_string(),
            "   ".to_string(),
            "fuzzword = TEST".to_string(),
            "// Another comment".to_string(),
            "wordlist = dictionary.txt".to_string(),
            "threads = 5".to_string(),
        ];

        let args = HashMap::new();

        let result = parser.parse(&config, &args).unwrap();

        assert_eq!(result.fuzz_word(), "TEST");
        assert_eq!(result.wordlist(), "dictionary.txt");
        assert_eq!(result.threads(), 5);
    }

    #[test]
    fn test_invalid_format() {
        let mut output = Vec::new();
        let mut parser = HuggiesParser::new(&mut output);

        let config = vec![
            "fuzzword = TEST".to_string(),
            "invalid line with no equals sign".to_string(),
            "wordlist = dictionary.txt".to_string(),
            "threads = 5".to_string(),
        ];

        let args = HashMap::new();

        let result = parser.parse(&config, &args).unwrap();

        assert_eq!(result.fuzz_word(), "TEST");
        assert_eq!(result.wordlist(), "dictionary.txt");
        assert_eq!(result.threads(), 5);
    }
}
