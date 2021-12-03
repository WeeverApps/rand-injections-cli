mod config;
use colored::Colorize;
use structopt::StructOpt;

extern crate rand;
use rand::thread_rng;
use rand::Rng;

use fake::faker::company::en::*;
use fake::{Fake, Faker};

use std::{thread, time};

mod dsm;
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
        let fetched_tiers = dsm::tier::tiers(&opt.app_slugs[app], token.clone()).await;
        if fetched_tiers.tiers.len() <= 0 {
            println!("{}", "ERROR: There isn't any tiers for this app.".red());
            break;
        }
        // Create Entities for top tier in case there isn't any.
        let mut rand_num_asset: i32 = rng.gen_range(1..limit);
        for _asset in 0..rand_num_asset {
            let fake_dse = dsm::entity::DataSourceEntity {
                tier_id: fetched_tiers.tiers[0].id,
                parent_id: None,
                name: Buzzword().fake(),
                note: CatchPhase().fake(),
                status: Faker.fake::<dsm::entity::EntityStatus>(),
            };
            // post entity
            dsm::entity::post_entity(&opt.app_slugs[app], vec![fake_dse], token.clone()).await;
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
            let entities = dsm::entity::get_entities(
                &opt.app_slugs[app],
                token.clone(),
                fetched_tiers.tiers[tier - 1].id,
            )
            .await;

            // For every entity this tier has, randomly generate more child entities.
            for entity in entities.assets {
                // Creating random number of entities for tier
                rand_num_asset = rng.gen_range(1..limit);
                println!(
                    "CREATING {:?} entities for tier {:?}...",
                    rand_num_asset, tier
                );
                for _rand_asset in 0..rand_num_asset {
                    let fake_dse = dsm::entity::DataSourceEntity {
                        tier_id: fetched_tiers.tiers[tier].id,
                        parent_id: Some(entity.id),
                        name: Buzzword().fake(),
                        note: CatchPhase().fake(),
                        status: Faker.fake::<dsm::entity::EntityStatus>(),
                    };
                    // post entity
                    dsm::entity::post_entity(&opt.app_slugs[app], vec![fake_dse], token.clone())
                        .await;
                }
            }
            // Need delay between each entity creation in a tier
            println!("\nPOST delay...(30 secs)");
            let post_delay = time::Duration::from_millis(30000);
            thread::sleep(post_delay);
        }
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
