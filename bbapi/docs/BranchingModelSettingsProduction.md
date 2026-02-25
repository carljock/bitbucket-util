# BranchingModelSettingsProduction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_valid** | Option<**bool**> | Indicates if the configured branch is valid, that is, if the configured branch actually exists currently. Is always `true` when `use_mainbranch` is `true` (even if the main branch does not exist). This field is read-only. This field is ignored when updating/creating settings. | [optional]
**name** | Option<**String**> | The configured branch. It must be `null` when `use_mainbranch` is `true`. Otherwise it must be a non-empty value. It is possible for the configured branch to not exist (e.g. it was deleted after the settings are set). In this case `is_valid` will be `false`. The branch must exist when updating/setting the `name` or an error will occur. | [optional]
**use_mainbranch** | Option<**bool**> | Indicates if the setting points at an explicit branch (`false`) or tracks the main branch (`true`). When `true` the `name` must be `null` or not provided. When `false` the `name` must contain a non-empty branch name. | [optional]
**enabled** | Option<**bool**> | Indicates if branch is enabled or not. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


