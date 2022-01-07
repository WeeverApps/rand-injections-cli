use crate::config::api::platform::connection_url as platform_url;
use eyre::eyre;
use eyre::Error;
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    url: String,
    name: String,
    pub uuid: Uuid,
    app_id: String,
    images: Option<serde_json::Value>,
    status: String,
    action: Option<serde_json::Value>,
    authors: String,
    details: Option<Details>,
    __weever: Option<Weever>,
    datetime: Option<Datetime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weever {
    synced: Option<bool>,
    __synced: Option<bool>,
    timestamps: Timestamps,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Timestamps {
    updated: Updated,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Updated {
    json: Option<String>,
    unix: i64,
    to_string: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Datetime {
    created: i64,
    updated: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Details {
    properties: WxConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WxConfig {
    wx_config: FormData,
    category_id: String,
    category_name: String,
    submission_count: Option<i32>,
    most_recent_submission: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    name: Option<String>,
    wx_type: Option<String>,
    required: Option<bool>,
    placeholder: Option<String>,
}
pub async fn fetch(app_slug: &str, token: String) -> Result<Vec<Form>, Error> {
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

    let url = format!(
        "{}/applications/{}/categories/forms/{}",
        platform_url(),
        app_slug,
        category_id
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();

    if response.status().is_success() {
        Ok(response.json::<Vec<Form>>().await.unwrap())
    } else {
        Err(eyre!(
            "ERROR - {}: Form fetch was unsuccessful",
            response.status()
        ))
    }
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
    if response.status().is_success() {
        response.json::<CategoriesResults>().await.unwrap()
    } else {
        CategoriesResults {
            message: Vec::new(),
        }
    }
}
