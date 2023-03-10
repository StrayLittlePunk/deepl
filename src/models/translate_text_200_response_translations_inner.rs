/*
 * DeepL API Documentation
 *
 * The DeepL API provides programmatic access to DeepL’s machine translation technology.
 *
 * The version of the OpenAPI document: 2.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TranslateText200ResponseTranslationsInner {
    #[serde(rename = "detected_source_language", skip_serializing_if = "Option::is_none")]
    pub detected_source_language: Option<Box<crate::models::SourceLanguage>>,
    /// The translated text.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl TranslateText200ResponseTranslationsInner {
    pub fn new() -> TranslateText200ResponseTranslationsInner {
        TranslateText200ResponseTranslationsInner {
            detected_source_language: None,
            text: None,
        }
    }
}


