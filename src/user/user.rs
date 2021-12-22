use crate::config::api::platform::connection_url as platform_url;

use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    total: i64,
    pub users: Vec<User>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: Uuid,
    email: String,
    first_name: String,
    last_name: String,
    status: String,
}

pub async fn fetch(app_slug: &str, token: String) -> Response {
    let url = format!("{}/applications/{}/users", platform_url(), app_slug);

    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();
    let json_response;
    if response.status().is_success() {
        json_response = response.json::<Response>().await.unwrap();
    } else {
        json_response = Response {
            users: Vec::new(),
            total: 0,
        };
    }
    json_response
}
