use walkdir::WalkDir;
use regex::escape;

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

fn main() {
    let attachments: Vec<String> = get_attachments();
    println!("{:#?}", attachments);
    let pattern = build_regex_string(&attachments);
    println!("{}", pattern)
}
