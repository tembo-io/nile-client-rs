/*
 * Nile API
 *
 * Making SaaS chill.
 *
 * The version of the OpenAPI document: 0.1.0-fdd7cd5
 * Contact: support@thenile.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "error_code")]
    pub error_code: ErrorCode,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "status_code")]
    pub status_code: i32,
}

impl Error {
    pub fn new(error_code: ErrorCode, message: String, status_code: i32) -> Error {
        Error {
            error_code,
            message,
            status_code,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "internal_error")]
    InternalError,
    #[serde(rename = "bad_request")]
    BadRequest,
    #[serde(rename = "unauthorized_credentials")]
    UnauthorizedCredentials,
    #[serde(rename = "user_not_found")]
    UserNotFound,
    #[serde(rename = "org_not_found")]
    OrgNotFound,
    #[serde(rename = "workspace_not_found")]
    WorkspaceNotFound,
    #[serde(rename = "invite_not_found")]
    InviteNotFound,
    #[serde(rename = "duplicate_org_name")]
    DuplicateOrgName,
    #[serde(rename = "duplicate_workspace_name")]
    DuplicateWorkspaceName,
    #[serde(rename = "empty_org_name")]
    EmptyOrgName,
    #[serde(rename = "empty_workspace_name")]
    EmptyWorkspaceName,
    #[serde(rename = "duplicate_user_email")]
    DuplicateUserEmail,
    #[serde(rename = "user_already_in_org")]
    UserAlreadyInOrg,
    #[serde(rename = "duplicate_entity_name")]
    DuplicateEntityName,
    #[serde(rename = "entity_not_found")]
    EntityNotFound,
    #[serde(rename = "instance_not_found")]
    InstanceNotFound,
    #[serde(rename = "access_policy_not_found")]
    AccessPolicyNotFound,
    #[serde(rename = "invalid_entity_schema")]
    InvalidEntitySchema,
    #[serde(rename = "invalid_id")]
    InvalidId,
    #[serde(rename = "invalid_action")]
    InvalidAction,
    #[serde(rename = "empty_actions")]
    EmptyActions,
    #[serde(rename = "invalid_action_combination")]
    InvalidActionCombination,
    #[serde(rename = "invalid_policy_variable")]
    InvalidPolicyVariable,
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "metric_not_found")]
    MetricNotFound,
    #[serde(rename = "conflict")]
    Conflict,
    #[serde(rename = "access_token_not_found")]
    AccessTokenNotFound,
    #[serde(rename = "precondition_failed")]
    PreconditionFailed,
}

impl Default for ErrorCode {
    fn default() -> ErrorCode {
        Self::InternalError
    }
}
