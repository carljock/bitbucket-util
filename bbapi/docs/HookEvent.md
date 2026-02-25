# HookEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event** | Option<**Event**> | The event identifier. (enum: issue:comment_created, issue:created, issue:updated, pipeline:span_created, project:updated, pullrequest:approved, pullrequest:changes_request_created, pullrequest:changes_request_removed, pullrequest:comment_created, pullrequest:comment_deleted, pullrequest:comment_reopened, pullrequest:comment_resolved, pullrequest:comment_updated, pullrequest:created, pullrequest:fulfilled, pullrequest:push, pullrequest:rejected, pullrequest:unapproved, pullrequest:updated, repo:commit_comment_created, repo:commit_status_created, repo:commit_status_updated, repo:created, repo:deleted, repo:fork, repo:imported, repo:push, repo:transfer, repo:updated) | [optional]
**category** | Option<**String**> | The category this event belongs to. | [optional]
**label** | Option<**String**> | Summary of the webhook event type. | [optional]
**description** | Option<**String**> | More detailed description of the webhook event type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


