use structopt::StructOpt;
use colored::Colorize;

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Opt {
    /// Limit scan to given applications
    #[structopt(short, long = "app-slug", name = "slug", about = "App name that will receive injections")]
    app_slugs: Vec<String>,
    /// Maximum count of forms to process
    #[structopt(short = "l", long, name = "count", about = "Number data source that will be injected")]
    dsm_limit: Option<usize>,
}

async fn process(opt: &Opt){
    println!("APP STATED: {:?}",&opt.app_slugs);
    println!("DSM STATED: {:?}",&opt.dsm_limit);
}

#[tokio::main]
async fn main() {
   let opt = Opt::from_args();
   process(&opt).await;

   println!("{}", "Finished!".green());
}
