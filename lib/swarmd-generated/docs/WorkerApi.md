# \WorkerApi

All URIs are relative to *http://127.0.0.1:8087*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organization_id_project_project_id_worker_upload_post**](WorkerApi.md#organization_id_project_project_id_worker_upload_post) | **POST** /{organization_id}/project/{project_id}/worker/upload | Upload a new worker for the project



## organization_id_project_project_id_worker_upload_post

> crate::models::UploadWorkerResponse organization_id_project_project_id_worker_upload_post(organization_id, project_id, body)
Upload a new worker for the project

Upload a worker for a project, the worker won't be active and routed, it'll just be available for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**project_id** | **String** |  | [required] |
**body** | **String** | Raw javascript script. | [required] |

### Return type

[**crate::models::UploadWorkerResponse**](UploadWorkerResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/javascript
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

