# PullrequestMergeParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**message** | Option<**String**> | The commit message that will be used on the resulting commit. Note that the size of the message is limited to 128 KiB. | [optional]
**close_source_branch** | Option<**bool**> | Whether the source branch should be deleted. If this is not provided, we fallback to the value used when the pull request was created, which defaults to False | [optional]
**merge_strategy** | Option<**MergeStrategy**> | The merge strategy that will be used to merge the pull request. (enum: merge_commit, squash, fast_forward, squash_fast_forward, rebase_fast_forward, rebase_merge) | [optional][default to MergeCommit]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


