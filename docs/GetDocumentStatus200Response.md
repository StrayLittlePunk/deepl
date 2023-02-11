# GetDocumentStatus200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document_id** | **String** | A unique ID assigned to the uploaded document and the requested translation process. The same ID that was used when requesting the translation status. | 
**status** | **String** | A short description of the state the document translation process is currently in. Possible values are:  * `queued` - the translation job is waiting in line to be processed  * `translating` - the translation is currently ongoing  * `done` - the translation is done and the translated document is ready for download  * `error` - an irrecoverable error occurred while translating the document | 
**seconds_remaining** | Option<**i32**> | Estimated number of seconds until the translation is done. This parameter is only included while `status` is `\"translating\"`. | [optional]
**billed_characters** | Option<**i32**> | The number of characters billed to your account. | [optional]
**error_message** | Option<**String**> | A short description of the error, if available. Note that the content is subject to change. This parameter may be included if an error occurred during translation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


