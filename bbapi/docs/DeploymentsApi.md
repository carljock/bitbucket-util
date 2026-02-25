# \DeploymentsApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_environment**](DeploymentsApi.md#create_environment) | **POST** /repositories/{workspace}/{repo_slug}/environments | Create an environment
[**delete_environment_for_repository**](DeploymentsApi.md#delete_environment_for_repository) | **DELETE** /repositories/{workspace}/{repo_slug}/environments/{environment_uuid} | Delete an environment
[**get_deployment_for_repository**](DeploymentsApi.md#get_deployment_for_repository) | **GET** /repositories/{workspace}/{repo_slug}/deployments/{deployment_uuid} | Get a deployment
[**get_deployments_for_repository**](DeploymentsApi.md#get_deployments_for_repository) | **GET** /repositories/{workspace}/{repo_slug}/deployments | List deployments
[**get_environment_for_repository**](DeploymentsApi.md#get_environment_for_repository) | **GET** /repositories/{workspace}/{repo_slug}/environments/{environment_uuid} | Get an environment
[**get_environments_for_repository**](DeploymentsApi.md#get_environments_for_repository) | **GET** /repositories/{workspace}/{repo_slug}/environments | List environments
[**repositories_workspace_repo_slug_deploy_keys_get**](DeploymentsApi.md#repositories_workspace_repo_slug_deploy_keys_get) | **GET** /repositories/{workspace}/{repo_slug}/deploy-keys | List repository deploy keys
[**repositories_workspace_repo_slug_deploy_keys_key_id_delete**](DeploymentsApi.md#repositories_workspace_repo_slug_deploy_keys_key_id_delete) | **DELETE** /repositories/{workspace}/{repo_slug}/deploy-keys/{key_id} | Delete a repository deploy key
[**repositories_workspace_repo_slug_deploy_keys_key_id_get**](DeploymentsApi.md#repositories_workspace_repo_slug_deploy_keys_key_id_get) | **GET** /repositories/{workspace}/{repo_slug}/deploy-keys/{key_id} | Get a repository deploy key
[**repositories_workspace_repo_slug_deploy_keys_key_id_put**](DeploymentsApi.md#repositories_workspace_repo_slug_deploy_keys_key_id_put) | **PUT** /repositories/{workspace}/{repo_slug}/deploy-keys/{key_id} | Update a repository deploy key
[**repositories_workspace_repo_slug_deploy_keys_post**](DeploymentsApi.md#repositories_workspace_repo_slug_deploy_keys_post) | **POST** /repositories/{workspace}/{repo_slug}/deploy-keys | Add a repository deploy key
[**update_environment_for_repository**](DeploymentsApi.md#update_environment_for_repository) | **POST** /repositories/{workspace}/{repo_slug}/environments/{environment_uuid}/changes | Update an environment
[**workspaces_workspace_projects_project_key_deploy_keys_get**](DeploymentsApi.md#workspaces_workspace_projects_project_key_deploy_keys_get) | **GET** /workspaces/{workspace}/projects/{project_key}/deploy-keys | List project deploy keys
[**workspaces_workspace_projects_project_key_deploy_keys_key_id_delete**](DeploymentsApi.md#workspaces_workspace_projects_project_key_deploy_keys_key_id_delete) | **DELETE** /workspaces/{workspace}/projects/{project_key}/deploy-keys/{key_id} | Delete a deploy key from a project
[**workspaces_workspace_projects_project_key_deploy_keys_key_id_get**](DeploymentsApi.md#workspaces_workspace_projects_project_key_deploy_keys_key_id_get) | **GET** /workspaces/{workspace}/projects/{project_key}/deploy-keys/{key_id} | Get a project deploy key
[**workspaces_workspace_projects_project_key_deploy_keys_post**](DeploymentsApi.md#workspaces_workspace_projects_project_key_deploy_keys_post) | **POST** /workspaces/{workspace}/projects/{project_key}/deploy-keys | Create a project deploy key



## create_environment

> models::DeploymentEnvironment create_environment(workspace, repo_slug, deployment_environment)
Create an environment

Create an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**deployment_environment** | [**DeploymentEnvironment**](DeploymentEnvironment.md) | The environment to create. | [required] |

### Return type

[**models::DeploymentEnvironment**](deployment_environment.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_environment_for_repository

> delete_environment_for_repository(workspace, repo_slug, environment_uuid)
Delete an environment

Delete an environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**environment_uuid** | **String** | The environment UUID. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment_for_repository

> models::Deployment get_deployment_for_repository(workspace, repo_slug, deployment_uuid)
Get a deployment

Retrieve a deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**deployment_uuid** | **String** | The deployment UUID. | [required] |

### Return type

[**models::Deployment**](deployment.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployments_for_repository

> models::PaginatedDeployments get_deployments_for_repository(workspace, repo_slug)
List deployments

Find deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

[**models::PaginatedDeployments**](paginated_deployments.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment_for_repository

> models::DeploymentEnvironment get_environment_for_repository(workspace, repo_slug, environment_uuid)
Get an environment

Retrieve an environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**environment_uuid** | **String** | The environment UUID. | [required] |

### Return type

[**models::DeploymentEnvironment**](deployment_environment.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environments_for_repository

> models::PaginatedEnvironments get_environments_for_repository(workspace, repo_slug)
List environments

Find environments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

[**models::PaginatedEnvironments**](paginated_environments.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_deploy_keys_get

> models::PaginatedDeployKeys repositories_workspace_repo_slug_deploy_keys_get(repo_slug, workspace)
List repository deploy keys

Returns all deploy-keys belonging to a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedDeployKeys**](paginated_deploy_keys.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_deploy_keys_key_id_delete

> repositories_workspace_repo_slug_deploy_keys_key_id_delete(key_id, repo_slug, workspace)
Delete a repository deploy key

This deletes a deploy key from a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The key ID matching the deploy key. | [required] |
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


## repositories_workspace_repo_slug_deploy_keys_key_id_get

> models::DeployKey repositories_workspace_repo_slug_deploy_keys_key_id_get(key_id, repo_slug, workspace)
Get a repository deploy key

Returns the deploy key belonging to a specific key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The key ID matching the deploy key. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::DeployKey**](deploy_key.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_deploy_keys_key_id_put

> models::DeployKey repositories_workspace_repo_slug_deploy_keys_key_id_put(key_id, repo_slug, workspace)
Update a repository deploy key

Create a new deploy key in a repository.  The same key needs to be passed in but the comment and label can change.  Example: ``` $ curl -X PUT \\ -H \"Authorization <auth header>\" \\ -H \"Content-type: application/json\" \\ https://api.bitbucket.org/2.0/repositories/mleu/test/deploy-keys/1234 -d \\ '{     \"label\": \"newlabel\",     \"key\": \"ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDAK/b1cHHDr/TEV1JGQl+WjCwStKG6Bhrv0rFpEsYlyTBm1fzN0VOJJYn4ZOPCPJwqse6fGbXntEs+BbXiptR+++HycVgl65TMR0b5ul5AgwrVdZdT7qjCOCgaSV74/9xlHDK8oqgGnfA7ZoBBU+qpVyaloSjBdJfLtPY/xqj4yHnXKYzrtn/uFc4Kp9Tb7PUg9Io3qohSTGJGVHnsVblq/rToJG7L5xIo0OxK0SJSQ5vuId93ZuFZrCNMXj8JDHZeSEtjJzpRCBEXHxpOPhAcbm4MzULgkFHhAVgp4JbkrT99/wpvZ7r9AdkTg7HGqL3rlaDrEcWfL7Lu6TnhBdq5 newcomment\", }' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The key ID matching the deploy key. | [required] |
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::DeployKey**](deploy_key.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repositories_workspace_repo_slug_deploy_keys_post

> models::DeployKey repositories_workspace_repo_slug_deploy_keys_post(repo_slug, workspace)
Add a repository deploy key

Create a new deploy key in a repository. Note: If authenticating a deploy key with an OAuth consumer, any changes to the OAuth consumer will subsequently invalidate the deploy key.   Example: ``` $ curl -X POST \\ -H \"Authorization <auth header>\" \\ -H \"Content-type: application/json\" \\ https://api.bitbucket.org/2.0/repositories/mleu/test/deploy-keys -d \\ '{     \"key\": \"ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDAK/b1cHHDr/TEV1JGQl+WjCwStKG6Bhrv0rFpEsYlyTBm1fzN0VOJJYn4ZOPCPJwqse6fGbXntEs+BbXiptR+++HycVgl65TMR0b5ul5AgwrVdZdT7qjCOCgaSV74/9xlHDK8oqgGnfA7ZoBBU+qpVyaloSjBdJfLtPY/xqj4yHnXKYzrtn/uFc4Kp9Tb7PUg9Io3qohSTGJGVHnsVblq/rToJG7L5xIo0OxK0SJSQ5vuId93ZuFZrCNMXj8JDHZeSEtjJzpRCBEXHxpOPhAcbm4MzULgkFHhAVgp4JbkrT99/wpvZ7r9AdkTg7HGqL3rlaDrEcWfL7Lu6TnhBdq5 mleu@C02W454JHTD8\",     \"label\": \"mydeploykey\" }' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_slug** | **String** | This can either be the repository slug or the UUID of the repository, surrounded by curly-braces, for example: `{repository UUID}`.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::DeployKey**](deploy_key.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_environment_for_repository

> update_environment_for_repository(workspace, repo_slug, environment_uuid)
Update an environment

Update an environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**environment_uuid** | **String** | The environment UUID. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_deploy_keys_get

> models::PaginatedProjectDeployKeys workspaces_workspace_projects_project_key_deploy_keys_get(project_key, workspace)
List project deploy keys

Returns all deploy keys belonging to a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual `key` assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::PaginatedProjectDeployKeys**](paginated_project_deploy_keys.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_deploy_keys_key_id_delete

> workspaces_workspace_projects_project_key_deploy_keys_key_id_delete(key_id, project_key, workspace)
Delete a deploy key from a project

This deletes a deploy key from a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The key ID matching the project deploy key. | [required] |
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


## workspaces_workspace_projects_project_key_deploy_keys_key_id_get

> models::ProjectDeployKey workspaces_workspace_projects_project_key_deploy_keys_key_id_get(key_id, project_key, workspace)
Get a project deploy key

Returns the deploy key belonging to a specific key ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The key ID matching the project deploy key. | [required] |
**project_key** | **String** | The project in question. This is the actual `key` assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::ProjectDeployKey**](project_deploy_key.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_workspace_projects_project_key_deploy_keys_post

> models::ProjectDeployKey workspaces_workspace_projects_project_key_deploy_keys_post(project_key, workspace)
Create a project deploy key

Create a new deploy key in a project.  Example: ``` $ curl -X POST \\ -H \"Authorization <auth header>\" \\ -H \"Content-type: application/json\" \\ https://api.bitbucket.org/2.0/workspaces/standard/projects/TEST_PROJECT/deploy-keys/ -d \\ '{     \"key\": \"ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDAK/b1cHHDr/TEV1JGQl+WjCwStKG6Bhrv0rFpEsYlyTBm1fzN0VOJJYn4ZOPCPJwqse6fGbXntEs+BbXiptR+++HycVgl65TMR0b5ul5AgwrVdZdT7qjCOCgaSV74/9xlHDK8oqgGnfA7ZoBBU+qpVyaloSjBdJfLtPY/xqj4yHnXKYzrtn/uFc4Kp9Tb7PUg9Io3qohSTGJGVHnsVblq/rToJG7L5xIo0OxK0SJSQ5vuId93ZuFZrCNMXj8JDHZeSEtjJzpRCBEXHxpOPhAcbm4MzULgkFHhAVgp4JbkrT99/wpvZ7r9AdkTg7HGqL3rlaDrEcWfL7Lu6TnhBdq5 mleu@C02W454JHTD8\",     \"label\": \"mydeploykey\" }' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | **String** | The project in question. This is the actual `key` assigned to the project.  | [required] |
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example: `{workspace UUID}`.  | [required] |

### Return type

[**models::ProjectDeployKey**](project_deploy_key.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

