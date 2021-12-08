use crate::config::api::inspections_v2::connection_url as inspections_v2_url;
// use crate::logger::{debug, error, log_fields};
// use actix_web::{HttpResponse, Responder};
use reqwest::Client;
use serde::Serialize;
use serde_json::{json, Value as JsonValue};

pub async fn service<T>(token: String, app_slug: Option<&str>, command_type: String, entities: T)
where
    T: IntoIterator,
    <T as IntoIterator>::Item: Serialize,
{
    let app_slug = match app_slug {
        Some(app_slug) => app_slug.trim(),
        _ => {
            error!(
                logger, "Failed to parse app_slug";
                log_fields!( "app_slug" => format!("{:?}", app_slug) )
            );
            return HttpResponse::InternalServerError().finish();
        }
    };

    let url = format!(
        "{hostname}/{app_slug}/command",
        hostname = inspections_v2_url(),
        app_slug = app_slug,
    );

    let logger = logger.new(log_fields!(
        "app_slug" => String::from(app_slug),
        "bearer_token" => token.clone(),
        "command_type" => command_type.clone(),
        "forward_request_url" => url.clone(),
    ));

    let commands = entities
        .into_iter()
        .map(|entity| json!({ command_type.as_str(): entity }))
        .collect::<Vec<_>>();

    let response = Client::new()
        .post(&url)
        .bearer_auth(token)
        .json(&json!({ "commands": commands }))
        .send()
        .await;

    match response {
        // the request to the central server was successful
        Ok(response) if response.status().is_success() => {
            let status = response.status();
            let json_response = response.json::<JsonValue>().await;

            debug!(
                logger, "Request to central api server successful";
                log_fields!(
                    "status" => status.as_u16(),
                    "response" => format!("{:?}", json_response),
                )
            );

            HttpResponse::build(status).finish()
        }
        // the request failed to reach the central server
        Err(error) => {
            error!(
                logger, "Failed to reach central api server";
                log_fields!( "error" => format!("{:?}", error) )
            );

            HttpResponse::InternalServerError().finish()
        }
    }
}
