# PipelineRefTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ref_type** | Option<**RefType**> | The type of reference (branch/tag). (enum: branch, tag, named_branch, bookmark) | [optional]
**ref_name** | Option<**String**> | The name of the reference. | [optional]
**commit** | Option<[**models::Commit**](Commit.md)> |  | [optional]
**selector** | Option<[**models::PipelineSelector**](PipelineSelector.md)> |  | [optional]
**r#type** | Option<**Type**> | Type discriminator for pipeline_ref_target (enum: pipeline_ref_target) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


