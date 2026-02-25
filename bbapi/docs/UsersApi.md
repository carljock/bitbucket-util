# \UsersApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_emails_email_get**](UsersApi.md#user_emails_email_get) | **GET** /user/emails/{email} | Get an email address for current user
[**user_emails_get**](UsersApi.md#user_emails_get) | **GET** /user/emails | List email addresses for current user
[**user_get**](UsersApi.md#user_get) | **GET** /user | Get current user
[**users_selected_user_get**](UsersApi.md#users_selected_user_get) | **GET** /users/{selected_user} | Get a user



## user_emails_email_get

> models::Error user_emails_email_get(email)
Get an email address for current user

Returns details about a specific one of the authenticated user's email addresses.  Details describe whether the address has been confirmed by the user and whether it is the user's primary address or not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address of the user. | [required] |

### Return type

[**models::Error**](error.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_emails_get

> models::Error user_emails_get()
List email addresses for current user

Returns all the authenticated user's email addresses. Both confirmed and unconfirmed.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Error**](error.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get

> models::Account user_get()
Get current user

Returns the currently logged in user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Account**](account.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_selected_user_get

> models::Account users_selected_user_get(selected_user)
Get a user

Gets the public information associated with a user account.  If the user's profile is private, `location`, `website` and `created_on` elements are omitted.  Note that the user object returned by this operation is changing significantly, due to privacy changes. See the [announcement](https://developer.atlassian.com/cloud/bitbucket/bitbucket-api-changes-gdpr/#changes-to-bitbucket-user-objects) for details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selected_user** | **String** | This can either be an Atlassian Account ID OR the UUID of the account, surrounded by curly-braces, for example: `{account UUID}`.  | [required] |

### Return type

[**models::Account**](account.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

