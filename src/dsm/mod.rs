pub mod entity;
pub mod tier;

use colored::Colorize;
extern crate rand;
use rand::thread_rng;
use rand::Rng;

use fake::faker::company::en::*;
use fake::{Fake, Faker};

use std::{thread, time};

pub async fn create_dsm(limit: i32, app_slugs: Vec<String>, token: String) {
    // Random number generator
    let mut rng = thread_rng();
    let post_delay = time::Duration::from_millis(30000);
    // Create dsm for each app
    for app in app_slugs {
        println!("APP: {:?}", app);
        // Get number of tiers
        let fetched_tiers = tier::tiers(&app, token.clone()).await;
        if fetched_tiers.tiers.len() <= 0 {
            println!("{}", "ERROR: There isn't any tiers for this app.".red());
            break;
        }
        // Create Entities for top tier in case there isn't any.
        let mut rand_num_asset: i32 = rng.gen_range(1..limit);
        for _asset in 0..rand_num_asset {
            let fake_dse = entity::DataSourceEntity {
                tier_id: fetched_tiers.tiers[0].id,
                parent_id: None,
                name: Buzzword().fake(),
                note: CatchPhase().fake(),
                status: Faker.fake::<entity::EntityStatus>(),
            };
            // post entity
            entity::post_entity(&app, vec![fake_dse], token.clone()).await;
        }

        println!(
            "CREATING {:?} entities for top tier...(30 secs)",
            rand_num_asset
        );
        // We need to wait a moment for the post to process since we're fetch these creations right after.
        thread::sleep(post_delay);

        // Create Entities for each tiers after top tier.
        for tier in 1..fetched_tiers.tiers.len() {
            // Get entities in the tier before to set up as parents
            let mut entities =
                entity::get_entities(&app, token.clone(), fetched_tiers.tiers[tier - 1].id).await;
            if entities.assets.len() <= 0 {
                println!("Couldn't find any entities.");
                // Need delay between each entity creation in a tier
                println!("Another POST delay...(30 secs)\n");
                // We need to wait a moment for the post to process since we're fetch these creations right after.
                thread::sleep(post_delay);
                // Get entities in the tier before to set up as parents
                entities =
                    entity::get_entities(&app, token.clone(), fetched_tiers.tiers[tier - 1].id)
                        .await;
            }
            let mut fake_dse = Vec::new();
            // For every entity this tier has, randomly generate more child entities.
            for entity in entities.assets {
                // Creating random number of entities for tier
                rand_num_asset = rng.gen_range(1..limit);
                println!(
                    "CREATING {:?} entities for tier {:?}...",
                    rand_num_asset, tier
                );
                for _rand_asset in 0..rand_num_asset {
                    fake_dse.push(entity::DataSourceEntity {
                        tier_id: fetched_tiers.tiers[tier].id,
                        parent_id: Some(entity.id),
                        name: Buzzword().fake(),
                        note: CatchPhase().fake(),
                        status: Faker.fake::<entity::EntityStatus>(),
                    });
                }
            }
            // post entities as an array - Should cause asset_paths -> meta_happened_at to have same times.
            entity::post_entity(&app, fake_dse, token.clone()).await;
            // Need delay between each entity creation in a tier
            println!("POST delay...(30 secs)\n");
            // We need to wait a moment for the post to process since we're fetch these creations right after.
            thread::sleep(post_delay);
        }
    }
}
