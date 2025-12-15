use std::collections::HashSet;
use serde::Deserialize;
use anyhow::Result;
use futures::{StreamExt, stream};
use hickory_resolver::Resolver;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::name_server::TokioConnectionProvider;

use crate::model::Subdomain;

pub async fn enumerate(http_client: &reqwest::Client, domain: &str) -> Result<Vec<Subdomain>> {
    // Declare needed response fields
    #[derive(Debug, Deserialize)]
    struct CrtShEntry {
        name_value: String,
    }

    // Get query result
    let entries = http_client
        .get(format!("https://crt.sh/?q=%25.{}&output=json", domain))
        .send()
        .await?
        .json::<Vec<CrtShEntry>>()
        .await?;

    // Clean & Deduplicate query result
    let mut subdomains = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .name_value
                .split("\n")
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain| subdomain != domain)
        .filter(|subdomain| !subdomain.contains("*"))
        .collect::<HashSet<String>>();

    // Insert target subdomain into HashSet
    subdomains.insert(domain.to_string());

    debug_assert!(!subdomains.is_empty());
    debug_assert!(subdomains.contains(domain));

    // Declare DNS resolver
    let resolver = Resolver::builder_with_config(
        ResolverConfig::default(),
        TokioConnectionProvider::default(),
    )
    .build();

    // Enumerate subdomains
    let subdomains = stream::iter(subdomains.into_iter())
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        .filter_map(|subdomain| {
            let _resolver = resolver.clone();
            async move {
                if is_resolvable(&_resolver, &subdomain.domain).await {
                    Some(subdomain)
                } else {
                    None
                }
            }
        })
        .collect::<Vec<Subdomain>>()
        .await;

    Ok(subdomains)
}

async fn is_resolvable(resolver: &Resolver<TokioConnectionProvider>, domain: &str) -> bool {
    resolver.lookup_ip(domain).await.is_ok()
}
