# \SshApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_selected_user_ssh_keys_get**](SshApi.md#users_selected_user_ssh_keys_get) | **GET** /users/{selected_user}/ssh-keys | List SSH keys
[**users_selected_user_ssh_keys_key_id_delete**](SshApi.md#users_selected_user_ssh_keys_key_id_delete) | **DELETE** /users/{selected_user}/ssh-keys/{key_id} | Delete a SSH key
[**users_selected_user_ssh_keys_key_id_get**](SshApi.md#users_selected_user_ssh_keys_key_id_get) | **GET** /users/{selected_user}/ssh-keys/{key_id} | Get a SSH key
[**users_selected_user_ssh_keys_key_id_put**](SshApi.md#users_selected_user_ssh_keys_key_id_put) | **PUT** /users/{selected_user}/ssh-keys/{key_id} | Update a SSH key
[**users_selected_user_ssh_keys_post**](SshApi.md#users_selected_user_ssh_keys_post) | **POST** /users/{selected_user}/ssh-keys | Add a new SSH key



## users_selected_user_ssh_keys_get

> models::PaginatedSshUserKeys users_selected_user_ssh_keys_get(selected_user)
List SSH keys

Returns a paginated list of the user's SSH public keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | This can either be an Atlassian Account ID OR the UUID of the account, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |

### Return type

[**models::PaginatedSshUserKeys**](paginated_ssh_user_keys.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_selected_user_ssh_keys_key_id_delete

> users_selected_user_ssh_keys_key_id_delete(key_id, selected_user)
Delete a SSH key

Deletes a specific SSH public key from a user's account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The SSH key's UUID value. | [required] |
**selected_user** | **String** | This can either be an Atlassian Account ID OR the UUID of the account, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_selected_user_ssh_keys_key_id_get

> models::SshAccountKey users_selected_user_ssh_keys_key_id_get(key_id, selected_user)
Get a SSH key

Returns a specific SSH public key belonging to a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The SSH key's UUID value. | [required] |
**selected_user** | **String** | This can either be an Atlassian Account ID OR the UUID of the account, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |

### Return type

[**models::SshAccountKey**](ssh_account_key.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_selected_user_ssh_keys_key_id_put

> models::SshAccountKey users_selected_user_ssh_keys_key_id_put(key_id, selected_user, ssh_account_key)
Update a SSH key

Updates a specific SSH public key on a user's account  Note: Only the 'comment' field can be updated using this API. To modify the key or comment values, you must delete and add the key again.  Example:  ``` $ curl -X PUT -H \"Content-Type: application/json\" -d '{\"label\": \"Work key\"}' https://api.bitbucket.org/2.0/users/{ed08f5e1-605b-4f4a-aee4-6c97628a673e}/ssh-keys/{b15b6026-9c02-4626-b4ad-b905f99f763a} ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The SSH key's UUID value. | [required] |
**selected_user** | **String** | This can either be an Atlassian Account ID OR the UUID of the account, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |
**ssh_account_key** | Option<[**SshAccountKey**](SshAccountKey.md)> | The updated SSH key object |  |

### Return type

[**models::SshAccountKey**](ssh_account_key.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_selected_user_ssh_keys_post

> models::SshAccountKey users_selected_user_ssh_keys_post(selected_user, expires_on, ssh_account_key)
Add a new SSH key

Adds a new SSH public key to the specified user account and returns the resulting key.  Example:  ``` $ curl -X POST -H \"Content-Type: application/json\" -d '{\"key\": \"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIKqP3Cr632C2dNhhgKVcon4ldUSAeKiku2yP9O9/bDtY user@myhost\"}' https://api.bitbucket.org/2.0/users/{ed08f5e1-605b-4f4a-aee4-6c97628a673e}/ssh-keys ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | This can either be an Atlassian Account ID OR the UUID of the account, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |
**expires_on** | Option<**String**> | The date or date-time of when the key will expire, in [ISO-8601](https://en.wikipedia.org/wiki/ISO_8601) format. Example: `YYYY-MM-DDTHH:mm:ss.sssZ` |  |
**ssh_account_key** | Option<[**SshAccountKey**](SshAccountKey.md)> | The new SSH key object. Note that the username property has been deprecated due to [privacy changes](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-changes-gdpr/#removal-of-usernames-from-user-referencing-apis). |  |

### Return type

[**models::SshAccountKey**](ssh_account_key.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

