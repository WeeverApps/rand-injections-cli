use crate::config::api::dsm::connection_url as dsm_url;
use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;
// use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TierRecord {
    app_slug: String,
    parent_id: Option<Uuid>,
    pub id: Uuid,
    name: String,
    note: Option<String>,
    status: TierStatus,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TierStatus {
    Published,
    Disabled,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TiersResult {
    #[serde(alias = "records")]
    pub tiers: Vec<TierRecord>,
}
pub async fn tiers(app_slug: &str, token: String) -> TiersResult {
    let url = format!("{}/v1/{}/tiers", dsm_url(), app_slug);
    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();

    let json_response;
    if response.status().is_success() {
        json_response = response.json::<TiersResult>().await.unwrap();
    } else {
        json_response = TiersResult { tiers: Vec::new() };
    }
    json_response
}
