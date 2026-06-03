# PipelineRunner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**uuid** | Option<**String**> | The UUID identifying the runner. | [optional]
**name** | Option<**String**> | The name of the runner. | [optional]
**labels** | Option<**Vec<String>**> | Labels assigned to the runner for identification and routing. | [optional]
**state** | Option<[**models::PipelineRunnerState**](PipelineRunnerState.md)> |  | [optional]
**created_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The timestamp when the runner was created. | [optional]
**updated_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The timestamp when the runner was last updated. | [optional]
**oauth_client** | Option<[**models::PipelineRunnerOauthClient**](PipelineRunnerOauthClient.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


