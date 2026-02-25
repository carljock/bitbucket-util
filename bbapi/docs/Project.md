# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**links** | Option<[**models::ProjectLinks**](ProjectLinks.md)> |  | [optional]
**uuid** | Option<**String**> | The project's immutable id. | [optional]
**key** | Option<**String**> | The project's key. | [optional]
**owner** | Option<[**models::Team**](Team.md)> |  | [optional]
**name** | Option<**String**> | The name of the project. | [optional]
**description** | Option<**String**> |  | [optional]
**is_private** | Option<**bool**> |  Indicates whether the project is publicly accessible, or whether it is private to the team and consequently only visible to team members. Note that private projects cannot contain public repositories. | [optional]
**created_on** | Option<**String**> |  | [optional]
**updated_on** | Option<**String**> |  | [optional]
**has_publicly_visible_repos** | Option<**bool**> |  Indicates whether the project contains publicly visible repositories. Note that private projects cannot contain public repositories. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


