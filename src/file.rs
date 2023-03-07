use crate::{entry::Entry, utils::log_info};
use lazy_static::lazy_static;
use regex::Regex;
use walkdir::WalkDir;

pub fn get_files(dir: &str, verbose: bool) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        if entry.file_type().is_file() && file_name.ends_with(".srt") {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }

    if verbose {
        log_info(format!("Found {} srt files in {}", files.len(), dir).as_str());
    }

    files
}

pub fn read_file(file_address: &str) -> Result<Vec<Entry>, ()> {
    lazy_static! {
        static ref SRT_VALUE_REGEX: Regex = Regex::new(
            r#"(\d{1,})\n(\d{2,}:\d{2}:\d{2},\d{3}) --> (\d{2,}:\d{2}:\d{2},\d{3})\n(.*)\n"#
        )
        .unwrap();
    }

    let content = std::fs::read_to_string(file_address);
    let mut content = match content {
        Ok(content) => content,
        Err(_) => return Err(()),
    };

    content.push_str("\n\n");

    let matches = SRT_VALUE_REGEX.captures_iter(&content);
    let mut entries: Vec<Entry> = Vec::new();
    for capture in matches {
        let id = capture.get(1).unwrap().as_str().to_string();
        let start_time = capture.get(2).unwrap().as_str().to_string();
        let end_time = capture.get(3).unwrap().as_str().to_string();
        let text = capture.get(4).unwrap().as_str().to_string();

        let entry = Entry {
            file_address: file_address.to_string(),
            id,
            start_time,
            end_time,
            text,
        };

        entries.push(entry);
    }

    let lentght = entries.len();
    log_info(format!("Found {} entries in {}", lentght, file_address).as_str());

    Ok(entries)
}
