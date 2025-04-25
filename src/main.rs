mod args;
mod huggz_parser;
use std::fs::read_to_string;

fn main() {
    let wobbwz_logo = "
˚∧＿∧  　+        —̳͟͞͞💗
(  •‿• )つ  —̳͟͞͞ 💗         —̳͟͞͞💗  < hehe url go boom
(つ　 <                —̳͟͞͞💗
｜　 _つ      +  —̳͟͞͞💗         —̳͟͞͞💗 ˚
`し´
        wobbwz
          by purrbytes.sh
";

    let mut parser = args::ArgsParser::new();
    parser.add_argument(args::Arg::new("target", "person to attack!", true));

    let parsed_args = parser.parse();

    if parser.validate(&parsed_args) {
        let _target = parsed_args.get("target").unwrap().to_owned();

        println!("{}", wobbwz_logo);
        let huggz_file = read_lines("config.huggz");

        let _dats = huggz_parser::parse(&huggz_file);
    } else {
        println!("i failed to wun :(")
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
