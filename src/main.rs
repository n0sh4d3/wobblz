mod args;
mod huggies_parser;

use std::error::Error;
use std::fs::read_to_string;
use std::io::{self, Write};

const WOBBWZ_LOGO: &str = "
˚∧＿∧  　+        —̳͟͞͞💗
(  •‿• )つ  —̳͟͞͞ 💗         —̳͟͞͞💗  < hehe url go boom
(つ　 <                —̳͟͞͞💗
｜　 _つ      +  —̳͟͞͞💗         —̳͟͞͞💗 ˚
`し´
        wobbwz
          by purrbytes.sh
";

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct App<W: Write> {
    writer: W,
    args_parser: args::ArgsParser,
    file_reader: Box<dyn FileReader>,
}

impl Default for App<io::Stdout> {
    fn default() -> Self {
        let mut parser = args::ArgsParser::new();
        parser.add_argument(args::Arg::new("target", "person to attack!", true));
        parser.add_argument(args::Arg::new("wordlist", "wowds to use :3", true));

        Self {
            writer: io::stdout(),
            args_parser: parser,
            file_reader: Box::new(RealFileReader {}),
        }
    }
}

impl<W: Write> App<W> {
    pub fn new(writer: W, args_parser: args::ArgsParser, file_reader: Box<dyn FileReader>) -> Self {
        Self {
            writer,
            args_parser,
            file_reader,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let parsed_args = self.args_parser.parse();

        if self.args_parser.validate(&parsed_args) {
            let target = parsed_args.get("target").unwrap().to_owned();
            let wordlist = parsed_args.get("wordlist").unwrap().to_owned();

            writeln!(
                self.writer,
                "(ﾉ>ω<)ﾉ :｡･:*:･ﾟ'★,｡･:*:･ﾟ'☆ hewe's u youw wobbwz wogo~"
            )?;
            writeln!(self.writer, "{}", WOBBWZ_LOGO)?;

            let huggz_file = self.file_reader.read_lines("config.huggz")?;
            let data = huggies_parser::parse(&huggz_file, &parsed_args);

            writeln!(self.writer, "(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄) hewe's youw dataa~ {:?}", data)?;

            Ok(())
        } else {
            writeln!(self.writer, "(｡•́︿•̀｡) i faiwed to wun :(")?;
            Ok(())
        }
    }
}

// trait for file reading to allow mocking
pub trait FileReader {
    fn read_lines(&self, path: &str) -> Result<Vec<String>>;
}

struct RealFileReader;

impl FileReader for RealFileReader {
    fn read_lines(&self, path: &str) -> Result<Vec<String>> {
        let content = read_to_string(path)?;
        Ok(content.lines().map(String::from).collect())
    }
}

fn main() -> Result<()> {
    let mut app = App::default();
    app.run()
}
