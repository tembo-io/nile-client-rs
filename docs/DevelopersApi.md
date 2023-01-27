# \DevelopersApi

All URIs are relative to *https://raw.githubusercontent.com/TheNileDev/nile-py/main/spec/net-lb-prod-6faf2ab-850391211.us-west-2.elb.amazonaws.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_developer**](DevelopersApi.md#create_developer) | **POST** /developers | Create a developer
[**developer_google_o_auth_callback**](DevelopersApi.md#developer_google_o_auth_callback) | **GET** /auth/oauth/google/callback | Developer Google OAuth flow callback
[**login_developer**](DevelopersApi.md#login_developer) | **POST** /auth/login | Log in a developer to nile
[**start_developer_google_o_auth**](DevelopersApi.md#start_developer_google_o_auth) | **GET** /auth/oauth/google | Start the developer Google OAuth flow
[**validate_developer**](DevelopersApi.md#validate_developer) | **POST** /auth/validate | Validate a developer token



## create_developer

> crate::models::User create_developer(create_user_request)
Create a developer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## developer_google_o_auth_callback

> crate::models::DeveloperGoogleOAuthResponse developer_google_o_auth_callback(code, state)
Developer Google OAuth flow callback

This endpoint is called automatically by Google after the user authenticates successfully. It's here for documentation purposes only, and it shouldn't be called directly. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | Option<**String**> |  |  |
**state** | Option<**String**> |  |  |

### Return type

[**crate::models::DeveloperGoogleOAuthResponse**](DeveloperGoogleOAuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_developer

> crate::models::Token login_developer(login_info)
Log in a developer to nile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_info** | [**LoginInfo**](LoginInfo.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_developer_google_o_auth

> start_developer_google_o_auth(redirect_to)
Start the developer Google OAuth flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**redirect_to** | Option<**String**> | An optional URL to redirect to after a successful login/signup. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_developer

> validate_developer(token)
Validate a developer token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | [**Token**](Token.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

