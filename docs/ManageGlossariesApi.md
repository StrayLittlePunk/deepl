# \ManageGlossariesApi

All URIs are relative to *https://api.deepl.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_glossary**](ManageGlossariesApi.md#create_glossary) | **POST** /glossaries | Create a Glossary
[**delete_glossary**](ManageGlossariesApi.md#delete_glossary) | **DELETE** /glossaries/{glossary_id} | Delete a Glossary
[**get_glossary**](ManageGlossariesApi.md#get_glossary) | **GET** /glossaries/{glossary_id} | Retrieve Glossary Details
[**get_glossary_entries**](ManageGlossariesApi.md#get_glossary_entries) | **GET** /glossaries/{glossary_id}/entries | Retrieve Glossary Entries
[**list_glossaries**](ManageGlossariesApi.md#list_glossaries) | **GET** /glossaries | List all Glossaries
[**list_glossary_languages**](ManageGlossariesApi.md#list_glossary_languages) | **GET** /glossary-language-pairs | List Language Pairs Supported by Glossaries



## create_glossary

> crate::models::Glossary create_glossary(name, source_lang, target_lang, entries, entries_format)
Create a Glossary

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name to be associated with the glossary. | [required] |
**source_lang** | [**crate::models::GlossarySourceLanguage**](GlossarySourceLanguage.md) |  | [required] |
**target_lang** | [**crate::models::GlossaryTargetLanguage**](GlossaryTargetLanguage.md) |  | [required] |
**entries** | **String** | The entries of the glossary. The entries have to be specified in the format provided by the `entries_format` parameter. | [required] |
**entries_format** | **String** | The format in which the glossary entries are provided. Formats currently available: - `tsv` (default) - tab-separated values - `csv` - comma-separated values  See [Supported Glossary Formats](https://www.deepl.com/docs-api/glossaries/formats) for details about each format. | [required] |[default to tsv]

### Return type

[**crate::models::Glossary**](Glossary.md)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_glossary

> delete_glossary(glossary_id)
Delete a Glossary

Deletes the specified glossary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**glossary_id** | **String** | A unique ID assigned to the glossary. | [required] |

### Return type

 (empty response body)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_glossary

> crate::models::Glossary get_glossary(glossary_id)
Retrieve Glossary Details

Retrieve meta information for a single glossary, omitting the glossary entries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**glossary_id** | **String** | A unique ID assigned to the glossary. | [required] |

### Return type

[**crate::models::Glossary**](Glossary.md)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_glossary_entries

> get_glossary_entries(glossary_id, accept)
Retrieve Glossary Entries

List the entries of a single glossary in the format specified by the `Accept` header.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**glossary_id** | **String** | A unique ID assigned to the glossary. | [required] |
**accept** | Option<**String**> | The requested format of the returned glossary entries. Currently, supports only `text/tab-separated-values`. |  |[default to text/tab-separated-values]

### Return type

 (empty response body)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/tab-separated-values, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_glossaries

> crate::models::ListGlossaries200Response list_glossaries()
List all Glossaries

List all glossaries and their meta-information, but not the glossary entries.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListGlossaries200Response**](listGlossaries_200_response.md)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_glossary_languages

> crate::models::ListGlossaryLanguages200Response list_glossary_languages()
List Language Pairs Supported by Glossaries

Retrieve the list of language pairs supported by the glossary feature.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListGlossaryLanguages200Response**](listGlossaryLanguages_200_response.md)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

