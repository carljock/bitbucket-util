# CommentInline

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from** | Option<**i32**> | The comment's anchor line in the old version of the file. If the comment is a multi-line comment, this is the ending line number in the old version of the file. | [optional]
**to** | Option<**i32**> | The comment's anchor line in the new version of the file. If the comment is a multi-line comment, this is the ending line number in the new version of the file. | [optional]
**start_from** | Option<**i32**> | The starting line number in the old version of the file, if the comment is a multi-line comment. This is null otherwise. | [optional]
**start_to** | Option<**i32**> | The starting line number in the new version of the file, if the comment is a multi-line comment. This is null otherwise. | [optional]
**path** | **String** | The path of the file this comment is anchored to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


