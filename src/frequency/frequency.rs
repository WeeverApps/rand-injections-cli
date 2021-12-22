use super::super::inspection_builder::ScaleOfDateRange;
use crate::config::api::inspections_v2::connection_url as inspections_v2_url;
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Frequency {
    pub frequency: Vec<FrequencyDetail>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"), deny_unknown_fields)]
pub struct FrequencyDetail {
    pub id: Uuid,
    pub frequency_count: i64,
    pub frequency_unit: ScaleOfDateRange,
    frequency_text: String,
    frequency_value: String,
}

pub async fn fetch(app_slug: &str, token: String) -> Frequency {
    let url = format!("{}/{}/frequency", inspections_v2_url(), app_slug);

    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();
    let json_response;
    if response.status().is_success() {
        json_response = response.json::<Frequency>().await.unwrap();
    } else {
        json_response = Frequency {
            frequency: Vec::new(),
        };
    }
    json_response
}
