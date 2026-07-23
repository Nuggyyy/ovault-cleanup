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
    #[argh(option, description = "the directory path for the attachments. relative path should suffice. important: make sure the path ends with '/' for MacOS/Linux and with '\\' for Windows, otherwise delete function won't work.")]
    attachments_dir: String,

    #[argh(option, description = "the directory path for the entire vault. relative path should suffice.")]
    vault: Option<String>,
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
        let file_path = attachments_dir.to_owned() + attachment.as_str();
        println!("Do you want to delete {file_path}? (y/n)");
        let mut decision = String::new();
        io::stdin()
            .read_line(&mut decision)
            .unwrap();

        let decision = decision.trim().to_lowercase();

        if decision == "y" {
            fs::remove_file(file_path)?;
            println!("File deleted.");
        } else {
            println!("File not deleted.");
        }
    }
    Ok(())
}

fn main() {
    // parses command-line arguments
    let args: Directories = argh::from_env();
    let mut vault = String::new();
    if let Some(dir) = args.vault {
        vault += dir.as_str();
    } else {
        vault += ".";
    }

    // creates a list of attachments
    let attachments: Vec<String> = get_attachments(&args.attachments_dir);
    println!("attachments: {:#?}", &attachments);

    // builds a regex pattern-string with the list of attachments, in order to traverse the obsidian vault only once
    let pattern = build_regex_string(&attachments);
    println!("regex pattern: {}", &pattern);

    // creates the regex and walks through the obsidian vault to find mentions for each attachment
    let re = Regex::new(&pattern).unwrap();
    let mentioned = find_mentioned(&re, &vault);
    println!("mentioned attachments: {:#?}", &mentioned);

    // build the list of unmentioned attachments, to know which ones can be deleted
    let unmentioned: Vec<&String> = attachments.iter()
        .filter(|a| !mentioned.contains(a.as_str()))
        .collect();
    println!("attachments to delete: {:#?}", &unmentioned);

    // the deletion function
    let result = delete(&unmentioned, &args.attachments_dir);
    if let Err(..) = result {
        println!("{:#?}", result)
    }
}
