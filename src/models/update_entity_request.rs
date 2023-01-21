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
pub struct UpdateEntityRequest {
    /// A JSON Schema
    #[serde(rename = "schema")]
    pub schema: serde_json::Value,
}

impl UpdateEntityRequest {
    pub fn new(schema: serde_json::Value) -> UpdateEntityRequest {
        UpdateEntityRequest {
            schema,
        }
    }
}

