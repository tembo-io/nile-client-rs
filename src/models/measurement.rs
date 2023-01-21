/*
 * Nile API
 *
 * Making SaaS chill.
 *
 * The version of the OpenAPI document: 0.1.0-fdd7cd5
 * Contact: support@thenile.dev
 * Generated by: https://openapi-generator.tech
 */

/// Measurement : Measurements associated with this metric



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Measurement {
    /// An ISO-8601 formatted date-time, i.e., 2018-11-13T20:20:39+00:00, that represents the time the measurement was created.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// the measured value
    #[serde(rename = "value")]
    pub value: f64,
    /// InstanceId of the Nile instance this measurement is related to
    #[serde(rename = "instance_id")]
    pub instance_id: String,
}

impl Measurement {
    /// Measurements associated with this metric
    pub fn new(timestamp: String, value: f64, instance_id: String) -> Measurement {
        Measurement {
            timestamp,
            value,
            instance_id,
        }
    }
}


