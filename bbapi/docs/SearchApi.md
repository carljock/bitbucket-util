# \SearchApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_account**](SearchApi.md#search_account) | **GET** /users/{selected_user}/search/code | Search for code in a user's repositories
[**search_team**](SearchApi.md#search_team) | **GET** /teams/{username}/search/code | Search for code in a team's repositories
[**search_workspace**](SearchApi.md#search_workspace) | **GET** /workspaces/{workspace}/search/code | Search for code in a workspace



## search_account

> models::SearchResultPage search_account(selected_user, search_query, page, pagelen)
Search for code in a user's repositories

Search for code in the repositories of the specified user.  Note that searches can match in the file's text (`content_matches`), the path (`path_matches`), or both.  You can use the same syntax for the search query as in the UI. E.g. to search for \"foo\" only within the repository \"demo\", use the query parameter `search_query=foo+repo:demo`.  Similar to other APIs, you can request more fields using a `fields` query parameter. E.g. to get some more information about the repository of matched files, use the query parameter `search_query=foo&fields=%2Bvalues.file.commit.repository` (the `%2B` is a URL-encoded `+`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | Either the UUID of the account surrounded by curly-braces, for example `{account UUID}`, OR an Atlassian Account ID. | [required] |
**search_query** | **String** | The search query | [required] |
**page** | Option<**i32**> | Which page of the search results to retrieve |  |[default to 1]
**pagelen** | Option<**i32**> | How many search results to retrieve per page |  |[default to 10]

### Return type

[**models::SearchResultPage**](search_result_page.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_team

> models::SearchResultPage search_team(username, search_query, page, pagelen)
Search for code in a team's repositories

Search for code in the repositories of the specified team.  Note that searches can match in the file's text (`content_matches`), the path (`path_matches`), or both.  You can use the same syntax for the search query as in the UI. E.g. to search for \"foo\" only within the repository \"demo\", use the query parameter `search_query=foo+repo:demo`.  Similar to other APIs, you can request more fields using a `fields` query parameter. E.g. to get some more information about the repository of matched files, use the query parameter `search_query=foo&fields=%2Bvalues.file.commit.repository` (the `%2B` is a URL-encoded `+`).  Try `fields=%2Bvalues.*.*.*.*` to get an idea what's possible. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The account to search in; either the username or the UUID in curly braces | [required] |
**search_query** | **String** | The search query | [required] |
**page** | Option<**i32**> | Which page of the search results to retrieve |  |[default to 1]
**pagelen** | Option<**i32**> | How many search results to retrieve per page |  |[default to 10]

### Return type

[**models::SearchResultPage**](search_result_page.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_workspace

> models::SearchResultPage search_workspace(workspace, search_query, page, pagelen)
Search for code in a workspace

Search for code in the repositories of the specified workspace.  Note that searches can match in the file's text (`content_matches`), the path (`path_matches`), or both.  You can use the same syntax for the search query as in the UI. E.g. to search for \"foo\" only within the repository \"demo\", use the query parameter `search_query=foo+repo:demo`.  Similar to other APIs, you can request more fields using a `fields` query parameter. E.g. to get some more information about the repository of matched files, use the query parameter `search_query=foo&fields=%2Bvalues.file.commit.repository` (the `%2B` is a URL-encoded `+`).  Try `fields=%2Bvalues.*.*.*.*` to get an idea what's possible. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The workspace to search in; either the slug or the UUID in curly braces | [required] |
**search_query** | **String** | The search query | [required] |
**page** | Option<**i32**> | Which page of the search results to retrieve |  |[default to 1]
**pagelen** | Option<**i32**> | How many search results to retrieve per page |  |[default to 10]

### Return type

[**models::SearchResultPage**](search_result_page.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

