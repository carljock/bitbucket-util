# \ProjectsApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workspaces_workspace_projects_post**](ProjectsApi.md#workspaces_workspace_projects_post) | **POST** /workspaces/{workspace}/projects | Create a project in a workspace
[**workspaces_workspace_projects_project_key_default_reviewers_get**](ProjectsApi.md#workspaces_workspace_projects_project_key_default_reviewers_get) | **GET** /workspaces/{workspace}/projects/{project_key}/default-reviewers | List the default reviewers in a project
[**workspaces_workspace_projects_project_key_default_reviewers_selected_user_delete**](ProjectsApi.md#workspaces_workspace_projects_project_key_default_reviewers_selected_user_delete) | **DELETE** /workspaces/{workspace}/projects/{project_key}/default-reviewers/{selected_user} | Remove the specific user from the project's default reviewers
[**workspaces_workspace_projects_project_key_default_reviewers_selected_user_get**](ProjectsApi.md#workspaces_workspace_projects_project_key_default_reviewers_selected_user_get) | **GET** /workspaces/{workspace}/projects/{project_key}/default-reviewers/{selected_user} | Get a default reviewer
[**workspaces_workspace_projects_project_key_default_reviewers_selected_user_put**](ProjectsApi.md#workspaces_workspace_projects_project_key_default_reviewers_selected_user_put) | **PUT** /workspaces/{workspace}/projects/{project_key}/default-reviewers/{selected_user} | Add the specific user as a default reviewer for the project
[**workspaces_workspace_projects_project_key_delete**](ProjectsApi.md#workspaces_workspace_projects_project_key_delete) | **DELETE** /workspaces/{workspace}/projects/{project_key} | Delete a project for a workspace
[**workspaces_workspace_projects_project_key_get**](ProjectsApi.md#workspaces_workspace_projects_project_key_get) | **GET** /workspaces/{workspace}/projects/{project_key} | Get a project for a workspace
[**workspaces_workspace_projects_project_key_permissions_config_groups_get**](ProjectsApi.md#workspaces_workspace_projects_project_key_permissions_config_groups_get) | **GET** /workspaces/{workspace}/projects/{project_key}/permissions-config/groups | List explicit group permissions for a project
[**workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_delete**](ProjectsApi.md#workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_delete) | **DELETE** /workspaces/{workspace}/projects/{project_key}/permissions-config/groups/{group_slug} | Delete an explicit group permission for a project
[**workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_get**](ProjectsApi.md#workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_get) | **GET** /workspaces/{workspace}/projects/{project_key}/permissions-config/groups/{group_slug} | Get an explicit group permission for a project
[**workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_put**](ProjectsApi.md#workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_put) | **PUT** /workspaces/{workspace}/projects/{project_key}/permissions-config/groups/{group_slug} | Update an explicit group permission for a project
[**workspaces_workspace_projects_project_key_permissions_config_users_get**](ProjectsApi.md#workspaces_workspace_projects_project_key_permissions_config_users_get) | **GET** /workspaces/{workspace}/projects/{project_key}/permissions-config/users | List explicit user permissions for a project
[**workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_delete**](ProjectsApi.md#workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_delete) | **DELETE** /workspaces/{workspace}/projects/{project_key}/permissions-config/users/{selected_user_id} | Delete an explicit user permission for a project
[**workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_get**](ProjectsApi.md#workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_get) | **GET** /workspaces/{workspace}/projects/{project_key}/permissions-config/users/{selected_user_id} | Get an explicit user permission for a project
[**workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_put**](ProjectsApi.md#workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_put) | **PUT** /workspaces/{workspace}/projects/{project_key}/permissions-config/users/{selected_user_id} | Update an explicit user permission for a project
[**workspaces_workspace_projects_project_key_put**](ProjectsApi.md#workspaces_workspace_projects_project_key_put) | **PUT** /workspaces/{workspace}/projects/{project_key} | Update a project for a workspace



## workspaces_workspace_projects_post

> models::Project workspaces_workspace_projects_post(workspace, project)
Create a project in a workspace

Creates a new project.  Note that the avatar has to be embedded as either a data-url or a URL to an external image as shown in the examples below:  ``` $ body=$(cat << EOF {     \"name\": \"Mars Project\",     \"key\": \"MARS\",     \"description\": \"Software for colonizing mars.\",     \"links\": {         \"avatar\": {             \"href\": \"data:image/gif;base64,R0lGODlhEAAQAMQAAORHHOVSKudfOulrSOp3WOyDZu6QdvCchPGolfO0o/...\"         }     },     \"is_private\": false } EOF ) $ curl -H \"Content-Type: application/json\" \\        -X POST \\        -d \"$body\" \\        https://api.bitbucket.org/2.0/workspaces/teams-in-space/projects/ | jq . {   // Serialized project document } ```  or even:  ``` $ body=$(cat << EOF {     \"name\": \"Mars Project\",     \"key\": \"MARS\",     \"description\": \"Software for colonizing mars.\",     \"links\": {         \"avatar\": {             \"href\": \"http://i.imgur.com/72tRx4w.gif\"         }     },     \"is_private\": false } EOF ) $ curl -H \"Content-Type: application/json\" \\        -X POST \\        -d \"$body\" \\        https://api.bitbucket.org/2.0/workspaces/teams-in-space/projects/ | jq . {   // Serialized project document } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**models::Project**](project.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_default_reviewers_get

> models::PaginatedDefaultReviewerAndType workspaces_workspace_projects_project_key_default_reviewers_get(project_key, workspace)
List the default reviewers in a project

Return a list of all default reviewers for a project. This is a list of users that will be added as default reviewers to pull requests for any repository within the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual `key` assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedDefaultReviewerAndType**](paginated_default_reviewer_and_type.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_default_reviewers_selected_user_delete

> workspaces_workspace_projects_project_key_default_reviewers_selected_user_delete(project_key, selected_user, workspace)
Remove the specific user from the project's default reviewers

Removes a default reviewer from the project.  Example: ``` $ curl https://api.bitbucket.org/2.0/.../default-reviewers/%7Bf0e0e8e9-66c1-4b85-a784-44a9eb9ef1a6%7D  HTTP/1.1 204 ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This can either be the actual `key` assigned to the project or the `UUID` (surrounded by curly-braces (`{}`)).  | [required] |
**selected_user** | **String** | This can either be the username or the UUID of the default reviewer, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_default_reviewers_selected_user_get

> models::User workspaces_workspace_projects_project_key_default_reviewers_selected_user_get(project_key, selected_user, workspace)
Get a default reviewer

Returns the specified default reviewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This can either be the actual `key` assigned to the project or the `UUID` (surrounded by curly-braces (`{}`)).  | [required] |
**selected_user** | **String** | This can either be the username or the UUID of the default reviewer, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::User**](user.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_default_reviewers_selected_user_put

> models::User workspaces_workspace_projects_project_key_default_reviewers_selected_user_put(project_key, selected_user, workspace)
Add the specific user as a default reviewer for the project

Adds the specified user to the project's list of default reviewers. The method is idempotent. Accepts an optional body containing the `uuid` of the user to be added.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This can either be the actual `key` assigned to the project or the `UUID` (surrounded by curly-braces (`{}`)).  | [required] |
**selected_user** | **String** | This can either be the username or the UUID of the default reviewer, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::User**](user.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_delete

> workspaces_workspace_projects_project_key_delete(project_key, workspace)
Delete a project for a workspace

Deletes this project. This is an irreversible operation.  You cannot delete a project that still contains repositories. To delete the project, [delete](/cloud/bitbucket/rest/api-group-repositories/#api-repositories-workspace-repo-slug-delete) or transfer the repositories first.  Example: ``` $ curl -X DELETE https://api.bitbucket.org/2.0/workspaces/bbworkspace1/projects/PROJ ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual `key` assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

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


## workspaces_workspace_projects_project_key_permissions_config_groups_get

> models::PaginatedProjectGroupPermissions workspaces_workspace_projects_project_key_permissions_config_groups_get(project_key, workspace)
List explicit group permissions for a project

Returns a paginated list of explicit group permissions for the given project. This endpoint does not support BBQL features.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual key assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedProjectGroupPermissions**](paginated_project_group_permissions.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_delete

> workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_delete(group_slug, project_key, workspace)
Delete an explicit group permission for a project

Deletes the project group permission between the requested project and group, if one exists.  Only users with admin permission for the project may access this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_slug** | **String** | Slug of the requested group. | [required] |
**project_key** | **String** | The project in question. This is the actual key assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_get

> models::ProjectGroupPermission workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_get(group_slug, project_key, workspace)
Get an explicit group permission for a project

Returns the group permission for a given group and project.  Only users with admin permission for the project may access this resource.  Permissions can be:  * `admin` * `create-repo` * `write` * `read` * `none`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_slug** | **String** | Slug of the requested group. | [required] |
**project_key** | **String** | The project in question. This is the actual key assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::ProjectGroupPermission**](project_group_permission.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_put

> models::ProjectGroupPermission workspaces_workspace_projects_project_key_permissions_config_groups_group_slug_put(group_slug, project_key, workspace, bitbucket_apps_permissions_serializers_project_permission_update_schema)
Update an explicit group permission for a project

Updates the group permission, or grants a new permission if one does not already exist.  Only users with admin permission for the project may access this resource.  Due to security concerns, the JWT and OAuth authentication methods are unsupported. This is to ensure integrations and add-ons are not allowed to change permissions.  Permissions can be:  * `admin` * `create-repo` * `write` * `read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_slug** | **String** | Slug of the requested group. | [required] |
**project_key** | **String** | The project in question. This is the actual key assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**bitbucket_apps_permissions_serializers_project_permission_update_schema** | [**BitbucketAppsPermissionsSerializersProjectPermissionUpdateSchema**](BitbucketAppsPermissionsSerializersProjectPermissionUpdateSchema.md) | The permission to grant | [required] |

### Return type

[**models::ProjectGroupPermission**](project_group_permission.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_permissions_config_users_get

> models::PaginatedProjectUserPermissions workspaces_workspace_projects_project_key_permissions_config_users_get(project_key, workspace)
List explicit user permissions for a project

Returns a paginated list of explicit user permissions for the given project. This endpoint does not support BBQL features.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual key assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedProjectUserPermissions**](paginated_project_user_permissions.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_delete

> workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_delete(project_key, selected_user_id, workspace)
Delete an explicit user permission for a project

Deletes the project user permission between the requested project and user, if one exists.  Only users with admin permission for the project may access this resource.  Due to security concerns, the JWT and OAuth authentication methods are unsupported. This is to ensure integrations and add-ons are not allowed to change permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual key assigned to the project.  | [required] |
**selected_user_id** | **String** | This can either be the username, the user's UUID surrounded by curly-braces, for example: {account UUID}, or the user's Atlassian ID.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_get

> models::ProjectUserPermission workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_get(project_key, selected_user_id, workspace)
Get an explicit user permission for a project

Returns the explicit user permission for a given user and project.  Only users with admin permission for the project may access this resource.  Permissions can be:  * `admin` * `create-repo` * `write` * `read` * `none`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual key assigned to the project.  | [required] |
**selected_user_id** | **String** | This can either be the username, the user's UUID surrounded by curly-braces, for example: {account UUID}, or the user's Atlassian ID.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::ProjectUserPermission**](project_user_permission.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_put

> models::ProjectUserPermission workspaces_workspace_projects_project_key_permissions_config_users_selected_user_id_put(project_key, selected_user_id, workspace, bitbucket_apps_permissions_serializers_project_permission_update_schema)
Update an explicit user permission for a project

Updates the explicit user permission for a given user and project. The selected user must be a member of the workspace, and cannot be the workspace owner.  Only users with admin permission for the project may access this resource.  Due to security concerns, the JWT and OAuth authentication methods are unsupported. This is to ensure integrations and add-ons are not allowed to change permissions.  Permissions can be:  * `admin` * `create-repo` * `write` * `read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual key assigned to the project.  | [required] |
**selected_user_id** | **String** | This can either be the username, the user's UUID surrounded by curly-braces, for example: {account UUID}, or the user's Atlassian ID.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**bitbucket_apps_permissions_serializers_project_permission_update_schema** | [**BitbucketAppsPermissionsSerializersProjectPermissionUpdateSchema**](BitbucketAppsPermissionsSerializersProjectPermissionUpdateSchema.md) | The permission to grant | [required] |

### Return type

[**models::ProjectUserPermission**](project_user_permission.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_put

> models::Project workspaces_workspace_projects_project_key_put(project_key, workspace, project)
Update a project for a workspace

Since this endpoint can be used to both update and to create a project, the request body depends on the intent.  #### Creation  See the POST documentation for the project collection for an example of the request body.  Note: The `key` should not be specified in the body of request (since it is already present in the URL). The `name` is required, everything else is optional.  #### Update  See the POST documentation for the project collection for an example of the request body.  Note: The key is not required in the body (since it is already in the URL). The key may be specified in the body, if the intent is to change the key itself. In such a scenario, the location of the project is changed and is returned in the `Location` header of the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual `key` assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**models::Project**](project.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

