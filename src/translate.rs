use crate::iso_639::LanguageCodes;
use reqwest::get;
use serde_json::Value;

pub async fn translate(
    source_lang: LanguageCodes,
    target_lang: LanguageCodes,
    text: String,
) -> Result<String, ()> {
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        source_lang.as_ref(),
        target_lang.as_ref(),
        text
    );
    let req_response_raw = match get(&url).await.unwrap().text().await {
        Ok(res) => res,
        Err(_) => return Err(()),
    };
    let req_response: Value = match serde_json::from_str(req_response_raw.as_str()) {
        Ok(res) => res,
        Err(_) => return Err(()),
    };
    let translations = match req_response[0].as_array() {
        Some(res) => res,
        None => return Err(()),
    };

    let mut translated_text = String::new();
    for translation in translations {
        if !translated_text.is_empty() {
            translated_text.push_str(" ");
        }
        let translated_sentence = match translation[0].as_str() {
            Some(res) => res,
            None => return Err(()),
        };
        translated_text.push_str(translated_sentence);
    }

    Ok(translated_text)
}
