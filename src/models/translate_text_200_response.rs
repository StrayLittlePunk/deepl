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
pub struct TranslateText200Response {
    #[serde(rename = "translations", skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<crate::models::TranslateText200ResponseTranslationsInner>>,
}

impl TranslateText200Response {
    pub fn new() -> TranslateText200Response {
        TranslateText200Response {
            translations: None,
        }
    }
}


