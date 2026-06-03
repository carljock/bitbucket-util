# SshKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**uuid** | Option<**String**> | The SSH key's immutable ID. | [optional]
**key** | Option<**String**> | The SSH public key value in OpenSSH format. | [optional]
**comment** | Option<**String**> | The comment parsed from the SSH key (if present) | [optional]
**label** | Option<**String**> | The user-defined label for the SSH key | [optional]
**created_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**last_used** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**links** | Option<[**models::BranchingModelSettingsLinks**](BranchingModelSettingsLinks.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


