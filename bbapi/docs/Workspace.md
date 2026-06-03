# Workspace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**links** | Option<[**models::WorkspaceLinks**](WorkspaceLinks.md)> |  | [optional]
**uuid** | Option<**String**> | The workspace's immutable id. | [optional]
**name** | Option<**String**> | The name of the workspace. | [optional]
**slug** | Option<**String**> | The short label that identifies this workspace. | [optional]
**is_private** | Option<**bool**> | Indicates whether the workspace is publicly accessible, or whether it is private to the members and consequently only visible to members. | [optional]
**is_privacy_enforced** | Option<**bool**> | Indicates whether the workspace enforces private content, or whether it allows public content. | [optional]
**forking_mode** | Option<**ForkingMode**> | Controls the rules for forking repositories within this workspace.  * **allow_forks**: unrestricted forking * **internal_only**: prevents forking of private repositories outside the workspace or to public repositories  (enum: allow_forks, internal_only) | [optional]
**created_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**updated_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


