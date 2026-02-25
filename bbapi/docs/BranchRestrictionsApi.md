# \BranchRestrictionsApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**repositories_workspace_repo_slug_branch_restrictions_get**](BranchRestrictionsApi.md#repositories_workspace_repo_slug_branch_restrictions_get) | **GET** /repositories/{workspace}/{repo_slug}/branch-restrictions | List branch restrictions
[**repositories_workspace_repo_slug_branch_restrictions_id_delete**](BranchRestrictionsApi.md#repositories_workspace_repo_slug_branch_restrictions_id_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/branch-restrictions/{id} | Delete a branch restriction rule
[**repositories_workspace_repo_slug_branch_restrictions_id_get**](BranchRestrictionsApi.md#repositories_workspace_repo_slug_branch_restrictions_id_get) | **GET** /repositories/{workspace}/{repo_slug}/branch-restrictions/{id} | Get a branch restriction rule
[**repositories_workspace_repo_slug_branch_restrictions_id_put**](BranchRestrictionsApi.md#repositories_workspace_repo_slug_branch_restrictions_id_put) | **PUT** /repositories/{workspace}/{repo_slug}/branch-restrictions/{id} | Update a branch restriction rule
[**repositories_workspace_repo_slug_branch_restrictions_post**](BranchRestrictionsApi.md#repositories_workspace_repo_slug_branch_restrictions_post) | **POST** /repositories/{workspace}/{repo_slug}/branch-restrictions | Create a branch restriction rule



## repositories_workspace_repo_slug_branch_restrictions_get

> models::PaginatedBranchrestrictions repositories_workspace_repo_slug_branch_restrictions_get(repo_slug, workspace, kind, pattern)
List branch restrictions

Returns a paginated list of all branch restrictions on the repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**kind** | Option<**String**> | Branch restrictions of this type |  |
**pattern** | Option<**String**> | Branch restrictions applied to branches of this pattern |  |

### Return type

[**models::PaginatedBranchrestrictions**](paginated_branchrestrictions.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_branch_restrictions_id_delete

> repositories_workspace_repo_slug_branch_restrictions_id_delete(id, repo_slug, workspace)
Delete a branch restriction rule

Deletes an existing branch restriction rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The restriction rule's id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_branch_restrictions_id_get

> models::Branchrestriction repositories_workspace_repo_slug_branch_restrictions_id_get(id, repo_slug, workspace)
Get a branch restriction rule

Returns a specific branch restriction rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The restriction rule's id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Branchrestriction**](branchrestriction.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_branch_restrictions_id_put

> models::Branchrestriction repositories_workspace_repo_slug_branch_restrictions_id_put(id, repo_slug, workspace, branchrestriction)
Update a branch restriction rule

Updates an existing branch restriction rule.  Fields not present in the request body are ignored.  See [`POST`](/cloud/bitbucket/rest/api-group-branch-restrictions/#api-repositories-workspace-repo-slug-branch-restrictions-post) for details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The restriction rule's id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**branchrestriction** | [**Branchrestriction**](Branchrestriction.md) | The new version of the existing rule | [required] |

### Return type

[**models::Branchrestriction**](branchrestriction.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_branch_restrictions_post

> models::Branchrestriction repositories_workspace_repo_slug_branch_restrictions_post(repo_slug, workspace, branchrestriction)
Create a branch restriction rule

Creates a new branch restriction rule for a repository.  `kind` describes what will be restricted. Allowed values include: `push`, `force`, `delete`, `restrict_merges`, `require_tasks_to_be_completed`, `require_approvals_to_merge`, `require_default_reviewer_approvals_to_merge`, `require_no_changes_requested`, `require_passing_builds_to_merge`, `require_commits_behind`, `reset_pullrequest_approvals_on_change`, `smart_reset_pullrequest_approvals`, `reset_pullrequest_changes_requested_on_change`, `require_all_dependencies_merged`, `enforce_merge_checks`, and `allow_auto_merge_when_builds_pass`.  Different kinds of branch restrictions have different requirements:  * `push` and `restrict_merges` require `users` and `groups` to be   specified. Empty lists are allowed, in which case permission is   denied for everybody.  The restriction applies to all branches that match. There are two ways to match a branch. It is configured in `branch_match_kind`:  1. `glob`: Matches a branch against the `pattern`. A `'*'` in    `pattern` will expand to match zero or more characters, and every    other character matches itself. For example, `'foo*'` will match    `'foo'` and `'foobar'`, but not `'barfoo'`. `'*'` will match all    branches. 2. `branching_model`: Matches a branch against the repository's    branching model. The `branch_type` controls the type of branch    to match. Allowed values include: `production`, `development`,    `bugfix`, `release`, `feature` and `hotfix`.  The combination of `kind` and match must be unique. This means that two `glob` restrictions in a repository cannot have the same `kind` and `pattern`. Additionally, two `branching_model` restrictions in a repository cannot have the same `kind` and `branch_type`.  `users` and `groups` are lists of users and groups that are except from the restriction. They can only be configured in `push` and `restrict_merges` restrictions. The `push` restriction stops a user pushing to matching branches unless that user is in `users` or is a member of a group in `groups`. The `restrict_merges` stops a user merging pull requests to matching branches unless that user is in `users` or is a member of a group in `groups`. Adding new users or groups to an existing restriction should be done via `PUT`.  Note that branch restrictions with overlapping matchers is allowed, but the resulting behavior may be surprising.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**branchrestriction** | [**Branchrestriction**](Branchrestriction.md) | The new rule | [required] |

### Return type

[**models::Branchrestriction**](branchrestriction.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

