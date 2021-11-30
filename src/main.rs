// use crate::config::api::dsm::connection_url;
mod config;
use chrono::{offset::Utc, DateTime};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use structopt::StructOpt;
// use uuid_5::Uuid;
use uuid_5::Uuid;

extern crate rand;
use rand::thread_rng;
use rand::Rng;

use fake::{Dummy, Fake, Faker};
// use fake::faker::name::en::*;
use fake::faker::company::en::*;
// use fake::faker::chrono::raw::*;
// use fake::uuid::*;
// use rand::rngs::StdRng;
// use rand::SeedableRng;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::{json, Value as JsonValue};
#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Opt {
    /// Limit scan to given applications. App name(s) that will receive injections
    #[structopt(short, long = "app-slug", name = "slug")]
    app_slugs: Vec<String>,
    /// Number data source that will be injected
    #[structopt(short = "l", long, name = "limit")]
    dsm_limit: Option<i32>,
}

#[derive(Serialize, Debug)]
pub struct DataSourceEntity {
    tier_id: Uuid,
    parent_id: Option<Uuid>,
    // #[dummy(faker = "Buzzword()")]
    name: String,
    // #[dummy(faker = "CatchPhase()")]
    notes: Option<String>,
    status: EntityStatus,
}

#[derive(Serialize, Debug, Dummy)]
#[serde(rename_all = "lowercase")]
pub enum EntityStatus {
    Published,
    Disabled,
}

impl FromStr for EntityStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "published" => Ok(EntityStatus::Published),
            "disabled" => Ok(EntityStatus::Disabled),
            _ => Err("Invalid status value"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetRecord {
    app_slug: String,
    parent_id: Option<Uuid>,
    id: Uuid,
    name: String,
    note: Option<String>,
    status: AssetStatus,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetStatus {
    Published,
    Disabled,
}

impl AssetStatus {
    pub fn from(text: &String) -> Self {
        match text.as_str() {
            "published" => Self::Published,
            // currently allowing an invalid default to Disabled
            _ => Self::Disabled,
        }
    }
}

async fn process(opt: &Opt, token: String) {
    println!("APP STATED: {:?}", &opt.app_slugs);
    println!("DSM STATED: {:?}", &opt.dsm_limit);
    println!("number of appslugs: {:?}", &opt.app_slugs.len());
    // Set limit for dsm
    let limit;
    match opt.dsm_limit {
        Some(val) => {
            if val > 1 {
                limit = opt.dsm_limit.unwrap();
            } else {
                println!("{}", "Invalid dsm limit. Will default to 10".yellow());
                // need refactor
                limit = 10;
            }
        }
        _ => limit = 10,
    };
    // Random number generator
    let mut rng = thread_rng();

    // Create dsm for each app
    for app in 0..opt.app_slugs.len() {
        println!("APP: {:?}", &opt.app_slugs[app]);
        // Get number of tiers
        let fetched_tiers = tiers(&opt.app_slugs[app], token.clone()).await;
        println!("number of tiers: {:?}", fetched_tiers.tiers.len());
        // Create Entities for each tiers
        for tier in 0..fetched_tiers.tiers.len() {
            let rand_num_asset: i32 = rng.gen_range(1..limit);
            println!("RANDOM Number: {:?}", rand_num_asset);
            // Creating random number of entities for tier
            for asset in 0..rand_num_asset {
                println!("Asset #{:?}", asset);
                let fake_dse: DataSourceEntity = DataSourceEntity {
                    tier_id: fetched_tiers.tiers[tier].id,
                    parent_id: fetched_tiers.tiers[tier].parent_id,
                    name: Buzzword().fake(),
                    notes: CatchPhase().fake(),
                    status: Faker.fake::<EntityStatus>(),
                };
                println!("TIER DATA: {:?}", fetched_tiers.tiers[tier]);
                println!("DSE for tier - {:?}: {:?}", tier, fake_dse);

                // post entity
                post_entity(&opt.app_slugs[app], fake_dse, token.clone()).await;
            }
        }
    }
    // Connect to app
    // GET num of levels this app has for dsm
    // "/<app_slug>/assets?<asset_id>&<parent_asset_id>&<tier_id>&<top_level_assets>"
    // loop the num of levels
    // POST:
    // set random number of assets for this level
    /*let mut rng = thread_rng();
    let y: f64 = rng.gen_range(-10.0, 10.0);
    let rand_num_asset: i32 = rng.gen_range(1, 10);
    println!("RANDOM Number: {:?}", rand_num_asset);
    */
    // create random data
    // make request.
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FetchResult {
    #[serde(alias = "records")]
    pub tiers: Vec<AssetRecord>,
}

pub async fn tiers(app_slug: &str, token: String) -> FetchResult {
    let hostname = config::api::dsm::connection_url();
    let url = format!("{}/v1/{}/tiers?", hostname, app_slug);
    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();

    let json_response;
    if response.status().is_success() {
        json_response = response.json::<FetchResult>().await.unwrap();
    } else {
        json_response = FetchResult { tiers: Vec::new() };
    }
    json_response
}

pub async fn post_entity(app_slug: &str, fake_dse: DataSourceEntity, token: String) {
    println!("TOKEN {:?}", token);
    let hostname = config::api::dsm::connection_url();
    let url = format!("{}/v1/{}/assets", hostname, app_slug);
    println!("URL: {:?}", url);
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(token)
        .json(&fake_dse)
        .send()
        .await
        .unwrap();

    println!("post entity status: {:?}", response.status());
}

fn get_token() -> String {
    let token;
    match std::env::var("BEARER_TOKEN") {
        Ok(val) => token = val,
        Err(_e) => token = "".to_string(),
    }
    token
}

#[tokio::main]
async fn main() {
    let token = get_token();
    let opt = Opt::from_args();
    process(&opt, token).await;

    println!("{}", "Finished!".green());
}
