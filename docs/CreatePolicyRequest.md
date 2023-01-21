# CreatePolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject** | Option<[**crate::models::Subject**](Subject.md)> |  | [optional]
**resource** | Option<[**crate::models::Resource**](Resource.md)> |  | [optional]
**actions** | [**Vec<crate::models::Action>**](Action.md) | The actions to be allowed on the resource if an access policy matches a request.  At least one action must be provided and executable actions (i.e: `read`, `write`) cannot be combined with non-executable actions (i.e: `deny`).  If multiple access policies match a request, policies with a `deny` action take precedence over policies with a `read` action. You can define `deny` access policies to make exceptions in your policies that allow access.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


