use crate::config::api::inspections_v2::connection_url as inspections_v2_url;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase"), deny_unknown_fields)]
pub struct InspectionType {
    pub id: Uuid,

    pub name: String,
    pub note: Option<String>,
    pub status: InspectionTypeStatus,

    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InspectionTypeStatus {
    #[serde(alias = "active")]
    Published,

    #[serde(alias = "paused", alias = "disabled")]
    Draft,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FetchResult {
    #[serde(alias = "records")]
    pub inspection_types: Vec<InspectionType>,

    pub total: u32,
}

pub async fn fetch(app_slug: &str, token: String) -> FetchResult {
    let url = format!(
        "{hostname}/{app_slug}/query/inspection-types",
        hostname = inspections_v2_url(),
        app_slug = app_slug
    );
    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();

    let json_response;
    if response.status().is_success() {
        json_response = response.json::<FetchResult>().await.unwrap();
    } else {
        json_response = FetchResult {
            inspection_types: Vec::new(),
            total: 0,
        };
    }
    json_response
}

// TODO - If there isn't any inspection types to create one.
/*
pub async fn create(app_slug: &str, token: String, fake_inspection_type: Vec<_>) {
    let url = format!("{}/v1/{}/inspection-types", inspections_v2_url(), app_slug);

    let client = reqwest::Client::new();

    let response = client
        .post(&url)
        .bearer_auth(token)
        .json(&fake_inspection_type)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        println!("{:?}", "ERROR: Issue with post inspection type".red());
    }
}
*/
