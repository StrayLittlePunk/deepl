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
pub struct ListGlossaries200Response {
    #[serde(rename = "glossaries", skip_serializing_if = "Option::is_none")]
    pub glossaries: Option<Vec<crate::models::Glossary>>,
}

impl ListGlossaries200Response {
    pub fn new() -> ListGlossaries200Response {
        ListGlossaries200Response {
            glossaries: None,
        }
    }
}


