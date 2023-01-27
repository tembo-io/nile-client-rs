# \MetricsApi

All URIs are relative to *https://raw.githubusercontent.com/TheNileDev/nile-py/main/spec/net-lb-prod-6faf2ab-850391211.us-west-2.elb.amazonaws.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aggregate_metrics**](MetricsApi.md#aggregate_metrics) | **POST** /workspaces/{workspace}/metrics/{metric_name}/aggregate | Perform sum, min, max, avg, and percentile aggregations over a metric 
[**filter_metrics**](MetricsApi.md#filter_metrics) | **POST** /workspaces/{workspace}/metrics/filter | List of metrics matching the filter
[**filter_metrics_for_entity_type**](MetricsApi.md#filter_metrics_for_entity_type) | **POST** /workspaces/{workspace}/metrics/entities/{entity_type}/filter | List metrics for the entity matching the filter
[**list_metric_definitions**](MetricsApi.md#list_metric_definitions) | **GET** /workspaces/{workspace}/metrics/metric_definitions | List metric definitions in a workspace
[**list_metric_definitions_for_entity_type**](MetricsApi.md#list_metric_definitions_for_entity_type) | **GET** /workspaces/{workspace}/metrics/entities/{entity_type}/metric_definitions | List metric definitions for an entity
[**produce_batch_of_metrics**](MetricsApi.md#produce_batch_of_metrics) | **POST** /workspaces/{workspace}/metrics | Produce a Batch of Metrics



## aggregate_metrics

> Vec<crate::models::Bucket> aggregate_metrics(workspace, metric_name, aggregation_request)
Perform sum, min, max, avg, and percentile aggregations over a metric 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**metric_name** | **String** |  | [required] |
**aggregation_request** | [**AggregationRequest**](AggregationRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::Bucket>**](Bucket.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## filter_metrics

> Vec<crate::models::Metric> filter_metrics(workspace, filter)
List of metrics matching the filter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**filter** | [**Filter**](Filter.md) |  | [required] |

### Return type

[**Vec<crate::models::Metric>**](Metric.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## filter_metrics_for_entity_type

> Vec<crate::models::Metric> filter_metrics_for_entity_type(workspace, entity_type, filter)
List metrics for the entity matching the filter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**entity_type** | **String** |  | [required] |
**filter** | [**Filter**](Filter.md) |  | [required] |

### Return type

[**Vec<crate::models::Metric>**](Metric.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metric_definitions

> crate::models::ListMetricDefinitionsResponse list_metric_definitions(workspace)
List metric definitions in a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |

### Return type

[**crate::models::ListMetricDefinitionsResponse**](ListMetricDefinitionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metric_definitions_for_entity_type

> crate::models::ListMetricDefinitionsResponse list_metric_definitions_for_entity_type(workspace, entity_type)
List metric definitions for an entity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**entity_type** | **String** |  | [required] |

### Return type

[**crate::models::ListMetricDefinitionsResponse**](ListMetricDefinitionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## produce_batch_of_metrics

> produce_batch_of_metrics(workspace, metric)
Produce a Batch of Metrics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**metric** | [**Vec<crate::models::Metric>**](Metric.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

