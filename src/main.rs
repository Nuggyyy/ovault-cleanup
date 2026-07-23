use walkdir::WalkDir;
use regex::escape;
use regex::Regex;
use std::fs;
use std::io;
use std::collections::HashSet;
use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "{command_name} is a tool to provide directory paths. it is required to provide both --attachments_directory and --vault.")]
struct Directories {
    #[argh(option, description = "the directory path for the attachments. relative path should suffice.")]
    attachments_dir: String,

    #[argh(option, description = "the directory path for the entire vault. relative path should suffice.")]
    vault: String,
}

fn get_attachments(dir: &str) -> Vec<String> {
    let mut attachments: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir) {
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

fn find_mentioned(re: &Regex, vault: &str) -> HashSet<String> {
    let mut mentioned = HashSet::new();

    for entry in WalkDir::new(vault)
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

fn delete(unmentioned: &Vec<&String>, attachments_dir: &str) -> std::io::Result<()> {
    for attachment in unmentioned {
        let full_path = String::from("./") + attachments_dir + attachment.as_str();
        println!("Do you want to delete {full_path}? (y/n)");
        let mut decision = String::new();
        io::stdin()
            .read_line(&mut decision)
            .unwrap();

        let decision = decision.trim().to_lowercase();

        if decision == "y" {
            fs::remove_file(attachment)?;
            println!("File deleted.");
        } else {
            println!("File not deleted.");
        }
    }
    Ok(())
}

fn main() {
    let args: Directories = argh::from_env();
    let attachments: Vec<String> = get_attachments(&args.attachments_dir);
    println!("attachments: {:#?}", &attachments);
    let pattern = build_regex_string(&attachments);
    println!("regex pattern: {}", &pattern);
    let re = Regex::new(&pattern).unwrap();
    let mentioned = find_mentioned(&re, &args.vault);
    println!("mentioned attachments: {:#?}", &mentioned);
    let unmentioned: Vec<&String> = attachments.iter()
        .filter(|a| !mentioned.contains(a.as_str()))
        .collect();
    println!("attachments to delete: {:#?}", &unmentioned);
    let result = delete(&unmentioned, &args.attachments_dir);
    println!("{:#?}", &result);
}
