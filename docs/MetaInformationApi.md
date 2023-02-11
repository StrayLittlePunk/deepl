# \MetaInformationApi

All URIs are relative to *https://api.deepl.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_languages**](MetaInformationApi.md#get_languages) | **GET** /languages | Retrieve Supported Languages
[**get_usage**](MetaInformationApi.md#get_usage) | **GET** /usage | Check Usage and Limits



## get_languages

> Vec<crate::models::GetLanguages200ResponseInner> get_languages(r#type)
Retrieve Supported Languages

Retrieve the list of languages that are currently supported for translation, either as source or target language, respectively.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Sets whether source or target languages should be listed. Possible options are:  * `source` (default): For languages that can be used in the `source_lang` parameter of [translate](https://www.deepl.com/docs-api/translate-text/translate-text) requests.  * `target`: For languages that can be used in the `target_lang` parameter of [translate](https://www.deepl.com/docs-api/translate-text/translate-text) requests. |  |[default to source]

### Return type

[**Vec<crate::models::GetLanguages200ResponseInner>**](getLanguages_200_response_inner.md)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage

> crate::models::GetUsage200Response get_usage()
Check Usage and Limits

Retrieve usage information within the current billing period together with the corresponding account limits. Usage is returned for: - translated characters - translated documents - translated documents, team totals (for team accounts only)  Character usage includes both text and document translations, and is measured by the source text length in Unicode code points, so for example \"A\", \"Δ\", \"あ\", and \"深\" are each counted as a single character.  Document usage only includes document translations, and is measured in individual documents.  Depending on the user account type, some usage types will be omitted. Character usage is only included for developer accounts. Document usage is only included for non-developer accounts, and team-combined document usage is only included for non-developer team accounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetUsage200Response**](getUsage_200_response.md)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

