# PaginatedPipelineVariables

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number of the current results. This is an optional element that is not provided in all responses. | [optional]
**values** | Option<[**Vec<models::PipelineVariable>**](PipelineVariable.md)> | The values of the current page. | [optional]
**size** | Option<**i32**> | Total number of objects in the response. This is an optional element that is not provided in all responses, as it can be expensive to compute. | [optional]
**pagelen** | Option<**i32**> | Current number of objects on the existing page. The default value is 10 with 100 being the maximum allowed value. Individual APIs may enforce different values. | [optional]
**next** | Option<**String**> | Link to the next page if it exists. The last page of a collection does not have this value. Use this link to navigate the result set and refrain from constructing your own URLs. | [optional]
**previous** | Option<**String**> | Link to previous page if it exists. A collections first page does not have this value. This is an optional element that is not provided in all responses. Some result sets strictly support forward navigation and never provide previous links. Clients must anticipate that backwards navigation is not always available. Use this link to navigate the result set and refrain from constructing your own URLs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


