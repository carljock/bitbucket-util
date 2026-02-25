# \GpgApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_selected_user_gpg_keys_fingerprint_delete**](GpgApi.md#users_selected_user_gpg_keys_fingerprint_delete) | **DELETE** /users/{selected_user}/gpg-keys/{fingerprint} | Delete a GPG key
[**users_selected_user_gpg_keys_fingerprint_get**](GpgApi.md#users_selected_user_gpg_keys_fingerprint_get) | **GET** /users/{selected_user}/gpg-keys/{fingerprint} | Get a GPG key
[**users_selected_user_gpg_keys_get**](GpgApi.md#users_selected_user_gpg_keys_get) | **GET** /users/{selected_user}/gpg-keys | List GPG keys
[**users_selected_user_gpg_keys_post**](GpgApi.md#users_selected_user_gpg_keys_post) | **POST** /users/{selected_user}/gpg-keys | Add a new GPG key



## users_selected_user_gpg_keys_fingerprint_delete

> users_selected_user_gpg_keys_fingerprint_delete(fingerprint, selected_user)
Delete a GPG key

Deletes a specific GPG public key from a user's account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fingerprint** | **String** | A GPG key fingerprint.  | [required] |
**selected_user** | **String** | This can either be an Atlassian Account ID OR the UUID of the account, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_selected_user_gpg_keys_fingerprint_get

> models::GpgAccountKey users_selected_user_gpg_keys_fingerprint_get(fingerprint, selected_user)
Get a GPG key

Returns a specific GPG public key belonging to a user. The `key` and `subkeys` fields can also be requested from the endpoint. See [Partial Responses](/cloud/bitbucket/rest/intro/#partial-response) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fingerprint** | **String** | A GPG key fingerprint.  | [required] |
**selected_user** | **String** | This can either be an Atlassian Account ID OR the UUID of the account, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |

### Return type

[**models::GpgAccountKey**](GPG_account_key.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_selected_user_gpg_keys_get

> models::PaginatedGpgUserKeys users_selected_user_gpg_keys_get(selected_user)
List GPG keys

Returns a paginated list of the user's GPG public keys. The `key` and `subkeys` fields can also be requested from the endpoint. See [Partial Responses](/cloud/bitbucket/rest/intro/#partial-response) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | This can either be an Atlassian Account ID OR the UUID of the account, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |

### Return type

[**models::PaginatedGpgUserKeys**](paginated_gpg_user_keys.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_selected_user_gpg_keys_post

> models::GpgAccountKey users_selected_user_gpg_keys_post(selected_user, gpg_account_key)
Add a new GPG key

Adds a new GPG public key to the specified user account and returns the resulting key.  Example:  ``` $ curl -X POST -H \"Content-Type: application/json\" -d '{\"key\": \"<insert GPG Key>\"}' https://api.bitbucket.org/2.0/users/{d7dd0e2d-3994-4a50-a9ee-d260b6cefdab}/gpg-keys ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | This can either be an Atlassian Account ID OR the UUID of the account, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |
**gpg_account_key** | Option<[**GpgAccountKey**](GpgAccountKey.md)> | The new GPG key object. |  |

### Return type

[**models::GpgAccountKey**](GPG_account_key.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

