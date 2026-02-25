# PullRequestBranch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**merge_strategies** | Option<**Vec<MergeStrategies>**> | Available merge strategies, when this endpoint is the destination of the pull request. (enum: merge_commit, squash, fast_forward, squash_fast_forward, rebase_fast_forward, rebase_merge) | [optional]
**default_merge_strategy** | Option<**String**> | The default merge strategy, when this endpoint is the destination of the pull request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


