# \CommitsApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_create_or_update_annotations**](CommitsApi.md#bulk_create_or_update_annotations) | **POST** /repositories/{workspace}/{repo_slug}/commit/{commit}/reports/{reportId}/annotations | Bulk create or update annotations
[**create_or_update_annotation**](CommitsApi.md#create_or_update_annotation) | **PUT** /repositories/{workspace}/{repo_slug}/commit/{commit}/reports/{reportId}/annotations/{annotationId} | Create or update an annotation
[**create_or_update_report**](CommitsApi.md#create_or_update_report) | **PUT** /repositories/{workspace}/{repo_slug}/commit/{commit}/reports/{reportId} | Create or update a report
[**delete_annotation**](CommitsApi.md#delete_annotation) | **DELETE** /repositories/{workspace}/{repo_slug}/commit/{commit}/reports/{reportId}/annotations/{annotationId} | Delete an annotation
[**delete_report**](CommitsApi.md#delete_report) | **DELETE** /repositories/{workspace}/{repo_slug}/commit/{commit}/reports/{reportId} | Delete a report
[**get_annotation**](CommitsApi.md#get_annotation) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit}/reports/{reportId}/annotations/{annotationId} | Get an annotation
[**get_annotations_for_report**](CommitsApi.md#get_annotations_for_report) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit}/reports/{reportId}/annotations | List annotations
[**get_report**](CommitsApi.md#get_report) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit}/reports/{reportId} | Get a report
[**get_reports_for_commit**](CommitsApi.md#get_reports_for_commit) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit}/reports | List reports
[**repositories_workspace_repo_slug_commit_commit_approve_delete**](CommitsApi.md#repositories_workspace_repo_slug_commit_commit_approve_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/commit/{commit}/approve | Unapprove a commit
[**repositories_workspace_repo_slug_commit_commit_approve_post**](CommitsApi.md#repositories_workspace_repo_slug_commit_commit_approve_post) | **POST** /repositories/{workspace}/{repo_slug}/commit/{commit}/approve | Approve a commit
[**repositories_workspace_repo_slug_commit_commit_comments_comment_id_delete**](CommitsApi.md#repositories_workspace_repo_slug_commit_commit_comments_comment_id_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/commit/{commit}/comments/{comment_id} | Delete a commit comment
[**repositories_workspace_repo_slug_commit_commit_comments_comment_id_get**](CommitsApi.md#repositories_workspace_repo_slug_commit_commit_comments_comment_id_get) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit}/comments/{comment_id} | Get a commit comment
[**repositories_workspace_repo_slug_commit_commit_comments_comment_id_put**](CommitsApi.md#repositories_workspace_repo_slug_commit_commit_comments_comment_id_put) | **PUT** /repositories/{workspace}/{repo_slug}/commit/{commit}/comments/{comment_id} | Update a commit comment
[**repositories_workspace_repo_slug_commit_commit_comments_get**](CommitsApi.md#repositories_workspace_repo_slug_commit_commit_comments_get) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit}/comments | List a commit's comments
[**repositories_workspace_repo_slug_commit_commit_comments_post**](CommitsApi.md#repositories_workspace_repo_slug_commit_commit_comments_post) | **POST** /repositories/{workspace}/{repo_slug}/commit/{commit}/comments | Create comment for a commit
[**repositories_workspace_repo_slug_commit_commit_get**](CommitsApi.md#repositories_workspace_repo_slug_commit_commit_get) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit} | Get a commit
[**repositories_workspace_repo_slug_commits_get**](CommitsApi.md#repositories_workspace_repo_slug_commits_get) | **GET** /repositories/{workspace}/{repo_slug}/commits | List commits
[**repositories_workspace_repo_slug_commits_post**](CommitsApi.md#repositories_workspace_repo_slug_commits_post) | **POST** /repositories/{workspace}/{repo_slug}/commits | List commits with include/exclude
[**repositories_workspace_repo_slug_commits_revision_get**](CommitsApi.md#repositories_workspace_repo_slug_commits_revision_get) | **GET** /repositories/{workspace}/{repo_slug}/commits/{revision} | List commits for revision
[**repositories_workspace_repo_slug_commits_revision_post**](CommitsApi.md#repositories_workspace_repo_slug_commits_revision_post) | **POST** /repositories/{workspace}/{repo_slug}/commits/{revision} | List commits for revision using include/exclude
[**repositories_workspace_repo_slug_diff_spec_get**](CommitsApi.md#repositories_workspace_repo_slug_diff_spec_get) | **GET** /repositories/{workspace}/{repo_slug}/diff/{spec} | Compare two commits
[**repositories_workspace_repo_slug_diffstat_spec_get**](CommitsApi.md#repositories_workspace_repo_slug_diffstat_spec_get) | **GET** /repositories/{workspace}/{repo_slug}/diffstat/{spec} | Compare two commit diff stats
[**repositories_workspace_repo_slug_merge_base_revspec_get**](CommitsApi.md#repositories_workspace_repo_slug_merge_base_revspec_get) | **GET** /repositories/{workspace}/{repo_slug}/merge-base/{revspec} | Get the common ancestor between two commits
[**repositories_workspace_repo_slug_patch_spec_get**](CommitsApi.md#repositories_workspace_repo_slug_patch_spec_get) | **GET** /repositories/{workspace}/{repo_slug}/patch/{spec} | Get a patch for two commits



## bulk_create_or_update_annotations

> Vec<models::ReportAnnotation> bulk_create_or_update_annotations(workspace, repo_slug, commit, report_id, report_annotation)
Bulk create or update annotations

Bulk upload of annotations. Annotations are individual findings that have been identified as part of a report, for example, a line of code that represents a vulnerability. These annotations can be attached to a specific file and even a specific line in that file, however, that is optional. Annotations are not mandatory and a report can contain up to 1000 annotations.  Add the annotations you want to upload as objects in a JSON array and make sure each annotation has the external_id field set to a unique value. If you want to use an existing id from your own system, we recommend prefixing it with your system's name to avoid collisions, for example, mySystem-annotation001. The external id can later be used to identify the report as an alternative to the generated [UUID](https://developer.atlassian.com/bitbucket/api/2/reference/meta/uri-uuid#uuid). You can upload up to 100 annotations per POST request.  ### Sample cURL request: ``` curl --location 'https://api.bitbucket.org/2.0/repositories/<username>/<reposity-name>/commit/<commit-hash>/reports/mysystem-001/annotations' \\ --header 'Content-Type: application/json' \\ --data-raw '[   {         \"external_id\": \"mysystem-annotation001\",         \"title\": \"Security scan report\",         \"annotation_type\": \"VULNERABILITY\",         \"summary\": \"This line represents a security threat.\",         \"severity\": \"HIGH\",       \"path\": \"my-service/src/main/java/com/myCompany/mysystem/logic/Main.java\",         \"line\": 42   },   {         \"external_id\": \"mySystem-annotation002\",         \"title\": \"Bug report\",         \"annotation_type\": \"BUG\",         \"result\": \"FAILED\",         \"summary\": \"This line might introduce a bug.\",         \"severity\": \"MEDIUM\",       \"path\": \"my-service/src/main/java/com/myCompany/mysystem/logic/Helper.java\",         \"line\": 13   } ]' ```  ### Possible field values: annotation_type: VULNERABILITY, CODE_SMELL, BUG result: PASSED, FAILED, IGNORED, SKIPPED severity: HIGH, MEDIUM, LOW, CRITICAL  Please refer to the [Code Insights documentation](https://confluence.atlassian.com/bitbucket/code-insights-994316785.html) for more information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit for which to retrieve reports. | [required] |
**report_id** | **String** | Uuid or external-if of the report for which to get annotations for. | [required] |
**report_annotation** | [**Vec<models::ReportAnnotation>**](ReportAnnotation.md) | The annotations to create or update | [required] |

### Return type

[**Vec<models::ReportAnnotation>**](report_annotation.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_or_update_annotation

> models::ReportAnnotation create_or_update_annotation(workspace, repo_slug, commit, report_id, annotation_id, report_annotation)
Create or update an annotation

Creates or updates an individual annotation for the specified report. Annotations are individual findings that have been identified as part of a report, for example, a line of code that represents a vulnerability. These annotations can be attached to a specific file and even a specific line in that file, however, that is optional. Annotations are not mandatory and a report can contain up to 1000 annotations.  Just as reports, annotation needs to be uploaded with a unique ID that can later be used to identify the report as an alternative to the generated [UUID](https://developer.atlassian.com/bitbucket/api/2/reference/meta/uri-uuid#uuid). If you want to use an existing id from your own system, we recommend prefixing it with your system's name to avoid collisions, for example, mySystem-annotation001.  ### Sample cURL request: ``` curl --request PUT 'https://api.bitbucket.org/2.0/repositories/<username>/<reposity-name>/commit/<commit-hash>/reports/mySystem-001/annotations/mysystem-annotation001' \\ --header 'Content-Type: application/json' \\ --data-raw '{     \"title\": \"Security scan report\",     \"annotation_type\": \"VULNERABILITY\",     \"summary\": \"This line represents a security thread.\",     \"severity\": \"HIGH\",     \"path\": \"my-service/src/main/java/com/myCompany/mysystem/logic/Main.java\",     \"line\": 42 }' ```  ### Possible field values: annotation_type: VULNERABILITY, CODE_SMELL, BUG result: PASSED, FAILED, IGNORED, SKIPPED severity: HIGH, MEDIUM, LOW, CRITICAL  Please refer to the [Code Insights documentation](https://confluence.atlassian.com/bitbucket/code-insights-994316785.html) for more information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit the report belongs to. | [required] |
**report_id** | **String** | Either the uuid or external-id of the report. | [required] |
**annotation_id** | **String** | Either the uuid or external-id of the annotation. | [required] |
**report_annotation** | [**ReportAnnotation**](ReportAnnotation.md) | The annotation to create or update | [required] |

### Return type

[**models::ReportAnnotation**](report_annotation.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_or_update_report

> models::Report create_or_update_report(workspace, repo_slug, commit, report_id, report)
Create or update a report

Creates or updates a report for the specified commit. To upload a report, make sure to generate an ID that is unique across all reports for that commit. If you want to use an existing id from your own system, we recommend prefixing it with your system's name to avoid collisions, for example, mySystem-001.  ### Sample cURL request: ``` curl --request PUT 'https://api.bitbucket.org/2.0/repositories/<username>/<reposity-name>/commit/<commit-hash>/reports/mysystem-001' \\ --header 'Content-Type: application/json' \\ --data-raw '{     \"title\": \"Security scan report\",     \"details\": \"This pull request introduces 10 new dependency vulnerabilities.\",     \"report_type\": \"SECURITY\",     \"reporter\": \"mySystem\",     \"link\": \"http://www.mysystem.com/reports/001\",     \"result\": \"FAILED\",     \"data\": [         {             \"title\": \"Duration (seconds)\",             \"type\": \"DURATION\",             \"value\": 14         },         {             \"title\": \"Safe to merge?\",             \"type\": \"BOOLEAN\",             \"value\": false         }     ] }' ```  ### Possible field values: report_type: SECURITY, COVERAGE, TEST, BUG result: PASSED, FAILED, PENDING data.type: BOOLEAN, DATE, DURATION, LINK, NUMBER, PERCENTAGE, TEXT  #### Data field formats | Type  Field   | Value Field Type  | Value Field Display | |:--------------|:------------------|:--------------------| | None/ Omitted | Number, String or Boolean (not an array or object) | Plain text | | BOOLEAN | Boolean | The value will be read as a JSON boolean and displayed as 'Yes' or 'No'. | | DATE  | Number | The value will be read as a JSON number in the form of a Unix timestamp (milliseconds) and will be displayed as a relative date if the date is less than one week ago, otherwise  it will be displayed as an absolute date. | | DURATION | Number | The value will be read as a JSON number in milliseconds and will be displayed in a human readable duration format. | | LINK | Object: `{\"text\": \"Link text here\", \"href\": \"https://link.to.annotation/in/external/tool\"}` | The value will be read as a JSON object containing the fields \"text\" and \"href\" and will be displayed as a clickable link on the report. | | NUMBER | Number | The value will be read as a JSON number and large numbers will be  displayed in a human readable format (e.g. 14.3k). | | PERCENTAGE | Number (between 0 and 100) | The value will be read as a JSON number between 0 and 100 and will be displayed with a percentage sign. | | TEXT | String | The value will be read as a JSON string and will be displayed as-is |  Please refer to the [Code Insights documentation](https://confluence.atlassian.com/bitbucket/code-insights-994316785.html) for more information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit the report belongs to. | [required] |
**report_id** | **String** | Either the uuid or external-id of the report. | [required] |
**report** | [**Report**](Report.md) | The report to create or update | [required] |

### Return type

[**models::Report**](report.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_annotation

> delete_annotation(workspace, repo_slug, commit, report_id, annotation_id)
Delete an annotation

Deletes a single Annotation matching the provided ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit the annotation belongs to. | [required] |
**report_id** | **String** | Either the uuid or external-id of the annotation. | [required] |
**annotation_id** | **String** | Either the uuid or external-id of the annotation. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_report

> delete_report(workspace, repo_slug, commit, report_id)
Delete a report

Deletes a single Report matching the provided ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit the report belongs to. | [required] |
**report_id** | **String** | Either the uuid or external-id of the report. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_annotation

> models::ReportAnnotation get_annotation(workspace, repo_slug, commit, report_id, annotation_id)
Get an annotation

Returns a single Annotation matching the provided ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit the report belongs to. | [required] |
**report_id** | **String** | Either the uuid or external-id of the report. | [required] |
**annotation_id** | **String** | Either the uuid or external-id of the annotation. | [required] |

### Return type

[**models::ReportAnnotation**](report_annotation.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_annotations_for_report

> models::PaginatedAnnotations get_annotations_for_report(workspace, repo_slug, commit, report_id)
List annotations

Returns a paginated list of Annotations for a specified report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit for which to retrieve reports. | [required] |
**report_id** | **String** | Uuid or external-if of the report for which to get annotations for. | [required] |

### Return type

[**models::PaginatedAnnotations**](paginated_annotations.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_report

> models::Report get_report(workspace, repo_slug, commit, report_id)
Get a report

Returns a single Report matching the provided ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit the report belongs to. | [required] |
**report_id** | **String** | Either the uuid or external-id of the report. | [required] |

### Return type

[**models::Report**](report.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reports_for_commit

> models::PaginatedReports get_reports_for_commit(workspace, repo_slug, commit)
List reports

Returns a paginated list of Reports linked to this commit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit for which to retrieve reports. | [required] |

### Return type

[**models::PaginatedReports**](paginated_reports.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commit_commit_approve_delete

> repositories_workspace_repo_slug_commit_commit_approve_delete(commit, repo_slug, workspace)
Unapprove a commit

Redact the authenticated user's approval of the specified commit.  This operation is only available to users that have explicit access to the repository. In contrast, just the fact that a repository is publicly accessible to users does not give them the ability to approve commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**commit** | **String** | The commit's SHA1. | [required] |
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


## repositories_workspace_repo_slug_commit_commit_approve_post

> models::Participant repositories_workspace_repo_slug_commit_commit_approve_post(commit, repo_slug, workspace)
Approve a commit

Approve the specified commit as the authenticated user.  This operation is only available to users that have explicit access to the repository. In contrast, just the fact that a repository is publicly accessible to users does not give them the ability to approve commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**commit** | **String** | The commit's SHA1. | [required] |
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


## repositories_workspace_repo_slug_commit_commit_comments_comment_id_delete

> repositories_workspace_repo_slug_commit_commit_comments_comment_id_delete(comment_id, commit, repo_slug, workspace)
Delete a commit comment

Deletes the specified commit comment.  Note that deleting comments that have visible replies that point to them will not really delete the resource. This is to retain the integrity of the original comment tree. Instead, the `deleted` element is set to `true` and the content is blanked out. The comment will continue to be returned by the collections and self endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**commit** | **String** | The commit's SHA1. | [required] |
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


## repositories_workspace_repo_slug_commit_commit_comments_comment_id_get

> models::CommitComment repositories_workspace_repo_slug_commit_commit_comments_comment_id_get(comment_id, commit, repo_slug, workspace)
Get a commit comment

Returns the specified commit comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**commit** | **String** | The commit's SHA1. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::CommitComment**](commit_comment.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commit_commit_comments_comment_id_put

> repositories_workspace_repo_slug_commit_commit_comments_comment_id_put(comment_id, commit, repo_slug, workspace, commit_comment)
Update a commit comment

Used to update the contents of a comment. Only the content of the comment can be updated.  ``` $ curl https://api.bitbucket.org/2.0/repositories/atlassian/prlinks/commit/7f71b5/comments/5728901 \\   -X PUT -u evzijst \\   -H 'Content-Type: application/json' \\   -d '{\"content\": {\"raw\": \"One more thing!\"}' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **i64** | The id of the comment. | [required] |
**commit** | **String** | The commit's SHA1. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**commit_comment** | [**CommitComment**](CommitComment.md) | The updated comment. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commit_commit_comments_get

> models::PaginatedCommitComments repositories_workspace_repo_slug_commit_commit_comments_get(commit, repo_slug, workspace, q, sort)
List a commit's comments

Returns the commit's comments.  This includes both global and inline comments.  The default sorting is oldest to newest and can be overridden with the `sort` query parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**commit** | **String** | The commit's SHA1. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**q** | Option<**String**> | Query string to narrow down the response as per [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering).  |  |
**sort** | Option<**String**> | Field by which the results should be sorted as per [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering).  |  |

### Return type

[**models::PaginatedCommitComments**](paginated_commit_comments.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commit_commit_comments_post

> repositories_workspace_repo_slug_commit_commit_comments_post(commit, repo_slug, workspace, commit_comment)
Create comment for a commit

Creates new comment on the specified commit.  To post a reply to an existing comment, include the `parent.id` field:  ``` $ curl https://api.bitbucket.org/2.0/repositories/atlassian/prlinks/commit/db9ba1e031d07a02603eae0e559a7adc010257fc/comments/ \\   -X POST -u evzijst \\   -H 'Content-Type: application/json' \\   -d '{\"content\": {\"raw\": \"One more thing!\"},        \"parent\": {\"id\": 5728901}}' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**commit** | **String** | The commit's SHA1. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**commit_comment** | [**CommitComment**](CommitComment.md) | The specified comment. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commit_commit_get

> models::Commit repositories_workspace_repo_slug_commit_commit_get(commit, repo_slug, workspace)
Get a commit

Returns the specified commit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**commit** | **String** | The commit's SHA1. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Commit**](commit.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commits_get

> models::PaginatedChangeset repositories_workspace_repo_slug_commits_get(repo_slug, workspace)
List commits

These are the repository's commits. They are paginated and returned in reverse chronological order, similar to the output of `git log`. Like these tools, the DAG can be filtered.  #### GET /repositories/{workspace}/{repo_slug}/commits/  Returns all commits in the repo in topological order (newest commit first). All branches and tags are included (similar to `git log --all`).  #### GET /repositories/{workspace}/{repo_slug}/commits/?exclude=master  Returns all commits in the repo that are not on master (similar to `git log --all ^master`).  #### GET /repositories/{workspace}/{repo_slug}/commits/?include=foo&include=bar&exclude=fu&exclude=fubar  Returns all commits that are on refs `foo` or `bar`, but not on `fu` or `fubar` (similar to `git log foo bar ^fu ^fubar`).  An optional `path` parameter can be specified that will limit the results to commits that affect that path. `path` can either be a file or a directory. If a directory is specified, commits are returned that have modified any file in the directory tree rooted by `path`. It is important to note that if the `path` parameter is specified, the commits returned by this endpoint may no longer be a DAG, parent commits that do not modify the path will be omitted from the response.  #### GET /repositories/{workspace}/{repo_slug}/commits/?path=README.md&include=foo&include=bar&exclude=master  Returns all commits that are on refs `foo` or `bar`, but not on `master` that changed the file README.md.  #### GET /repositories/{workspace}/{repo_slug}/commits/?path=src/&include=foo&include=bar&exclude=master  Returns all commits that are on refs `foo` or `bar`, but not on `master` that changed to a file in any file in the directory src or its children.  Because the response could include a very large number of commits, it is paginated. Follow the 'next' link in the response to navigate to the next page of commits. As with other paginated resources, do not construct your own links.  When the include and exclude parameters are more than can fit in a query string, clients can use a `x-www-form-urlencoded` POST instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedChangeset**](paginated_changeset.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commits_post

> models::PaginatedChangeset repositories_workspace_repo_slug_commits_post(repo_slug, workspace)
List commits with include/exclude

Identical to `GET /repositories/{workspace}/{repo_slug}/commits`, except that POST allows clients to place the include and exclude parameters in the request body to avoid URL length issues.  **Note that this resource does NOT support new commit creation.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedChangeset**](paginated_changeset.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commits_revision_get

> models::PaginatedChangeset repositories_workspace_repo_slug_commits_revision_get(repo_slug, revision, workspace)
List commits for revision

These are the repository's commits. They are paginated and returned in reverse chronological order, similar to the output of `git log`. Like these tools, the DAG can be filtered.  #### GET /repositories/{workspace}/{repo_slug}/commits/master  Returns all commits on ref `master` (similar to `git log master`).  #### GET /repositories/{workspace}/{repo_slug}/commits/dev?include=foo&exclude=master  Returns all commits on ref `dev` or `foo`, except those that are reachable on `master` (similar to `git log dev foo ^master`).  An optional `path` parameter can be specified that will limit the results to commits that affect that path. `path` can either be a file or a directory. If a directory is specified, commits are returned that have modified any file in the directory tree rooted by `path`. It is important to note that if the `path` parameter is specified, the commits returned by this endpoint may no longer be a DAG, parent commits that do not modify the path will be omitted from the response.  #### GET /repositories/{workspace}/{repo_slug}/commits/dev?path=README.md&include=foo&include=bar&exclude=master  Returns all commits that are on refs `dev` or `foo` or `bar`, but not on `master` that changed the file README.md.  #### GET /repositories/{workspace}/{repo_slug}/commits/dev?path=src/&include=foo&exclude=master  Returns all commits that are on refs `dev` or `foo`, but not on `master` that changed to a file in any file in the directory src or its children.  Because the response could include a very large number of commits, it is paginated. Follow the 'next' link in the response to navigate to the next page of commits. As with other paginated resources, do not construct your own links.  When the include and exclude parameters are more than can fit in a query string, clients can use a `x-www-form-urlencoded` POST instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**revision** | **String** | A commit SHA1 or ref name. | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedChangeset**](paginated_changeset.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_commits_revision_post

> models::PaginatedChangeset repositories_workspace_repo_slug_commits_revision_post(repo_slug, revision, workspace)
List commits for revision using include/exclude

Identical to `GET /repositories/{workspace}/{repo_slug}/commits/{revision}`, except that POST allows clients to place the include and exclude parameters in the request body to avoid URL length issues.  **Note that this resource does NOT support new commit creation.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**revision** | **String** | A commit SHA1 or ref name. | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedChangeset**](paginated_changeset.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_diff_spec_get

> repositories_workspace_repo_slug_diff_spec_get(repo_slug, spec, workspace, context, path, ignore_whitespace, binary, renames, merge, topic)
Compare two commits

Produces a raw git-style diff.  #### Single commit spec  If the `spec` argument to this API is a single commit, the diff is produced against the first parent of the specified commit.  #### Two commit spec  Two commits separated by `..` may be provided as the `spec`, e.g., `3a8b42..9ff173`. When two commits are provided and the `topic` query parameter is true, this API produces a 2-way three dot diff. This is the diff between source commit and the merge base of the source commit and the destination commit. When the `topic` query param is false, a simple git-style diff is produced.  The two commits are interpreted as follows:  * First commit: the commit containing the changes we wish to preview * Second commit: the commit representing the state to which we want to   compare the first commit * **Note**: This is the opposite of the order used in `git diff`.  #### Comparison to patches  While similar to patches, diffs:  * Don't have a commit header (username, commit message, etc) * Support the optional `path=foo/bar.py` query param to filter   the diff to just that one file diff  #### Response  The raw diff is returned as-is, in whatever encoding the files in the repository use. It is not decoded into unicode. As such, the content-type is `text/plain`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**spec** | **String** | A commit SHA (e.g. `3a8b42`) or a commit range using double dot notation (e.g. `3a8b42..9ff173`).  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**context** | Option<**i32**> | Generate diffs with <n> lines of context instead of the usual three. |  |
**path** | Option<**String**> | Limit the diff to a particular file (this parameter can be repeated for multiple paths). |  |
**ignore_whitespace** | Option<**bool**> | Generate diffs that ignore whitespace. |  |
**binary** | Option<**bool**> | Generate diffs that include binary files, true if omitted. |  |
**renames** | Option<**bool**> | Whether to perform rename detection, true if omitted. |  |
**merge** | Option<**bool**> | This parameter is deprecated. The 'topic' parameter should be used instead. The 'merge' and 'topic' parameters cannot be both used at the same time.  If true, the source commit is merged into the destination commit, and then a diff from the destination to the merge result is returned. If false, a simple 'two dot' diff between the source and destination is returned. True if omitted. |  |
**topic** | Option<**bool**> | If true, returns 2-way 'three-dot' diff. This is a diff between the source commit and the merge base of the source commit and the destination commit. If false, a simple 'two dot' diff between the source and destination is returned. |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_diffstat_spec_get

> models::PaginatedDiffstats repositories_workspace_repo_slug_diffstat_spec_get(repo_slug, spec, workspace, ignore_whitespace, merge, path, renames, topic)
Compare two commit diff stats

Produces a response in JSON format with a record for every path modified, including information on the type of the change and the number of lines added and removed.  #### Single commit spec  If the `spec` argument to this API is a single commit, the diff is produced against the first parent of the specified commit.  #### Two commit spec  Two commits separated by `..` may be provided as the `spec`, e.g., `3a8b42..9ff173`. When two commits are provided and the `topic` query parameter is true, this API produces a 2-way three dot diff. This is the diff between source commit and the merge base of the source commit and the destination commit. When the `topic` query param is false, a simple git-style diff is produced.  The two commits are interpreted as follows:  * First commit: the commit containing the changes we wish to preview * Second commit: the commit representing the state to which we want to   compare the first commit * **Note**: This is the opposite of the order used in `git diff`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**spec** | **String** | A commit SHA (e.g. `3a8b42`) or a commit range using double dot notation (e.g. `3a8b42..9ff173`).  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**ignore_whitespace** | Option<**bool**> | Generate diffs that ignore whitespace |  |
**merge** | Option<**bool**> | This parameter is deprecated. The 'topic' parameter should be used instead. The 'merge' and 'topic' parameters cannot be both used at the same time.  If true, the source commit is merged into the destination commit, and then a diffstat from the destination to the merge result is returned. If false, a simple 'two dot' diffstat between the source and destination is returned. True if omitted. |  |
**path** | Option<**String**> | Limit the diffstat to a particular file (this parameter can be repeated for multiple paths). |  |
**renames** | Option<**bool**> | Whether to perform rename detection, true if omitted. |  |
**topic** | Option<**bool**> | If true, returns 2-way 'three-dot' diff. This is a diff between the source commit and the merge base of the source commit and the destination commit. If false, a simple 'two dot' diff between the source and destination is returned. |  |

### Return type

[**models::PaginatedDiffstats**](paginated_diffstats.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_merge_base_revspec_get

> models::Commit repositories_workspace_repo_slug_merge_base_revspec_get(repo_slug, revspec, workspace)
Get the common ancestor between two commits

Returns the best common ancestor between two commits, specified in a revspec of 2 commits (e.g. 3a8b42..9ff173).  If more than one best common ancestor exists, only one will be returned. It is unspecified which will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**revspec** | **String** | A commit range using double dot notation (e.g. `3a8b42..9ff173`).  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Commit**](commit.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_patch_spec_get

> repositories_workspace_repo_slug_patch_spec_get(repo_slug, spec, workspace)
Get a patch for two commits

Produces a raw patch for a single commit (diffed against its first parent), or a patch-series for a revspec of 2 commits (e.g. `3a8b42..9ff173` where the first commit represents the source and the second commit the destination).  In case of the latter (diffing a revspec), a patch series is returned for the commits on the source branch (`3a8b42` and its ancestors in our example).  While similar to diffs, patches:  * Have a commit header (username, commit message, etc) * Do not support the `path=foo/bar.py` query parameter  The raw patch is returned as-is, in whatever encoding the files in the repository use. It is not decoded into unicode. As such, the content-type is `text/plain`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**spec** | **String** | A commit SHA (e.g. `3a8b42`) or a commit range using double dot notation (e.g. `3a8b42..9ff173`).  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

