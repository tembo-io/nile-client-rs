# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**updated** | Option<**String**> |  | [optional][readonly]
**seq** | Option<**i64**> |  | [optional][readonly]
**r#type** | **String** |  | 
**metadata** | Option<[**serde_json::Value**](.md)> | Arbitrary metadata. | [optional]
**org_memberships** | Option<[**::std::collections::HashMap<String, crate::models::OrgMembership>**](OrgMembership.md)> |  | [optional]
**email** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


