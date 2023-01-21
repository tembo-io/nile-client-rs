# AggregationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bucket_size** | Option<**String**> | The size of the bucket | [optional]
**buckets** | Option<**i32**> | Number of buckets to return. Defaults to 3 if not provided. | [optional][default to 3]
**start_time** | **String** | An ISO-8601 formatted date-time, i.e., 2018-11-13T20:20:39+00:00, that the aggregation should start at. This time will be truncated based on bucket_size, i.e., if bucket_size is 1h, then the start_time will be truncated to the nearest hour. | 
**organization_id** | Option<**String**> | The Nile organization id to aggregate on | [optional]
**instance_id** | Option<**String**> | The optional Nile instance id to aggregate on | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


