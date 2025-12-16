mod errors;
mod model;
mod ports;
mod subdomain;

use anyhow::{Result, bail};
use futures::{StreamExt, stream};
use reqwest::Client;
use std::env;
use std::time::Duration;
use std::time::Instant;

use crate::errors::Error;
use crate::model::Subdomain;

const SUBDOMAIN_CONCURRENCY: usize = 128;
const PORTS_CONCURRENCY: usize = 256;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Parse arguments & Input validation
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        bail!(Error::CliUsage);
    }

    let target = args[1].as_str().to_lowercase();

    // Declare HTTP client and start scan timer
    let http_client = Client::builder().timeout(Duration::from_secs(30)).build()?;
    let scan_started = Instant::now();

    // Enumerate subdomains
    let subdomains = subdomain::enumerate(&http_client, &target).await?;

    // Scan top 100 ports for each subdomain (concurrently)
    // - SUBDOMAIN_CONCURRENCY: Ｔhe number of subdomains to scan concurrently
    // - PORTS_CONCURRENCY: The number of ports to scan concurrently per subdomain
    // e.g. Maximum number of concurrent scans is `SUBDOMAIN_CONCURRENCY * PORTS_CONCURRENCY`
    let subdomains = stream::iter(subdomains.into_iter())
        .map(|subdomain| ports::scan_top_100_ports(PORTS_CONCURRENCY, subdomain))
        .buffer_unordered(SUBDOMAIN_CONCURRENCY)
        .collect::<Vec<Subdomain>>()
        .await;

    // Stop scan timer
    let scan_duration = scan_started.elapsed();

    println!(
        "Port scan completed in {} seconds",
        scan_duration.as_secs_f32()
    );

    println!("{}", "-".repeat(50));

    // Print scan results
    for subdomain in subdomains {
        println!("{}", subdomain.domain);
        for port in subdomain.open_ports {
            println!("\t{}: opened", port);
        }
        println!();
    }

    Ok(())
}
