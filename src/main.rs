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

use fake::{Dummy, Fake, Faker};
// use fake::faker::name::en::*;
use fake::faker::company::en::*;
// use fake::faker::chrono::raw::*;
// use fake::uuid::*;
// use rand::rngs::StdRng;
// use rand::SeedableRng;

use std::{thread, time};
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

        // Create Entities for top tier in case there isn't any.
        let rand_num_asset: i32 = rng.gen_range(1..limit);
        println!("RANDOM Number: {:?}", rand_num_asset);
        for _asset in 0..rand_num_asset {
            let fake_dse = DataSourceEntity {
                tier_id: fetched_tiers.tiers[0].id,
                parent_id: None,
                name: Buzzword().fake(),
                note: CatchPhase().fake(),
                status: Faker.fake::<EntityStatus>(),
            };
            println!("TOP ----- TIER DATA: {:?}", fetched_tiers.tiers[0]);
            println!("TOP ----- DSE for tier - {:?}: {:?}", 0, fake_dse);
            // post entity
            post_entity(&opt.app_slugs[app], vec![fake_dse], token.clone()).await;
        }

        println!("POST DELAY 30 secs...\n\n");
        let post_delay = time::Duration::from_millis(30000);
        thread::sleep(post_delay);

        // Create Entities for each tiers after top tier.
        for tier in 1..fetched_tiers.tiers.len() {
            // println!("TIER {:?}", tier);
            println!(
                "{:?} ON TIER @@@@@@ {:?}",
                tier,
                fetched_tiers.tiers[tier - 1]
            );
            println!(
                "{:?} ON TIER ID @@@@@@ {:?}",
                tier,
                fetched_tiers.tiers[tier - 1].id
            );

            // Get entities in the tier before to set up as parents
            let entities = get_entities(
                &opt.app_slugs[app],
                token.clone(),
                fetched_tiers.tiers[tier - 1].id,
            )
            .await;
            println!(
                "TOTAL: {:?} \nFETCH Entities: {:?}",
                entities.assets.len(),
                entities
            );

            // For every entity this tier has, randomly generate more child entities.
            for entity in entities.assets {
                println!("\n\nEntity --------- {:?} ", entity.id);
                // Creating random number of entities for tier
                let rand_num_asset: i32 = rng.gen_range(1..limit);
                println!("\n\nRANDOM CHILD ASSET: {:?}", rand_num_asset);

                for rand_asset in 0..rand_num_asset {
                    println!("\n\nRand Asset #{:?}", rand_asset);
                    let fake_dse = DataSourceEntity {
                        tier_id: fetched_tiers.tiers[tier].id,
                        parent_id: Some(entity.id),
                        name: Buzzword().fake(),
                        note: CatchPhase().fake(),
                        status: Faker.fake::<EntityStatus>(),
                    };
                    println!("{:?} TIER DATA: {:?}", tier, fetched_tiers.tiers[tier].id);
                    println!("{:?} DSE for tier: {:?}", tier, fake_dse);

                    // post entity
                    post_entity(&opt.app_slugs[app], vec![fake_dse], token.clone()).await;
                }
            }
            // Need delay between each entity creation in a tier
            println!("POST DELAY 30 secs...\n\n");
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
    println!("TIER RESPONSE {:?}", json_response);
    json_response
}

pub async fn get_entities(app_slug: &str, token: String, tier_id: Uuid) -> EntitiesResult {
    println!("GET ENTITIES where TIER ID: {:?}", tier_id);
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

    println!("post entity status: {:?}", response.status());
    println!("post entity response: {:?}", response);
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
