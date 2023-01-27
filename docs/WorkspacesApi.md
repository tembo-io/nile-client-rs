# \WorkspacesApi

All URIs are relative to *https://raw.githubusercontent.com/TheNileDev/nile-py/main/spec/net-lb-prod-6faf2ab-850391211.us-west-2.elb.amazonaws.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_access_token**](WorkspacesApi.md#create_access_token) | **POST** /workspaces/{workspace}/access_tokens | Create an access token
[**create_workspace**](WorkspacesApi.md#create_workspace) | **POST** /workspaces | Create a workspace
[**delete_access_token**](WorkspacesApi.md#delete_access_token) | **DELETE** /workspaces/{workspace}/access_tokens/{id} | Delete an access token
[**get_access_token**](WorkspacesApi.md#get_access_token) | **GET** /workspaces/{workspace}/access_tokens/{id} | Get access token by id
[**get_workspace_open_api**](WorkspacesApi.md#get_workspace_open_api) | **GET** /workspaces/{workspace}/openapi | Get the OpenAPI specification for all events and entities in this workspace
[**list_access_tokens**](WorkspacesApi.md#list_access_tokens) | **GET** /workspaces/{workspace}/access_tokens | List access tokens
[**list_workspaces**](WorkspacesApi.md#list_workspaces) | **GET** /workspaces | List all workspaces
[**update_access_token**](WorkspacesApi.md#update_access_token) | **PUT** /workspaces/{workspace}/access_tokens/{id} | Update an access token



## create_access_token

> crate::models::CreateAccessTokenResponse create_access_token(workspace, create_access_token_request)
Create an access token

Create a workspace token. Workspace tokens have authorization to perform most actions within a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**create_access_token_request** | [**CreateAccessTokenRequest**](CreateAccessTokenRequest.md) |  | [required] |

### Return type

[**crate::models::CreateAccessTokenResponse**](CreateAccessTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_workspace

> crate::models::Workspace create_workspace(create_workspace_request)
Create a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_workspace_request** | [**CreateWorkspaceRequest**](CreateWorkspaceRequest.md) |  | [required] |

### Return type

[**crate::models::Workspace**](Workspace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_access_token

> delete_access_token(workspace, id)
Delete an access token

Delete a workspace access token. Any users or services using this token will no longer be able to access the workspace.

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


## get_access_token

> crate::models::AccessTokenInfo get_access_token(workspace, id)
Get access token by id

Get information about a workspace access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::AccessTokenInfo**](AccessTokenInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspace_open_api

> String get_workspace_open_api(workspace)
Get the OpenAPI specification for all events and entities in this workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/yaml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_access_tokens

> Vec<crate::models::AccessTokenInfo> list_access_tokens(workspace)
List access tokens

List all workspace access tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |

### Return type

[**Vec<crate::models::AccessTokenInfo>**](AccessTokenInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workspaces

> Vec<crate::models::Workspace> list_workspaces()
List all workspaces

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Workspace>**](Workspace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_access_token

> crate::models::AccessTokenInfo update_access_token(workspace, id, create_access_token_request)
Update an access token

Update a workspace access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**id** | **String** |  | [required] |
**create_access_token_request** | [**CreateAccessTokenRequest**](CreateAccessTokenRequest.md) |  | [required] |

### Return type

[**crate::models::AccessTokenInfo**](AccessTokenInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

