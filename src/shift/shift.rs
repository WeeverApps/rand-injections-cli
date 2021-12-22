use super::Shift;
use crate::config::api::inspections_v2::connection_url as inspections_v2_url;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FetchResult {
    #[serde(alias = "records")]
    pub shifts: Vec<Shift>,
    pub total: u32,
}

pub async fn fetch(app_slug: &str, token: String) -> FetchResult {
    let url = format!(
        "{hostname}/{app_slug}/query/shifts",
        hostname = inspections_v2_url(),
        app_slug = app_slug,
    );

    let response = Client::new()
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .unwrap();

    let json_response;
    if response.status().is_success() {
        json_response = response.json::<FetchResult>().await.unwrap();
    } else {
        json_response = FetchResult {
            shifts: Vec::new(),
            total: 0,
        };
    }
    json_response
}
