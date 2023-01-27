# \EntitiesApi

All URIs are relative to *https://raw.githubusercontent.com/TheNileDev/nile-py/main/spec/net-lb-prod-6faf2ab-850391211.us-west-2.elb.amazonaws.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_entity**](EntitiesApi.md#create_entity) | **POST** /workspaces/{workspace}/entities | Create an entity
[**create_instance**](EntitiesApi.md#create_instance) | **POST** /workspaces/{workspace}/orgs/{org}/instances/{type} | Create a new instance
[**delete_instance**](EntitiesApi.md#delete_instance) | **DELETE** /workspaces/{workspace}/orgs/{org}/instances/{type}/{id} | Delete an instance
[**get_entity**](EntitiesApi.md#get_entity) | **GET** /workspaces/{workspace}/entities/{type} | Get an entity
[**get_instance**](EntitiesApi.md#get_instance) | **GET** /workspaces/{workspace}/orgs/{org}/instances/{type}/{id} | Get an instance
[**get_open_api**](EntitiesApi.md#get_open_api) | **GET** /workspaces/{workspace}/entities/{type}/openapi | Get a yaml OpenAPI description of an entity
[**instance_period_events**](EntitiesApi.md#instance_period_events) | **GET** /workspaces/{workspace}/events/{type} | Get instance events
[**list_entities**](EntitiesApi.md#list_entities) | **GET** /workspaces/{workspace}/entities | List all entities
[**list_instances**](EntitiesApi.md#list_instances) | **GET** /workspaces/{workspace}/orgs/{org}/instances/{type} |  List all instances
[**list_instances_in_workspace**](EntitiesApi.md#list_instances_in_workspace) | **GET** /workspaces/{workspace}/instances/{type} |  List all instances
[**patch_instance**](EntitiesApi.md#patch_instance) | **PATCH** /workspaces/{workspace}/orgs/{org}/instances/{type}/{id} | Patch an Instance
[**update_entity**](EntitiesApi.md#update_entity) | **PUT** /workspaces/{workspace}/entities/{type} | Update an entity
[**update_instance**](EntitiesApi.md#update_instance) | **PUT** /workspaces/{workspace}/orgs/{org}/instances/{type}/{id} | Update an instance



## create_entity

> crate::models::Entity create_entity(workspace, create_entity_request)
Create an entity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**create_entity_request** | [**CreateEntityRequest**](CreateEntityRequest.md) |  | [required] |

### Return type

[**crate::models::Entity**](Entity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> crate::models::Instance create_instance(workspace, org, r#type, body)
Create a new instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::Instance**](Instance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance

> delete_instance(workspace, org, r#type, id)
Delete an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entity

> crate::models::Entity get_entity(workspace, r#type)
Get an entity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**r#type** | **String** |  | [required] |

### Return type

[**crate::models::Entity**](Entity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance

> crate::models::Instance get_instance(workspace, org, r#type, id)
Get an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::Instance**](Instance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_open_api

> String get_open_api(workspace, r#type)
Get a yaml OpenAPI description of an entity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**r#type** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_period_events

> Vec<crate::models::InstanceEvent> instance_period_events(workspace, r#type, seq, limit)
Get instance events

Gets all events for the given entity type. As access policies are enforced on events, if the `before` instance fails access control, it will be omitted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**r#type** | **String** |  | [required] |
**seq** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 20]

### Return type

[**Vec<crate::models::InstanceEvent>**](InstanceEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_entities

> Vec<crate::models::Entity> list_entities(workspace)
List all entities

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |

### Return type

[**Vec<crate::models::Entity>**](Entity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instances

> Vec<crate::models::Instance> list_instances(workspace, org, r#type)
 List all instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**r#type** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Instance>**](Instance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instances_in_workspace

> Vec<crate::models::Instance> list_instances_in_workspace(workspace, r#type)
 List all instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**r#type** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Instance>**](Instance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_instance

> patch_instance(workspace, org, r#type, id, patch_operation, if_match)
Patch an Instance

If the instance field `seq` is provided in the `If-Match` header then this will be a Compare And Set operation, i.e., if the seq of the current instance doesn't match the one provided then a 412 status will be returned. If the header is not provided then this patch may overwrite other updates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**id** | **String** |  | [required] |
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |
**if_match** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_entity

> crate::models::Entity update_entity(workspace, r#type, update_entity_request)
Update an entity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**r#type** | **String** |  | [required] |
**update_entity_request** | [**UpdateEntityRequest**](UpdateEntityRequest.md) |  | [required] |

### Return type

[**crate::models::Entity**](Entity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance

> crate::models::Instance update_instance(workspace, org, r#type, id, update_instance_request, if_match)
Update an instance

If the instance field `seq` is provided in the `If-Match` header then this will be a Compare And Set operation, i.e., if the seq of the current instance doesn't match the one provided then a 412 status will be returned. If the header is not provided then this update will overwrite any other updates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_instance_request** | [**UpdateInstanceRequest**](UpdateInstanceRequest.md) |  | [required] |
**if_match** | Option<**String**> |  |  |

### Return type

[**crate::models::Instance**](Instance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

