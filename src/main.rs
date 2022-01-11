mod config;
use colored::Colorize;
use structopt::StructOpt;

pub mod command;
mod dsm;
mod frequency;
mod inspection_builder;
pub mod query_stringer;
mod running_schedule;
mod shift;
mod user;

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Opt {
    /// Limit scan to given applications. App name(s) that will receive injections.
    #[structopt(short, long = "app-slug", name = "slug")]
    app_slugs: Vec<String>,
    /// Number data source that will be injected. Number needs to be bigger than 1.
    #[structopt(short = "dsm-limit", long, name = "data sourse limit")]
    dsm_limit: Option<i32>,
    /// Number inspection builder items that will be injected. Number needs to be bigger than 1.
    #[structopt(short = "ib-limit", long, name = "inspection builder limit")]
    ib_limit: Option<i32>,
    /// Randomize running schedule. Set to true to use, default is false.
    #[structopt(short = "random-rs", long, name = "rsl")]
    random_rs: Option<bool>,
}

async fn process(opt: &Opt, token: String) {
    // Check command type
    // Set limit for dsm
    match opt.dsm_limit {
        Some(val) => {
            // TO DO: app slugs could be Empty. Need error handle
            if val > 1 {
                dsm::create_dsm(opt.dsm_limit.unwrap(), opt.app_slugs.clone(), token.clone()).await;
            } else {
                println!("{}", "Invalid dsm limit. Will default to 10".yellow());
                dsm::create_dsm(10, opt.app_slugs.clone(), token.clone()).await;
            }
        }
        None => {}
    };
    // Set limit for inspection builder
    match opt.ib_limit {
        Some(val) => {
            // TO DO: app slugs could be Empty. Need error handle
            if val > 1 {
                inspection_builder::create_inspection_builder(
                    opt.ib_limit.unwrap(),
                    opt.app_slugs.clone(),
                    token.clone(),
                )
                .await;
            } else {
                println!(
                    "{}",
                    "Invalid inspection builder limit. Will default to 10".yellow()
                );
                inspection_builder::create_inspection_builder(
                    10,
                    opt.app_slugs.clone(),
                    token.clone(),
                )
                .await;
            }
        }
        None => {}
    }
    // Scramble for running schedule.
    match opt.random_rs {
        Some(true) => {
            running_schedule::random_schedule(opt.app_slugs.clone(), token.clone()).await;
        }
        Some(false) => {
            println!("{}", "No changes made for running schedule.".yellow());
        }
        None => {}
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
