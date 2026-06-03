# \IssueTrackerApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**repositories_workspace_repo_slug_components_component_id_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_components_component_id_get) | **GET** /repositories/{workspace}/{repo_slug}/components/{component_id} | Get a component for issues
[**repositories_workspace_repo_slug_components_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_components_get) | **GET** /repositories/{workspace}/{repo_slug}/components | List components
[**repositories_workspace_repo_slug_issues_export_post**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_export_post) | **POST** /repositories/{workspace}/{repo_slug}/issues/export | Export issues
[**repositories_workspace_repo_slug_issues_export_repo_name_issues_task_id_zip_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_export_repo_name_issues_task_id_zip_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/export/{repo_name}-issues-{task_id}.zip | Check issue export status
[**repositories_workspace_repo_slug_issues_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_get) | **GET** /repositories/{workspace}/{repo_slug}/issues | List issues
[**repositories_workspace_repo_slug_issues_import_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_import_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/import | Check issue import status
[**repositories_workspace_repo_slug_issues_import_post**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_import_post) | **POST** /repositories/{workspace}/{repo_slug}/issues/import | Import issues
[**repositories_workspace_repo_slug_issues_issue_id_attachments_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_attachments_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/attachments | List attachments for an issue
[**repositories_workspace_repo_slug_issues_issue_id_attachments_path_delete**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_attachments_path_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/attachments/{path} | Delete an attachment for an issue
[**repositories_workspace_repo_slug_issues_issue_id_attachments_path_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_attachments_path_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/attachments/{path} | Get attachment for an issue
[**repositories_workspace_repo_slug_issues_issue_id_attachments_post**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_attachments_post) | **POST** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/attachments | Upload an attachment to an issue
[**repositories_workspace_repo_slug_issues_issue_id_changes_change_id_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_changes_change_id_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/changes/{change_id} | Get issue change object
[**repositories_workspace_repo_slug_issues_issue_id_changes_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_changes_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/changes | List changes on an issue
[**repositories_workspace_repo_slug_issues_issue_id_changes_post**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_changes_post) | **POST** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/changes | Modify the state of an issue
[**repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_delete**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/comments/{comment_id} | Delete a comment on an issue
[**repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/comments/{comment_id} | Get a comment on an issue
[**repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_put**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_put) | **PUT** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/comments/{comment_id} | Update a comment on an issue
[**repositories_workspace_repo_slug_issues_issue_id_comments_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_comments_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/comments | List comments on an issue
[**repositories_workspace_repo_slug_issues_issue_id_comments_post**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_comments_post) | **POST** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/comments | Create a comment on an issue
[**repositories_workspace_repo_slug_issues_issue_id_delete**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/issues/{issue_id} | Delete an issue
[**repositories_workspace_repo_slug_issues_issue_id_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/{issue_id} | Get an issue
[**repositories_workspace_repo_slug_issues_issue_id_put**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_put) | **PUT** /repositories/{workspace}/{repo_slug}/issues/{issue_id} | Update an issue
[**repositories_workspace_repo_slug_issues_issue_id_vote_delete**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_vote_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/vote | Remove vote for an issue
[**repositories_workspace_repo_slug_issues_issue_id_vote_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_vote_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/vote | Check if current user voted for an issue
[**repositories_workspace_repo_slug_issues_issue_id_vote_put**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_vote_put) | **PUT** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/vote | Vote for an issue
[**repositories_workspace_repo_slug_issues_issue_id_watch_delete**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_watch_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/watch | Stop watching an issue
[**repositories_workspace_repo_slug_issues_issue_id_watch_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_watch_get) | **GET** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/watch | Check if current user is watching a issue
[**repositories_workspace_repo_slug_issues_issue_id_watch_put**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_issue_id_watch_put) | **PUT** /repositories/{workspace}/{repo_slug}/issues/{issue_id}/watch | Watch an issue
[**repositories_workspace_repo_slug_issues_post**](IssueTrackerApi.md#repositories_workspace_repo_slug_issues_post) | **POST** /repositories/{workspace}/{repo_slug}/issues | Create an issue
[**repositories_workspace_repo_slug_milestones_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_milestones_get) | **GET** /repositories/{workspace}/{repo_slug}/milestones | List milestones
[**repositories_workspace_repo_slug_milestones_milestone_id_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_milestones_milestone_id_get) | **GET** /repositories/{workspace}/{repo_slug}/milestones/{milestone_id} | Get a milestone
[**repositories_workspace_repo_slug_versions_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_versions_get) | **GET** /repositories/{workspace}/{repo_slug}/versions | List defined versions for issues
[**repositories_workspace_repo_slug_versions_version_id_get**](IssueTrackerApi.md#repositories_workspace_repo_slug_versions_version_id_get) | **GET** /repositories/{workspace}/{repo_slug}/versions/{version_id} | Get a defined version for issues



## repositories_workspace_repo_slug_components_component_id_get

> models::Component repositories_workspace_repo_slug_components_component_id_get(component_id, repo_slug, workspace)
Get a component for issues

Returns the specified issue tracker component object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component_id** | **i32** | The component's id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Component**](component.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_components_get

> models::PaginatedComponents repositories_workspace_repo_slug_components_get(repo_slug, workspace)
List components

Returns the components that have been defined in the issue tracker.  This resource is only available on repositories that have the issue tracker enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedComponents**](paginated_components.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_export_post

> repositories_workspace_repo_slug_issues_export_post(repo_slug, workspace, export_options)
Export issues

A POST request to this endpoint initiates a new background celery task that archives the repo's issues.  When the job has been accepted, it will return a 202 (Accepted) along with a unique url to this job in the 'Location' response header. This url is the endpoint for where the user can obtain their zip files.\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**export_options** | Option<[**ExportOptions**](ExportOptions.md)> | The options to apply to the export. Available options include `project_key` and `project_name` which, if specified, are used as the Jira space key and name in the exported Jira json format. Option `send_email` specifies whether an email should be sent upon export result. Option `include_attachments` specifies whether attachments are included in the export. |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_export_repo_name_issues_task_id_zip_get

> models::IssueJobStatus repositories_workspace_repo_slug_issues_export_repo_name_issues_task_id_zip_get(repo_name, repo_slug, task_id, workspace)
Check issue export status

This endpoint is used to poll for the progress of an issue export job and return the zip file after the job is complete. As long as the job is running, this will return a 202 response with in the response body a description of the current status.  After the job has been scheduled, but before it starts executing, the endpoint returns a 202 response with status `ACCEPTED`.  Once it starts running, it is a 202 response with status `STARTED` and progress filled.  After it is finished, it becomes a 200 response with status `SUCCESS` or `FAILURE`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_name** | **String** | The name of the repo | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**task_id** | **String** | The ID of the export task | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::IssueJobStatus**](issue_job_status.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_get

> models::PaginatedIssues repositories_workspace_repo_slug_issues_get(repo_slug, workspace)
List issues

Returns the issues in the issue tracker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedIssues**](paginated_issues.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_import_get

> models::IssueJobStatus repositories_workspace_repo_slug_issues_import_get(repo_slug, workspace)
Check issue import status

When using GET, this endpoint reports the status of the current import task.  After the job has been scheduled, but before it starts executing, the endpoint returns a 202 response with status `ACCEPTED`.  Once it starts running, it is a 202 response with status `STARTED` and progress filled.  After it is finished, it becomes a 200 response with status `SUCCESS` or `FAILURE`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::IssueJobStatus**](issue_job_status.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_import_post

> models::IssueJobStatus repositories_workspace_repo_slug_issues_import_post(repo_slug, workspace)
Import issues

A POST request to this endpoint will import the zip file given by the archive parameter into the repository. All existing issues will be deleted and replaced by the contents of the imported zip file.  Imports are done through a multipart/form-data POST. There is one valid and required form field, with the name \"archive,\" which needs to be a file field:  ``` $ curl -u <username> -X POST -F archive=@/path/to/file.zip https://api.bitbucket.org/2.0/repositories/<owner_username>/<repo_slug>/issues/import ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::IssueJobStatus**](issue_job_status.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_attachments_get

> models::PaginatedIssueAttachments repositories_workspace_repo_slug_issues_issue_id_attachments_get(issue_id, repo_slug, workspace)
List attachments for an issue

Returns all attachments for this issue.  This returns the files' meta data. This does not return the files' actual contents.  The files are always ordered by their upload date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedIssueAttachments**](paginated_issue_attachments.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_attachments_path_delete

> repositories_workspace_repo_slug_issues_issue_id_attachments_path_delete(issue_id, path, repo_slug, workspace)
Delete an attachment for an issue

Deletes an attachment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**path** | **String** | Path to the file. | [required] |
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


## repositories_workspace_repo_slug_issues_issue_id_attachments_path_get

> repositories_workspace_repo_slug_issues_issue_id_attachments_path_get(issue_id, path, repo_slug, workspace)
Get attachment for an issue

Returns the contents of the specified file attachment.  Note that this endpoint does not return a JSON response, but instead returns a redirect pointing to the actual file that in turn will return the raw contents.  The redirect URL contains a one-time token that has a limited lifetime. As a result, the link should not be persisted, stored, or shared.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**path** | **String** | Path to the file. | [required] |
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


## repositories_workspace_repo_slug_issues_issue_id_attachments_post

> repositories_workspace_repo_slug_issues_issue_id_attachments_post(issue_id, repo_slug, workspace)
Upload an attachment to an issue

Upload new issue attachments.  To upload files, perform a `multipart/form-data` POST containing one or more file fields.  When a file is uploaded with the same name as an existing attachment, then the existing file will be replaced.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
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


## repositories_workspace_repo_slug_issues_issue_id_changes_change_id_get

> models::IssueChange repositories_workspace_repo_slug_issues_issue_id_changes_change_id_get(change_id, issue_id, repo_slug, workspace)
Get issue change object

Returns the specified issue change object.  This resource is only available on repositories that have the issue tracker enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_id** | **String** | The issue change id | [required] |
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::IssueChange**](issue_change.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_changes_get

> models::PaginatedLogEntries repositories_workspace_repo_slug_issues_issue_id_changes_get(issue_id, repo_slug, workspace, q, sort)
List changes on an issue

Returns the list of all changes that have been made to the specified issue. Changes are returned in chronological order with the oldest change first.  Each time an issue is edited in the UI or through the API, an immutable change record is created under the `/issues/123/changes` endpoint. It also has a comment associated with the change.  Note that this operation is changing significantly, due to privacy changes. See the [announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-changes-gdpr/#changes-to-the-issue-changes-api) for details.  Changes support [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering) that can be used to search for specific changes. For instance, to see when an issue transitioned to \"resolved\":  ``` $ curl -s https://api.bitbucket.org/2.0/repositories/site/master/issues/1/changes \\    -G --data-urlencode='q=changes.state.new = \"resolved\"' ```  This resource is only available on repositories that have the issue tracker enabled.  N.B.  The `changes.assignee` and `changes.assignee_account_id` fields are not a `user` object. Instead, they contain the raw `username` and `account_id` of the user. This is to protect the integrity of the audit log even after a user account gets deleted.  The `changes.assignee` field is deprecated will disappear in the future. Use `changes.assignee_account_id` instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**q** | Option<**String**> |  Query string to narrow down the response. See [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering) for details. |  |
**sort** | Option<**String**> |  Name of a response property to sort results. See [filtering and sorting](/cloud/bitbucket/rest/intro/#sorting-query-results) for details.  |  |

### Return type

[**models::PaginatedLogEntries**](paginated_log_entries.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_changes_post

> models::IssueChange repositories_workspace_repo_slug_issues_issue_id_changes_post(issue_id, repo_slug, workspace, issue_change)
Modify the state of an issue

Makes a change to the specified issue.  For example, to change an issue's state and assignee, create a new change object that modifies these fields:  ``` curl https://api.bitbucket.org/2.0/site/master/issues/1234/changes \\   -s -u evzijst -X POST -H \"Content-Type: application/json\" \\   -d '{     \"changes\": {       \"assignee_account_id\": {         \"new\": \"557058:c0b72ad0-1cb5-4018-9cdc-0cde8492c443\"       },       \"state\": {         \"new\": 'resolved\"       }     }     \"message\": {       \"raw\": \"This is now resolved.\"     }   }' ```  The above example also includes a custom comment to go alongside the change. This comment will also be visible on the issue page in the UI.  The fields of the `changes` object are strings, not objects. This allows for immutable change log records, even after user accounts, milestones, or other objects recorded in a change entry, get renamed or deleted.  The `assignee_account_id` field stores the account id. When POSTing a new change and changing the assignee, the client should therefore use the user's account_id in the `changes.assignee_account_id.new` field.  This call requires authentication. Private repositories or private issue trackers require the caller to authenticate with an account that has appropriate authorization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**issue_change** | [**IssueChange**](IssueChange.md) | The new issue state change. The only required elements are `changes.[].new`. All other elements can be omitted from the body. | [required] |

### Return type

[**models::IssueChange**](issue_change.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_delete

> repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_delete(comment_id, issue_id, repo_slug, workspace)
Delete a comment on an issue

Deletes the specified comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**issue_id** | **String** | The issue id | [required] |
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


## repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_get

> models::IssueComment repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_get(comment_id, issue_id, repo_slug, workspace)
Get a comment on an issue

Returns the specified issue comment object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::IssueComment**](issue_comment.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_put

> models::IssueComment repositories_workspace_repo_slug_issues_issue_id_comments_comment_id_put(comment_id, issue_id, repo_slug, workspace, issue_comment)
Update a comment on an issue

Updates the content of the specified issue comment. Note that only the `content.raw` field can be modified.  ``` $ curl https://api.bitbucket.org/2.0/repositories/atlassian/prlinks/issues/42/comments/5728901 \\   -X PUT -u evzijst \\   -H 'Content-Type: application/json' \\   -d '{\"content\": {\"raw\": \"Lorem ipsum.\"}' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**issue_comment** | [**IssueComment**](IssueComment.md) | The updated comment. | [required] |

### Return type

[**models::IssueComment**](issue_comment.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_comments_get

> models::PaginatedIssueComments repositories_workspace_repo_slug_issues_issue_id_comments_get(issue_id, repo_slug, workspace, q)
List comments on an issue

Returns a paginated list of all comments that were made on the specified issue.  The default sorting is oldest to newest and can be overridden with the `sort` query parameter.  This endpoint also supports filtering and sorting of the results. See [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**q** | Option<**String**> |  Query string to narrow down the response as per [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering). |  |

### Return type

[**models::PaginatedIssueComments**](paginated_issue_comments.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_comments_post

> repositories_workspace_repo_slug_issues_issue_id_comments_post(issue_id, repo_slug, workspace, issue_comment)
Create a comment on an issue

Creates a new issue comment.  ``` $ curl https://api.bitbucket.org/2.0/repositories/atlassian/prlinks/issues/42/comments/ \\   -X POST -u evzijst \\   -H 'Content-Type: application/json' \\   -d '{\"content\": {\"raw\": \"Lorem ipsum.\"}}' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**issue_comment** | [**IssueComment**](IssueComment.md) | The new issue comment object. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_delete

> repositories_workspace_repo_slug_issues_issue_id_delete(issue_id, repo_slug, workspace)
Delete an issue

Deletes the specified issue. This requires write access to the repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
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


## repositories_workspace_repo_slug_issues_issue_id_get

> models::Issue repositories_workspace_repo_slug_issues_issue_id_get(issue_id, repo_slug, workspace)
Get an issue

Returns the specified issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Issue**](issue.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_put

> models::Issue repositories_workspace_repo_slug_issues_issue_id_put(issue_id, repo_slug, workspace)
Update an issue

Modifies the issue.  ``` $ curl https://api.bitbucket.org/2.0/repostories/evzijst/dogslow/issues/123 \\   -u evzijst -s -X PUT -H 'Content-Type: application/json' \\   -d '{   \"title\": \"Updated title\",   \"assignee\": {     \"account_id\": \"5d5355e8c6b9320d9ea5b28d\"   },   \"priority\": \"minor\",   \"version\": {     \"name\": \"1.0\"   },   \"component\": null }' ```  This example changes the `title`, `assignee`, `priority` and the `version`. It also removes the value of the `component` from the issue by setting the field to `null`. Any field not present keeps its existing value.  Each time an issue is edited in the UI or through the API, an immutable change record is created under the `/issues/123/changes` endpoint. It also has a comment associated with the change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Issue**](issue.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_vote_delete

> models::Error repositories_workspace_repo_slug_issues_issue_id_vote_delete(issue_id, repo_slug, workspace)
Remove vote for an issue

Retract your vote.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Error**](error.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_vote_get

> models::Error repositories_workspace_repo_slug_issues_issue_id_vote_get(issue_id, repo_slug, workspace)
Check if current user voted for an issue

Check whether the authenticated user has voted for this issue. A 204 status code indicates that the user has voted, while a 404 implies they haven't.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Error**](error.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_vote_put

> models::Error repositories_workspace_repo_slug_issues_issue_id_vote_put(issue_id, repo_slug, workspace)
Vote for an issue

Vote for this issue.  To cast your vote, do an empty PUT. The 204 status code indicates that the operation was successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Error**](error.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_watch_delete

> models::Error repositories_workspace_repo_slug_issues_issue_id_watch_delete(issue_id, repo_slug, workspace)
Stop watching an issue

Stop watching this issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Error**](error.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_watch_get

> models::Error repositories_workspace_repo_slug_issues_issue_id_watch_get(issue_id, repo_slug, workspace)
Check if current user is watching a issue

Indicated whether or not the authenticated user is watching this issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Error**](error.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_issue_id_watch_put

> models::Error repositories_workspace_repo_slug_issues_issue_id_watch_put(issue_id, repo_slug, workspace)
Watch an issue

Start watching this issue.  To start watching this issue, do an empty PUT. The 204 status code indicates that the operation was successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The issue id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Error**](error.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_issues_post

> models::Issue repositories_workspace_repo_slug_issues_post(repo_slug, workspace, issue)
Create an issue

Creates a new issue.  This call requires authentication. Private repositories or private issue trackers require the caller to authenticate with an account that has appropriate authorization.  The authenticated user is used for the issue's `reporter` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**issue** | [**Issue**](Issue.md) | The new issue. The only required element is `title`. All other elements can be omitted from the body. | [required] |

### Return type

[**models::Issue**](issue.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_milestones_get

> models::PaginatedMilestones repositories_workspace_repo_slug_milestones_get(repo_slug, workspace)
List milestones

Returns the milestones that have been defined in the issue tracker.  This resource is only available on repositories that have the issue tracker enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedMilestones**](paginated_milestones.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_milestones_milestone_id_get

> models::Milestone repositories_workspace_repo_slug_milestones_milestone_id_get(milestone_id, repo_slug, workspace)
Get a milestone

Returns the specified issue tracker milestone object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**milestone_id** | **i32** | The milestone's id | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Milestone**](milestone.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_versions_get

> models::PaginatedVersions repositories_workspace_repo_slug_versions_get(repo_slug, workspace)
List defined versions for issues

Returns the versions that have been defined in the issue tracker.  This resource is only available on repositories that have the issue tracker enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedVersions**](paginated_versions.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_versions_version_id_get

> models::Version repositories_workspace_repo_slug_versions_version_id_get(repo_slug, version_id, workspace)
Get a defined version for issues

Returns the specified issue tracker version object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**version_id** | **i32** | The version's id | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Version**](version.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

