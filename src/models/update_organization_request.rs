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
pub struct UpdateOrganizationRequest {
    #[serde(rename = "name")]
    pub name: String,
}

impl UpdateOrganizationRequest {
    pub fn new(name: String) -> UpdateOrganizationRequest {
        UpdateOrganizationRequest {
            name,
        }
    }
}


