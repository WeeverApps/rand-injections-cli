mod config;
use colored::Colorize;
use structopt::StructOpt;

extern crate rand;
// use rand::thread_rng;
// use rand::Rng;

// use fake::faker::company::en::*;
// use fake::{Fake, Faker};

// use std::{thread, time};

mod dsm;
#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Opt {
    /// Limit scan to given applications. App name(s) that will receive injections.
    #[structopt(short, long = "app-slug", name = "slug")]
    app_slugs: Vec<String>,
    /// Number data source that will be injected. Number needs to be bigger than 1.
    #[structopt(short = "dsm-limit", long, name = "dl")]
    dsm_limit: Option<i32>,
    /// Number inspection builder items that will be injected. Number needs to be bigger than 1.
    #[structopt(short = "ib-limit", long, name = "ibl")]
    ib_limit: Option<i32>,
}

async fn process(opt: &Opt, token: String) {
    // Check command type
    println!("IB #: {:?}", opt.ib_limit);
    // Set limit for dsm
    match opt.dsm_limit {
        Some(val) => {
            // TO DO: app slugs could be Empty. Need error handle
            if val > 1 {
                dsm::create_dsm(opt.dsm_limit.unwrap(), opt.app_slugs.clone(), token).await;
            } else {
                println!("{}", "Invalid dsm limit. Will default to 10".yellow());
                dsm::create_dsm(10, opt.app_slugs.clone(), token).await;
            }
        }
        None => {}
    };
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
