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
pub struct RemoveOperation {
    /// The JSON Pointer path to remove
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "op")]
    pub op: Op,
    /// the value to add or replace
    #[serde(rename = "value")]
    pub value: serde_json::Value,
    /// The JSON Pointer path you would like to move/copy from
    #[serde(rename = "from")]
    pub from: String,
}

impl RemoveOperation {
    pub fn new(path: String, op: Op, value: serde_json::Value, from: String) -> RemoveOperation {
        RemoveOperation {
            path,
            op,
            value,
            from,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Op {
    #[serde(rename = "remove")]
    Remove,
}

impl Default for Op {
    fn default() -> Op {
        Self::Remove
    }
}

