mod args;

const UWU: &str = "UWU";

fn main() {
    let mut parser = args::ArgsParser::new();
    parser.add_argument(args::Arg::new("target", "person to attack!", true));

    let parsed_args = parser.parse();

    let target = parsed_args.get("target").unwrap();
    if parser.validate(&parsed_args) {
        let target = parsed_args.get("target").unwrap().to_owned();

        check_for_fuzz(&target)
    }
}

fn check_for_fuzz(target: &String) -> bool {
    if target.contains(&UWU.to_string()) {
        true
    } else {
        false
    }
}
