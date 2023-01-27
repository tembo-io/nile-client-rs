# \UsersApi

All URIs are relative to *https://raw.githubusercontent.com/TheNileDev/nile-py/main/spec/net-lb-prod-6faf2ab-850391211.us-west-2.elb.amazonaws.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_developer_owned_user**](UsersApi.md#create_developer_owned_user) | **POST** /workspaces/{workspace}/internal/users | Create a user
[**create_user**](UsersApi.md#create_user) | **POST** /workspaces/{workspace}/users | Create a user
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /workspaces/{workspace}/users/{id} | Delete a user
[**get_user**](UsersApi.md#get_user) | **GET** /workspaces/{workspace}/users/{id} | Get a user by id
[**list_users**](UsersApi.md#list_users) | **GET** /workspaces/{workspace}/users | List all users for a workspace
[**login_user**](UsersApi.md#login_user) | **POST** /workspaces/{workspace}/auth/login | Log in a user
[**me**](UsersApi.md#me) | **GET** /me | Get information about current authenticated user
[**token**](UsersApi.md#token) | **GET** /me/token | Get the auth token of the currently authenticated user
[**update_user**](UsersApi.md#update_user) | **PUT** /workspaces/{workspace}/users/{id} | Update a user
[**validate_user**](UsersApi.md#validate_user) | **POST** /workspaces/{workspace}/auth/validate | Validate a user token



## create_developer_owned_user

> crate::models::User create_developer_owned_user(workspace, create_developer_owned_user_request)
Create a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**create_developer_owned_user_request** | [**CreateDeveloperOwnedUserRequest**](CreateDeveloperOwnedUserRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::User create_user(workspace, create_user_request)
Create a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(workspace, id)
Delete a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::User get_user(workspace, id)
Get a user by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> Vec<crate::models::User> list_users(workspace)
List all users for a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_user

> crate::models::Token login_user(workspace, login_info)
Log in a user

Login a user to Nile. This operation returns a JWT token. Most Nile operations require authentication and expect this token in the 'Authorization: Bearer <token>' header

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**login_info** | [**LoginInfo**](LoginInfo.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## me

> crate::models::User me()
Get information about current authenticated user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token

> crate::models::Token token()
Get the auth token of the currently authenticated user

Echoes the auth token of the currently authenticated user. This operation requires that the auth token is passed either as a Bearer token in the authorization header or as a cookie named 'token'. When both are present, they must match.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Token**](Token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::User update_user(workspace, id, update_user_request)
Update a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**id** | **String** |  | [required] |
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_user

> validate_user(workspace, token)
Validate a user token

Validates a user token. Use this when using Nile authentication to validate access to non-Nile resources. See the [Add Authentication Guide](https://nile-docs.vercel.app/docs/current/guides/how-to/add_signup_authn#decorating-the-endpoint) for a full example

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**token** | [**Token**](Token.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

