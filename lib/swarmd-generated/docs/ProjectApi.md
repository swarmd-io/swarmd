# \ProjectApi

All URIs are relative to *http://127.0.0.1:8087*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organization_id_project_create_post**](ProjectApi.md#organization_id_project_create_post) | **POST** /{organization_id}/project/create | Create Project
[**organization_id_project_get**](ProjectApi.md#organization_id_project_get) | **GET** /{organization_id}/project/ | List projects
[**organization_id_project_project_id_publish_put**](ProjectApi.md#organization_id_project_project_id_publish_put) | **PUT** /{organization_id}/project/{project_id}/publish | Publish a worker
[**organization_id_project_slug_project_slug_get**](ProjectApi.md#organization_id_project_slug_project_slug_get) | **GET** /{organization_id}/project/slug/{project_slug} | Get Project



## organization_id_project_create_post

> crate::models::CreateProjectResponse organization_id_project_create_post(organization_id, create_project_payload)
Create Project

Create a new project for the current organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**create_project_payload** | [**CreateProjectPayload**](CreateProjectPayload.md) |  | [required] |

### Return type

[**crate::models::CreateProjectResponse**](CreateProjectResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_id_project_get

> crate::models::ListProjectResponse organization_id_project_get(organization_id, list_projects_params)
List projects

List projects for the current organization, you must be at least a Member of this organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**list_projects_params** | [**ListProjectsParams**](ListProjectsParams.md) |  | [required] |

### Return type

[**crate::models::ListProjectResponse**](ListProjectResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_id_project_project_id_publish_put

> crate::models::PublishWorkerResponse organization_id_project_project_id_publish_put(organization_id, project_id, publish_worker_payload)
Publish a worker

Change the main worker for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**project_id** | **String** |  | [required] |
**publish_worker_payload** | [**PublishWorkerPayload**](PublishWorkerPayload.md) |  | [required] |

### Return type

[**crate::models::PublishWorkerResponse**](PublishWorkerResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_id_project_slug_project_slug_get

> crate::models::GetBySlugResponse organization_id_project_slug_project_slug_get(organization_id, project_slug)
Get Project

Get a project by slug, you must be at least a Member of this organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**project_slug** | **String** |  | [required] |

### Return type

[**crate::models::GetBySlugResponse**](GetBySlugResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

