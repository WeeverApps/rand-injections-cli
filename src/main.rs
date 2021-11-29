// use crate::config::api::dsm::connection_url;
mod config;
use structopt::StructOpt;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use chrono::{offset::Utc, DateTime};
// use uuid_5::Uuid;
use uuid_5::Uuid;

extern crate rand;
// use rand::thread_rng;
// use rand::Rng;

use fake::{Dummy, Fake, Faker};
// use fake::faker::name::en::*;
use fake::faker::company::en::*;
// use fake::faker::chrono::raw::*;
// use fake::uuid::*;
// use rand::rngs::StdRng;
// use rand::SeedableRng;

// use rocket::request::Request;
#[macro_use] extern crate rocket;
use rocket::{get, routes, Build};
use rocket::local::asynchronous::Client;
// use rocket::response::{self, Response, Responder};

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Opt {
    /// Limit scan to given applications. App name(s) that will receive injections
    #[structopt(short, long = "app-slug", name = "slug")]
    app_slugs: Vec<String>,
    /// Number data source that will be injected
    #[structopt(short = "l", long, name = "limit")]
    dsm_limit: Option<usize>,
}

#[derive(Debug, Dummy)]
pub struct DataSourceEntity {
    id: String,
    tier: Option<String>,
    parent: Option<String>,
    #[dummy(faker = "Buzzword()")]
    name: String,
    #[dummy(faker = "CatchPhase()")]
    notes: Option<String>,
    status: EntityStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AssetRecord {
    app_slug: String,
    tier_id: Uuid,
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

async fn process(opt: &Opt){
    println!("APP STATED: {:?}",&opt.app_slugs);
    println!("DSM STATED: {:?}",&opt.dsm_limit);
    let fake_Dse: DataSourceEntity = Faker.fake();
    println!("DSE: {:?}", fake_Dse);
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

#[get("/tiers/<app_slug>")]
pub async fn tiers(app_slug: &str) -> (){
    // let app_slug = "mars-clv";
    let hostname = config::api::dsm::connection_url();
    let url = format!("{}/v1/{}/tiers?",
    hostname, app_slug);
    let token = std::env::var("BEARER_TOKEN");
    
    let client = Client::tracked(rocket()).await.expect("valid rocket");
    let response = client.get(&url).dispatch();
    println!("STATUS: {:?}", response.await.status());
}

fn rocket() -> rocket::Rocket<Build> {
    rocket::build().mount("/", routes![tiers])
}

#[rocket::main]
async fn main() {
    rocket().launch().await;
    let opt = Opt::from_args();
    process(&opt).await;

    println!("{}", "Finished!".green());

}
