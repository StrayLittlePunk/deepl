# TranslateDocumentsApi

All URIs are relative to *https://api.deepl.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_document**](TranslateDocumentsApi.md#download_document) | **POST** /document/{document_id}/result | Download Translated Document
[**get_document_status**](TranslateDocumentsApi.md#get_document_status) | **POST** /document/{document_id} | Check Document Status
[**translate_document**](TranslateDocumentsApi.md#translate_document) | **POST** /document | Upload and Translate a Document



## download_document

> std::path::PathBuf download_document(document_id, get_document_status_request)
Download Translated Document

Once the status of the document translation process is `done`, the result can be downloaded.   For privacy reasons the translated document is automatically removed from the server once it was downloaded and cannot be downloaded again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | The document ID that was sent to the client when the document was uploaded to the API. | [required] |
**get_document_status_request** | [**GetDocumentStatusRequest**](GetDocumentStatusRequest.md) |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_status

> crate::models::GetDocumentStatus200Response get_document_status(document_id, get_document_status_request)
Check Document Status

Retrieve the current status of a document translation process. If the translation is still in progress, the estimated time remaining is also included in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | The document ID that was sent to the client when the document was uploaded to the API. | [required] |
**get_document_status_request** | [**GetDocumentStatusRequest**](GetDocumentStatusRequest.md) |  | [required] |

### Return type

[**crate::models::GetDocumentStatus200Response**](getDocumentStatus_200_response.md)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## translate_document

> crate::models::TranslateDocument200Response translate_document(target_lang, file, source_lang, filename, formality, glossary_id)
Upload and Translate a Document

This call uploads a document and queues it for translation. The call returns once the upload is complete, returning a document ID and key which can be used to [query the translation status](https://www.deepl.com/docs-api/documents/get-document-status) and to [download the translated document](https://www.deepl.com/docs-api/documents/download-document) once translation is complete.    Because the request includes a file upload, it must be an HTTP POST request with content type `multipart/form-data`.   Please be aware that the uploaded document is automatically removed from the server once the translated document has been downloaded. You have to upload the document again in order to restart the translation.   The maximum upload limit for documents is [available here](https://support.deepl.com/hc/articles/360020582359-Document-formats) and may vary based on API plan and document type.   You may specify the glossary to use for the document translation using the `glossary_id` parameter. **Important:** This requires the `source_lang` parameter to be set and the language pair of the glossary has to match the language pair of the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_lang** | [**crate::models::TargetLanguage**](TargetLanguage.md) |  | [required] |
**file** | **std::path::PathBuf** | The document file to be translated. The file name should be included in this part's content disposition. As an alternative, the filename parameter can be used. The following file types and extensions are supported:   * `docx` - Microsoft Word Document   * `pptx` - Microsoft PowerPoint Document   * `pdf` - Portable Document Format   * `htm / html` - HTML Document   * `txt` - Plain Text Document   * `xlf / xliff` - XLIFF Document, version 2.1  Please note that in order to translate PDF documents you need to give one-time consent to using the Adobe API via the account interface. | [required] |
**source_lang** | Option<[**crate::models::SourceLanguage**](SourceLanguage.md)> |  |  |
**filename** | Option<**String**> | The name of the uploaded file. Can be used as an alternative to including the file name in the file part's content disposition. |  |
**formality** | Option<[**crate::models::Formality**](Formality.md)> |  |  |
**glossary_id** | Option<**String**> | A unique ID assigned to a glossary. |  |

### Return type

[**crate::models::TranslateDocument200Response**](translateDocument_200_response.md)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

