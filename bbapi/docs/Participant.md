# Participant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**user** | Option<[**models::Account**](Account.md)> |  | [optional]
**role** | Option<**Role**> |  (enum: PARTICIPANT, REVIEWER) | [optional]
**approved** | Option<**bool**> |  | [optional]
**state** | Option<**State**> |  (enum: approved, changes_requested, ) | [optional]
**participated_on** | Option<**String**> | The ISO8601 timestamp of the participant's action. For approvers, this is the time of their approval. For commenters and pull request reviewers who are not approvers, this is the time they last commented, or null if they have not commented. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


