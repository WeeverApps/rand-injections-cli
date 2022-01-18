use crate::command::InspectionsCommandRequest;
use crate::config::api::inspections_v2::connection_url as inspections_v2_url;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct InspectionsEventTransaction {
    pub stream_id: Uuid,
    pub transaction_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CommandResponse {
    status: String,
    receipts: Vec<InspectionsEventTransaction>,
}

pub async fn post(app_slug: &str, token: String, command: InspectionsCommandRequest) {
    let hostname = inspections_v2_url();

    let url = format!("{}/{}/command", hostname, app_slug);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(token)
        .json(&command)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        println!("Command Randomized {} item(s).", command.commands.len());
        println!("{}", "POST Command was successful!".green());
    } else {
        let error: &str = &format!(
            "ERROR - {}: Command post was unsuccessful.",
            response.status()
        );
        println!("{}", error.red());
    }
}
