# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**links** | Option<[**models::UserLinks**](UserLinks.md)> |  | [optional]
**created_on** | Option<**String**> |  | [optional]
**display_name** | Option<**String**> |  | [optional]
**uuid** | Option<**String**> |  | [optional]
**account_id** | Option<**String**> | The user's Atlassian account ID. | [optional]
**account_status** | Option<**String**> | The status of the account. Currently the only possible value is \"active\", but more values may be added in the future. | [optional]
**has_2fa_enabled** | Option<**bool**> |  | [optional]
**nickname** | Option<**String**> | Account name defined by the owner. Should be used instead of the \"username\" field. Note that \"nickname\" cannot be used in place of \"username\" in URLs and queries, as \"nickname\" is not guaranteed to be unique. | [optional]
**is_staff** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


