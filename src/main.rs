mod errors;
mod model;
mod ports;
mod subdomain;

use anyhow::{Result, bail};
use futures::{StreamExt, stream};
use reqwest::Client;
use std::time::Duration;

use crate::errors::Error;
use crate::model::Subdomain;

const SUBDOMAIN_CONCURRENCY: usize = 128;
const PORTS_CONCURRENCY: usize = 256;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        bail!(Error::CliUsage);
    }

    let target = args[1].as_str();
    let http_client = Client::builder().timeout(Duration::from_secs(30)).build()?;

    // Declare a timer
    let scan_started = std::time::Instant::now();

    // Enumerate subdomains
    let subdomains = subdomain::enumerate(&http_client, target).await?;

    // Scan top 100 ports for each subdomain (concurrently)
    // - SUBDOMAIN_CONCURRENCY: Ｔhe number of subdomains to scan concurrently
    // - PORTS_CONCURRENCY: The number of ports to scan concurrently per subdomain
    let subdomains = stream::iter(subdomains.into_iter())
        .map(|subdomain| ports::scan_top_100_ports(PORTS_CONCURRENCY, subdomain))
        .buffer_unordered(SUBDOMAIN_CONCURRENCY)
        .collect::<Vec<Subdomain>>()
        .await;

    let scan_duration = scan_started.elapsed();

    println!(
        "Port scan completed in {} seconds",
        scan_duration.as_secs_f32()
    );

    println!("{}", "-".repeat(50));

    for subdomain in subdomains {
        println!("{}", &subdomain.domain);
        for port in subdomain.open_ports {
            println!("\t{}: opened", port.port);
        }
        println!();
    }

    Ok(())
}
