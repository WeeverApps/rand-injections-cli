use crate::config::api::inspections_v2::connection_url as inspections_url;
use eyre::eyre;
use eyre::Error;
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct DowntimeRecord {
    pub downtime_id: Uuid,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
    pub shift_id: Uuid,
    pub shift_name: String,
    pub asset_id: Uuid,
    pub asset_name: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub cancelled_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct RouteResponse {
    status: String,
    downtimes: Vec<DowntimeRecord>,
}

pub async fn fetch(app_slug: &str, token: String) -> Result<RouteResponse, Error> {
    let url = format!("{}/{}/downtimes", inspections_url(), app_slug,);

    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();
    if response.status().is_success() {
        Ok(response.json::<RouteResponse>().await.unwrap())
    } else {
        Err(eyre!("ERROR: Downtime fetch was unsuccessful"))
    }
}
