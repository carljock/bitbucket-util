# Branchrestriction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**links** | Option<[**models::BranchingModelSettingsLinks**](BranchingModelSettingsLinks.md)> |  | [optional]
**id** | Option<**i32**> | The branch restriction status' id. | [optional]
**kind** | **Kind** | The type of restriction that is being applied. (enum: push, delete, force, restrict_merges, require_tasks_to_be_completed, require_approvals_to_merge, require_review_group_approvals_to_merge, require_default_reviewer_approvals_to_merge, require_no_changes_requested, require_passing_builds_to_merge, require_commits_behind, reset_pullrequest_approvals_on_change, smart_reset_pullrequest_approvals, reset_pullrequest_changes_requested_on_change, require_all_dependencies_merged, enforce_merge_checks, allow_auto_merge_when_builds_pass, require_all_comments_resolved) | 
**branch_match_kind** | **BranchMatchKind** | Indicates how the restriction is matched against a branch. The default is `glob`. (enum: branching_model, glob) | 
**branch_type** | Option<**BranchType**> | Apply the restriction to branches of this type. Active when `branch_match_kind` is `branching_model`. The branch type will be calculated using the branching model configured for the repository. (enum: feature, bugfix, release, hotfix, development, production) | [optional]
**pattern** | **String** | Apply the restriction to branches that match this pattern. Active when `branch_match_kind` is `glob`. Will be empty when `branch_match_kind` is `branching_model`. | 
**value** | Option<**i32**> | Value with kind-specific semantics:  * `require_approvals_to_merge` uses it to require a minimum number of approvals on a PR.  * `require_default_reviewer_approvals_to_merge` uses it to require a minimum number of approvals from default reviewers on a PR.  * `require_passing_builds_to_merge` uses it to require a minimum number of passing builds.  * `require_commits_behind` uses it to require the current branch is up to a maximum number of commits behind it destination. | [optional]
**users** | Option<[**Vec<models::Account>**](Account.md)> |  | [optional]
**groups** | Option<[**Vec<models::Group>**](Group.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


