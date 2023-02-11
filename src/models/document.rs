#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct GetDocumentStatusRequest {
    document_key: String,
}
