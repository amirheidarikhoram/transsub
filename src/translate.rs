use crate::iso_639::LanguageCodes;
use regex::Regex;
use reqwest::get;

pub async fn translate(
    source_lang: LanguageCodes,
    target_lang: LanguageCodes,
    text: String,
) -> Result<String, ()> {
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TRANSLATION_REGEX: Regex = Regex::new(r#"^\[\[\["(.+?)".*$"#).unwrap();
    }

    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        source_lang.as_ref(),
        target_lang.as_ref(),
        text
    );
    let req_response = get(&url).await.unwrap().text().await.unwrap();
    if let Some(captures) = TRANSLATION_REGEX.captures(&req_response) {
        Ok(captures[1].to_string())
    } else {
        Err(())
    }
}