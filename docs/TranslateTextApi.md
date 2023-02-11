# \TranslateTextApi

All URIs are relative to *https://api.deepl.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**translate_text**](TranslateTextApi.md#translate_text) | **POST** /translate | Request Translation



## translate_text

> crate::models::TranslateText200Response translate_text(text, target_lang, source_lang, split_sentences, preserve_formatting, formality, glossary_id, tag_handling, non_splitting_tags, outline_detection, splitting_tags, ignore_tags)
Request Translation

The translate function.   The total request body size must not exceed 128 KiB (128 · 1024 bytes). Please split up your text into multiple calls if it exceeds this limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text** | [**Vec<String>**](String.md) | Text to be translated. Only UTF-8-encoded plain text is supported. The parameter may be specified multiple times and translations are returned in the same order as they are requested. Each of the parameter values may contain multiple sentences. | [required] |
**target_lang** | [**crate::models::TargetLanguage**](TargetLanguage.md) |  | [required] |
**source_lang** | Option<[**crate::models::SourceLanguage**](SourceLanguage.md)> |  |  |
**split_sentences** | Option<**String**> | Sets whether the translation engine should first split the input into sentences. For text translations where `tag_handling` is not set to `html`, the default value is `1`, meaning the engine splits on punctuation and on newlines.  For text translations where `tag_handling=html`, the default value is `nonewlines`, meaning the engine splits on punctuation only, ignoring newlines.  The use of `nonewlines` as the default value for text translations where `tag_handling=html` is new behavior that was implemented in November 2022, when HTML handling was moved out of beta.  Possible values are:   * `0` - no splitting at all, whole input is treated as one sentence  * `1` (default when `tag_handling` is not set to `html`) - splits on punctuation and on newlines  * `nonewlines` (default when `tag_handling=html`) - splits on punctuation only, ignoring newlines  For applications that send one sentence per text parameter, we recommend setting `split_sentences` to `0`, in order to prevent the engine from splitting the sentence unintentionally.   Please note that newlines will split sentences when `split_sentences=1`. We recommend cleaning files so they don't contain breaking sentences or setting the parameter `split_sentences` to `nonewlines`. |  |[default to 1]
**preserve_formatting** | Option<**String**> | Sets whether the translation engine should respect the original formatting, even if it would usually correct some aspects. Possible values are:  * `0` (default)  * `1`  The formatting aspects affected by this setting include:  * Punctuation at the beginning and end of the sentence  * Upper/lower case at the beginning of the sentence |  |[default to 0]
**formality** | Option<[**crate::models::Formality**](Formality.md)> |  |  |
**glossary_id** | Option<**String**> | Specify the glossary to use for the translation. **Important:** This requires the `source_lang` parameter to be set and the language pair of the glossary has to match the language pair of the request. |  |
**tag_handling** | Option<**String**> | Sets which kind of tags should be handled. Options currently available:  * `xml`: Enable XML tag handling; see [XML Handling](https://www.deepl.com/docs-api/xml).  * `html`: Enable HTML tag handling; see [HTML Handling](https://www.deepl.com/docs-api/html). |  |
**non_splitting_tags** | Option<**String**> | Comma-separated list of XML tags which never split sentences.   For some XML files, finding tags with textual content and splitting sentences using those tags won't yield the best results. The following example shows the engine splitting sentences on `par` tags and proceeding to translate the parts separately, resulting in an incorrect translation:  * Example request: ``` <par>The firm said it had been </par><par> conducting an internal investigation.</par> ```  * Example response: ``` <par>Die Firma sagte, es sei eine gute Idee gewesen.</par><par> Durchführung einer internen Untersuchung.</par> ```   As this can lead to bad translations, this type of structure should either be avoided, or the `non_splitting_tags` parameter should be set. The following example shows the same call, with the parameter set to `par`:  * Example request: ``` <par>The firm said it had been </par><par> conducting an internal investigation.</par> ```  * Example response: ``` <par>Die Firma sagte, dass sie</par><par> eine interne Untersuchung durchgeführt</par><par> habe</par><par>.</par> ```   This time, the sentence is translated as a whole. The XML tags are now considered markup and copied into the translated sentence. As the translation of the words \\\"had been\\\" has moved to another position in the German sentence, the two par tags are duplicated (which is expected here). |  |
**outline_detection** | Option<**String**> | The automatic detection of the XML structure won't yield best results in all XML files. You can disable this automatic mechanism altogether by setting the `outline_detection` parameter to `0` and selecting the tags that should be considered structure tags. This will split sentences using the `splitting_tags` parameter.   In the example below, we achieve the same results as the automatic engine by disabling automatic detection with `outline_detection=0` and setting the parameters manually to `tag_handling=xml`, `split_sentences=nonewlines`,  and `splitting_tags=par,title`.  * Example request:    ```    <document>      <meta>        <title>A document's title</title>      </meta>      <content>        <par>This is the first sentence. Followed by a second one.</par>        <par>This is the third sentence.</par>      </content>    </document>    ```  * Example response:    ```    <document>      <meta>        <title>Der Titel eines Dokuments</title>      </meta>      <content>        <par>Das ist der erste Satz. Gefolgt von einem zweiten.</par>        <par>Dies ist der dritte Satz.</par>      </content>    </document>    ``` While this approach is slightly more complicated, it allows for greater control over the structure of the translation output. |  |
**splitting_tags** | Option<**String**> | Comma-separated list of XML tags which always cause splits.   See the example in the `outline_detection` parameter's description. |  |
**ignore_tags** | Option<**String**> | Comma-separated list of XML tags that indicate text not to be translated.   Use this paramter to ensure that elements in the original text are not altered in the translation (e.g., trademarks, product names) and insert tags into your original text. In the following example, the `ignore_tags` parameter is set to `keep`:  * Example request:    ```    Please open the page <keep>Settings</keep> to configure your system.    ```  * Example response:    ```    Bitte öffnen Sie die Seite <keep>Settings</keep> um Ihr System zu konfigurieren.    ``` |  |

### Return type

[**crate::models::TranslateText200Response**](translateText_200_response.md)

### Authorization

[auth_header](../README.md#auth_header)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

