use std::collections::HashMap;

#[derive(Debug)]
pub struct Huggies {
    fuzz_word: String,
    wordlist: String,
    threads: i32,
}

pub fn parse(content: &[String], args: &HashMap<String, String>) -> Huggies {
    // em varwiables!! (｡•̀ᴗ-)✧
    let fuzzword = "fuzzword".to_string();
    let wordlist = "wordlist".to_string();
    let threads = "threads".to_string();

    let mut huggies = Huggies {
        fuzz_word: "".to_string(),
        wordlist: "".to_string(),
        threads: 0,
    };
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

        if key == fuzzword {
            if config.contains_key(key) {
                println!(
                    "(｡•́︿•̀｡) oopsie woopsie!! '{}' was decwawed mowe than once!!",
                    key
                );
            } else {
                config.insert(key.to_string(), value.to_string());
                huggies.fuzz_word = value.to_string();
            }
        }
        if key == wordlist {
            if config.contains_key(key) {
                println!("(ó﹏ò｡) oh nuu!! '{}' is awweady thewe!!", key);
            } else {
                config.insert(key.to_string(), value.to_string());
                huggies.wordlist = value.to_string()
            }
        }
        if key == threads {
            if config.contains_key(key) {
                println!("(ノ_<。) uh-oh!! '{}' is duplicated!!", key);
            } else {
                config.insert(key.to_string(), value.to_string());
                huggies.threads = value
                    .to_string()
                    .parse()
                    .expect("✧･ﾟ: *✧･ﾟ:* invawid thweads count >w< *:･ﾟ✧*:･ﾟ✧");
            }
        }
    }

    if !config.contains_key(&fuzzword.to_string()) {
        println!("(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄) no fuzzwowd~ using defauwt 'HUGGIE'~");
        huggies.fuzz_word = "HUGGIE".to_string()
    }

    if !config.contains_key(&wordlist.to_string()) {
        println!("(｡>﹏<｡) no wowdlist p-pwovided~ using awgs one~!");
        huggies.wordlist = args.get("wordlist").unwrap().to_owned();
    }

    if !config.contains_key(&threads) {
        println!("(✿^‿^) no thweads specified, using 10 nya~");
        huggies.threads = 10
    }

    huggies
}
