# Pullrequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**links** | Option<[**models::PullRequestLinks**](PullRequestLinks.md)> |  | [optional]
**id** | Option<**i32**> | The pull request's unique ID. Note that pull request IDs are only unique within their associated repository. | [optional]
**title** | Option<**String**> | Title of the pull request. | [optional]
**rendered** | Option<[**models::RenderedPullRequestMarkup**](RenderedPullRequestMarkup.md)> |  | [optional]
**summary** | Option<[**models::CommentContent**](CommentContent.md)> |  | [optional]
**state** | Option<**State**> | The pull request's current status. (enum: OPEN, DRAFT, QUEUED, MERGED, DECLINED, SUPERSEDED) | [optional]
**author** | Option<[**models::Account**](Account.md)> |  | [optional]
**source** | Option<[**models::PullrequestEndpoint**](PullrequestEndpoint.md)> |  | [optional]
**destination** | Option<[**models::PullrequestEndpoint**](PullrequestEndpoint.md)> |  | [optional]
**merge_commit** | Option<[**models::PullRequestCommit1**](PullRequestCommit1.md)> |  | [optional]
**comment_count** | Option<**i32**> | The number of comments for a specific pull request. | [optional]
**task_count** | Option<**i32**> | The number of open tasks for a specific pull request. | [optional]
**close_source_branch** | Option<**bool**> | A boolean flag indicating if merging the pull request closes the source branch. | [optional]
**closed_by** | Option<[**models::Account**](Account.md)> |  | [optional]
**reason** | Option<**String**> | Explains why a pull request was declined. This field is only applicable to pull requests in rejected state. | [optional]
**created_on** | Option<**String**> | The ISO8601 timestamp the request was created. | [optional]
**updated_on** | Option<**String**> | The ISO8601 timestamp the request was last updated. | [optional]
**reviewers** | Option<[**Vec<models::Account>**](Account.md)> | The list of users that were added as reviewers on this pull request when it was created. For performance reasons, the API only includes this list on a pull request's `self` URL. | [optional]
**participants** | Option<[**Vec<models::Participant>**](Participant.md)> |         The list of users that are collaborating on this pull request.         Collaborators are user that:          * are added to the pull request as a reviewer (part of the reviewers           list)         * are not explicit reviewers, but have commented on the pull request         * are not explicit reviewers, but have approved the pull request          Each user is wrapped in an object that indicates the user's role and         whether they have approved the pull request. For performance reasons,         the API only returns this list when an API requests a pull request by         id.          | [optional]
**draft** | Option<**bool**> | A boolean flag indicating whether the pull request is a draft. | [optional]
**queued** | Option<**bool**> | A boolean flag indicating whether the pull request is queued | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


