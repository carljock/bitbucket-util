# GpgAccountKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**owner** | Option<[**models::Account**](Account.md)> |  | [optional]
**key** | Option<**String**> | The GPG key value in X format. | [optional]
**key_id** | Option<**String**> | The unique identifier for the GPG key | [optional]
**fingerprint** | Option<**String**> | The GPG key fingerprint. | [optional]
**parent_fingerprint** | Option<**String**> | The fingerprint of the parent key. This value is null unless the current key is a subkey. | [optional]
**comment** | Option<**String**> | The comment parsed from the GPG key (if present) | [optional]
**name** | Option<**String**> | The user-defined label for the GPG key | [optional]
**expires_on** | Option<**String**> |  | [optional]
**created_on** | Option<**String**> |  | [optional]
**added_on** | Option<**String**> |  | [optional]
**last_used** | Option<**String**> |  | [optional]
**subkeys** | Option<[**HashSet<models::GpgAccountKey>**](GPGAccountKey.md)> |  | [optional]
**links** | Option<[**models::BranchingModelSettingsLinks**](BranchingModelSettingsLinks.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


