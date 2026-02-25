# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**links** | Option<[**models::GroupLinks**](GroupLinks.md)> |  | [optional]
**owner** | Option<[**models::Account**](Account.md)> |  | [optional]
**workspace** | Option<[**models::Workspace**](Workspace.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**slug** | Option<**String**> | The \"sluggified\" version of the group's name. This contains only ASCII characters and can therefore be slightly different than the name | [optional]
**full_slug** | Option<**String**> | The concatenation of the workspace's slug and the group's slug, separated with a colon (e.g. `acme:developers`)  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


