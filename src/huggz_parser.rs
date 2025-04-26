use std::collections::HashMap;

pub fn parse(content: &[String]) -> HashMap<String, String> {
    //em varwiables!!
    let allowed_keys = ["fuzzword", "wordlist"];
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

        if allowed_keys.contains(&key) {
            if config.contains_key(key) {
                println!("Error: '{}' declared more than once", key);
            } else {
                config.insert(key.to_string(), value.to_string());
            }
        }
    }

    config
}
