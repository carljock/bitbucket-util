# Report

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**uuid** | Option<**String**> | The UUID that can be used to identify the report. | [optional]
**title** | Option<**String**> | The title of the report. | [optional]
**details** | Option<**String**> | A string to describe the purpose of the report. | [optional]
**external_id** | Option<**String**> | ID of the report provided by the report creator. It can be used to identify the report as an alternative to it's generated uuid. It is not used by Bitbucket, but only by the report creator for updating or deleting this specific report. Needs to be unique. | [optional]
**reporter** | Option<**String**> | A string to describe the tool or company who created the report. | [optional]
**link** | Option<**String**> | A URL linking to the results of the report in an external tool. | [optional]
**remote_link_enabled** | Option<**bool**> | If enabled, a remote link is created in Jira for the work item associated with the commit the report belongs to. | [optional]
**logo_url** | Option<**String**> | A URL to the report logo. If none is provided, the default insights logo will be used. | [optional]
**report_type** | Option<**ReportType**> | The type of the report. (enum: SECURITY, COVERAGE, TEST, BUG) | [optional]
**result** | Option<**Result**> | The state of the report. May be set to PENDING and later updated. (enum: PASSED, FAILED, PENDING) | [optional]
**data** | Option<[**Vec<models::ReportData>**](ReportData.md)> | An array of data fields to display information on the report. Maximum 10. | [optional]
**created_on** | Option<**String**> | The timestamp when the report was created. | [optional]
**updated_on** | Option<**String**> | The timestamp when the report was updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


