mod config;
use chrono::{offset::Utc, DateTime};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use structopt::StructOpt;
use uuid_5::Uuid;

extern crate rand;
use rand::thread_rng;
use rand::Rng;

use fake::faker::company::en::*;
use fake::{Dummy, Fake, Faker};

use std::{thread, time};
#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Opt {
    /// Limit scan to given applications. App name(s) that will receive injections.
    #[structopt(short, long = "app-slug", name = "slug")]
    app_slugs: Vec<String>,
    /// Number data source that will be injected. Number needs to be bigger than 1.
    #[structopt(short = "l", long, name = "limit")]
    dsm_limit: Option<i32>,
}

#[derive(Serialize, Debug)]
pub struct DataSourceEntity {
    tier_id: Uuid,
    parent_id: Option<Uuid>,
    name: String,
    note: Option<String>,
    status: EntityStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Dummy)]
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
pub struct TierRecord {
    app_slug: String,
    parent_id: Option<Uuid>,
    id: Uuid,
    name: String,
    note: Option<String>,
    status: TierStatus,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TierStatus {
    Published,
    Disabled,
}

impl TierStatus {
    pub fn from(text: &String) -> Self {
        match text.as_str() {
            "published" => Self::Published,
            // currently allowing an invalid default to Disabled
            _ => Self::Disabled,
        }
    }
}

async fn process(opt: &Opt, token: String) {
    // Set limit for dsm
    let limit = match opt.dsm_limit {
        Some(val) => {
            if val > 1 {
                opt.dsm_limit.unwrap()
            } else {
                println!("{}", "Invalid dsm limit. Will default to 10".yellow());
                10
            }
        }
        _ => 10,
    };
    // Random number generator
    let mut rng = thread_rng();

    // Create dsm for each app
    for app in 0..opt.app_slugs.len() {
        println!("APP: {:?}", &opt.app_slugs[app]);
        // Get number of tiers
        let fetched_tiers = tiers(&opt.app_slugs[app], token.clone()).await;

        // Create Entities for top tier in case there isn't any.
        let mut rand_num_asset: i32 = rng.gen_range(1..limit);
        for _asset in 0..rand_num_asset {
            let fake_dse = DataSourceEntity {
                tier_id: fetched_tiers.tiers[0].id,
                parent_id: None,
                name: Buzzword().fake(),
                note: CatchPhase().fake(),
                status: Faker.fake::<EntityStatus>(),
            };
            // post entity
            post_entity(&opt.app_slugs[app], vec![fake_dse], token.clone()).await;
        }

        println!(
            "CREATING {:?} entities for top tier...(30 secs)",
            rand_num_asset
        );
        let post_delay = time::Duration::from_millis(30000);
        thread::sleep(post_delay);

        // Create Entities for each tiers after top tier.
        for tier in 1..fetched_tiers.tiers.len() {
            // Get entities in the tier before to set up as parents
            let entities = get_entities(
                &opt.app_slugs[app],
                token.clone(),
                fetched_tiers.tiers[tier - 1].id,
            )
            .await;

            // For every entity this tier has, randomly generate more child entities.
            for entity in entities.assets {
                // Creating random number of entities for tier
                rand_num_asset = rng.gen_range(1..limit);

                for _rand_asset in 0..rand_num_asset {
                    let fake_dse = DataSourceEntity {
                        tier_id: fetched_tiers.tiers[tier].id,
                        parent_id: Some(entity.id),
                        name: Buzzword().fake(),
                        note: CatchPhase().fake(),
                        status: Faker.fake::<EntityStatus>(),
                    };
                    // post entity
                    post_entity(&opt.app_slugs[app], vec![fake_dse], token.clone()).await;
                }
            }
            // Need delay between each entity creation in a tier
            println!(
                "\nCREATING {:?} entities for tier {:?}...(30 secs)",
                rand_num_asset, tier
            );
            let post_delay = time::Duration::from_millis(30000);
            thread::sleep(post_delay);
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TiersResult {
    #[serde(alias = "records")]
    pub tiers: Vec<TierRecord>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntitiesResult {
    #[serde(alias = "records")]
    pub assets: Vec<EntityRecord>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityRecord {
    app_slug: String,
    tier_id: Uuid,
    parent_id: Option<Uuid>,
    id: Uuid,
    name: String,
    note: Option<String>,
    status: EntityStatus,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

pub async fn tiers(app_slug: &str, token: String) -> TiersResult {
    let hostname = config::api::dsm::connection_url();
    let url = format!("{}/v1/{}/tiers", hostname, app_slug);
    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();

    let json_response;
    if response.status().is_success() {
        json_response = response.json::<TiersResult>().await.unwrap();
    } else {
        json_response = TiersResult { tiers: Vec::new() };
    }
    json_response
}

pub async fn get_entities(app_slug: &str, token: String, tier_id: Uuid) -> EntitiesResult {
    let hostname = config::api::dsm::connection_url();
    let url = format!("{}/v1/{}/assets?tier_id={}", hostname, app_slug, tier_id);
    let client = reqwest::Client::new();
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();

    let json_response;
    if response.status().is_success() {
        json_response = response.json::<EntitiesResult>().await.unwrap();
    } else {
        json_response = EntitiesResult { assets: Vec::new() };
    }
    json_response
}

pub async fn post_entity(app_slug: &str, fake_dse: Vec<DataSourceEntity>, token: String) {
    let hostname = config::api::dsm::connection_url();
    let url = format!("{}/v1/{}/assets", hostname, app_slug);

    let client = reqwest::Client::new();

    let response = client
        .post(&url)
        .bearer_auth(token)
        .json(&fake_dse)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        println!("{:?}", "ERROR: Issue with post entity".red());
    }
}

fn get_token() -> String {
    match std::env::var("BEARER_TOKEN") {
        Ok(val) => val,
        Err(_) => "".to_string(),
    }
}

#[tokio::main]
async fn main() {
    let token = get_token();
    let opt = Opt::from_args();
    process(&opt, token).await;

    println!("{}", "Finished!".green());
}
