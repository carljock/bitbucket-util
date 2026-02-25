# \PullrequestsApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_pullrequests_for_commit**](PullrequestsApi.md#get_pullrequests_for_commit) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit}/pullrequests | List pull requests that contain a commit
[**repositories_workspace_repo_slug_default_reviewers_get**](PullrequestsApi.md#repositories_workspace_repo_slug_default_reviewers_get) | **GET** /repositories/{workspace}/{repo_slug}/default-reviewers | List default reviewers
[**repositories_workspace_repo_slug_default_reviewers_target_username_delete**](PullrequestsApi.md#repositories_workspace_repo_slug_default_reviewers_target_username_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/default-reviewers/{target_username} | Remove a user from the default reviewers
[**repositories_workspace_repo_slug_default_reviewers_target_username_get**](PullrequestsApi.md#repositories_workspace_repo_slug_default_reviewers_target_username_get) | **GET** /repositories/{workspace}/{repo_slug}/default-reviewers/{target_username} | Get a default reviewer
[**repositories_workspace_repo_slug_default_reviewers_target_username_put**](PullrequestsApi.md#repositories_workspace_repo_slug_default_reviewers_target_username_put) | **PUT** /repositories/{workspace}/{repo_slug}/default-reviewers/{target_username} | Add a user to the default reviewers
[**repositories_workspace_repo_slug_effective_default_reviewers_get**](PullrequestsApi.md#repositories_workspace_repo_slug_effective_default_reviewers_get) | **GET** /repositories/{workspace}/{repo_slug}/effective-default-reviewers | List effective default reviewers
[**repositories_workspace_repo_slug_pullrequests_activity_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_activity_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/activity | List a pull request activity log
[**repositories_workspace_repo_slug_pullrequests_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests | List pull requests
[**repositories_workspace_repo_slug_pullrequests_post**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_post) | **POST** /repositories/{workspace}/{repo_slug}/pullrequests | Create a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_activity_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_activity_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/activity | List a pull request activity log
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_approve_delete**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_approve_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/approve | Unapprove a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_approve_post**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_approve_post) | **POST** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/approve | Approve a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_delete**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/comments/{comment_id} | Delete a comment on a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/comments/{comment_id} | Get a comment on a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_put**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_put) | **PUT** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/comments/{comment_id} | Update a comment on a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_resolve_delete**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_resolve_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/comments/{comment_id}/resolve | Reopen a comment thread
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_resolve_post**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_resolve_post) | **POST** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/comments/{comment_id}/resolve | Resolve a comment thread
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/comments | List comments on a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_post**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_post) | **POST** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/comments | Create a comment on a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_commits_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_commits_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/commits | List commits on a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_decline_post**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_decline_post) | **POST** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/decline | Decline a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_diff_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_diff_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/diff | List changes in a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_diffstat_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_diffstat_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/diffstat | Get the diff stat for a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id} | Get a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_merge_post**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_merge_post) | **POST** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/merge | Merge a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_merge_task_status_task_id_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_merge_task_status_task_id_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/merge/task-status/{task_id} | Get the merge task status for a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_patch_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_patch_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/patch | Get the patch for a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_put**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_put) | **PUT** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id} | Update a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_request_changes_delete**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_request_changes_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/request-changes | Remove change request for a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_request_changes_post**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_request_changes_post) | **POST** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/request-changes | Request changes for a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_statuses_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_statuses_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/statuses | List commit statuses for a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/tasks | List tasks on a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_post**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_post) | **POST** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/tasks | Create a task on a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_delete**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/tasks/{task_id} | Delete a task on a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_get**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/tasks/{task_id} | Get a task on a pull request
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_put**](PullrequestsApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_put) | **PUT** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/tasks/{task_id} | Update a task on a pull request
[**workspaces_workspace_pullrequests_selected_user_get**](PullrequestsApi.md#workspaces_workspace_pullrequests_selected_user_get) | **GET** /workspaces/{workspace}/pullrequests/{selected_user} | List workspace pull requests for a user



## get_pullrequests_for_commit

> models::PaginatedPullrequests get_pullrequests_for_commit(workspace, repo_slug, commit, page, pagelen)
List pull requests that contain a commit

Returns a paginated list of all pull requests as part of which this commit was reviewed. Pull Request Commit Links app must be installed first before using this API; installation automatically occurs when 'Go to pull request' is clicked from the web interface for a commit's details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces | [required] |
**repo_slug** | **String** | The repository; either the UUID in curly braces, or the slug | [required] |
**commit** | **String** | The SHA1 of the commit | [required] |
**page** | Option<**i32**> | Which page to retrieve |  |[default to 1]
**pagelen** | Option<**i32**> | How many pull requests to retrieve per page |  |[default to 30]

### Return type

[**models::PaginatedPullrequests**](paginated_pullrequests.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_default_reviewers_get

> models::PaginatedAccounts repositories_workspace_repo_slug_default_reviewers_get(repo_slug, workspace)
List default reviewers

Returns the repository's default reviewers.  These are the users that are automatically added as reviewers on every new pull request that is created. To obtain the repository's default reviewers as well as the default reviewers inherited from the project, use the [effective-default-reveiwers](#api-repositories-workspace-repo-slug-effective-default-reviewers-get) endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedAccounts**](paginated_accounts.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_default_reviewers_target_username_delete

> repositories_workspace_repo_slug_default_reviewers_target_username_delete(repo_slug, target_username, workspace)
Remove a user from the default reviewers

Removes a default reviewer from the repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**target_username** | **String** | This can either be the username or the UUID of the default reviewer, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_default_reviewers_target_username_get

> models::Account repositories_workspace_repo_slug_default_reviewers_target_username_get(repo_slug, target_username, workspace)
Get a default reviewer

Returns the specified reviewer.  This can be used to test whether a user is among the repository's default reviewers list. A 404 indicates that that specified user is not a default reviewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**target_username** | **String** | This can either be the username or the UUID of the default reviewer, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Account**](account.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_default_reviewers_target_username_put

> models::Account repositories_workspace_repo_slug_default_reviewers_target_username_put(repo_slug, target_username, workspace)
Add a user to the default reviewers

Adds the specified user to the repository's list of default reviewers.  This method is idempotent. Adding a user a second time has no effect.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**target_username** | **String** | This can either be the username or the UUID of the default reviewer, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Account**](account.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_effective_default_reviewers_get

> models::PaginatedDefaultReviewerAndType repositories_workspace_repo_slug_effective_default_reviewers_get(repo_slug, workspace)
List effective default reviewers

Returns the repository's effective default reviewers. This includes both default reviewers defined at the repository level as well as those inherited from its project.  These are the users that are automatically added as reviewers on every new pull request that is created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedDefaultReviewerAndType**](paginated_default_reviewer_and_type.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_activity_get

> repositories_workspace_repo_slug_pullrequests_activity_get(repo_slug, workspace)
List a pull request activity log

Returns a paginated list of the pull request's activity log.  This handler serves both a v20 and internal endpoint. The v20 endpoint returns reviewer comments, updates, approvals and request changes. The internal endpoint includes those plus tasks and attachments.  Comments created on a file or a line of code have an inline property.  Comment example: ``` {     \"pagelen\": 20,     \"values\": [         {             \"comment\": {                 \"links\": {                     \"self\": {                         \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695/comments/118571088\"                     },                     \"html\": {                         \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695/_/diff#comment-118571088\"                     }                 },                 \"deleted\": false,                 \"pullrequest\": {                     \"type\": \"pullrequest\",                     \"id\": 5695,                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695\"                         }                     },                     \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\"                 },                 \"content\": {                     \"raw\": \"inline with to a dn from lines\",                     \"markup\": \"markdown\",                     \"html\": \"<p>inline with to a dn from lines</p>\",                     \"type\": \"rendered\"                 },                 \"created_on\": \"2019-09-27T00:33:46.039178+00:00\",                 \"user\": {                     \"display_name\": \"Name Lastname\",                     \"uuid\": \"{}\",                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/users/%7B%7D\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/%7B%7D/\"                         },                         \"avatar\": {                             \"href\": \"https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/:/128\"                         }                     },                     \"type\": \"user\",                     \"nickname\": \"Name\",                     \"account_id\": \"\"                 },                 \"created_on\": \"2019-09-27T00:33:46.039178+00:00\",                 \"user\": {                     \"display_name\": \"Name Lastname\",                     \"uuid\": \"{}\",                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/users/%7B%7D\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/%7B%7D/\"                         },                         \"avatar\": {                             \"href\": \"https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/:/128\"                         }                     },                     \"type\": \"user\",                     \"nickname\": \"Name\",                     \"account_id\": \"\"                 },                 \"updated_on\": \"2019-09-27T00:33:46.055384+00:00\",                 \"inline\": {                     \"context_lines\": \"\",                     \"to\": null,                     \"path\": \"\",                     \"outdated\": false,                     \"from\": 211                 },                 \"type\": \"pullrequest_comment\",                 \"id\": 118571088             },             \"pull_request\": {                 \"type\": \"pullrequest\",                 \"id\": 5695,                 \"links\": {                     \"self\": {                         \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695\"                     },                     \"html\": {                         \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695\"                     }                 },                 \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\"             }         }     ] } ```  Updates include a state property of OPEN, MERGED, or DECLINED.  Update example: ``` {     \"pagelen\": 20,     \"values\": [         {             \"update\": {                 \"description\": \"\",                 \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\",                 \"destination\": {                     \"commit\": {                         \"type\": \"commit\",                         \"hash\": \"6a2c16e4a152\",                         \"links\": {                             \"self\": {                                 \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/commit/6a2c16e4a152\"                             },                             \"html\": {                                 \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/commits/6a2c16e4a152\"                             }                         }                     },                     \"branch\": {                         \"name\": \"master\"                     },                     \"repository\": {                         \"name\": \"Atlaskit-MK-2\",                         \"type\": \"repository\",                         \"full_name\": \"atlassian/atlaskit-mk-2\",                         \"links\": {                             \"self\": {                                 \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2\"                             },                             \"html\": {                                 \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2\"                             },                             \"avatar\": {                                 \"href\": \"https://bytebucket.org/ravatar/%7B%7D?ts=js\"                             }                         },                         \"uuid\": \"{}\"                     }                 },                 \"reason\": \"\",                 \"source\": {                     \"commit\": {                         \"type\": \"commit\",                         \"hash\": \"728c8bad1813\",                         \"links\": {                             \"self\": {                                 \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/commit/728c8bad1813\"                             },                             \"html\": {                                 \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/commits/728c8bad1813\"                             }                         }                     },                     \"branch\": {                         \"name\": \"username/NONE-add-onClick-prop-for-accessibility\"                     },                     \"repository\": {                         \"name\": \"Atlaskit-MK-2\",                         \"type\": \"repository\",                         \"full_name\": \"atlassian/atlaskit-mk-2\",                         \"links\": {                             \"self\": {                                 \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2\"                             },                             \"html\": {                                 \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2\"                             },                             \"avatar\": {                                 \"href\": \"https://bytebucket.org/ravatar/%7B%7D?ts=js\"                             }                         },                         \"uuid\": \"{}\"                     }                 },                 \"state\": \"OPEN\",                 \"author\": {                     \"display_name\": \"Name Lastname\",                     \"uuid\": \"{}\",                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/users/%7B%7D\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/%7B%7D/\"                         },                         \"avatar\": {                             \"href\": \"https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/:/128\"                         }                     },                     \"type\": \"user\",                     \"nickname\": \"Name\",                     \"account_id\": \"\"                 },                 \"date\": \"2019-05-10T06:48:25.305565+00:00\"             },             \"pull_request\": {                 \"type\": \"pullrequest\",                 \"id\": 5695,                 \"links\": {                     \"self\": {                         \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695\"                     },                     \"html\": {                         \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695\"                     }                 },                 \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\"             }         }     ] } ```  Approval example: ``` {     \"pagelen\": 20,     \"values\": [         {             \"approval\": {                 \"date\": \"2019-09-27T00:37:19.849534+00:00\",                 \"pullrequest\": {                     \"type\": \"pullrequest\",                     \"id\": 5695,                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695\"                         }                     },                     \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\"                 },                 \"user\": {                     \"display_name\": \"Name Lastname\",                     \"uuid\": \"{}\",                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/users/%7B%7D\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/%7B%7D/\"                         },                         \"avatar\": {                             \"href\": \"https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/:/128\"                         }                     },                     \"type\": \"user\",                     \"nickname\": \"Name\",                     \"account_id\": \"\"                 }             },             \"pull_request\": {                 \"type\": \"pullrequest\",                 \"id\": 5695,                 \"links\": {                     \"self\": {                         \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695\"                     },                     \"html\": {                         \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695\"                     }                 },                 \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\"             }         }     ] } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## repositories_workspace_repo_slug_pullrequests_get

> models::PaginatedPullrequests repositories_workspace_repo_slug_pullrequests_get(repo_slug, workspace, state)
List pull requests

Returns all pull requests on the specified repository.  By default only open pull requests are returned. This can be controlled using the `state` query parameter. To retrieve pull requests that are in one of multiple states, repeat the `state` parameter for each individual state.  This endpoint also supports filtering and sorting of the results. See [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**state** | Option<**String**> | Only return pull requests that are in this state. This parameter can be repeated. |  |

### Return type

[**models::PaginatedPullrequests**](paginated_pullrequests.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_post

> models::Pullrequest repositories_workspace_repo_slug_pullrequests_post(repo_slug, workspace, pullrequest)
Create a pull request

Creates a new pull request where the destination repository is this repository and the author is the authenticated user.  The minimum required fields to create a pull request are `title` and `source`, specified by a branch name.  ``` curl https://api.bitbucket.org/2.0/repositories/my-workspace/my-repository/pullrequests \\     -u my-username:my-password \\     --request POST \\     --header 'Content-Type: application/json' \\     --data '{         \"title\": \"My Title\",         \"source\": {             \"branch\": {                 \"name\": \"staging\"             }         }     }' ```  If the pull request's `destination` is not specified, it will default to the `repository.mainbranch`. To open a pull request to a different branch, say from a feature branch to a staging branch, specify a `destination` (same format as the `source`):  ``` {     \"title\": \"My Title\",     \"source\": {         \"branch\": {             \"name\": \"my-feature-branch\"         }     },     \"destination\": {         \"branch\": {             \"name\": \"staging\"         }     } } ```  Reviewers can be specified by adding an array of user objects as the `reviewers` property.  ``` {     \"title\": \"My Title\",     \"source\": {         \"branch\": {             \"name\": \"my-feature-branch\"         }     },     \"reviewers\": [         {             \"uuid\": \"{504c3b62-8120-4f0c-a7bc-87800b9d6f70}\"         }     ] } ```  Other fields:  * `description` - a string * `close_source_branch` - boolean that specifies if the source branch should be closed upon merging * `draft` - boolean that specifies whether the pull request is a draft

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**pullrequest** | Option<[**Pullrequest**](Pullrequest.md)> | The new pull request.  The request URL you POST to becomes the destination repository URL. For this reason, you must specify an explicit source repository in the request object if you want to pull from a different repository (fork).  Since not all elements are required or even mutable, you only need to include the elements you want to initialize, such as the source branch and the title. |  |

### Return type

[**models::Pullrequest**](pullrequest.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_activity_get

> repositories_workspace_repo_slug_pullrequests_pull_request_id_activity_get(pull_request_id, repo_slug, workspace)
List a pull request activity log

Returns a paginated list of the pull request's activity log.  This handler serves both a v20 and internal endpoint. The v20 endpoint returns reviewer comments, updates, approvals and request changes. The internal endpoint includes those plus tasks and attachments.  Comments created on a file or a line of code have an inline property.  Comment example: ``` {     \"pagelen\": 20,     \"values\": [         {             \"comment\": {                 \"links\": {                     \"self\": {                         \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695/comments/118571088\"                     },                     \"html\": {                         \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695/_/diff#comment-118571088\"                     }                 },                 \"deleted\": false,                 \"pullrequest\": {                     \"type\": \"pullrequest\",                     \"id\": 5695,                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695\"                         }                     },                     \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\"                 },                 \"content\": {                     \"raw\": \"inline with to a dn from lines\",                     \"markup\": \"markdown\",                     \"html\": \"<p>inline with to a dn from lines</p>\",                     \"type\": \"rendered\"                 },                 \"created_on\": \"2019-09-27T00:33:46.039178+00:00\",                 \"user\": {                     \"display_name\": \"Name Lastname\",                     \"uuid\": \"{}\",                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/users/%7B%7D\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/%7B%7D/\"                         },                         \"avatar\": {                             \"href\": \"https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/:/128\"                         }                     },                     \"type\": \"user\",                     \"nickname\": \"Name\",                     \"account_id\": \"\"                 },                 \"created_on\": \"2019-09-27T00:33:46.039178+00:00\",                 \"user\": {                     \"display_name\": \"Name Lastname\",                     \"uuid\": \"{}\",                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/users/%7B%7D\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/%7B%7D/\"                         },                         \"avatar\": {                             \"href\": \"https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/:/128\"                         }                     },                     \"type\": \"user\",                     \"nickname\": \"Name\",                     \"account_id\": \"\"                 },                 \"updated_on\": \"2019-09-27T00:33:46.055384+00:00\",                 \"inline\": {                     \"context_lines\": \"\",                     \"to\": null,                     \"path\": \"\",                     \"outdated\": false,                     \"from\": 211                 },                 \"type\": \"pullrequest_comment\",                 \"id\": 118571088             },             \"pull_request\": {                 \"type\": \"pullrequest\",                 \"id\": 5695,                 \"links\": {                     \"self\": {                         \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695\"                     },                     \"html\": {                         \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695\"                     }                 },                 \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\"             }         }     ] } ```  Updates include a state property of OPEN, MERGED, or DECLINED.  Update example: ``` {     \"pagelen\": 20,     \"values\": [         {             \"update\": {                 \"description\": \"\",                 \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\",                 \"destination\": {                     \"commit\": {                         \"type\": \"commit\",                         \"hash\": \"6a2c16e4a152\",                         \"links\": {                             \"self\": {                                 \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/commit/6a2c16e4a152\"                             },                             \"html\": {                                 \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/commits/6a2c16e4a152\"                             }                         }                     },                     \"branch\": {                         \"name\": \"master\"                     },                     \"repository\": {                         \"name\": \"Atlaskit-MK-2\",                         \"type\": \"repository\",                         \"full_name\": \"atlassian/atlaskit-mk-2\",                         \"links\": {                             \"self\": {                                 \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2\"                             },                             \"html\": {                                 \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2\"                             },                             \"avatar\": {                                 \"href\": \"https://bytebucket.org/ravatar/%7B%7D?ts=js\"                             }                         },                         \"uuid\": \"{}\"                     }                 },                 \"reason\": \"\",                 \"source\": {                     \"commit\": {                         \"type\": \"commit\",                         \"hash\": \"728c8bad1813\",                         \"links\": {                             \"self\": {                                 \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/commit/728c8bad1813\"                             },                             \"html\": {                                 \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/commits/728c8bad1813\"                             }                         }                     },                     \"branch\": {                         \"name\": \"username/NONE-add-onClick-prop-for-accessibility\"                     },                     \"repository\": {                         \"name\": \"Atlaskit-MK-2\",                         \"type\": \"repository\",                         \"full_name\": \"atlassian/atlaskit-mk-2\",                         \"links\": {                             \"self\": {                                 \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2\"                             },                             \"html\": {                                 \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2\"                             },                             \"avatar\": {                                 \"href\": \"https://bytebucket.org/ravatar/%7B%7D?ts=js\"                             }                         },                         \"uuid\": \"{}\"                     }                 },                 \"state\": \"OPEN\",                 \"author\": {                     \"display_name\": \"Name Lastname\",                     \"uuid\": \"{}\",                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/users/%7B%7D\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/%7B%7D/\"                         },                         \"avatar\": {                             \"href\": \"https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/:/128\"                         }                     },                     \"type\": \"user\",                     \"nickname\": \"Name\",                     \"account_id\": \"\"                 },                 \"date\": \"2019-05-10T06:48:25.305565+00:00\"             },             \"pull_request\": {                 \"type\": \"pullrequest\",                 \"id\": 5695,                 \"links\": {                     \"self\": {                         \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695\"                     },                     \"html\": {                         \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695\"                     }                 },                 \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\"             }         }     ] } ```  Approval example: ``` {     \"pagelen\": 20,     \"values\": [         {             \"approval\": {                 \"date\": \"2019-09-27T00:37:19.849534+00:00\",                 \"pullrequest\": {                     \"type\": \"pullrequest\",                     \"id\": 5695,                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695\"                         }                     },                     \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\"                 },                 \"user\": {                     \"display_name\": \"Name Lastname\",                     \"uuid\": \"{}\",                     \"links\": {                         \"self\": {                             \"href\": \"https://api.bitbucket.org/2.0/users/%7B%7D\"                         },                         \"html\": {                             \"href\": \"https://bitbucket.org/%7B%7D/\"                         },                         \"avatar\": {                             \"href\": \"https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/:/128\"                         }                     },                     \"type\": \"user\",                     \"nickname\": \"Name\",                     \"account_id\": \"\"                 }             },             \"pull_request\": {                 \"type\": \"pullrequest\",                 \"id\": 5695,                 \"links\": {                     \"self\": {                         \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/atlaskit-mk-2/pullrequests/5695\"                     },                     \"html\": {                         \"href\": \"https://bitbucket.org/atlassian/atlaskit-mk-2/pull-requests/5695\"                     }                 },                 \"title\": \"username/NONE: small change from onFocus to onClick to handle tabbing through the page and not expand the editor unless a click event triggers it\"             }         }     ] } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
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


## repositories_workspace_repo_slug_pullrequests_pull_request_id_approve_delete

> repositories_workspace_repo_slug_pullrequests_pull_request_id_approve_delete(pull_request_id, repo_slug, workspace)
Unapprove a pull request

Redact the authenticated user's approval of the specified pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
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


## repositories_workspace_repo_slug_pullrequests_pull_request_id_approve_post

> models::Participant repositories_workspace_repo_slug_pullrequests_pull_request_id_approve_post(pull_request_id, repo_slug, workspace)
Approve a pull request

Approve the specified pull request as the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Participant**](participant.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_delete

> repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_delete(comment_id, pull_request_id, repo_slug, workspace)
Delete a comment on a pull request

Deletes a specific pull request comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**pull_request_id** | **i32** | The id of the pull request. | [required] |
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


## repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_get

> models::PullrequestComment repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_get(comment_id, pull_request_id, repo_slug, workspace)
Get a comment on a pull request

Returns a specific pull request comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PullrequestComment**](pullrequest_comment.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_put

> models::PullrequestComment repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_put(comment_id, pull_request_id, repo_slug, workspace, pullrequest_comment)
Update a comment on a pull request

Updates a specific pull request comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**pullrequest_comment** | [**PullrequestComment**](PullrequestComment.md) | The contents of the updated comment. | [required] |

### Return type

[**models::PullrequestComment**](pullrequest_comment.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_resolve_delete

> repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_resolve_delete(comment_id, pull_request_id, repo_slug, workspace)
Reopen a comment thread



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**pull_request_id** | **i32** | The id of the pull request. | [required] |
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


## repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_resolve_post

> models::CommentResolution repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_comment_id_resolve_post(comment_id, pull_request_id, repo_slug, workspace)
Resolve a comment thread



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::CommentResolution**](comment_resolution.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_get

> models::PaginatedPullrequestComments repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_get(pull_request_id, repo_slug, workspace)
List comments on a pull request

Returns a paginated list of the pull request's comments.  This includes both global, inline comments and replies.  The default sorting is oldest to newest and can be overridden with the `sort` query parameter.  This endpoint also supports filtering and sorting of the results. See [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedPullrequestComments**](paginated_pullrequest_comments.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_post

> models::PullrequestComment repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_post(pull_request_id, repo_slug, workspace, pullrequest_comment)
Create a comment on a pull request

Creates a new pull request comment.  Returns the newly created pull request comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**pullrequest_comment** | [**PullrequestComment**](PullrequestComment.md) | The comment object. | [required] |

### Return type

[**models::PullrequestComment**](pullrequest_comment.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_commits_get

> repositories_workspace_repo_slug_pullrequests_pull_request_id_commits_get(pull_request_id, repo_slug, workspace)
List commits on a pull request

Returns a paginated list of the pull request's commits.  These are the commits that are being merged into the destination branch when the pull requests gets accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
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


## repositories_workspace_repo_slug_pullrequests_pull_request_id_decline_post

> models::Pullrequest repositories_workspace_repo_slug_pullrequests_pull_request_id_decline_post(pull_request_id, repo_slug, workspace)
Decline a pull request

Declines the pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Pullrequest**](pullrequest.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_diff_get

> repositories_workspace_repo_slug_pullrequests_pull_request_id_diff_get(pull_request_id, repo_slug, workspace)
List changes in a pull request

Redirects to the [repository diff](/cloud/bitbucket/rest/api-group-commits/#api-repositories-workspace-repo-slug-diff-spec-get) with the revspec that corresponds to the pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_diffstat_get

> repositories_workspace_repo_slug_pullrequests_pull_request_id_diffstat_get(pull_request_id, repo_slug, workspace)
Get the diff stat for a pull request

Redirects to the [repository diffstat](/cloud/bitbucket/rest/api-group-commits/#api-repositories-workspace-repo-slug-diffstat-spec-get) with the revspec that corresponds to the pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_get

> models::Pullrequest repositories_workspace_repo_slug_pullrequests_pull_request_id_get(pull_request_id, repo_slug, workspace)
Get a pull request

Returns the specified pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Pullrequest**](pullrequest.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_merge_post

> models::Pullrequest repositories_workspace_repo_slug_pullrequests_pull_request_id_merge_post(pull_request_id, repo_slug, workspace, r#async, pullrequest_merge_parameters)
Merge a pull request

Merges the pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**r#async** | Option<**bool**> | Default value is false.   When set to true, runs merge asynchronously and immediately returns a 202 with polling link to the task-status API in the Location header.   When set to false, runs merge and waits for it to complete, returning 200 when it succeeds. If the duration of the merge exceeds a timeout threshold, the API returns a 202 with polling link to the task-status API in the Location header. |  |
**pullrequest_merge_parameters** | Option<[**PullrequestMergeParameters**](PullrequestMergeParameters.md)> |  |  |

### Return type

[**models::Pullrequest**](pullrequest.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_merge_task_status_task_id_get

> repositories_workspace_repo_slug_pullrequests_pull_request_id_merge_task_status_task_id_get(pull_request_id, repo_slug, task_id, workspace)
Get the merge task status for a pull request

When merging a pull request takes too long, the client receives a task ID along with a 202 status code. The task ID can be used in a call to this endpoint to check the status of a merge task.  ``` curl -X GET https://api.bitbucket.org/2.0/repositories/atlassian/bitbucket/pullrequests/2286/merge/task-status/<task_id> ```  If the merge task is not yet finished, a PENDING status will be returned.  ``` HTTP/2 200 {     \"task_status\": \"PENDING\",     \"links\": {         \"self\": {             \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/bitbucket/pullrequests/2286/merge/task-status/<task_id>\"         }     } } ```  If the merge was successful, a SUCCESS status will be returned.  ``` HTTP/2 200 {     \"task_status\": \"SUCCESS\",     \"links\": {         \"self\": {             \"href\": \"https://api.bitbucket.org/2.0/repositories/atlassian/bitbucket/pullrequests/2286/merge/task-status/<task_id>\"         }     },     \"merge_result\": <the merged pull request object> } ```  If the merge task failed, an error will be returned.  ``` {     \"type\": \"error\",     \"error\": {         \"message\": \"<error message>\"     } } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**task_id** | **String** | ID of the merge task | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_patch_get

> repositories_workspace_repo_slug_pullrequests_pull_request_id_patch_get(pull_request_id, repo_slug, workspace)
Get the patch for a pull request

Redirects to the [repository patch](/cloud/bitbucket/rest/api-group-commits/#api-repositories-workspace-repo-slug-patch-spec-get) with the revspec that corresponds to pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_put

> models::Pullrequest repositories_workspace_repo_slug_pullrequests_pull_request_id_put(pull_request_id, repo_slug, workspace, pullrequest)
Update a pull request

Mutates the specified pull request.  This can be used to change the pull request's branches or description.  Only open pull requests can be mutated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**pullrequest** | Option<[**Pullrequest**](Pullrequest.md)> | The pull request that is to be updated. |  |

### Return type

[**models::Pullrequest**](pullrequest.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_request_changes_delete

> repositories_workspace_repo_slug_pullrequests_pull_request_id_request_changes_delete(pull_request_id, repo_slug, workspace)
Remove change request for a pull request



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
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


## repositories_workspace_repo_slug_pullrequests_pull_request_id_request_changes_post

> models::Participant repositories_workspace_repo_slug_pullrequests_pull_request_id_request_changes_post(pull_request_id, repo_slug, workspace)
Request changes for a pull request



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Participant**](participant.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_statuses_get

> models::PaginatedCommitstatuses repositories_workspace_repo_slug_pullrequests_pull_request_id_statuses_get(pull_request_id, repo_slug, workspace, q, sort)
List commit statuses for a pull request

Returns all statuses (e.g. build results) for the given pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**q** | Option<**String**> | Query string to narrow down the response as per [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering).  |  |
**sort** | Option<**String**> | Field by which the results should be sorted as per [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering). Defaults to `created_on`.  |  |

### Return type

[**models::PaginatedCommitstatuses**](paginated_commitstatuses.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_get

> models::PaginatedTasks repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_get(pull_request_id, repo_slug, workspace, q, sort, pagelen)
List tasks on a pull request

Returns a paginated list of the pull request's tasks.  This endpoint supports filtering and sorting of the results by the 'task' field. See [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**q** | Option<**String**> |  Query string to narrow down the response. See [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering) for details. |  |
**sort** | Option<**String**> |  Field by which the results should be sorted as per [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering). Defaults to `created_on`.  |  |
**pagelen** | Option<**i32**> |  Current number of objects on the existing page. The default value is 10 with 100 being the maximum allowed value. Individual APIs may enforce different values.  |  |

### Return type

[**models::PaginatedTasks**](paginated_tasks.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_post

> models::PullrequestCommentTask repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_post(pull_request_id, repo_slug, workspace, pullrequest_task_create)
Create a task on a pull request

Creates a new pull request task.  Returns the newly created pull request task.  Tasks can optionally be created in relation to a comment specified by the comment's ID which will cause the task to appear below the comment on a pull request when viewed in Bitbucket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**pullrequest_task_create** | [**PullrequestTaskCreate**](PullrequestTaskCreate.md) | The contents of the task | [required] |

### Return type

[**models::PullrequestCommentTask**](pullrequest_comment_task.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_delete

> repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_delete(pull_request_id, repo_slug, task_id, workspace)
Delete a task on a pull request

Deletes a specific pull request task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**task_id** | **i64** | The ID of the task. | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_get

> models::PullrequestCommentTask repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_get(pull_request_id, repo_slug, task_id, workspace)
Get a task on a pull request

Returns a specific pull request task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**task_id** | **i64** | The ID of the task. | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PullrequestCommentTask**](pullrequest_comment_task.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_put

> models::PullrequestCommentTask repositories_workspace_repo_slug_pullrequests_pull_request_id_tasks_task_id_put(pull_request_id, repo_slug, task_id, workspace, pullrequest_task_update)
Update a task on a pull request

Updates a specific pull request task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request_id** | **i32** | The id of the pull request. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**task_id** | **i64** | The ID of the task. | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**pullrequest_task_update** | [**PullrequestTaskUpdate**](PullrequestTaskUpdate.md) | The updated state and content of the task. | [required] |

### Return type

[**models::PullrequestCommentTask**](pullrequest_comment_task.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_pullrequests_selected_user_get

> models::PaginatedPullrequests workspaces_workspace_pullrequests_selected_user_get(selected_user, workspace, state)
List workspace pull requests for a user

Returns all workspace pull requests authored by the specified user.  By default only open pull requests are returned. This can be controlled using the `state` query parameter. To retrieve pull requests that are in one of multiple states, repeat the `state` parameter for each individual state.  This endpoint also supports filtering and sorting of the results. See [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | This can either be the username of the pull request author, the author's UUID surrounded by curly-braces, for example: `{account UUID}`, or the author's Atlassian ID.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**state** | Option<**String**> | Only return pull requests that are in this state. This parameter can be repeated. |  |

### Return type

[**models::PaginatedPullrequests**](paginated_pullrequests.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

