# Filter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metric_name** | Option<**String**> | Name of the metric to filter on | [optional]
**instance_id** | Option<**String**> | The Nile instance id to filter on | [optional]
**entity_type** | Option<**String**> | The Nile entity type to filter on. This is ignored if entity_type is on a URL param. | [optional]
**organization_id** | Option<**String**> | The Nile organization id to filter on | [optional]
**start_time** | Option<**String**> | The ISO-8601 formatted timestamp used to begin searching for matching metrics, i.e., 2018-11-13T20:20:39+00:00. If not provided the range will start from the epoch. Results returned are inclusive of this timestamp. | [optional]
**duration** | Option<**i32**> | The duration is added to from_timestamp to limit the time range of the query. i.e., the query will be restricted to metric.timestamp >= from_timestamp AND metric.timestamp < from_timestamp + duration.  If not provided or the duration is <=0 then the end timestamp is set to now | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


