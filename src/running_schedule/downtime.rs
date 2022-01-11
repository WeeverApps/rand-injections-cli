use crate::config::api::inspections_v2::connection_url as inspections_url;
use crate::query_stringer;
use chrono::NaiveDate;
use eyre::eyre;
use eyre::Error;
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub downtimes: Vec<DowntimeRecord>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct QueryParams {
    pub date_to: Option<NaiveDate>,
    pub date_from: Option<NaiveDate>,
}

pub async fn fetch(
    app_slug: &str,
    token: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<RouteResponse, Error> {
    let query_string: &str = &format!("dateFrom={}&dateTo={}", start_date, end_date);
    let params: QueryParams = query_stringer::parse_encoded_qs(&query_string);
    let encoded_query_string = query_stringer::to_encoded_qs(&params);
    let url = format!(
        "{}/{}/downtimes?{}",
        inspections_url(),
        app_slug,
        encoded_query_string
    );
    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();
    if response.status().is_success() {
        Ok(response.json::<RouteResponse>().await.unwrap())
    } else {
        Err(eyre!(
            "ERROR - {}: Downtime fetch was unsuccessful",
            response.status()
        ))
    }
}
