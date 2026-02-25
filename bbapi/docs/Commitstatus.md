# Commitstatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**links** | Option<[**models::CommitStatusLinks**](CommitStatusLinks.md)> |  | [optional]
**key** | **String** | An identifier for the status that's unique to         its type (current \"build\" is the only supported type) and the vendor,         e.g. BB-DEPLOY | 
**refname** | Option<**String**> |  The name of the ref that pointed to this commit at the time the status object was created. Note that this the ref may since have moved off of the commit. This optional field can be useful for build systems whose build triggers and configuration are branch-dependent (e.g. a Pipeline build). It is legitimate for this field to not be set, or even apply (e.g. a static linting job). | [optional]
**url** | Option<**String**> | A URL linking back to the vendor or build system, for providing more information about whatever process produced this status. Accepts context variables `repository` and `commit` that Bitbucket will evaluate at runtime whenever at runtime. For example, one could use https://foo.com/builds/{repository.full_name} which Bitbucket will turn into https://foo.com/builds/foo/bar at render time. | [optional]
**state** | **State** | Provides some indication of the status of this commit (enum: FAILED, INPROGRESS, STOPPED, SUCCESSFUL) | 
**name** | Option<**String**> | An identifier for the build itself, e.g. BB-DEPLOY-1 | [optional]
**description** | Option<**String**> | A description of the build (e.g. \"Unit tests in Bamboo\") | [optional]
**created_on** | Option<**String**> |  | [optional]
**updated_on** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


