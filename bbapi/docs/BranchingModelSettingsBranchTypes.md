# BranchingModelSettingsBranchTypes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Whether the branch type is enabled or not. A disabled branch type may contain an invalid `prefix`. | [optional]
**kind** | **Kind** | The kind of the branch type. (enum: feature, bugfix, release, hotfix) | 
**prefix** | Option<**String**> | The prefix for this branch type. A branch with this prefix will be classified as per `kind`. The `prefix` of an enabled branch type must be a valid branch prefix.Additionally, it cannot be blank, empty or `null`. The `prefix` for a disabled branch type can be empty or invalid. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


