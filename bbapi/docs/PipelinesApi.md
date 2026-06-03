# \PipelinesApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_deployment_variable**](PipelinesApi.md#create_deployment_variable) | **POST** /repositories/{workspace}/{repo_slug}/deployments_config/environments/{environment_uuid}/variables | Create a variable for an environment
[**create_pipeline_for_repository**](PipelinesApi.md#create_pipeline_for_repository) | **POST** /repositories/{workspace}/{repo_slug}/pipelines | Run a pipeline
[**create_pipeline_variable_for_team**](PipelinesApi.md#create_pipeline_variable_for_team) | **POST** /teams/{username}/pipelines_config/variables | Create a variable for a user
[**create_pipeline_variable_for_user**](PipelinesApi.md#create_pipeline_variable_for_user) | **POST** /users/{selected_user}/pipelines_config/variables | Create a variable for a user
[**create_pipeline_variable_for_workspace**](PipelinesApi.md#create_pipeline_variable_for_workspace) | **POST** /workspaces/{workspace}/pipelines-config/variables | Create a variable for a workspace
[**create_repository_pipeline_known_host**](PipelinesApi.md#create_repository_pipeline_known_host) | **POST** /repositories/{workspace}/{repo_slug}/pipelines_config/ssh/known_hosts | Create a known host
[**create_repository_pipeline_schedule**](PipelinesApi.md#create_repository_pipeline_schedule) | **POST** /repositories/{workspace}/{repo_slug}/pipelines_config/schedules | Create a schedule
[**create_repository_pipeline_variable**](PipelinesApi.md#create_repository_pipeline_variable) | **POST** /repositories/{workspace}/{repo_slug}/pipelines_config/variables | Create a variable for a repository
[**create_repository_runner**](PipelinesApi.md#create_repository_runner) | **POST** /repositories/{workspace}/{repo_slug}/pipelines-config/runners | Create repository runner
[**create_workspace_runner**](PipelinesApi.md#create_workspace_runner) | **POST** /workspaces/{workspace}/pipelines-config/runners | Create workspace runner
[**delete_deployment_variable**](PipelinesApi.md#delete_deployment_variable) | **DELETE** /repositories/{workspace}/{repo_slug}/deployments_config/environments/{environment_uuid}/variables/{variable_uuid} | Delete a variable for an environment
[**delete_pipeline_variable_for_team**](PipelinesApi.md#delete_pipeline_variable_for_team) | **DELETE** /teams/{username}/pipelines_config/variables/{variable_uuid} | Delete a variable for a team
[**delete_pipeline_variable_for_user**](PipelinesApi.md#delete_pipeline_variable_for_user) | **DELETE** /users/{selected_user}/pipelines_config/variables/{variable_uuid} | Delete a variable for a user
[**delete_pipeline_variable_for_workspace**](PipelinesApi.md#delete_pipeline_variable_for_workspace) | **DELETE** /workspaces/{workspace}/pipelines-config/variables/{variable_uuid} | Delete a variable for a workspace
[**delete_repository_pipeline_cache**](PipelinesApi.md#delete_repository_pipeline_cache) | **DELETE** /repositories/{workspace}/{repo_slug}/pipelines-config/caches/{cache_uuid} | Delete a cache
[**delete_repository_pipeline_caches**](PipelinesApi.md#delete_repository_pipeline_caches) | **DELETE** /repositories/{workspace}/{repo_slug}/pipelines-config/caches | Delete caches
[**delete_repository_pipeline_key_pair**](PipelinesApi.md#delete_repository_pipeline_key_pair) | **DELETE** /repositories/{workspace}/{repo_slug}/pipelines_config/ssh/key_pair | Delete SSH key pair
[**delete_repository_pipeline_known_host**](PipelinesApi.md#delete_repository_pipeline_known_host) | **DELETE** /repositories/{workspace}/{repo_slug}/pipelines_config/ssh/known_hosts/{known_host_uuid} | Delete a known host
[**delete_repository_pipeline_schedule**](PipelinesApi.md#delete_repository_pipeline_schedule) | **DELETE** /repositories/{workspace}/{repo_slug}/pipelines_config/schedules/{schedule_uuid} | Delete a schedule
[**delete_repository_pipeline_variable**](PipelinesApi.md#delete_repository_pipeline_variable) | **DELETE** /repositories/{workspace}/{repo_slug}/pipelines_config/variables/{variable_uuid} | Delete a variable for a repository
[**delete_repository_runner**](PipelinesApi.md#delete_repository_runner) | **DELETE** /repositories/{workspace}/{repo_slug}/pipelines-config/runners/{runner_uuid} | Delete repository runner
[**delete_workspace_runner**](PipelinesApi.md#delete_workspace_runner) | **DELETE** /workspaces/{workspace}/pipelines-config/runners/{runner_uuid} | Delete workspace runner
[**get_deployment_variables**](PipelinesApi.md#get_deployment_variables) | **GET** /repositories/{workspace}/{repo_slug}/deployments_config/environments/{environment_uuid}/variables | List variables for an environment
[**get_oidc_configuration**](PipelinesApi.md#get_oidc_configuration) | **GET** /workspaces/{workspace}/pipelines-config/identity/oidc/.well-known/openid-configuration | Get OpenID configuration for OIDC in Pipelines
[**get_oidc_keys**](PipelinesApi.md#get_oidc_keys) | **GET** /workspaces/{workspace}/pipelines-config/identity/oidc/keys.json | Get keys for OIDC in Pipelines
[**get_pipeline_container_log**](PipelinesApi.md#get_pipeline_container_log) | **GET** /repositories/{workspace}/{repo_slug}/pipelines/{pipeline_uuid}/steps/{step_uuid}/logs/{log_uuid} | Get the logs for the build container or a service container for a given step of a pipeline.
[**get_pipeline_for_repository**](PipelinesApi.md#get_pipeline_for_repository) | **GET** /repositories/{workspace}/{repo_slug}/pipelines/{pipeline_uuid} | Get a pipeline
[**get_pipeline_step_for_repository**](PipelinesApi.md#get_pipeline_step_for_repository) | **GET** /repositories/{workspace}/{repo_slug}/pipelines/{pipeline_uuid}/steps/{step_uuid} | Get a step of a pipeline
[**get_pipeline_step_log_for_repository**](PipelinesApi.md#get_pipeline_step_log_for_repository) | **GET** /repositories/{workspace}/{repo_slug}/pipelines/{pipeline_uuid}/steps/{step_uuid}/log | Get log file for a step
[**get_pipeline_steps_for_repository**](PipelinesApi.md#get_pipeline_steps_for_repository) | **GET** /repositories/{workspace}/{repo_slug}/pipelines/{pipeline_uuid}/steps | List steps for a pipeline
[**get_pipeline_test_report_test_case_reasons**](PipelinesApi.md#get_pipeline_test_report_test_case_reasons) | **GET** /repositories/{workspace}/{repo_slug}/pipelines/{pipeline_uuid}/steps/{step_uuid}/test_reports/test_cases/{test_case_uuid}/test_case_reasons | Get test case reasons (output) for a given test case in a step of a pipeline.
[**get_pipeline_test_report_test_cases**](PipelinesApi.md#get_pipeline_test_report_test_cases) | **GET** /repositories/{workspace}/{repo_slug}/pipelines/{pipeline_uuid}/steps/{step_uuid}/test_reports/test_cases | Get test cases for a given step of a pipeline.
[**get_pipeline_test_reports**](PipelinesApi.md#get_pipeline_test_reports) | **GET** /repositories/{workspace}/{repo_slug}/pipelines/{pipeline_uuid}/steps/{step_uuid}/test_reports | Get a summary of test reports for a given step of a pipeline.
[**get_pipeline_variable_for_team**](PipelinesApi.md#get_pipeline_variable_for_team) | **GET** /teams/{username}/pipelines_config/variables/{variable_uuid} | Get a variable for a team
[**get_pipeline_variable_for_user**](PipelinesApi.md#get_pipeline_variable_for_user) | **GET** /users/{selected_user}/pipelines_config/variables/{variable_uuid} | Get a variable for a user
[**get_pipeline_variable_for_workspace**](PipelinesApi.md#get_pipeline_variable_for_workspace) | **GET** /workspaces/{workspace}/pipelines-config/variables/{variable_uuid} | Get variable for a workspace
[**get_pipeline_variables_for_team**](PipelinesApi.md#get_pipeline_variables_for_team) | **GET** /teams/{username}/pipelines_config/variables | List variables for an account
[**get_pipeline_variables_for_user**](PipelinesApi.md#get_pipeline_variables_for_user) | **GET** /users/{selected_user}/pipelines_config/variables | List variables for a user
[**get_pipeline_variables_for_workspace**](PipelinesApi.md#get_pipeline_variables_for_workspace) | **GET** /workspaces/{workspace}/pipelines-config/variables | List variables for a workspace
[**get_pipelines_for_repository**](PipelinesApi.md#get_pipelines_for_repository) | **GET** /repositories/{workspace}/{repo_slug}/pipelines | List pipelines
[**get_repository_pipeline_cache_content_uri**](PipelinesApi.md#get_repository_pipeline_cache_content_uri) | **GET** /repositories/{workspace}/{repo_slug}/pipelines-config/caches/{cache_uuid}/content-uri | Get cache content URI
[**get_repository_pipeline_caches**](PipelinesApi.md#get_repository_pipeline_caches) | **GET** /repositories/{workspace}/{repo_slug}/pipelines-config/caches | List caches
[**get_repository_pipeline_config**](PipelinesApi.md#get_repository_pipeline_config) | **GET** /repositories/{workspace}/{repo_slug}/pipelines_config | Get configuration
[**get_repository_pipeline_known_host**](PipelinesApi.md#get_repository_pipeline_known_host) | **GET** /repositories/{workspace}/{repo_slug}/pipelines_config/ssh/known_hosts/{known_host_uuid} | Get a known host
[**get_repository_pipeline_known_hosts**](PipelinesApi.md#get_repository_pipeline_known_hosts) | **GET** /repositories/{workspace}/{repo_slug}/pipelines_config/ssh/known_hosts | List known hosts
[**get_repository_pipeline_schedule**](PipelinesApi.md#get_repository_pipeline_schedule) | **GET** /repositories/{workspace}/{repo_slug}/pipelines_config/schedules/{schedule_uuid} | Get a schedule
[**get_repository_pipeline_schedule_executions**](PipelinesApi.md#get_repository_pipeline_schedule_executions) | **GET** /repositories/{workspace}/{repo_slug}/pipelines_config/schedules/{schedule_uuid}/executions | List executions of a schedule
[**get_repository_pipeline_schedules**](PipelinesApi.md#get_repository_pipeline_schedules) | **GET** /repositories/{workspace}/{repo_slug}/pipelines_config/schedules | List schedules
[**get_repository_pipeline_ssh_key_pair**](PipelinesApi.md#get_repository_pipeline_ssh_key_pair) | **GET** /repositories/{workspace}/{repo_slug}/pipelines_config/ssh/key_pair | Get SSH key pair
[**get_repository_pipeline_variable**](PipelinesApi.md#get_repository_pipeline_variable) | **GET** /repositories/{workspace}/{repo_slug}/pipelines_config/variables/{variable_uuid} | Get a variable for a repository
[**get_repository_pipeline_variables**](PipelinesApi.md#get_repository_pipeline_variables) | **GET** /repositories/{workspace}/{repo_slug}/pipelines_config/variables | List variables for a repository
[**get_repository_runner**](PipelinesApi.md#get_repository_runner) | **GET** /repositories/{workspace}/{repo_slug}/pipelines-config/runners/{runner_uuid} | Get repository runner
[**get_repository_runners**](PipelinesApi.md#get_repository_runners) | **GET** /repositories/{workspace}/{repo_slug}/pipelines-config/runners | Get repository runners
[**get_workspace_runner**](PipelinesApi.md#get_workspace_runner) | **GET** /workspaces/{workspace}/pipelines-config/runners/{runner_uuid} | Get workspace runner
[**get_workspace_runners**](PipelinesApi.md#get_workspace_runners) | **GET** /workspaces/{workspace}/pipelines-config/runners | Get workspace runners
[**stop_pipeline**](PipelinesApi.md#stop_pipeline) | **POST** /repositories/{workspace}/{repo_slug}/pipelines/{pipeline_uuid}/stopPipeline | Stop a pipeline
[**update_deployment_variable**](PipelinesApi.md#update_deployment_variable) | **PUT** /repositories/{workspace}/{repo_slug}/deployments_config/environments/{environment_uuid}/variables/{variable_uuid} | Update a variable for an environment
[**update_pipeline_variable_for_team**](PipelinesApi.md#update_pipeline_variable_for_team) | **PUT** /teams/{username}/pipelines_config/variables/{variable_uuid} | Update a variable for a team
[**update_pipeline_variable_for_user**](PipelinesApi.md#update_pipeline_variable_for_user) | **PUT** /users/{selected_user}/pipelines_config/variables/{variable_uuid} | Update a variable for a user
[**update_pipeline_variable_for_workspace**](PipelinesApi.md#update_pipeline_variable_for_workspace) | **PUT** /workspaces/{workspace}/pipelines-config/variables/{variable_uuid} | Update variable for a workspace
[**update_repository_build_number**](PipelinesApi.md#update_repository_build_number) | **PUT** /repositories/{workspace}/{repo_slug}/pipelines_config/build_number | Update the next build number
[**update_repository_pipeline_config**](PipelinesApi.md#update_repository_pipeline_config) | **PUT** /repositories/{workspace}/{repo_slug}/pipelines_config | Update configuration
[**update_repository_pipeline_key_pair**](PipelinesApi.md#update_repository_pipeline_key_pair) | **PUT** /repositories/{workspace}/{repo_slug}/pipelines_config/ssh/key_pair | Update SSH key pair
[**update_repository_pipeline_known_host**](PipelinesApi.md#update_repository_pipeline_known_host) | **PUT** /repositories/{workspace}/{repo_slug}/pipelines_config/ssh/known_hosts/{known_host_uuid} | Update a known host
[**update_repository_pipeline_schedule**](PipelinesApi.md#update_repository_pipeline_schedule) | **PUT** /repositories/{workspace}/{repo_slug}/pipelines_config/schedules/{schedule_uuid} | Update a schedule
[**update_repository_pipeline_variable**](PipelinesApi.md#update_repository_pipeline_variable) | **PUT** /repositories/{workspace}/{repo_slug}/pipelines_config/variables/{variable_uuid} | Update a variable for a repository
[**update_repository_runner**](PipelinesApi.md#update_repository_runner) | **PUT** /repositories/{workspace}/{repo_slug}/pipelines-config/runners/{runner_uuid} | Update repository runner
[**update_workspace_runner**](PipelinesApi.md#update_workspace_runner) | **PUT** /workspaces/{workspace}/pipelines-config/runners/{runner_uuid} | Update workspace runner



## create_deployment_variable

> models::DeploymentVariable create_deployment_variable(workspace, repo_slug, environment_uuid, deployment_variable)
Create a variable for an environment

Create a deployment environment level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**environment_uuid** | **String** | The environment. | [required] |
**deployment_variable** | [**DeploymentVariable**](DeploymentVariable.md) | The variable to create | [required] |

### Return type

[**models::DeploymentVariable**](deployment_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pipeline_for_repository

> models::Pipeline create_pipeline_for_repository(workspace, repo_slug, pipeline)
Run a pipeline

Endpoint to create and initiate a pipeline. There are a number of different options to initiate a pipeline, where the payload of the request will determine which type of pipeline will be instantiated.  ## Trigger a pipeline for a branch  One way to trigger pipelines is by specifying the branch for which you want to trigger a pipeline. The specified branch will be used to determine which pipeline definition from the `bitbucket-pipelines.yml` file will be applied to initiate the pipeline. The pipeline will then do a clone of the repository and checkout the latest revision of the specified branch.  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/json' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines/ \\       -d '       {         \"target\": {           \"ref_type\": \"branch\",           \"type\": \"pipeline_ref_target\",           \"ref_name\": \"master\"         }       }' ```  ## Trigger a pipeline for a commit on a branch or tag  You can initiate a pipeline for a specific commit and in the context of a specified reference (e.g. a branch, tag or bookmark). The specified reference will be used to determine which pipeline definition from the bitbucket-pipelines.yml file will be applied to initiate the pipeline. The pipeline will clone the repository and then do a checkout the specified reference.  The following reference types are supported:  * `branch` * `named_branch` * `bookmark`  * `tag`  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/json' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines/ \\       -d '       {         \"target\": {           \"commit\": {             \"type\": \"commit\",             \"hash\": \"ce5b7431602f7cbba007062eeb55225c6e18e956\"           },           \"ref_type\": \"branch\",           \"type\": \"pipeline_ref_target\",           \"ref_name\": \"master\"         }       }' ```  ## Trigger a specific pipeline definition for a commit  You can trigger a specific pipeline that is defined in your `bitbucket-pipelines.yml` file for a specific commit. In addition to the commit revision, you specify the type and pattern of the selector that identifies the pipeline definition. The resulting pipeline will then clone the repository and checkout the specified revision.  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/json' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines/ \\       -d '       {         \"target\": {           \"commit\": {             \"hash\":\"a3c4e02c9a3755eccdc3764e6ea13facdf30f923\",             \"type\":\"commit\"           },           \"selector\": {             \"type\":\"custom\",             \"pattern\":\"Deploy to production\"           },           \"type\":\"pipeline_commit_target\"         }       }' ```  ## Trigger a specific pipeline definition for a commit on a branch or tag  You can trigger a specific pipeline that is defined in your `bitbucket-pipelines.yml` file for a specific commit in the context of a specified reference. In addition to the commit revision, you specify the type and pattern of the selector that identifies the pipeline definition, as well as the reference information. The resulting pipeline will then clone the repository a checkout the specified reference.  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/json' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines/ \\       -d '       {         \"target\": {           \"commit\": {             \"hash\":\"a3c4e02c9a3755eccdc3764e6ea13facdf30f923\",             \"type\":\"commit\"           },           \"selector\": {             \"type\": \"custom\",             \"pattern\": \"Deploy to production\"           },           \"type\": \"pipeline_ref_target\",           \"ref_name\": \"master\",           \"ref_type\": \"branch\"         }       }' ```  ## Trigger a custom pipeline with variables  In addition to triggering a custom pipeline that is defined in your `bitbucket-pipelines.yml` file as shown in the examples above, you can specify variables that will be available for your build. In the request, provide a list of variables, specifying the following for each variable: key, value, and whether it should be secured or not (this field is optional and defaults to not secured).  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/json' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines/ \\       -d '       {         \"target\": {           \"type\": \"pipeline_ref_target\",           \"ref_type\": \"branch\",           \"ref_name\": \"master\",           \"selector\": {             \"type\": \"custom\",             \"pattern\": \"Deploy to production\"           }         },         \"variables\": [           {             \"key\": \"var1key\",             \"value\": \"var1value\",             \"secured\": true           },           {             \"key\": \"var2key\",             \"value\": \"var2value\"           }         ]       }' ```  ## Trigger a pull request pipeline  You can also initiate a pipeline for a specific pull request.  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/json' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines/ \\       -d '       {         \"target\": {           \"type\": \"pipeline_pullrequest_target\",           \"source\": \"pull-request-branch\",           \"destination\": \"master\",           \"destination_commit\": {             \"hash\": \"9f848b7\"           },           \"commit\": {             \"hash\": \"1a372fc\"           },           \"pullrequest\": {             \"id\": \"3\"           },           \"selector\": {             \"type\": \"pull-requests\",             \"pattern\": \"**\"           }         }       }' ```  # On-demand pipeline  By default, pipelines run using the YAML in the repository’s `bitbucket-pipelines.yml` configuration file. With an _on-demand_ pipeline, you include the pipeline’s YAML in the request body. That YAML applies only to that run and overrides the YAML in `bitbucket-pipelines.yml`.  Just like with regular pipelines, there is a number of different options to initiate an on-demand pipeline. However, since the payload contains YAML configuration in this case, _query parameters_ are used to supply the necessary metadata to determine which type of pipeline will be instantiated. These query parameters are derived from the JSON equivalent by turning each property into a key-value pair with the JSON path of the property as the new key.  ## Trigger on-demand pipeline for a branch  You can initiate an on-demand pipeline for a specific branch. This branch will be used to determine which pipeline definition from the supplied YAML configuration will be applied to initiate the pipeline. The pipeline will then do a clone of the repository and check out the latest revision of the specified branch.  To trigger an on-demand pipeline for a _branch_ the requesting user must have **write permission** for that branch (which can be limited by [branch restrictions](https://support.atlassian.com/bitbucket-cloud/docs/use-branch-permissions/)).  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/yaml' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines?target.type=pipeline_ref_target&target.ref_type=branch&target.ref_name=master \\       -d ' pipelines:   default:     - step:         script:           - echo This is an on-demand pipeline' ```  ## Trigger on-demand pipeline for a commit on a branch or tag  You can initiate an on-demand pipeline for a specific commit and in the context of a specified reference (branch or tag). The specified reference will be used to determine which pipeline definition from the supplied YAML configuration will be applied to initiate the pipeline. The pipeline will clone the repository and check out the specified reference.  To trigger an on-demand pipeline for a _branch_ the requesting user must have **write permission** for that branch (which can be limited by [branch restrictions](https://support.atlassian.com/bitbucket-cloud/docs/use-branch-permissions/)).  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/yaml' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines?target.type=pipeline_ref_target&target.ref_type=branch&target.ref_name=master&target.commit.hash=ce5b7431602f7cbba007062eeb55225c6e18e956 \\       -d ' pipelines:   default:     - step:         script:           - echo This is an on-demand pipeline' ```  ## Trigger a specific on-demand pipeline definition for a commit  You can trigger a specific pipeline that is defined in the supplied YAML configuration for a specific commit. In addition to the commit revision, you specify the type and pattern of the selector that identifies the pipeline definition. The resulting pipeline will then clone the repository and checkout the specified revision.  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/yaml' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines?target.type=pipeline_commit_target&target.commit.hash=a3c4e02c9a3755eccdc3764e6ea13facdf30f923&target.selector.type=custom&target.selector.pattern=security-scan \\       -d ' pipelines:   custom:     security-scan:       - step:           script:             - echo Run on-demand security scan ```  ## Trigger a custom on-demand pipeline with variables  In addition to triggering a custom on-demand pipeline that is defined in the supplied YAML configuration as shown in the examples above, you can specify variables that will be available for your build. In the request, provide each variable as an indexed set of query parameters representing its key, value, and whether it should be secured or not (this field is optional and defaults to not secured).  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/yaml' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines?target.type=pipeline_ref_target&target.ref_type=branch&target.ref_name=master&target.selector.type=custom&target.selector.pattern=security-scan&variables[0].key=var1key&variables[0].value=var1value&variables[0].secured=true&variables[1].key=var2key&variables[1].value=var2value \\       -d ' pipelines:   custom:     security-scan:       - variables:           - name: var1key           - name: var2key       - step:           script:             - echo Run on-demand security scan' ```  ## Trigger a pull request pipeline  You can also initiate an on-demand pipeline for a specific pull request.  ### Example  ``` $ curl -X POST -is -u '{atlassian_account_email}:{api_token}' \\       -H 'Content-Type: application/yaml' \\       https://api.bitbucket.org/2.0/repositories/{workspace}/{repo_slug}/pipelines?target.type=pipeline_pullrequest_target&target.source=pull-request-branch&target.destination=destination&target.destination_commit.hash=9f848b7&target.commit.hash=1a372fc&target.pullrequest.id=3&target.selector.type=pull-requests&target.selector.pattern=** \\       -d ' pipelines:   pull-requests:     \"**\":       - step:           script:             - echo This is an on-demand pipeline' ``` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline** | [**Pipeline**](Pipeline.md) | The pipeline to initiate. | [required] |

### Return type

[**models::Pipeline**](pipeline.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pipeline_variable_for_team

> models::PipelineVariable create_pipeline_variable_for_team(username, pipeline_variable)
Create a variable for a user

Create an account level variable. This endpoint has been deprecated, and you should use the new workspaces endpoint. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The account. | [required] |
**pipeline_variable** | Option<[**PipelineVariable**](PipelineVariable.md)> | The variable to create. |  |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pipeline_variable_for_user

> models::PipelineVariable create_pipeline_variable_for_user(selected_user, pipeline_variable)
Create a variable for a user

Create a user level variable. This endpoint has been deprecated, and you should use the new workspaces endpoint. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | Either the UUID of the account surrounded by curly-braces, for example `{account UUID}`, OR an Atlassian Account ID. | [required] |
**pipeline_variable** | Option<[**PipelineVariable**](PipelineVariable.md)> | The variable to create. |  |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pipeline_variable_for_workspace

> models::PipelineVariable create_pipeline_variable_for_workspace(workspace, pipeline_variable)
Create a variable for a workspace

Create a workspace level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**pipeline_variable** | Option<[**PipelineVariable**](PipelineVariable.md)> | The variable to create. |  |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_repository_pipeline_known_host

> models::PipelineKnownHost create_repository_pipeline_known_host(workspace, repo_slug, pipeline_known_host)
Create a known host

Create a repository level known host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_known_host** | [**PipelineKnownHost**](PipelineKnownHost.md) | The known host to create. | [required] |

### Return type

[**models::PipelineKnownHost**](pipeline_known_host.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_repository_pipeline_schedule

> models::PipelineSchedule create_repository_pipeline_schedule(workspace, repo_slug, pipeline_schedule_post_request_body)
Create a schedule

Create a schedule for the given repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_schedule_post_request_body** | [**PipelineSchedulePostRequestBody**](PipelineSchedulePostRequestBody.md) | The schedule to create. | [required] |

### Return type

[**models::PipelineSchedule**](pipeline_schedule.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_repository_pipeline_variable

> models::PipelineVariable create_repository_pipeline_variable(workspace, repo_slug, pipeline_variable)
Create a variable for a repository

Create a repository level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_variable** | [**PipelineVariable**](PipelineVariable.md) | The variable to create. | [required] |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_repository_runner

> models::PipelineRunner create_repository_runner(workspace, repo_slug)
Create repository runner

Create repository runner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

[**models::PipelineRunner**](pipeline_runner.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_workspace_runner

> models::PipelineRunner create_workspace_runner(workspace)
Create workspace runner

Create workspace runner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |

### Return type

[**models::PipelineRunner**](pipeline_runner.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deployment_variable

> delete_deployment_variable(workspace, repo_slug, environment_uuid, variable_uuid)
Delete a variable for an environment

Delete a deployment environment level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**environment_uuid** | **String** | The environment. | [required] |
**variable_uuid** | **String** | The UUID of the variable to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pipeline_variable_for_team

> delete_pipeline_variable_for_team(username, variable_uuid)
Delete a variable for a team

Delete a team level variable. This endpoint has been deprecated, and you should use the new workspaces endpoint. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The account. | [required] |
**variable_uuid** | **String** | The UUID of the variable to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pipeline_variable_for_user

> delete_pipeline_variable_for_user(selected_user, variable_uuid)
Delete a variable for a user

Delete an account level variable. This endpoint has been deprecated, and you should use the new workspaces endpoint. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | Either the UUID of the account surrounded by curly-braces, for example `{account UUID}`, OR an Atlassian Account ID. | [required] |
**variable_uuid** | **String** | The UUID of the variable to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pipeline_variable_for_workspace

> delete_pipeline_variable_for_workspace(workspace, variable_uuid)
Delete a variable for a workspace

Delete a workspace level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**variable_uuid** | **String** | The UUID of the variable to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repository_pipeline_cache

> delete_repository_pipeline_cache(workspace, repo_slug, cache_uuid)
Delete a cache

Delete a repository cache.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The account. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**cache_uuid** | **String** | The UUID of the cache to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repository_pipeline_caches

> delete_repository_pipeline_caches(workspace, repo_slug, name)
Delete caches

Delete repository cache versions by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The account. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**name** | **String** | The cache name. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repository_pipeline_key_pair

> delete_repository_pipeline_key_pair(workspace, repo_slug)
Delete SSH key pair

Delete the repository SSH key pair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repository_pipeline_known_host

> delete_repository_pipeline_known_host(workspace, repo_slug, known_host_uuid)
Delete a known host

Delete a repository level known host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**known_host_uuid** | **String** | The UUID of the known host to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repository_pipeline_schedule

> delete_repository_pipeline_schedule(workspace, repo_slug, schedule_uuid)
Delete a schedule

Delete a schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**schedule_uuid** | **String** | The uuid of the schedule. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repository_pipeline_variable

> delete_repository_pipeline_variable(workspace, repo_slug, variable_uuid)
Delete a variable for a repository

Delete a repository level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**variable_uuid** | **String** | The UUID of the variable to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repository_runner

> delete_repository_runner(workspace, repo_slug, runner_uuid)
Delete repository runner

Delete repository runner by uuid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**runner_uuid** | **String** | The runner uuid. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workspace_runner

> delete_workspace_runner(workspace, runner_uuid)
Delete workspace runner

Delete workspace runner by uuid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**runner_uuid** | **String** | The runner uuid. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment_variables

> models::PaginatedDeploymentVariable get_deployment_variables(workspace, repo_slug, environment_uuid)
List variables for an environment

Find deployment environment level variables.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**environment_uuid** | **String** | The environment. | [required] |

### Return type

[**models::PaginatedDeploymentVariable**](paginated_deployment_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oidc_configuration

> get_oidc_configuration(workspace)
Get OpenID configuration for OIDC in Pipelines

This is part of OpenID Connect for Pipelines, see https://support.atlassian.com/bitbucket-cloud/docs/integrate-pipelines-with-resource-servers-using-oidc/

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oidc_keys

> get_oidc_keys(workspace)
Get keys for OIDC in Pipelines

This is part of OpenID Connect for Pipelines, see https://support.atlassian.com/bitbucket-cloud/docs/integrate-pipelines-with-resource-servers-using-oidc/

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_container_log

> get_pipeline_container_log(workspace, repo_slug, pipeline_uuid, step_uuid, log_uuid)
Get the logs for the build container or a service container for a given step of a pipeline.

Retrieve the log file for a build container or service container.  This endpoint supports (and encourages!) the use of [HTTP Range requests](https://tools.ietf.org/html/rfc7233) to deal with potentially very large log files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_uuid** | **String** | The UUID of the pipeline. | [required] |
**step_uuid** | **String** | The UUID of the step. | [required] |
**log_uuid** | **String** | For the main build container specify the step UUID; for a service container specify the service container UUID | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_for_repository

> models::Pipeline get_pipeline_for_repository(workspace, repo_slug, pipeline_uuid)
Get a pipeline

Retrieve a specified pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_uuid** | **String** | The pipeline UUID. | [required] |

### Return type

[**models::Pipeline**](pipeline.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_step_for_repository

> models::PipelineStep get_pipeline_step_for_repository(workspace, repo_slug, pipeline_uuid, step_uuid)
Get a step of a pipeline

Retrieve a given step of a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_uuid** | **String** | The UUID of the pipeline. | [required] |
**step_uuid** | **String** | The UUID of the step. | [required] |

### Return type

[**models::PipelineStep**](pipeline_step.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_step_log_for_repository

> get_pipeline_step_log_for_repository(workspace, repo_slug, pipeline_uuid, step_uuid)
Get log file for a step

Retrieve the log file for a given step of a pipeline.  This endpoint supports (and encourages!) the use of [HTTP Range requests](https://tools.ietf.org/html/rfc7233) to deal with potentially very large log files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_uuid** | **String** | The UUID of the pipeline. | [required] |
**step_uuid** | **String** | The UUID of the step. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_steps_for_repository

> models::PaginatedPipelineSteps get_pipeline_steps_for_repository(workspace, repo_slug, pipeline_uuid)
List steps for a pipeline

Find steps for the given pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_uuid** | **String** | The UUID of the pipeline. | [required] |

### Return type

[**models::PaginatedPipelineSteps**](paginated_pipeline_steps.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_test_report_test_case_reasons

> get_pipeline_test_report_test_case_reasons(workspace, repo_slug, pipeline_uuid, step_uuid, test_case_uuid)
Get test case reasons (output) for a given test case in a step of a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_uuid** | **String** | The UUID of the pipeline. | [required] |
**step_uuid** | **String** | The UUID of the step. | [required] |
**test_case_uuid** | **String** | The UUID of the test case. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_test_report_test_cases

> get_pipeline_test_report_test_cases(workspace, repo_slug, pipeline_uuid, step_uuid)
Get test cases for a given step of a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_uuid** | **String** | The UUID of the pipeline. | [required] |
**step_uuid** | **String** | The UUID of the step. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_test_reports

> get_pipeline_test_reports(workspace, repo_slug, pipeline_uuid, step_uuid)
Get a summary of test reports for a given step of a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_uuid** | **String** | The UUID of the pipeline. | [required] |
**step_uuid** | **String** | The UUID of the step. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_variable_for_team

> models::PipelineVariable get_pipeline_variable_for_team(username, variable_uuid)
Get a variable for a team

Retrieve a team level variable. This endpoint has been deprecated, and you should use the new workspaces endpoint. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The account. | [required] |
**variable_uuid** | **String** | The UUID of the variable to retrieve. | [required] |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_variable_for_user

> models::PipelineVariable get_pipeline_variable_for_user(selected_user, variable_uuid)
Get a variable for a user

Retrieve a user level variable. This endpoint has been deprecated, and you should use the new workspaces endpoint. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | Either the UUID of the account surrounded by curly-braces, for example `{account UUID}`, OR an Atlassian Account ID. | [required] |
**variable_uuid** | **String** | The UUID of the variable to retrieve. | [required] |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_variable_for_workspace

> models::PipelineVariable get_pipeline_variable_for_workspace(workspace, variable_uuid)
Get variable for a workspace

Retrieve a workspace level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**variable_uuid** | **String** | The UUID of the variable to retrieve. | [required] |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_variables_for_team

> models::PaginatedPipelineVariables get_pipeline_variables_for_team(username)
List variables for an account

Find account level variables. This endpoint has been deprecated, and you should use the new workspaces endpoint. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The account. | [required] |

### Return type

[**models::PaginatedPipelineVariables**](paginated_pipeline_variables.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_variables_for_user

> models::PaginatedPipelineVariables get_pipeline_variables_for_user(selected_user)
List variables for a user

Find user level variables. This endpoint has been deprecated, and you should use the new workspaces endpoint. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | Either the UUID of the account surrounded by curly-braces, for example `{account UUID}`, OR an Atlassian Account ID. | [required] |

### Return type

[**models::PaginatedPipelineVariables**](paginated_pipeline_variables.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_variables_for_workspace

> models::PaginatedPipelineVariables get_pipeline_variables_for_workspace(workspace)
List variables for a workspace

Find workspace level variables.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |

### Return type

[**models::PaginatedPipelineVariables**](paginated_pipeline_variables.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipelines_for_repository

> models::PaginatedPipelines get_pipelines_for_repository(workspace, repo_slug, creator_uuid, target_ref_type, target_ref_name, target_branch, target_commit_hash, target_selector_pattern, target_selector_type, created_on, trigger_type, status, sort, page, pagelen)
List pipelines

Find pipelines in a repository.  Note that unlike other endpoints in the Bitbucket API, this endpoint utilizes query parameters to allow filtering and sorting of returned results. See [query parameters](#api-repositories-workspace-repo-slug-pipelines-get-request-Query%20parameters) for specific details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**creator_uuid** | Option<**uuid::Uuid**> | The UUID of the creator of the pipeline to filter by. |  |
**target_ref_type** | Option<**String**> | The type of the reference to filter by. |  |
**target_ref_name** | Option<**String**> | The reference name to filter by. |  |
**target_branch** | Option<**String**> | The name of the branch to filter by. |  |
**target_commit_hash** | Option<**String**> | The revision to filter by. |  |
**target_selector_pattern** | Option<**String**> | The pipeline pattern to filter by. |  |
**target_selector_type** | Option<**String**> | The type of pipeline to filter by. |  |
**created_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The creation date to filter by. |  |
**trigger_type** | Option<**String**> | The trigger type to filter by. |  |
**status** | Option<**String**> | The pipeline status to filter by. |  |
**sort** | Option<**String**> | The attribute name to sort on. |  |
**page** | Option<**i32**> | The page number of elements to retrieve. |  |[default to 1]
**pagelen** | Option<**i32**> | The maximum number of results to return. |  |[default to 10]

### Return type

[**models::PaginatedPipelines**](paginated_pipelines.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_cache_content_uri

> models::PipelineCacheContentUri get_repository_pipeline_cache_content_uri(workspace, repo_slug, cache_uuid)
Get cache content URI

Retrieve the URI of the content of the specified cache.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The account. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**cache_uuid** | **String** | The UUID of the cache. | [required] |

### Return type

[**models::PipelineCacheContentUri**](pipeline_cache_content_uri.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_caches

> models::PaginatedPipelineCaches get_repository_pipeline_caches(workspace, repo_slug)
List caches

Retrieve the repository pipelines caches.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The account. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

[**models::PaginatedPipelineCaches**](paginated_pipeline_caches.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_config

> models::PipelinesConfig get_repository_pipeline_config(workspace, repo_slug)
Get configuration

Retrieve the repository pipelines configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The account. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

[**models::PipelinesConfig**](pipelines_config.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_known_host

> models::PipelineKnownHost get_repository_pipeline_known_host(workspace, repo_slug, known_host_uuid)
Get a known host

Retrieve a repository level known host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**known_host_uuid** | **String** | The UUID of the known host to retrieve. | [required] |

### Return type

[**models::PipelineKnownHost**](pipeline_known_host.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_known_hosts

> models::PaginatedPipelineKnownHosts get_repository_pipeline_known_hosts(workspace, repo_slug)
List known hosts

Find repository level known hosts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

[**models::PaginatedPipelineKnownHosts**](paginated_pipeline_known_hosts.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_schedule

> models::PipelineSchedule get_repository_pipeline_schedule(workspace, repo_slug, schedule_uuid)
Get a schedule

Retrieve a schedule by its UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**schedule_uuid** | **String** | The uuid of the schedule. | [required] |

### Return type

[**models::PipelineSchedule**](pipeline_schedule.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_schedule_executions

> models::PaginatedPipelineScheduleExecutions get_repository_pipeline_schedule_executions(workspace, repo_slug, schedule_uuid)
List executions of a schedule

Retrieve the executions of a given schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**schedule_uuid** | **String** | The uuid of the schedule. | [required] |

### Return type

[**models::PaginatedPipelineScheduleExecutions**](paginated_pipeline_schedule_executions.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_schedules

> models::PaginatedPipelineSchedules get_repository_pipeline_schedules(workspace, repo_slug)
List schedules

Retrieve the configured schedules for the given repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

[**models::PaginatedPipelineSchedules**](paginated_pipeline_schedules.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_ssh_key_pair

> models::PipelineSshKeyPair get_repository_pipeline_ssh_key_pair(workspace, repo_slug)
Get SSH key pair

Retrieve the repository SSH key pair excluding the SSH private key. The private key is a write only field and will never be exposed in the logs or the REST API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

[**models::PipelineSshKeyPair**](pipeline_ssh_key_pair.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_variable

> models::PipelineVariable get_repository_pipeline_variable(workspace, repo_slug, variable_uuid)
Get a variable for a repository

Retrieve a repository level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**variable_uuid** | **String** | The UUID of the variable to retrieve. | [required] |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_pipeline_variables

> models::PaginatedPipelineVariables get_repository_pipeline_variables(workspace, repo_slug)
List variables for a repository

Find repository level variables.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

[**models::PaginatedPipelineVariables**](paginated_pipeline_variables.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_runner

> models::PipelineRunner get_repository_runner(workspace, repo_slug, runner_uuid)
Get repository runner

Retrieve repository runner by uuid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**runner_uuid** | **String** | The runner uuid. | [required] |

### Return type

[**models::PipelineRunner**](pipeline_runner.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_runners

> models::PaginatedPipelineRunners get_repository_runners(workspace, repo_slug)
Get repository runners

Retrieve repository runners.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |

### Return type

[**models::PaginatedPipelineRunners**](paginated_pipeline_runners.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspace_runner

> models::PipelineRunner get_workspace_runner(workspace, runner_uuid)
Get workspace runner

Get workspace runner by uuid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**runner_uuid** | **String** | The runner uuid. | [required] |

### Return type

[**models::PipelineRunner**](pipeline_runner.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspace_runners

> models::PaginatedPipelineRunners get_workspace_runners(workspace)
Get workspace runners

Retrieve workspace runners.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |

### Return type

[**models::PaginatedPipelineRunners**](paginated_pipeline_runners.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_pipeline

> stop_pipeline(workspace, repo_slug, pipeline_uuid)
Stop a pipeline

Signal the stop of a pipeline and all of its steps that not have completed yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_uuid** | **String** | The UUID of the pipeline. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deployment_variable

> models::DeploymentVariable update_deployment_variable(workspace, repo_slug, environment_uuid, variable_uuid, deployment_variable)
Update a variable for an environment

Update a deployment environment level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**environment_uuid** | **String** | The environment. | [required] |
**variable_uuid** | **String** | The UUID of the variable to update. | [required] |
**deployment_variable** | [**DeploymentVariable**](DeploymentVariable.md) | The updated deployment variable. | [required] |

### Return type

[**models::DeploymentVariable**](deployment_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pipeline_variable_for_team

> models::PipelineVariable update_pipeline_variable_for_team(username, variable_uuid, pipeline_variable)
Update a variable for a team

Update a team level variable. This endpoint has been deprecated, and you should use the new workspaces endpoint. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The account. | [required] |
**variable_uuid** | **String** | The UUID of the variable. | [required] |
**pipeline_variable** | [**PipelineVariable**](PipelineVariable.md) | The updated variable. | [required] |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pipeline_variable_for_user

> models::PipelineVariable update_pipeline_variable_for_user(selected_user, variable_uuid, pipeline_variable)
Update a variable for a user

Update a user level variable. This endpoint has been deprecated, and you should use the new workspaces endpoint. For more information, see [the announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-teams-deprecation/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | Either the UUID of the account surrounded by curly-braces, for example `{account UUID}`, OR an Atlassian Account ID. | [required] |
**variable_uuid** | **String** | The UUID of the variable. | [required] |
**pipeline_variable** | [**PipelineVariable**](PipelineVariable.md) | The updated variable. | [required] |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pipeline_variable_for_workspace

> models::PipelineVariable update_pipeline_variable_for_workspace(workspace, variable_uuid, pipeline_variable)
Update variable for a workspace

Update a workspace level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**variable_uuid** | **String** | The UUID of the variable. | [required] |
**pipeline_variable** | [**PipelineVariable**](PipelineVariable.md) | The updated variable. | [required] |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository_build_number

> models::PipelineBuildNumber update_repository_build_number(workspace, repo_slug, pipeline_build_number)
Update the next build number

Update the next build number that should be assigned to a pipeline. The next build number that will be configured has to be strictly higher than the current latest build number for this repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_build_number** | [**PipelineBuildNumber**](PipelineBuildNumber.md) | The build number to update. | [required] |

### Return type

[**models::PipelineBuildNumber**](pipeline_build_number.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository_pipeline_config

> models::PipelinesConfig update_repository_pipeline_config(workspace, repo_slug, pipelines_config)
Update configuration

Update the pipelines configuration for a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipelines_config** | [**PipelinesConfig**](PipelinesConfig.md) | The updated repository pipelines configuration. | [required] |

### Return type

[**models::PipelinesConfig**](pipelines_config.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository_pipeline_key_pair

> models::PipelineSshKeyPair update_repository_pipeline_key_pair(workspace, repo_slug, pipeline_ssh_key_pair)
Update SSH key pair

Create or update the repository SSH key pair. The private key will be set as a default SSH identity in your build container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**pipeline_ssh_key_pair** | [**PipelineSshKeyPair**](PipelineSshKeyPair.md) | The created or updated SSH key pair. | [required] |

### Return type

[**models::PipelineSshKeyPair**](pipeline_ssh_key_pair.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository_pipeline_known_host

> models::PipelineKnownHost update_repository_pipeline_known_host(workspace, repo_slug, known_host_uuid, pipeline_known_host)
Update a known host

Update a repository level known host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**known_host_uuid** | **String** | The UUID of the known host to update. | [required] |
**pipeline_known_host** | [**PipelineKnownHost**](PipelineKnownHost.md) | The updated known host. | [required] |

### Return type

[**models::PipelineKnownHost**](pipeline_known_host.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository_pipeline_schedule

> models::PipelineSchedule update_repository_pipeline_schedule(workspace, repo_slug, schedule_uuid, pipeline_schedule_put_request_body)
Update a schedule

Update a schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**schedule_uuid** | **String** | The uuid of the schedule. | [required] |
**pipeline_schedule_put_request_body** | [**PipelineSchedulePutRequestBody**](PipelineSchedulePutRequestBody.md) | The schedule to update. | [required] |

### Return type

[**models::PipelineSchedule**](pipeline_schedule.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository_pipeline_variable

> models::PipelineVariable update_repository_pipeline_variable(workspace, repo_slug, variable_uuid, pipeline_variable)
Update a variable for a repository

Update a repository level variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**variable_uuid** | **String** | The UUID of the variable to update. | [required] |
**pipeline_variable** | [**PipelineVariable**](PipelineVariable.md) | The updated variable | [required] |

### Return type

[**models::PipelineVariable**](pipeline_variable.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository_runner

> models::PipelineRunner update_repository_runner(workspace, repo_slug, runner_uuid)
Update repository runner

Update repository runner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**repo_slug** | **String** | The repository. | [required] |
**runner_uuid** | **String** | The runner uuid. | [required] |

### Return type

[**models::PipelineRunner**](pipeline_runner.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workspace_runner

> models::PipelineRunner update_workspace_runner(workspace, runner_uuid)
Update workspace runner

Update workspace runner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | This can either be the workspace ID (slug) or the workspace UUID surrounded by curly-braces, for example `{workspace UUID}`. | [required] |
**runner_uuid** | **String** | The runner uuid. | [required] |

### Return type

[**models::PipelineRunner**](pipeline_runner.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

