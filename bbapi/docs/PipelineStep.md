# PipelineStep

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**uuid** | Option<**String**> | The UUID identifying the step. | [optional]
**started_on** | Option<**String**> | The timestamp when the step execution was started. This is not set when the step hasn't executed yet. | [optional]
**completed_on** | Option<**String**> | The timestamp when the step execution was completed. This is not set if the step is still in progress. | [optional]
**state** | Option<[**models::PipelineStepState**](PipelineStepState.md)> |  | [optional]
**image** | Option<[**models::PipelineImage**](PipelineImage.md)> |  | [optional]
**setup_commands** | Option<[**Vec<models::PipelineCommand>**](PipelineCommand.md)> | The list of commands that are executed as part of the setup phase of the build. These commands are executed outside the build container. | [optional]
**script_commands** | Option<[**Vec<models::PipelineCommand>**](PipelineCommand.md)> | The list of build commands. These commands are executed in the build container. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


