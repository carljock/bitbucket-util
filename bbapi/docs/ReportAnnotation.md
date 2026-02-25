# ReportAnnotation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**external_id** | Option<**String**> | ID of the annotation provided by the annotation creator. It can be used to identify the annotation as an alternative to it's generated uuid. It is not used by Bitbucket, but only by the annotation creator for updating or deleting this specific annotation. Needs to be unique. | [optional]
**uuid** | Option<**String**> | The UUID that can be used to identify the annotation. | [optional]
**annotation_type** | Option<**AnnotationType**> | The type of the report. (enum: VULNERABILITY, CODE_SMELL, BUG) | [optional]
**path** | Option<**String**> | The path of the file on which this annotation should be placed. This is the path of the file relative to the git repository. If no path is provided, then it will appear in the overview modal on all pull requests where the tip of the branch is the given commit, regardless of which files were modified. | [optional]
**line** | Option<**i32**> | The line number that the annotation should belong to. If no line number is provided, then it will default to 0 and in a pull request it will appear at the top of the file specified by the path field. | [optional]
**summary** | Option<**String**> | The message to display to users. | [optional]
**details** | Option<**String**> | The details to show to users when clicking on the annotation. | [optional]
**result** | Option<**Result**> | The state of the report. May be set to PENDING and later updated. (enum: PASSED, FAILED, SKIPPED, IGNORED) | [optional]
**severity** | Option<**Severity**> | The severity of the annotation. (enum: CRITICAL, HIGH, MEDIUM, LOW) | [optional]
**link** | Option<**String**> | A URL linking to the annotation in an external tool. | [optional]
**created_on** | Option<**String**> | The timestamp when the report was created. | [optional]
**updated_on** | Option<**String**> | The timestamp when the report was updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


