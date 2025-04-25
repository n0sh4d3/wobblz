use std::collections::HashMap;

pub struct HuggzParser {}

pub fn parse(content: &Vec<String>) -> Vec<String> {
    let current_section: HashMap<String, String> = HashMap::new();
    for line in content {
        let trimmed = line.trim();

        // comments
        if trimmed.starts_with("//") {
            continue;
        }

        if trimmed.starts_with("[") && trimmed.ends_with("]") {
            let mut section_name = trimmed.strip_prefix("[");
            section_name = trimmed.strip_suffix("]");

            if section_name.iter().any(|a| *a == ".") {}
        }
    }

    Vec::new()
}
