use anyhow::Result;
use futures::{StreamExt, stream};
use hickory_resolver::TokioAsyncResolver;
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use serde::Deserialize;
use std::collections::HashSet;

use crate::model::Subdomain;

pub async fn enumerate(http_client: &reqwest::Client, domain: &str) -> Result<Vec<Subdomain>> {
    // Declare needed API response fields
    #[derive(Debug, Deserialize)]
    struct CrtShEntry {
        name_value: String,
    }

    // Get CT log entries
    let entries: Vec<CrtShEntry> = http_client
        .get(format!("https://crt.sh/?q=%25.{}&output=json", domain))
        .send()
        .await?
        .json()
        .await?;

    // Get subdomains by parsing CT log entries
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .name_value
                .split("\n")
                .map(|subdomain| subdomain.trim().to_lowercase())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain| !subdomain.contains("*")) // Remove wildcard subdomains
        .collect();

    // Insert root domain `domain` into `subdomains` set
    subdomains.insert(domain.to_string());

    // Declare a DNS resolver for dependency injection
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    // Enumerate subdomains
    let subdomains: Vec<Subdomain> = stream::iter(subdomains.into_iter())
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        .filter_map(|subdomain| async {
            if is_resolvable(&resolver, &subdomain.domain).await {
                Some(subdomain)
            } else {
                None
            }
        })
        .collect()
        .await;

    Ok(subdomains)
}

async fn is_resolvable(resolver: &TokioAsyncResolver, domain: &str) -> bool {
    resolver.lookup_ip(domain).await.is_ok()
}
