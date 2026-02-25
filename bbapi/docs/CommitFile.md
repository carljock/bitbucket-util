# CommitFile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**path** | Option<**String**> | The path in the repository | [optional]
**commit** | Option<[**models::Commit**](Commit.md)> |  | [optional]
**attributes** | Option<**Attributes**> |  (enum: link, executable, subrepository, binary, lfs) | [optional]
**escaped_path** | Option<**String**> | The escaped version of the path as it appears in a diff. If the path does not require escaping this will be the same as path. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


