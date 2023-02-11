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
pub struct ListGlossaryLanguages200Response {
    /// The list of supported languages
    #[serde(rename = "supported_languages", skip_serializing_if = "Option::is_none")]
    pub supported_languages: Option<Vec<crate::models::ListGlossaryLanguages200ResponseSupportedLanguagesInner>>,
}

impl ListGlossaryLanguages200Response {
    pub fn new() -> ListGlossaryLanguages200Response {
        ListGlossaryLanguages200Response {
            supported_languages: None,
        }
    }
}

