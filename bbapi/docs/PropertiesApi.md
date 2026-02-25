# \PropertiesApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_commit_hosted_property_value**](PropertiesApi.md#delete_commit_hosted_property_value) | **DELETE** /repositories/{workspace}/{repo_slug}/commit/{commit}/properties/{app_key}/{property_name} | Delete a commit application property
[**delete_pull_request_hosted_property_value**](PropertiesApi.md#delete_pull_request_hosted_property_value) | **DELETE** /repositories/{workspace}/{repo_slug}/pullrequests/{pullrequest_id}/properties/{app_key}/{property_name} | Delete a pull request application property
[**delete_repository_hosted_property_value**](PropertiesApi.md#delete_repository_hosted_property_value) | **DELETE** /repositories/{workspace}/{repo_slug}/properties/{app_key}/{property_name} | Delete a repository application property
[**delete_user_hosted_property_value**](PropertiesApi.md#delete_user_hosted_property_value) | **DELETE** /users/{selected_user}/properties/{app_key}/{property_name} | Delete a user application property
[**get_commit_hosted_property_value**](PropertiesApi.md#get_commit_hosted_property_value) | **GET** /repositories/{workspace}/{repo_slug}/commit/{commit}/properties/{app_key}/{property_name} | Get a commit application property
[**get_pull_request_hosted_property_value**](PropertiesApi.md#get_pull_request_hosted_property_value) | **GET** /repositories/{workspace}/{repo_slug}/pullrequests/{pullrequest_id}/properties/{app_key}/{property_name} | Get a pull request application property
[**get_repository_hosted_property_value**](PropertiesApi.md#get_repository_hosted_property_value) | **GET** /repositories/{workspace}/{repo_slug}/properties/{app_key}/{property_name} | Get a repository application property
[**retrieve_user_hosted_property_value**](PropertiesApi.md#retrieve_user_hosted_property_value) | **GET** /users/{selected_user}/properties/{app_key}/{property_name} | Get a user application property
[**update_commit_hosted_property_value**](PropertiesApi.md#update_commit_hosted_property_value) | **PUT** /repositories/{workspace}/{repo_slug}/commit/{commit}/properties/{app_key}/{property_name} | Update a commit application property
[**update_pull_request_hosted_property_value**](PropertiesApi.md#update_pull_request_hosted_property_value) | **PUT** /repositories/{workspace}/{repo_slug}/pullrequests/{pullrequest_id}/properties/{app_key}/{property_name} | Update a pull request application property
[**update_repository_hosted_property_value**](PropertiesApi.md#update_repository_hosted_property_value) | **PUT** /repositories/{workspace}/{repo_slug}/properties/{app_key}/{property_name} | Update a repository application property
[**update_user_hosted_property_value**](PropertiesApi.md#update_user_hosted_property_value) | **PUT** /users/{selected_user}/properties/{app_key}/{property_name} | Update a user application property



## delete_commit_hosted_property_value

> delete_commit_hosted_property_value(workspace, repo_slug, commit, app_key, property_name)
Delete a commit application property

Delete an [application property](/cloud/bitbucket/application-properties/) value stored against a commit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The repository container; either the workspace slug or the UUID in curly braces. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pull_request_hosted_property_value

> delete_pull_request_hosted_property_value(workspace, repo_slug, pullrequest_id, app_key, property_name)
Delete a pull request application property

Delete an [application property](/cloud/bitbucket/application-properties/) value stored against a pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The repository container; either the workspace slug or the UUID in curly braces. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pullrequest_id** | **String** | The pull request ID. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repository_hosted_property_value

> delete_repository_hosted_property_value(workspace, repo_slug, app_key, property_name)
Delete a repository application property

Delete an [application property](/cloud/bitbucket/application-properties/) value stored against a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The repository container; either the workspace slug or the UUID in curly braces. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_hosted_property_value

> delete_user_hosted_property_value(selected_user, app_key, property_name)
Delete a user application property

Delete an [application property](/cloud/bitbucket/application-properties/) value stored against a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | Either the UUID of the account surrounded by curly-braces, for example `{account UUID}`, OR an Atlassian Account ID. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_commit_hosted_property_value

> models::ApplicationProperty get_commit_hosted_property_value(workspace, repo_slug, commit, app_key, property_name)
Get a commit application property

Retrieve an [application property](/cloud/bitbucket/application-properties/) value stored against a commit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The repository container; either the workspace slug or the UUID in curly braces. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |

### Return type

[**models::ApplicationProperty**](application_property.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pull_request_hosted_property_value

> models::ApplicationProperty get_pull_request_hosted_property_value(workspace, repo_slug, pullrequest_id, app_key, property_name)
Get a pull request application property

Retrieve an [application property](/cloud/bitbucket/application-properties/) value stored against a pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The repository container; either the workspace slug or the UUID in curly braces. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pullrequest_id** | **String** | The pull request ID. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |

### Return type

[**models::ApplicationProperty**](application_property.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_hosted_property_value

> models::ApplicationProperty get_repository_hosted_property_value(workspace, repo_slug, app_key, property_name)
Get a repository application property

Retrieve an [application property](/cloud/bitbucket/application-properties/) value stored against a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The repository container; either the workspace slug or the UUID in curly braces. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |

### Return type

[**models::ApplicationProperty**](application_property.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_hosted_property_value

> models::ApplicationProperty retrieve_user_hosted_property_value(selected_user, app_key, property_name)
Get a user application property

Retrieve an [application property](/cloud/bitbucket/application-properties/) value stored against a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | Either the UUID of the account surrounded by curly-braces, for example `{account UUID}`, OR an Atlassian Account ID. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |

### Return type

[**models::ApplicationProperty**](application_property.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_commit_hosted_property_value

> update_commit_hosted_property_value(workspace, repo_slug, commit, app_key, property_name, application_property)
Update a commit application property

Update an [application property](/cloud/bitbucket/application-properties/) value stored against a commit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The repository container; either the workspace slug or the UUID in curly braces. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**commit** | **String** | The commit. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |
**application_property** | [**ApplicationProperty**](ApplicationProperty.md) | The application property to create or update. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pull_request_hosted_property_value

> update_pull_request_hosted_property_value(workspace, repo_slug, pullrequest_id, app_key, property_name, application_property)
Update a pull request application property

Update an [application property](/cloud/bitbucket/application-properties/) value stored against a pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The repository container; either the workspace slug or the UUID in curly braces. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pullrequest_id** | **String** | The pull request ID. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |
**application_property** | [**ApplicationProperty**](ApplicationProperty.md) | The application property to create or update. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository_hosted_property_value

> update_repository_hosted_property_value(workspace, repo_slug, app_key, property_name, application_property)
Update a repository application property

Update an [application property](/cloud/bitbucket/application-properties/) value stored against a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The repository container; either the workspace slug or the UUID in curly braces. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |
**application_property** | [**ApplicationProperty**](ApplicationProperty.md) | The application property to create or update. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_hosted_property_value

> update_user_hosted_property_value(selected_user, app_key, property_name, application_property)
Update a user application property

Update an [application property](/cloud/bitbucket/application-properties/) value stored against a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | Either the UUID of the account surrounded by curly-braces, for example `{account UUID}`, OR an Atlassian Account ID. | [required] |
**app_key** | **String** | The key of the Connect app. | [required] |
**property_name** | **String** | The name of the property. | [required] |
**application_property** | [**ApplicationProperty**](ApplicationProperty.md) | The application property to create or update. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

