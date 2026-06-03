# Issue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**links** | Option<[**models::IssueLinks**](IssueLinks.md)> |  | [optional]
**id** | Option<**i32**> |  | [optional]
**repository** | Option<[**models::Repository**](Repository.md)> |  | [optional]
**title** | Option<**String**> |  | [optional]
**reporter** | Option<[**models::Account**](Account.md)> |  | [optional]
**assignee** | Option<[**models::Account**](Account.md)> |  | [optional]
**created_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**updated_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**edited_on** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**state** | Option<**State**> |  (enum: submitted, new, open, resolved, on hold, invalid, duplicate, wontfix, closed) | [optional]
**kind** | Option<**Kind**> |  (enum: bug, enhancement, proposal, task) | [optional]
**priority** | Option<**Priority**> |  (enum: trivial, minor, major, critical, blocker) | [optional]
**milestone** | Option<[**models::Milestone**](Milestone.md)> |  | [optional]
**version** | Option<[**models::Version**](Version.md)> |  | [optional]
**component** | Option<[**models::Component**](Component.md)> |  | [optional]
**votes** | Option<**i32**> |  | [optional]
**content** | Option<[**models::CommentContent**](CommentContent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


