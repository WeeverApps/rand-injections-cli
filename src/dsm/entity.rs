use crate::config::api::dsm::connection_url as dsm_url;
use chrono::{offset::Utc, DateTime};
use colored::Colorize;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid_5::Uuid;

#[derive(Serialize, Debug)]
pub struct DataSourceEntity {
    pub tier_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub note: Option<String>,
    pub status: EntityStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Dummy)]
#[serde(rename_all = "lowercase")]
pub enum EntityStatus {
    Published,
    Disabled,
}

impl FromStr for EntityStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "published" => Ok(EntityStatus::Published),
            "disabled" => Ok(EntityStatus::Disabled),
            _ => Err("Invalid status value"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntitiesResult {
    #[serde(alias = "records")]
    pub assets: Vec<EntityRecord>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityRecord {
    app_slug: String,
    tier_id: Uuid,
    parent_id: Option<Uuid>,
    pub id: Uuid,
    name: String,
    note: Option<String>,
    status: EntityStatus,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

pub async fn get_entities(app_slug: &str, token: String, tier_id: Uuid) -> EntitiesResult {
    // let hostname = config::api::dsm::connection_url();
    let url = format!("{}/v1/{}/assets?tier_id={}", dsm_url(), app_slug, tier_id);
    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();

    let json_response;
    if response.status().is_success() {
        json_response = response.json::<EntitiesResult>().await.unwrap();
    } else {
        json_response = EntitiesResult { assets: Vec::new() };
    }
    json_response
}

pub async fn post_entity(app_slug: &str, fake_dse: Vec<DataSourceEntity>, token: String) {
    // let hostname = config::api::dsm::connection_url();
    let url = format!("{}/v1/{}/assets", dsm_url(), app_slug);

    let client = reqwest::Client::new();

    let response = client
        .post(&url)
        .bearer_auth(token)
        .json(&fake_dse)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        println!("{:?}", "ERROR: Issue with post entity".red());
    }
}
