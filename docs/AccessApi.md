# \AccessApi

All URIs are relative to *https://raw.githubusercontent.com/TheNileDev/nile-py/main/spec/net-lb-prod-6faf2ab-850391211.us-west-2.elb.amazonaws.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policy**](AccessApi.md#create_policy) | **POST** /workspaces/{workspace}/orgs/{org}/access/policies | Create a new access policy
[**delete_policy**](AccessApi.md#delete_policy) | **DELETE** /workspaces/{workspace}/orgs/{org}/access/policies/{policyId} | Delete an access policy
[**get_policy**](AccessApi.md#get_policy) | **GET** /workspaces/{workspace}/orgs/{org}/access/policies/{policyId} | Get an access policy
[**list_policies**](AccessApi.md#list_policies) | **GET** /workspaces/{workspace}/orgs/{org}/access/policies | List all access policies
[**update_policy**](AccessApi.md#update_policy) | **PUT** /workspaces/{workspace}/orgs/{org}/access/policies/{policyId} | Update an access policy



## create_policy

> crate::models::Policy create_policy(workspace, org, create_policy_request)
Create a new access policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**create_policy_request** | [**CreatePolicyRequest**](CreatePolicyRequest.md) |  | [required] |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy

> delete_policy(workspace, org, policy_id)
Delete an access policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**policy_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy

> crate::models::Policy get_policy(workspace, org, policy_id)
Get an access policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**policy_id** | **String** |  | [required] |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_policies

> Vec<crate::models::Policy> list_policies(workspace, org)
List all access policies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Policy>**](Policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_policy

> crate::models::Policy update_policy(workspace, org, policy_id, update_policy_request)
Update an access policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**policy_id** | **String** |  | [required] |
**update_policy_request** | [**UpdatePolicyRequest**](UpdatePolicyRequest.md) |  | [required] |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

