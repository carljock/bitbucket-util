# \CommitStatusesApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**repositories_workspace_repo_slug_commit_commit_statuses_build_key_get**](CommitStatusesApi.md#repositories_workspace_repo_slug_commit_commit_statuses_build_key_get) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit}/statuses/build/{key} | Get a build status for a commit
[**repositories_workspace_repo_slug_commit_commit_statuses_build_key_put**](CommitStatusesApi.md#repositories_workspace_repo_slug_commit_commit_statuses_build_key_put) | **PUT** /repositories/{workspace}/{repo_slug}/commit/{commit}/statuses/build/{key} | Update a build status for a commit
[**repositories_workspace_repo_slug_commit_commit_statuses_build_post**](CommitStatusesApi.md#repositories_workspace_repo_slug_commit_commit_statuses_build_post) | **POST** /repositories/{workspace}/{repo_slug}/commit/{commit}/statuses/build | Create a build status for a commit
[**repositories_workspace_repo_slug_commit_commit_statuses_get**](CommitStatusesApi.md#repositories_workspace_repo_slug_commit_commit_statuses_get) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit}/statuses | List commit statuses for a commit
[**repositories_workspace_repo_slug_pullrequests_pull_request_id_statuses_get**](CommitStatusesApi.md#repositories_workspace_repo_slug_pullrequests_pull_request_id_statuses_get) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id}/statuses | List commit statuses for a pull request



## repositories_workspace_repo_slug_commit_commit_statuses_build_key_get

> models::Commitstatus repositories_workspace_repo_slug_commit_commit_statuses_build_key_get(commit, key, repo_slug, workspace)
Get a build status for a commit

Returns the specified build status for a commit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**commit** | **String** | The commit's SHA1. | [required] |
**key** | **String** | The build status' unique key | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Commitstatus**](commitstatus.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commit_commit_statuses_build_key_put

> models::Commitstatus repositories_workspace_repo_slug_commit_commit_statuses_build_key_put(commit, key, repo_slug, workspace, commitstatus)
Update a build status for a commit

Used to update the current status of a build status object on the specific commit.  This operation can also be used to change other properties of the build status:  * `state` * `name` * `description` * `url` * `refname`  The `key` cannot be changed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**commit** | **String** | The commit's SHA1. | [required] |
**key** | **String** | The build status' unique key | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**commitstatus** | Option<[**Commitstatus**](Commitstatus.md)> | The updated build status object |  |

### Return type

[**models::Commitstatus**](commitstatus.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commit_commit_statuses_build_post

> models::Commitstatus repositories_workspace_repo_slug_commit_commit_statuses_build_post(commit, repo_slug, workspace, commitstatus)
Create a build status for a commit

Creates a new build status against the specified commit.  If the specified key already exists, the existing status object will be overwritten.  Example:  ``` curl https://api.bitbucket.org/2.0/repositories/my-workspace/my-repo/commit/e10dae226959c2194f2b07b077c07762d93821cf/statuses/build/           -X POST -u jdoe -H 'Content-Type: application/json'           -d '{     \"key\": \"MY-BUILD\",     \"state\": \"SUCCESSFUL\",     \"description\": \"42 tests passed\",     \"url\": \"https://www.example.org/my-build-result\"   }' ```  When creating a new commit status, you can use a URI template for the URL. Templates are URLs that contain variable names that Bitbucket will evaluate at runtime whenever the URL is displayed anywhere similar to parameter substitution in [Bitbucket Connect](https://developer.atlassian.com/bitbucket/concepts/context-parameters.html). For example, one could use `https://foo.com/builds/{repository.full_name}` which Bitbucket will turn into `https://foo.com/builds/foo/bar` at render time. The context variables available are `repository` and `commit`.  To associate a commit status to a pull request, the refname field must be set to the source branch of the pull request.  Example: ``` curl https://api.bitbucket.org/2.0/repositories/my-workspace/my-repo/commit/e10dae226959c2194f2b07b077c07762d93821cf/statuses/build/           -X POST -u jdoe -H 'Content-Type: application/json'           -d '{     \"key\": \"MY-BUILD\",     \"state\": \"SUCCESSFUL\",     \"description\": \"42 tests passed\",     \"url\": \"https://www.example.org/my-build-result\",     \"refname\": \"my-pr-branch\"   }' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**commit** | **String** | The commit's SHA1. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**commitstatus** | Option<[**Commitstatus**](Commitstatus.md)> | The new commit status object. |  |

### Return type

[**models::Commitstatus**](commitstatus.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commit_commit_statuses_get

> models::PaginatedCommitstatuses repositories_workspace_repo_slug_commit_commit_statuses_get(commit, repo_slug, workspace, refname, q, sort)
List commit statuses for a commit

Returns all statuses (e.g. build results) for a specific commit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**commit** | **String** | The commit's SHA1. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**refname** | Option<**String**> | If specified, only return commit status objects that were either created without a refname, or were created with the specified refname  |  |
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

