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
pub struct TranslateDocument200Response {
    /// A unique ID assigned to the uploaded document and the translation process. Must be used when referring to this particular document in subsequent API requests.
    #[serde(rename = "document_id", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// A unique key that is used to encrypt the uploaded document as well as the resulting translation on the server side. Must be provided with every subsequent API request regarding this particular document.
    #[serde(rename = "document_key", skip_serializing_if = "Option::is_none")]
    pub document_key: Option<String>,
}

impl TranslateDocument200Response {
    pub fn new() -> TranslateDocument200Response {
        TranslateDocument200Response {
            document_id: None,
            document_key: None,
        }
    }
}


