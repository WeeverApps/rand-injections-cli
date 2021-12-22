use colored::Colorize;
extern crate rand;
use rand::thread_rng;
use rand::Rng;
pub mod downtime;

pub async fn random_schedule(limit: i32, app_slugs: Vec<String>, token: String) {
    // Random number generator
    let mut rng = thread_rng();

    // Scramble running schedule
    for app in 0..app_slugs.len() {
        // let mut rand_num;
        // Get downtime
        let fetched_downtime = downtime::fetch(&app_slugs[app], token.clone()).await;
        println!("DOWNTIME: {:?}", fetched_downtime);
        // randomize cancellation and scheduled downtime.
    }
}
