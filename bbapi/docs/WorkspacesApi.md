# \WorkspacesApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_permissions_workspaces_get**](WorkspacesApi.md#user_permissions_workspaces_get) | **GET** /user/permissions/workspaces | List workspaces for the current user
[**user_workspaces_get**](WorkspacesApi.md#user_workspaces_get) | **GET** /user/workspaces | List workspaces for the current user
[**user_workspaces_workspace_permission_get**](WorkspacesApi.md#user_workspaces_workspace_permission_get) | **GET** /user/workspaces/{workspace}/permission | Get user permission on a workspace
[**workspaces_get**](WorkspacesApi.md#workspaces_get) | **GET** /workspaces | List workspaces for user
[**workspaces_workspace_get**](WorkspacesApi.md#workspaces_workspace_get) | **GET** /workspaces/{workspace} | Get a workspace
[**workspaces_workspace_hooks_get**](WorkspacesApi.md#workspaces_workspace_hooks_get) | **GET** /workspaces/{workspace}/hooks | List webhooks for a workspace
[**workspaces_workspace_hooks_post**](WorkspacesApi.md#workspaces_workspace_hooks_post) | **POST** /workspaces/{workspace}/hooks | Create a webhook for a workspace
[**workspaces_workspace_hooks_uid_delete**](WorkspacesApi.md#workspaces_workspace_hooks_uid_delete) | **DELETE** /workspaces/{workspace}/hooks/{uid} | Delete a webhook for a workspace
[**workspaces_workspace_hooks_uid_get**](WorkspacesApi.md#workspaces_workspace_hooks_uid_get) | **GET** /workspaces/{workspace}/hooks/{uid} | Get a webhook for a workspace
[**workspaces_workspace_hooks_uid_put**](WorkspacesApi.md#workspaces_workspace_hooks_uid_put) | **PUT** /workspaces/{workspace}/hooks/{uid} | Update a webhook for a workspace
[**workspaces_workspace_members_get**](WorkspacesApi.md#workspaces_workspace_members_get) | **GET** /workspaces/{workspace}/members | List users in a workspace
[**workspaces_workspace_members_member_get**](WorkspacesApi.md#workspaces_workspace_members_member_get) | **GET** /workspaces/{workspace}/members/{member} | Get user membership for a workspace
[**workspaces_workspace_permissions_get**](WorkspacesApi.md#workspaces_workspace_permissions_get) | **GET** /workspaces/{workspace}/permissions | List user permissions in a workspace
[**workspaces_workspace_permissions_repositories_get**](WorkspacesApi.md#workspaces_workspace_permissions_repositories_get) | **GET** /workspaces/{workspace}/permissions/repositories | List all repository permissions for a workspace
[**workspaces_workspace_permissions_repositories_repo_slug_get**](WorkspacesApi.md#workspaces_workspace_permissions_repositories_repo_slug_get) | **GET** /workspaces/{workspace}/permissions/repositories/{repo_slug} | List a repository permissions for a workspace
[**workspaces_workspace_projects_get**](WorkspacesApi.md#workspaces_workspace_projects_get) | **GET** /workspaces/{workspace}/projects | List projects in a workspace
[**workspaces_workspace_projects_project_key_get**](WorkspacesApi.md#workspaces_workspace_projects_project_key_get) | **GET** /workspaces/{workspace}/projects/{project_key} | Get a project for a workspace
[**workspaces_workspace_pullrequests_selected_user_get**](WorkspacesApi.md#workspaces_workspace_pullrequests_selected_user_get) | **GET** /workspaces/{workspace}/pullrequests/{selected_user} | List workspace pull requests for a user
[**workspaces_workspace_settings_gpg_public_key_get**](WorkspacesApi.md#workspaces_workspace_settings_gpg_public_key_get) | **GET** /workspaces/{workspace}/settings/gpg/public-key | Get the workspace system GPG public key(s)



## user_permissions_workspaces_get

> models::PaginatedWorkspaceMemberships user_permissions_workspaces_get(q, sort)
List workspaces for the current user

**This endpoint is deprecated. Please use the supported alternatives:** * [List workspaces for user](/cloud/bitbucket/rest/api-group-workspaces/#api-user-workspaces-get) * [Get user permission on a workspace](/cloud/bitbucket/rest/api-group-workspaces/#api-user-workspaces-workspace-permission-get)  Returns an object for each workspace the caller is a member of, and their effective role - the highest level of privilege the caller has. If a user is a member of multiple groups with distinct roles, only the highest level is returned.  Permissions can be:  * `owner` * `collaborator` * `member`  **The `collaborator` role is being removed from the Bitbucket Cloud API. For more information, see the [deprecation announcement](/cloud/bitbucket/deprecation-notice-collaborator-role/).**  **When you move your administration from Bitbucket Cloud to admin.atlassian.com, the following fields on `workspace_membership` will no longer be present: `last_accessed` and `added_on`. See the [deprecation announcement](/cloud/bitbucket/announcement-breaking-change-workspace-membership/).**  Results may be further [filtered or sorted](/cloud/bitbucket/rest/intro/#filtering) by workspace or permission by adding the following query string parameters:  * `q=workspace.slug=\"bbworkspace1\"` or `q=permission=\"owner\"` * `sort=workspace.slug`  Note that the query parameter values need to be URL escaped so that `=` would become `%3D`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> |  Query string to narrow down the response. See [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering) for details. |  |
**sort** | Option<**String**> |  Name of a response property to sort results. See [filtering and sorting](/cloud/bitbucket/rest/intro/#sorting-query-results) for details.  |  |

### Return type

[**models::PaginatedWorkspaceMemberships**](paginated_workspace_memberships.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_workspaces_get

> models::PaginatedWorkspaceAccess user_workspaces_get(sort, administrator)
List workspaces for the current user

Returns an object for each workspace accessible to the caller. This object also contains details on whether the caller has admin permissions on the workspace (`\"administrator\" = true`) or not (`\"administrator\" = false`).  Queries support filtering based on administrator permissions, [sorting](/cloud/bitbucket/rest/intro/#sorting-query-results) or [filtering](/cloud/bitbucket/rest/intro/#filtering) by `slug`. Results can be [paginated](/cloud/bitbucket/rest/intro/#pagination).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Name of a response property to sort results (only slug is supported). |  |
**administrator** | Option<**bool**> | Filter workspaces based on which ones the caller has admin permissions or not. |  |

### Return type

[**models::PaginatedWorkspaceAccess**](paginated_workspace_access.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_workspaces_workspace_permission_get

> models::WorkspaceMembership user_workspaces_workspace_permission_get(workspace)
Get user permission on a workspace

Returns the caller's effective role; as in, the highest level of privilege the caller has for the workspace. If the calling user is a member of multiple groups with distinct roles, only the highest level is returned.  Permissions can be:  * `owner` * `create-project` * `collaborator` (deprecated; see this [deprecation announcement](/cloud/bitbucket/deprecation-notice-collaborator-role/) for more details) * `member`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::WorkspaceMembership**](workspace_membership.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_get

> models::PaginatedWorkspaces workspaces_get(role, q, sort)
List workspaces for user

**This endpoint is deprecated. Please use the [supported alternative](/cloud/bitbucket/rest/api-group-workspaces/#api-user-workspaces-get).**  Returns a list of workspaces accessible by the authenticated user.  Results may be further [filtered or sorted](/cloud/bitbucket/rest/intro/#filtering) by workspace or permission by adding the following query string parameters:  * `q=slug=\"bbworkspace1\"` or `q=is_private=true` * `sort=created_on`  Note that the query parameter values need to be URL escaped so that `=` would become `%3D`.  **The `collaborator` role is being removed from the Bitbucket Cloud API. For more information, see the [deprecation announcement](/cloud/bitbucket/deprecation-notice-collaborator-role/).**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | Option<**String**> |              Filters the workspaces based on the authenticated user's role on each workspace.              * **member**: returns a list of all the workspaces which the caller is a member of                 at least one workspace group or repository             * **collaborator**: returns a list of workspaces which the caller has write access                 to at least one repository in the workspace             * **owner**: returns a list of workspaces which the caller has administrator access              |  |
**q** | Option<**String**> |  Query string to narrow down the response. See [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering) for details. |  |
**sort** | Option<**String**> |  Name of a response property to sort results. See [filtering and sorting](/cloud/bitbucket/rest/intro/#sorting-query-results) for details.  |  |

### Return type

[**models::PaginatedWorkspaces**](paginated_workspaces.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_get

> models::Workspace workspaces_workspace_get(workspace)
Get a workspace

Returns the requested workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Workspace**](workspace.md)

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


## workspaces_workspace_members_get

> models::PaginatedWorkspaceMemberships workspaces_workspace_members_get(workspace)
List users in a workspace

Returns all members of the requested workspace.  This endpoint additionally supports [filtering](/cloud/bitbucket/rest/intro/#filtering) by email address, if called by a workspace administrator, integration or workspace access token. This is done by adding the following query string parameter:  * `q=user.email IN (\"user1@org.com\",\"user2@org.com\")`  When filtering by email, you can query up to 90 addresses at a time. Note that the query parameter values need to be URL escaped, so the final query string should be:  * `q=user.email%20IN%20(%22user1@org.com%22,%22user2@org.com%22)`  Email addresses that you filter by (and only these email addresses) can be included in the response using the `fields` query parameter:  * `&fields=+values.user.email` - add the `email` field to the default `user` response object * `&fields=values.user.email,values.user.account_id` - only return user email addresses and account IDs  Once again, all query parameter values must be URL escaped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedWorkspaceMemberships**](paginated_workspace_memberships.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_members_member_get

> models::WorkspaceMembership workspaces_workspace_members_member_get(member, workspace)
Get user membership for a workspace

Returns the workspace membership, which includes a `User` object for the member and a `Workspace` object for the requested workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member** | **String** | Member's UUID or Atlassian ID. | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::WorkspaceMembership**](workspace_membership.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_permissions_get

> models::PaginatedWorkspaceMemberships workspaces_workspace_permissions_get(workspace, q)
List user permissions in a workspace

Returns the list of members in a workspace and their permission levels. Permission can be: * `owner` * `collaborator` * `member`  **The `collaborator` role is being removed from the Bitbucket Cloud API. For more information, see the [deprecation announcement](/cloud/bitbucket/deprecation-notice-collaborator-role/).**  **When you move your administration from Bitbucket Cloud to admin.atlassian.com, the following fields on `workspace_membership` will no longer be present: `last_accessed` and `added_on`. See the [deprecation announcement](/cloud/bitbucket/announcement-breaking-change-workspace-membership/).**  Results may be further [filtered](/cloud/bitbucket/rest/intro/#filtering) by permission by adding the following query string parameters:  * `q=permission=\"owner\"`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**q** | Option<**String**> |  Query string to narrow down the response as per [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering). |  |

### Return type

[**models::PaginatedWorkspaceMemberships**](paginated_workspace_memberships.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_permissions_repositories_get

> models::PaginatedRepositoryPermissions workspaces_workspace_permissions_repositories_get(workspace, q, sort)
List all repository permissions for a workspace

Returns an object for each repository permission for all of a workspace's repositories.  Permissions returned are effective permissions: the highest level of permission the user has. This does not distinguish between direct and indirect (group) privileges.  Only users with admin permission for the team may access this resource.  Permissions can be:  * `admin` * `write` * `read`  Results may be further [filtered or sorted](/cloud/bitbucket/rest/intro/#filtering) by repository, user, or permission by adding the following query string parameters:  * `q=repository.name=\"geordi\"` or `q=permission>\"read\"` * `sort=user.display_name`  Note that the query parameter values need to be URL escaped so that `=` would become `%3D`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**q** | Option<**String**> |  Query string to narrow down the response as per [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering). |  |
**sort** | Option<**String**> |  Name of a response property sort the result by as per [filtering and sorting](/cloud/bitbucket/rest/intro/#sorting-query-results).  |  |

### Return type

[**models::PaginatedRepositoryPermissions**](paginated_repository_permissions.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_permissions_repositories_repo_slug_get

> models::PaginatedRepositoryPermissions workspaces_workspace_permissions_repositories_repo_slug_get(repo_slug, workspace, q, sort)
List a repository permissions for a workspace

Returns an object for the repository permission of each user in the requested repository.  Permissions returned are effective permissions: the highest level of permission the user has. This does not distinguish between direct and indirect (group) privileges.  Only users with admin permission for the repository may access this resource.  Permissions can be:  * `admin` * `write` * `read`  Results may be further [filtered or sorted](/cloud/bitbucket/rest/intro/#filtering) by user, or permission by adding the following query string parameters:  * `q=permission>\"read\"` * `sort=user.display_name`  Note that the query parameter values need to be URL escaped so that `=` would become `%3D`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**q** | Option<**String**> |  Query string to narrow down the response as per [filtering and sorting](/cloud/bitbucket/rest/intro/#filtering). |  |
**sort** | Option<**String**> |  Name of a response property sort the result by as per [filtering and sorting](/cloud/bitbucket/rest/intro/#sorting-query-results).  |  |

### Return type

[**models::PaginatedRepositoryPermissions**](paginated_repository_permissions.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_get

> models::PaginatedProjects workspaces_workspace_projects_get(workspace)
List projects in a workspace

Returns the list of projects in this workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedProjects**](paginated_projects.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_get

> models::Project workspaces_workspace_projects_project_key_get(project_key, workspace)
Get a project for a workspace

Returns the requested project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual `key` assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::Project**](project.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
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


## workspaces_workspace_settings_gpg_public_key_get

> workspaces_workspace_settings_gpg_public_key_get(workspace)
Get the workspace system GPG public key(s)

Returns the system public GPG key(s). In most cases a single key is returned. During a key rotation period, two keys may be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

