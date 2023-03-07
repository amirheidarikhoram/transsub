use crate::iso_639::LanguageCodes;
use reqwest::get;
use serde_json::Value;

pub async fn translate(
    source_lang: LanguageCodes,
    target_lang: LanguageCodes,
    text: String,
) -> Result<String, String> {
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

    Ok(translated_text)
}
