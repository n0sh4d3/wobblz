mod args;
mod huggies_parser;
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
    parser.add_argument(args::Arg::new("wordlist", "wowds to use :3", true));

    let parsed_args = parser.parse();

    if parser.validate(&parsed_args) {
        let _target = parsed_args.get("target").unwrap().to_owned();
        let _wordlist = parsed_args.get("wordlist").unwrap().to_owned();

        println!("(ﾉ>ω<)ﾉ :｡･:*:･ﾟ’★,｡･:*:･ﾟ’☆ hewe's u youw wobbwz wogo~");
        println!("{}", wobbwz_logo);
        let huggz_file = read_lines("config.huggz");

        let data = huggies_parser::parse(&huggz_file, &parsed_args);
        println!("(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄) hewe's youw dataa~ {:?}", data);
    } else {
        println!("(｡•́︿•̀｡) i faiwed to wun :(")
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
