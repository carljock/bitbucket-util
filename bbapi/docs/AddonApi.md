# \AddonApi

All URIs are relative to *https://api.bitbucket.org/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addon_delete**](AddonApi.md#addon_delete) | **DELETE** /addon | Delete an app
[**addon_linkers_get**](AddonApi.md#addon_linkers_get) | **GET** /addon/linkers | List linkers for an app
[**addon_linkers_linker_key_get**](AddonApi.md#addon_linkers_linker_key_get) | **GET** /addon/linkers/{linker_key} | Get a linker for an app
[**addon_linkers_linker_key_values_delete**](AddonApi.md#addon_linkers_linker_key_values_delete) | **DELETE** /addon/linkers/{linker_key}/values | Delete all linker values
[**addon_linkers_linker_key_values_get**](AddonApi.md#addon_linkers_linker_key_values_get) | **GET** /addon/linkers/{linker_key}/values | List linker values for a linker
[**addon_linkers_linker_key_values_post**](AddonApi.md#addon_linkers_linker_key_values_post) | **POST** /addon/linkers/{linker_key}/values | Create a linker value
[**addon_linkers_linker_key_values_put**](AddonApi.md#addon_linkers_linker_key_values_put) | **PUT** /addon/linkers/{linker_key}/values | Update a linker value
[**addon_linkers_linker_key_values_value_id_delete**](AddonApi.md#addon_linkers_linker_key_values_value_id_delete) | **DELETE** /addon/linkers/{linker_key}/values/{value_id} | Delete a linker value
[**addon_linkers_linker_key_values_value_id_get**](AddonApi.md#addon_linkers_linker_key_values_value_id_get) | **GET** /addon/linkers/{linker_key}/values/{value_id} | Get a linker value
[**addon_put**](AddonApi.md#addon_put) | **PUT** /addon | Update an installed app



## addon_delete

> addon_delete()
Delete an app

Deletes the application for the user.  This endpoint is intended to be used by Bitbucket Connect apps and only supports JWT authentication -- that is how Bitbucket identifies the particular installation of the app. Developers with applications registered in the \"Develop Apps\" section of Bitbucket Marketplace need not use this endpoint as updates for those applications can be sent out via the UI of that section.  ``` $ curl -X DELETE https://api.bitbucket.org/2.0/addon \\   -H \"Authorization: JWT <JWT Token>\" ```

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_linkers_get

> addon_linkers_get()
List linkers for an app

Gets a list of all [linkers](/cloud/bitbucket/modules/linker/) for the authenticated application.  This endpoint is deprecated and will be removed by May 2026.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_linkers_linker_key_get

> addon_linkers_linker_key_get(linker_key)
Get a linker for an app

Gets a [linker](/cloud/bitbucket/modules/linker/) specified by `linker_key` for the authenticated application.  This endpoint is deprecated and will be removed by May 2026.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linker_key** | **String** | The unique key of a [linker module](/cloud/bitbucket/modules/linker/) as defined in an application descriptor. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_linkers_linker_key_values_delete

> addon_linkers_linker_key_values_delete(linker_key)
Delete all linker values

Delete all [linker](/cloud/bitbucket/modules/linker/) values for the specified linker of the authenticated application.  This endpoint is deprecated and will be removed by May 2026.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linker_key** | **String** | The unique key of a [linker module](/cloud/bitbucket/modules/linker/) as defined in an application descriptor. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_linkers_linker_key_values_get

> addon_linkers_linker_key_values_get(linker_key)
List linker values for a linker

Gets a list of all [linker](/cloud/bitbucket/modules/linker/) values for the specified linker of the authenticated application.  A linker value lets applications supply values to modify its regular expression.  The base regular expression must use a Bitbucket-specific match group `(?K)` which will be translated to `([\\w\\-]+)`. A value must match this pattern.  [Read more about linker values](/cloud/bitbucket/modules/linker/#usingthebitbucketapitosupplyvalues)  This endpoint is deprecated and will be removed by May 2026.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linker_key** | **String** | The unique key of a [linker module](/cloud/bitbucket/modules/linker/) as defined in an application descriptor. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_linkers_linker_key_values_post

> addon_linkers_linker_key_values_post(linker_key)
Create a linker value

Creates a [linker](/cloud/bitbucket/modules/linker/) value for the specified linker of authenticated application.  A linker value lets applications supply values to modify its regular expression.  The base regular expression must use a Bitbucket-specific match group `(?K)` which will be translated to `([\\w\\-]+)`. A value must match this pattern.  [Read more about linker values](/cloud/bitbucket/modules/linker/#usingthebitbucketapitosupplyvalues)  This endpoint is deprecated and will be removed by May 2026.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linker_key** | **String** | The unique key of a [linker module](/cloud/bitbucket/modules/linker/) as defined in an application descriptor. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_linkers_linker_key_values_put

> addon_linkers_linker_key_values_put(linker_key)
Update a linker value

Bulk update [linker](/cloud/bitbucket/modules/linker/) values for the specified linker of the authenticated application.  A linker value lets applications supply values to modify its regular expression.  The base regular expression must use a Bitbucket-specific match group `(?K)` which will be translated to `([\\w\\-]+)`. A value must match this pattern.  [Read more about linker values](/cloud/bitbucket/modules/linker/#usingthebitbucketapitosupplyvalues)  This endpoint is deprecated and will be removed by May 2026.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linker_key** | **String** | The unique key of a [linker module](/cloud/bitbucket/modules/linker/) as defined in an application descriptor. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_linkers_linker_key_values_value_id_delete

> addon_linkers_linker_key_values_value_id_delete(linker_key, value_id)
Delete a linker value

Delete a single [linker](/cloud/bitbucket/modules/linker/) value of the authenticated application.  This endpoint is deprecated and will be removed by May 2026.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linker_key** | **String** | The unique key of a [linker module](/cloud/bitbucket/modules/linker/) as defined in an application descriptor. | [required] |
**value_id** | **i32** | The numeric ID of the linker value. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_linkers_linker_key_values_value_id_get

> addon_linkers_linker_key_values_value_id_get(linker_key, value_id)
Get a linker value

Get a single [linker](/cloud/bitbucket/modules/linker/) value of the authenticated application.  This endpoint is deprecated and will be removed by May 2026.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linker_key** | **String** | The unique key of a [linker module](/cloud/bitbucket/modules/linker/) as defined in an application descriptor. | [required] |
**value_id** | **i32** | The numeric ID of the linker value. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_put

> addon_put()
Update an installed app

Updates the application installation for the user.  This endpoint is intended to be used by Bitbucket Connect apps and only supports JWT authentication -- that is how Bitbucket identifies the particular installation of the app. Developers with applications registered in the \"Develop Apps\" section of Bitbucket need not use this endpoint as updates for those applications can be sent out via the UI of that section.  Passing an empty body will update the installation using the existing descriptor URL.  ``` $ curl -X PUT https://api.bitbucket.org/2.0/addon \\   -H \"Authorization: JWT <JWT Token>\" \\   --header \"Content-Type: application/json\" \\   --data '{}' ```  The new `descriptor` for the installation can be also provided in the body directly.  ``` $ curl -X PUT https://api.bitbucket.org/2.0/addon \\   -H \"Authorization: JWT <JWT Token>\" \\   --header \"Content-Type: application/json\" \\   --data '{\"descriptor\": $NEW_DESCRIPTOR}' ```  In both these modes the URL of the descriptor cannot be changed. To change the descriptor location and upgrade an installation the request must be made exclusively with a `descriptor_url`.   ``` $ curl -X PUT https://api.bitbucket.org/2.0/addon \\   -H \"Authorization: JWT <JWT Token>\" \\   --header \"Content-Type: application/json\" \\   --data '{\"descriptor_url\": $NEW_URL}' ```  The `descriptor_url` must exactly match the marketplace registration that Atlassian has for the application. Contact your Atlassian developer advocate to update this registration. Once the registration has been updated you may call this resource for each installation.  Note that the scopes of the application cannot be increased in the new descriptor nor reduced to none.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

