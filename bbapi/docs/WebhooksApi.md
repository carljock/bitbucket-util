# \WebhooksApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hook_events_get**](WebhooksApi.md#hook_events_get) | **GET** /hook_events | Get a webhook resource
[**hook_events_subject_type_get**](WebhooksApi.md#hook_events_subject_type_get) | **GET** /hook_events/{subject_type} | List subscribable webhook types
[**repositories_workspace_repo_slug_hooks_get**](WebhooksApi.md#repositories_workspace_repo_slug_hooks_get) | **GET** /repositories/{workspace}/{repo_slug}/hooks | List webhooks for a repository
[**repositories_workspace_repo_slug_hooks_post**](WebhooksApi.md#repositories_workspace_repo_slug_hooks_post) | **POST** /repositories/{workspace}/{repo_slug}/hooks | Create a webhook for a repository
[**repositories_workspace_repo_slug_hooks_uid_delete**](WebhooksApi.md#repositories_workspace_repo_slug_hooks_uid_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/hooks/{uid} | Delete a webhook for a repository
[**repositories_workspace_repo_slug_hooks_uid_get**](WebhooksApi.md#repositories_workspace_repo_slug_hooks_uid_get) | **GET** /repositories/{workspace}/{repo_slug}/hooks/{uid} | Get a webhook for a repository
[**repositories_workspace_repo_slug_hooks_uid_put**](WebhooksApi.md#repositories_workspace_repo_slug_hooks_uid_put) | **PUT** /repositories/{workspace}/{repo_slug}/hooks/{uid} | Update a webhook for a repository
[**workspaces_workspace_hooks_get**](WebhooksApi.md#workspaces_workspace_hooks_get) | **GET** /workspaces/{workspace}/hooks | List webhooks for a workspace
[**workspaces_workspace_hooks_post**](WebhooksApi.md#workspaces_workspace_hooks_post) | **POST** /workspaces/{workspace}/hooks | Create a webhook for a workspace
[**workspaces_workspace_hooks_uid_delete**](WebhooksApi.md#workspaces_workspace_hooks_uid_delete) | **DELETE** /workspaces/{workspace}/hooks/{uid} | Delete a webhook for a workspace
[**workspaces_workspace_hooks_uid_get**](WebhooksApi.md#workspaces_workspace_hooks_uid_get) | **GET** /workspaces/{workspace}/hooks/{uid} | Get a webhook for a workspace
[**workspaces_workspace_hooks_uid_put**](WebhooksApi.md#workspaces_workspace_hooks_uid_put) | **PUT** /workspaces/{workspace}/hooks/{uid} | Update a webhook for a workspace



## hook_events_get

> models::SubjectTypes hook_events_get()
Get a webhook resource

Returns the webhook resource or subject types on which webhooks can be registered.  Each resource/subject type contains an `events` link that returns the paginated list of specific events each individual subject type can emit.  This endpoint is publicly accessible and does not require authentication or scopes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SubjectTypes**](subject_types.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hook_events_subject_type_get

> models::PaginatedHookEvents hook_events_subject_type_get(subject_type)
List subscribable webhook types

Returns a paginated list of all valid webhook events for the specified entity. **The team and user webhooks are deprecated, and you should use workspace instead. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).**  This is public data that does not require any scopes or authentication.  NOTE: The example response is a truncated response object for the `workspace` `subject_type`. We return the same structure for the other `subject_type` objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_type** | **String** | A resource or subject type. | [required] |

### Return type

[**models::PaginatedHookEvents**](paginated_hook_events.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_hooks_get

> models::PaginatedWebhookSubscriptions repositories_workspace_repo_slug_hooks_get(repo_slug, workspace)
List webhooks for a repository

Returns a paginated list of webhooks installed on this repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedWebhookSubscriptions**](paginated_webhook_subscriptions.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_hooks_post

> models::WebhookSubscription repositories_workspace_repo_slug_hooks_post(repo_slug, workspace)
Create a webhook for a repository

Creates a new webhook on the specified repository.  Example:  ``` $ curl -X POST -u credentials -H 'Content-Type: application/json'   https://api.bitbucket.org/2.0/repositories/my-workspace/my-repo-slug/hooks   -d '     {       \"description\": \"Webhook Description\",       \"url\": \"https://example.com/\",       \"active\": true,       \"secret\": \"this is a really bad secret\",       \"events\": [         \"repo:push\",         \"issue:created\",         \"issue:updated\"       ]     }' ```  When the `secret` is provided it will be used as the key to generate a HMAC digest value sent in the `X-Hub-Signature` header at delivery time. Passing a `null` or empty `secret` or not passing a `secret` will leave the webhook's secret unset. Bitbucket only generates the `X-Hub-Signature` when the webhook's secret is set.  Note that this call requires the webhook scope, as well as any scope that applies to the events that the webhook subscribes to. In the example above that means: `webhook`, `repository` and `issue`.  Also note that the `url` must properly resolve and cannot be an internal, non-routed address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::WebhookSubscription**](webhook_subscription.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_hooks_uid_delete

> repositories_workspace_repo_slug_hooks_uid_delete(repo_slug, uid, workspace)
Delete a webhook for a repository

Deletes the specified webhook subscription from the given repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**uid** | **String** | Installed webhook's ID | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_hooks_uid_get

> models::WebhookSubscription repositories_workspace_repo_slug_hooks_uid_get(repo_slug, uid, workspace)
Get a webhook for a repository

Returns the webhook with the specified id installed on the specified repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**uid** | **String** | Installed webhook's ID | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::WebhookSubscription**](webhook_subscription.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_hooks_uid_put

> models::WebhookSubscription repositories_workspace_repo_slug_hooks_uid_put(repo_slug, uid, workspace)
Update a webhook for a repository

Updates the specified webhook subscription.  The following properties can be mutated:  * `description` * `url` * `secret` * `active` * `events`  The hook's secret is used as a key to generate the HMAC hex digest sent in the `X-Hub-Signature` header at delivery time. This signature is only generated when the hook has a secret.  Set the hook's secret by passing the new value in the `secret` field. Passing a `null` value in the `secret` field will remove the secret from the hook. The hook's secret can be left unchanged by not passing the `secret` field in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**uid** | **String** | Installed webhook's ID | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::WebhookSubscription**](webhook_subscription.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_hooks_get

> models::PaginatedWebhookSubscriptions workspaces_workspace_hooks_get(workspace)
List webhooks for a workspace

Returns a paginated list of webhooks installed on this workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedWebhookSubscriptions**](paginated_webhook_subscriptions.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_hooks_post

> models::WebhookSubscription workspaces_workspace_hooks_post(workspace)
Create a webhook for a workspace

Creates a new webhook on the specified workspace.  Workspace webhooks are fired for events from all repositories contained by that workspace.  Example:  ``` $ curl -X POST -u credentials -H 'Content-Type: application/json'   https://api.bitbucket.org/2.0/workspaces/my-workspace/hooks   -d '     {       \"description\": \"Webhook Description\",       \"url\": \"https://example.com/\",       \"active\": true,       \"secret\": \"this is a really bad secret\",       \"events\": [         \"repo:push\",         \"issue:created\",         \"issue:updated\"       ]     }' ```  When the `secret` is provided it will be used as the key to generate a HMAC digest value sent in the `X-Hub-Signature` header at delivery time. Passing a `null` or empty `secret` or not passing a `secret` will leave the webhook's secret unset. Bitbucket only generates the `X-Hub-Signature` when the webhook's secret is set.  This call requires the webhook scope, as well as any scope that applies to the events that the webhook subscribes to. In the example above that means: `webhook`, `repository` and `issue`.  The `url` must properly resolve and cannot be an internal, non-routed address.  Only workspace owners can install webhooks on workspaces.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::WebhookSubscription**](webhook_subscription.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_hooks_uid_delete

> workspaces_workspace_hooks_uid_delete(uid, workspace)
Delete a webhook for a workspace

Deletes the specified webhook subscription from the given workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | Installed webhook's ID | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_hooks_uid_get

> models::WebhookSubscription workspaces_workspace_hooks_uid_get(uid, workspace)
Get a webhook for a workspace

Returns the webhook with the specified id installed on the given workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | Installed webhook's ID | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::WebhookSubscription**](webhook_subscription.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_hooks_uid_put

> models::WebhookSubscription workspaces_workspace_hooks_uid_put(uid, workspace)
Update a webhook for a workspace

Updates the specified webhook subscription.  The following properties can be mutated:  * `description` * `url` * `secret` * `active` * `events`  The hook's secret is used as a key to generate the HMAC hex digest sent in the `X-Hub-Signature` header at delivery time. This signature is only generated when the hook has a secret.  Set the hook's secret by passing the new value in the `secret` field. Passing a `null` value in the `secret` field will remove the secret from the hook. The hook's secret can be left unchanged by not passing the `secret` field in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | Installed webhook's ID | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::WebhookSubscription**](webhook_subscription.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

