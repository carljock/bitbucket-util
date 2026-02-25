# \DownloadsApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**repositories_workspace_repo_slug_downloads_filename_delete**](DownloadsApi.md#repositories_workspace_repo_slug_downloads_filename_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/downloads/{filename} | Delete a download artifact
[**repositories_workspace_repo_slug_downloads_filename_get**](DownloadsApi.md#repositories_workspace_repo_slug_downloads_filename_get) | **GET** /repositories/{workspace}/{repo_slug}/downloads/{filename} | Get a download artifact link
[**repositories_workspace_repo_slug_downloads_get**](DownloadsApi.md#repositories_workspace_repo_slug_downloads_get) | **GET** /repositories/{workspace}/{repo_slug}/downloads | List download artifacts
[**repositories_workspace_repo_slug_downloads_post**](DownloadsApi.md#repositories_workspace_repo_slug_downloads_post) | **POST** /repositories/{workspace}/{repo_slug}/downloads | Upload a download artifact



## repositories_workspace_repo_slug_downloads_filename_delete

> repositories_workspace_repo_slug_downloads_filename_delete(filename, repo_slug, workspace)
Delete a download artifact

Deletes the specified download artifact from the repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filename** | **String** | Name of the file. | [required] |
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


## repositories_workspace_repo_slug_downloads_filename_get

> repositories_workspace_repo_slug_downloads_filename_get(filename, repo_slug, workspace)
Get a download artifact link

Return a redirect to the contents of a download artifact.  This endpoint returns the actual file contents and not the artifact's metadata.      $ curl -s -L https://api.bitbucket.org/2.0/repositories/evzijst/git-tests/downloads/hello.txt     Hello World

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filename** | **String** | Name of the file. | [required] |
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


## repositories_workspace_repo_slug_downloads_get

> repositories_workspace_repo_slug_downloads_get(repo_slug, workspace)
List download artifacts

Returns a list of download links associated with the repository.

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


## repositories_workspace_repo_slug_downloads_post

> repositories_workspace_repo_slug_downloads_post(repo_slug, workspace)
Upload a download artifact

Upload new download artifacts.  To upload files, perform a `multipart/form-data` POST containing one or more `files` fields:      $ echo Hello World > hello.txt     $ curl -s -u evzijst -X POST https://api.bitbucket.org/2.0/repositories/evzijst/git-tests/downloads -F files=@hello.txt  When a file is uploaded with the same name as an existing artifact, then the existing file will be replaced.

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

