#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sagemaker::{config::Region, meta::PKG_VERSION, Client, Error};
use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {
    
    #[structopt(short, long)]
    region: Option<String>,

    #[structopt(short, long)]
    verbose: bool,
}

// lists notebooks in the account
// snippet-start:[sagemaker.rust.sagemakercount]
async fn show_instances(client: &Client) -> Result<(), Error> {
    let notebooks = client.list_notebook_instances().send().await?;

    println!("Notebooks:");

    for n in notebooks.notebook_instances().unwrap_or_default() {
        let n_instance_type = n.instance_type().unwrap();
        let n_status = n.notebook_instance_status().unwrap();
        let n_name = n.notebook_instance_name().unwrap_or_default();
    


        println!("  Name :    {}", n_name);
        println!("  Status :  {}", n_status.as_ref());
        println!("  Instance Type: {}", n_instance_type.as_ref());
        println!(); // 

    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt { region, verbose } = Opt::parse();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    println!();

    if verbose {
        println!("SageMaker client version: {}", PKG_VERSION);
        println!(
            "Region:                   {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    show_instances(&client).await
}