use std::collections::HashMap;

use crate::{entry::Entry, file, iso_639::LanguageCodes, utils::log_error};
use futures::future::join_all;
use reqwest::get;
use serde_json::Value;

pub async fn translate(
    source_lang: LanguageCodes,
    target_lang: LanguageCodes,
    text: String,
    id: Option<String>,
) -> Result<(String, String), String> {
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        source_lang.as_ref(),
        target_lang.as_ref(),
        text
    );
    let req_response_raw = match get(&url).await.unwrap().text().await {
        Ok(res) => res,
        Err(_) => return Err("Error getting response from google translate".to_string()),
    };
    let req_response: Value = match serde_json::from_str(req_response_raw.as_str()) {
        Ok(res) => res,
        Err(_) => return Err("Error parsing response from google translate to json".to_string()),
    };
    let translations = match req_response[0].as_array() {
        Some(res) => res,
        None => return Err("Error getting translated texts from google translate".to_string()),
    };

    let mut translated_text = String::new();
    for translation in translations {
        if !translated_text.is_empty() {
            translated_text.push_str(" ");
        }
        let translated_sentence = match translation[0].as_str() {
            Some(res) => res,
            None => return Err("Error getting sentence from google response".to_string()),
        };
        translated_text.push_str(translated_sentence);
    }

    let id = match id {
        Some(id) => id,
        None => String::from("1"),
    };

    Ok((id, translated_text))
}

pub async fn translate_file(
    source_lang: LanguageCodes,
    target_lang: LanguageCodes,
    file: String,
    verbose: bool,
) -> Result<(), String> {
    let mut futures = vec![];
    let entry_results = file::read_file(file.as_str());
    let mut entry_map: HashMap<String, Entry> = HashMap::new();

    if let Ok(entries) = entry_results {
        for entry in entries.iter() {
            let entry = entry.clone();
            entry_map.insert(entry.id.clone(), entry.clone());
            let entry = entry.clone();
            futures.push(translate(
                source_lang,
                target_lang,
                entry.text,
                Some(entry.id),
            ));
        }
    } else {
        log_error(format!("Failed to read file {}", file.as_str()).as_str());
    }

    let res = join_all(futures).await;

    if res.iter().any(|res| res.is_err()) {
        if verbose {
            log_error("Failed to translate some entries");
        }
        return Err(file);
    } else {
        let mut translated_file_content = String::new();

        for tra in res {
            let tra = tra.unwrap();
            let entry = entry_map.get(tra.0.as_str()).unwrap();
            translated_file_content.push_str(
                format!(
                    "{}\n{} --> {}\n{}\n\n",
                    entry.id, entry.start_time, entry.end_time, tra.1
                )
                .as_str(),
            );
        }

        println!("{}", translated_file_content);
    }

    Ok(())
}
