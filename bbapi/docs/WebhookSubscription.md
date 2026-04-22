# WebhookSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**uuid** | Option<**String**> | The webhook's id | [optional]
**url** | Option<**String**> | The URL events get delivered to. | [optional]
**description** | Option<**String**> | A user-defined description of the webhook. | [optional]
**subject_type** | Option<**SubjectType**> | The type of entity. Set to either `repository` or `workspace` based on where the subscription is defined. (enum: repository, workspace) | [optional]
**subject** | Option<**models::Object**> |  | [optional]
**active** | Option<**bool**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**events** | Option<**HashSet<Events>**> | The events this webhook is subscribed to. (enum: issue:comment_created, issue:created, issue:updated, pipeline:span_created, project:updated, pullrequest:approved, pullrequest:changes_request_created, pullrequest:changes_request_removed, pullrequest:comment_created, pullrequest:comment_deleted, pullrequest:comment_reopened, pullrequest:comment_resolved, pullrequest:comment_updated, pullrequest:created, pullrequest:fulfilled, pullrequest:push, pullrequest:rejected, pullrequest:unapproved, pullrequest:updated, repo:commit_comment_created, repo:commit_status_created, repo:commit_status_updated, repo:created, repo:deleted, repo:fork, repo:imported, repo:push, repo:transfer, repo:updated) | [optional]
**secret_set** | Option<**bool**> | Indicates whether or not the hook has an associated secret. It is not possible to see the hook's secret. This field is ignored during updates. | [optional]
**secret** | Option<**String**> | The secret to associate with the hook. The secret is never returned via the API. As such, this field is only used during updates. The secret can be set to `null` or \"\" to remove the secret (or create a hook with no secret). Leaving out the secret field during updates will leave the secret unchanged. Leaving out the secret during creation will create a hook with no secret. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


