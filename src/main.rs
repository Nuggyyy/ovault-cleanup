use walkdir::WalkDir;
use regex::escape;
use regex::Regex;
use std::fs;
use std::collections::HashSet;



fn get_attachments() -> Vec<String> {
    let mut attachments: Vec<String> = Vec::new();
    for entry in WalkDir::new("test-dir/attachments/") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            attachments.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    attachments
}

fn build_regex_string(attachments: &[String]) -> String {
    let escaped: Vec<String> = attachments.iter().map(|a| escape(a)).collect();
    format!("({})", escaped.join("|"))
}

fn find_mentioned(re: &Regex) -> HashSet<String> {
    let mut mentioned = HashSet::new();

    for entry in WalkDir::new("test-dir")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            for m in re.find_iter(&content) {
                mentioned.insert(m.as_str().to_string());
            }
        }
    }

    mentioned
}

fn main() {
    let attachments: Vec<String> = get_attachments();
    println!("attachments: {:#?}", &attachments);
    let pattern = build_regex_string(&attachments);
    println!("regex pattern: {}", &pattern);
    let re = Regex::new(&pattern).unwrap();
    let mentioned = find_mentioned(&re);
    println!("mentioned attachments: {:#?}", &mentioned);
    let unmentioned: Vec<&String> = attachments.iter()
        .filter(|a| !mentioned.contains(a.as_str()))
        .collect();
    println!("attachments to delete: {:#?}", &unmentioned)
}
