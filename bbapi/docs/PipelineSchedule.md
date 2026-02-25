# PipelineSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**uuid** | Option<**String**> | The UUID identifying the schedule. | [optional]
**enabled** | Option<**bool**> | Whether the schedule is enabled. | [optional]
**target** | Option<[**models::PipelineRefTarget**](PipelineRefTarget.md)> |  | [optional]
**cron_pattern** | Option<**String**> | The cron expression with second precision (7 fields) that the schedule applies. For example, for expression: 0 0 12 * * ? *, will execute at 12pm UTC every day. | [optional]
**created_on** | Option<**String**> | The timestamp when the schedule was created. | [optional]
**updated_on** | Option<**String**> | The timestamp when the schedule was updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


