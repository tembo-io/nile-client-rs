# \OrganizationsApi

All URIs are relative to *https://raw.githubusercontent.com/TheNileDev/nile-py/main/spec/net-lb-prod-6faf2ab-850391211.us-west-2.elb.amazonaws.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invite**](OrganizationsApi.md#accept_invite) | **POST** /workspaces/{workspace}/orgs/{org}/invites/{code}/accept | Accept an invite
[**add_user_to_org**](OrganizationsApi.md#add_user_to_org) | **POST** /workspaces/{workspace}/orgs/{org}/users | Add a user to an organization
[**create_organization**](OrganizationsApi.md#create_organization) | **POST** /workspaces/{workspace}/orgs | Create a new organization
[**delete_organization**](OrganizationsApi.md#delete_organization) | **DELETE** /workspaces/{workspace}/orgs/{org} | Delete an organization by id
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /workspaces/{workspace}/orgs/{org} | Get an organization by name
[**list_invites**](OrganizationsApi.md#list_invites) | **GET** /workspaces/{workspace}/orgs/{org}/invites | List all Invites
[**list_organizations**](OrganizationsApi.md#list_organizations) | **GET** /workspaces/{workspace}/orgs | List all organizations
[**list_users_in_org**](OrganizationsApi.md#list_users_in_org) | **GET** /workspaces/{workspace}/orgs/{org}/users | List users in an organization
[**remove_user_from_org**](OrganizationsApi.md#remove_user_from_org) | **DELETE** /workspaces/{workspace}/orgs/{org}/users/{user} | Remove a user from an organization by user id
[**update_organization**](OrganizationsApi.md#update_organization) | **PUT** /workspaces/{workspace}/orgs/{org} | Update an organization
[**update_user_in_org**](OrganizationsApi.md#update_user_in_org) | **PUT** /workspaces/{workspace}/orgs/{org}/users/{user} | Update a user in an organization



## accept_invite

> accept_invite(workspace, org, code)
Accept an invite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**code** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_to_org

> crate::models::User add_user_to_org(workspace, org, add_user_to_org_request)
Add a user to an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**add_user_to_org_request** | [**AddUserToOrgRequest**](AddUserToOrgRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization

> crate::models::Organization create_organization(workspace, create_organization_request)
Create a new organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**create_organization_request** | [**CreateOrganizationRequest**](CreateOrganizationRequest.md) |  | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> delete_organization(workspace, org)
Delete an organization by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> crate::models::Organization get_organization(workspace, org)
Get an organization by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_invites

> Vec<crate::models::Invite> list_invites(workspace, org)
List all Invites

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Invite>**](Invite.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organizations

> Vec<crate::models::Organization> list_organizations(workspace)
List all organizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |

### Return type

[**Vec<crate::models::Organization>**](Organization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users_in_org

> Vec<crate::models::User> list_users_in_org(workspace, org)
List users in an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_org

> remove_user_from_org(workspace, org, user)
Remove a user from an organization by user id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**user** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> crate::models::Organization update_organization(workspace, org, update_organization_request)
Update an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**update_organization_request** | [**UpdateOrganizationRequest**](UpdateOrganizationRequest.md) |  | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_in_org

> crate::models::User update_user_in_org(workspace, org, user, update_organization_membership_request)
Update a user in an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace** | **String** | The name of the Nile workspace where all the data-plane metadata for this user is stored | [required] |
**org** | **String** |  | [required] |
**user** | **String** |  | [required] |
**update_organization_membership_request** | [**UpdateOrganizationMembershipRequest**](UpdateOrganizationMembershipRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

