use crate::config::api::inspections_v2::connection_url as inspections_v2_url;
use colored::Colorize;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;

pub async fn service<T>(token: String, app_slug: String, command_type: String, entities: T)
where
    T: IntoIterator,
    <T as IntoIterator>::Item: Serialize,
{
    let url = format!(
        "{hostname}/{app_slug}/command",
        hostname = inspections_v2_url(),
        app_slug = app_slug,
    );

    let commands = entities
        .into_iter()
        .map(|entity| json!({ command_type.as_str(): entity }))
        .collect::<Vec<_>>();

    let response = Client::new()
        .post(&url)
        .bearer_auth(token)
        .json(&json!({ "commands": commands }))
        .send()
        .await
        .unwrap();
    if !response.status().is_success() {
        println!("{}", "ERROR: Failed to create schedules".red());
    }
}
