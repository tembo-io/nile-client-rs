use dotenv::dotenv;
use reqwest;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use serde;
use serde_json;
use std::env;


#[derive(serde::Serialize, Debug)]
pub struct InstanceUpdate {
    pub op: String,
    pub path: String,
    pub value: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct EntityInstance {
    id: String,
    created: String,
    updated: String,
    seq: i32,

    #[serde(rename = "type")]
    type_: String,
    properties: serde_json::Value, // Properties are the entity spec
    org: String,
}

#[derive(serde::Deserialize, Debug)]
struct AuthResponse {
    token: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub enum EventType {
    CREATE,
    UPDATE,
    DELETE,
}

#[derive(serde::Deserialize, Debug)]
pub struct Event {
    pub timestamp: String,
    pub id: i32,
    pub event_type: EventType,
    pub before: Option<serde_json::Value>,
    pub after: serde_json::Value,
    pub org: String,
}

#[derive(Debug)]
pub struct NileClient {
    base_url: String,
    auth_path: String,
    _token: String,
}

// https://www.thenile.dev/rest-api#tag/entities/operation/getOpenAPI
impl Default for NileClient {
    fn default() -> NileClient {
        NileClient {
            base_url: "https://prod.thenile.dev".to_owned(),
            auth_path: "/auth/login".to_owned(),
            _token: "".to_owned(),
        }
    }
}

impl NileClient {
    pub async fn authenticate(
        &mut self,
        email: String,
        password: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        // TODO: add token authentication
        let body = &serde_json::json!({
            "email": email.to_owned(),
            "password": password.to_owned(),
        });

#[tokio::main]
impl NileClient {
pub async fn TokenAuthenticate() -> Result<(, Box<dyn std::error::Error>> {
    // load .env file into std::env
    dotenv().ok();

    // pull out token and key from .env variables
    let api_key: String = env::var(key: "NILE_API_KEY")?;
    let api_token: String = env::var(key: "NILE_API_TOKEN")?;

    // create a new header map to be used as request headers.
    let mut headers = HeaderMap::new();

    // populate headers map with token and key.
    // we have to parse the String into a HeaderValue using parse().
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_token).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());

    // create request client, sending request client with url, adding headers, and awaiting response.
    let client = reqwest::Client::new();
    let resp = client
        .get("/net-lb-prod-6faf2ab-850391211.us-west-2.elb.amazonaws.com/workspaces/{workspace}/auth/login")
        .headers(headers)
        .send()
        .await?;

    println!("{:#?}", resp);
}
}
        // TODO: handle errors. non-200 must be loud
        let auth = client
            .post(format!(
                "{base}{auth}",
                base = self.base_url,
                auth = self.auth_path
            ))
            .json(&body)
            .send()
            .await?
            .json::<AuthResponse>()
            .await?;

        self._token = auth.token;
        Ok(())
    }

    // poll for the events in a workspace/entity
    // TODO: how can we handle `seq`?
    pub async fn get_events(
        &self,
        workspace: &str,
        entity_name: &str,
        seq: i64,
        limit: i32,
    ) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        let uri = format!(
            "{base_url}/workspaces/{workspace}/events/{entity_name}?seq={seq}&limit={limit}",
            base_url = self.base_url,
            workspace = workspace,
            entity_name = entity_name,
            seq = seq,
            limit = limit
        );
        let client = reqwest::Client::new();
        let resp = client
            .get(uri)
            .header("Authorization", "Bearer ".to_owned() + &self._token)
            .send()
            .await?;
        let events = resp.json::<Vec<Event>>().await?;
        Ok(events)
    }

    // retrieve existing instances of the entity
    pub async fn get_instances(
        &self,
        workspace: &str,
        entity_name: &str,
    ) -> Result<Vec<EntityInstance>, Box<dyn std::error::Error>> {
        let uri = format!(
            "{base_url}/workspaces/{workspace}/instances/{entity_name}",
            base_url = self.base_url,
            workspace = workspace,
            entity_name = entity_name,
        );

        let client = reqwest::Client::new();
        let resp = client
            .get(uri)
            .header("Authorization", "Bearer ".to_owned() + &self._token)
            .send()
            .await?;
        let instances = resp.json::<Vec<EntityInstance>>().await?;
        Ok(instances)
    }

    // update attributes on an existing entity
    // only supports the ReplaceOperation
    // https://www.thenile.dev/rest-api#tag/entities/operation/patchInstance
    pub async fn patch_instance(
        &self,
        workspace: &str,
        org: &str,
        entity_name: &str,
        instance_id: &str,
        updates: Vec<InstanceUpdate>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let uri = format!(
            "{base_url}/workspaces/{workspace}/orgs/{org}/instances/{entity_name}/{id}",
            base_url = self.base_url,
            workspace = workspace,
            entity_name = entity_name,
            id = instance_id
        );

        let client = reqwest::Client::new();
        let resp = client
            .patch(uri)
            .json(&updates)
            .header("Authorization", "Bearer ".to_owned() + &self._token)
            .send()
            .await?;
        let resp_obj = resp.json::<serde_json::Value>().await.unwrap();
        Ok(resp_obj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event() {
        let event = Event {
            timestamp: "2020-10-01T00:00:00Z".to_owned(),
            id: 1,
            event_type: EventType::CREATE,
            before: None,
            after: serde_json::json!({}),
            org: "coredb".to_owned(),
        };
        assert_eq!(event.timestamp, "2020-10-01T00:00:00Z".to_owned());
        assert_eq!(event.id, 1);
        assert_eq!(event.event_type, EventType::CREATE);
        assert_eq!(event.before, None);
        assert_eq!(event.after, serde_json::json!({}));
        assert_eq!(event.org, "coredb".to_owned());
    }
}
