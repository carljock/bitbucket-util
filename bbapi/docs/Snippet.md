# Snippet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**id** | Option<**i32**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**scm** | Option<**Scm**> | The DVCS used to store the snippet. (enum: git) | [optional]
**created_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**updated_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**owner** | Option<[**models::Account**](Account.md)> |  | [optional]
**creator** | Option<[**models::Account**](Account.md)> |  | [optional]
**is_private** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


