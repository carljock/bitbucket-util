# PipelineRunnerState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**status** | Option<**Status**> | The current status of the runner. (enum: UNREGISTERED, ONLINE, OFFLINE, DISABLED, ENABLED, UNHEALTHY) | [optional]
**version** | Option<[**models::PipelineRunnerVersion**](PipelineRunnerVersion.md)> |  | [optional]
**updated_on** | Option<**String**> | The timestamp when the runner state was last updated. | [optional]
**cordoned** | Option<**bool**> | Whether the runner is cordoned (prevented from accepting new steps). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


