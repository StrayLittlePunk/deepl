# Glossary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**glossary_id** | Option<**String**> | A unique ID assigned to a glossary. | [optional]
**name** | Option<**String**> | Name associated with the glossary. | [optional]
**ready** | Option<**bool**> | Indicates if the newly created glossary can already be used in `translate` requests. If the created glossary is not yet ready, you have to wait and check the `ready` status of the glossary before using it in a `translate` request. | [optional]
**source_lang** | Option<[**crate::models::GlossarySourceLanguage**](GlossarySourceLanguage.md)> |  | [optional]
**target_lang** | Option<[**crate::models::GlossaryTargetLanguage**](GlossaryTargetLanguage.md)> |  | [optional]
**creation_time** | Option<**String**> | The creation time of the glossary in the ISO 8601-1:2019 format (e.g.: `2021-08-03T14:16:18.329Z`). | [optional]
**entry_count** | Option<**i32**> | The number of entries in the glossary. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


