use crate::config::api::platform::connection_url as platform_url;
use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use fake::faker::company::en::*;
use fake::faker::name::en::*;
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    url: String,
    name: String,
    uuid: String,
    app_id: String,
    images: Option<serde_json::Value>,
    status: String,
    action: Option<serde_json::Value>,
    authors: String,
    details: Option<Details>,
    __weever: Option<Weever>,
    datetime: Option<Datetime>,
}

#[derive(Deserialize, Serialize)]
pub struct Weever {
    synced: Option<bool>,
    __synced: Option<bool>,
    timestamps: Timestamps,
}

#[derive(Deserialize, Serialize)]
pub struct Timestamps {
    updated: Updated,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Updated {
    json: Option<String>,
    unix: i64,
    to_string: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Datetime {
    created: i64,
    updated: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Details {
    properties: WxConfig,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WxConfig {
    wx_config: FormData,
    category_id: String,
    category_name: String,
    submission_count: i32,
    most_recent_submission: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormData {
    title: String,
    pdf_header: Option<serde_json::Value>,
    upload_url: Option<String>,
    on_upload: Message,
    form_elements: Vec<FormElements>,
    comment_on_close: Option<bool>,
    assignee_sidebar_user_types: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    message: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormElements {
    uuid: String,
    label: Option<String>,
    control: String,
    title: Option<serde_json::Value>,
    attributes: Attributes,
    factory_name: Option<String>,
    builder_title: Option<String>,
    can_edit_label: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    name: Option<String>,
    wx_type: Option<String>,
    required: Option<bool>,
    placeholder: Option<String>,
}
pub async fn fetch(app_slug: &str, token: String) -> Form {
    let categories = form_categories(app_slug, token.clone()).await;
    let category_id: String = categories
        .message
        .into_iter()
        .filter_map(|value| {
            if value.name == "Inspections" {
                Some(value.id)
            } else {
                None
            }
        })
        .collect();
    println!("CATEGORY ID: {:?}", category_id);

    let url = format!(
        "{}/applications/{}/categories/forms/{}",
        platform_url(),
        app_slug,
        category_id
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();

    let mut json_response = Form {
        url: "TEST URL".to_string(),
        name: "TEST NAME".to_string(),
        uuid: "TEST UUID".to_string(),
        app_id: "TEST APP ID".to_string(),
        images: None,
        status: "TEST STATUS".to_string(),
        action: None,
        authors: Name().fake(),
        details: None,
        __weever: None,
        datetime: None,
    };

    if response.status().is_success() {
        json_response = response.json::<Form>().await.unwrap();
    }
    json_response
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoriesResults {
    message: Vec<FormCategory>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormCategory {
    id: String,
    name: String,
    status: String,
    url: String,
}

pub async fn form_categories(app_slug: &str, token: String) -> CategoriesResults {
    let url = format!(
        "{}/applications/{}/categories/forms",
        platform_url(),
        app_slug
    );
    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();

    let json_response;
    if response.status().is_success() {
        json_response = response.json::<CategoriesResults>().await.unwrap();
    } else {
        json_response = CategoriesResults {
            message: Vec::new(),
        };
    }
    println!("FORM CATEGORIES: {:?}", json_response);
    json_response
}
