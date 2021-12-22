use reqwest::{blocking::Client, header::CONTENT_TYPE};
use serde_json::Value;

const MAGIC_DOCKER_GATEWAY_URL: &'static str = "172.17.0.1";

/// Logs into platform with the local dev account `admin@weeverdev.com`
/// which should exist and have access to all applications.
fn request_new_access_token(hostname: &str) -> Result<String, String> {
    let url = format!("http://{}:8413/oauth/token", hostname);
    let body_fields = vec![
        "client_id=25f4933e-975e-4c19-bd5d-13c5246eeca2",
        // ideally this should not be stored in source,
        // however it is already public knowledge at this point
        "client_secret=37e82810-8f25-4ef2-a4ab-0dc6fa3b84c2",
        "scope=write_forms%2Cwrite_submissions",
        "grant_type=password",
        "username=admin%40weeverdev.com",
        "password=a",
    ]
    .join("&");

    let response: Value = Client::new()
        .post(url.as_str())
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body_fields)
        .send()
        .map_err(|error| format!("failed to reach platform; {:#?}", error))?
        .json()
        .map_err(|error| format!("failed to parse response; {:#?}", error))?;

    response["accessToken"]
        .as_str()
        .map(|slice| String::from(slice))
        .ok_or_else(|| String::from("token-not-received"))
}

/// Collect a new login token at build time,
/// so that it can be included with the binary of the application
fn get_access_token() -> Option<String> {
    // return already defined token if exists
    if let Ok(token) = std::env::var("BEARER_TOKEN") {
        return Some(token);
    } else if let Some(token) = option_env!("BEARER_TOKEN") {
        return Some(String::from(token));
    }

    match request_new_access_token("localhost") {
        Ok(token) => Some(token),
        Err(_local_error) => match request_new_access_token(MAGIC_DOCKER_GATEWAY_URL) {
            Ok(token) => Some(token),
            Err(_docker_error) => {
                println!("cargo:warning=Unable to find valid bearer token for API access.");
                println!("cargo:warning=Provide BEARER_TOKEN or ensure Platform API is running.");
                println!("cargo:warning=Communication with OData API will fail.");
                println!("cargo:warning=Local error: {:?}", _local_error);
                println!("cargo:warning=Docker error: {:?}", _docker_error);
                None
            }
        },
    }
}

fn main() {
    // fetch a fresh login token and save it to the environment
    if let Some(token) = get_access_token() {
        println!("cargo:rustc-env=BEARER_TOKEN={token}", token = &token);
        println!("cargo:warning=Setting BEARER_TOKEN={token}", token = &token);
    }
}
