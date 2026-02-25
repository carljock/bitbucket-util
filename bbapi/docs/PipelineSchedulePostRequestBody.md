# PipelineSchedulePostRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**target** | [**models::RequestBodyForPipelineSchedulePostRequestTarget**](RequestBodyForPipelineSchedulePOSTRequestTarget.md) |  | 
**enabled** | Option<**bool**> | Whether the schedule is enabled. | [optional]
**cron_pattern** | **String** | The cron expression with second precision (7 fields) that the schedule applies. For example, for expression: 0 0 12 * * ? *, will execute at 12pm UTC every day. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


