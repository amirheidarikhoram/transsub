use std::path;

use crate::{entry::Entry, iso_639::LanguageCodes, utils::log_info};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
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

pub fn read_file(file_address: &str) -> Result<Vec<Entry>, String> {
    lazy_static! {
        static ref SRT_VALUE_REGEX: Regex = Regex::new(
            r#"(\d{1,})\n(\d{2,}:\d{2}:\d{2},\d{3}) --> (\d{2,}:\d{2}:\d{2},\d{3})\n(.*)\n"#
        )
        .unwrap();
    }

    let content = std::fs::read_to_string(file_address);
    let mut content = match content {
        Ok(content) => content.replace("\r\n", "\n"),
        Err(_) => return Err("Error reading file".to_string()),
    };

    content.push_str("\n\n");

    let matches = SRT_VALUE_REGEX.captures_iter(&content);
    let mut entries: Vec<Entry> = Vec::new();
    for capture in matches {
        let id = match capture.get(1) {
            Some(id) => id.as_str().to_string(),
            None => return Err("Error matching subtitle id".to_string()),
        };
        let start_time = match capture.get(2) {
            Some(start_time) => start_time.as_str().to_string(),
            None => return Err("Error matching subtitle start time".to_string()),
        };
        let end_time = match capture.get(3) {
            Some(end_time) => end_time.as_str().to_string(),
            None => return Err("Error matching subtitle end time".to_string()),
        };
        let text = match capture.get(4) {
            Some(text) => text.as_str().to_string(),
            None => return Err("Error matching subtitle text".to_string()),
        };

        let entry = Entry {
            file_address: file_address.to_string(),
            id,
            start_time,
            end_time,
            text,
        };

        entries.push(entry);
    }

    Ok(entries)
}

pub fn write_file(
    file: String,
    output_dir: Option<String>,
    content: String,
    target_lang: LanguageCodes,
) -> Result<(), String> {
    let output_file_path = match output_dir {
        Some(output_dir) => {
            let original_path = path::Path::new(file.as_str());
            let original_file_name = original_path.file_name().unwrap().to_str().unwrap();
            let original_file_name = original_file_name
                .split(".")
                .next()
                .unwrap_or(original_file_name);

            let output_path = path::Path::new(output_dir.as_str());
            let output_file_name = format!("{}-{}.srt", original_file_name, target_lang.as_ref());

            output_path.join(output_file_name)
        }
        None => {
            let original_path = path::Path::new(file.as_str());
            let original_file_name = original_path.file_name().unwrap().to_str().unwrap();
            let original_file_name = original_file_name
                .split(".")
                .next()
                .unwrap_or(original_file_name);

            let output_file_name = format!("{}-{}.srt", original_file_name, target_lang.as_ref());

            original_path.with_file_name(output_file_name)
        }
    };

    let mut file = match File::create(output_file_path) {
        Ok(file) => file,
        Err(_) => return Err("Failed to create output file".to_string()),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err("Failed to write to output file".to_string()),
    }

    Ok(())
}
