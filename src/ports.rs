use futures::{StreamExt, future, stream};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::net::lookup_host;

use crate::model::Port;
use crate::model::Subdomain;

pub async fn scan_top_100_ports(concurrency: usize, mut subdomain: Subdomain) -> Subdomain {
    const TOP_100_PORTS: &[Port] = &[
        80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111, 995,
        993, 5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179,
        1026, 2000, 8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666,
        646, 5000, 5631, 631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513,
        990, 5357, 427, 49156, 543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070,
        5190, 3000, 5432, 1900, 3986, 13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899,
        9100, 119, 37,
    ];

    // Resolve domain to socket address
    // - Port 1337 is a dummy port in order to satisfy the `SocketAddr` type
    let socket_addr = lookup_host(format!("{}:1337", subdomain.domain))
        .await
        .expect("DNS lookup failed")
        .next()
        .expect("No IP address resolved");

    // Probe top 100 ports
    let mut open_ports: Vec<Port> = stream::iter(TOP_100_PORTS.iter().copied())
        .map(|port| {
            let socket_addr = SocketAddr::new(socket_addr.ip(), port);
            async move {
                let is_open = is_port_open(socket_addr).await;
                if is_open { Some(port) } else { None }
            }
        })
        .buffer_unordered(concurrency)
        .filter_map(future::ready) // drop None values
        .collect()
        .await;

    // Sort open ports in ascending order
    open_ports.sort_unstable();

    subdomain.open_ports = open_ports;

    subdomain
}

async fn is_port_open(socket_addr: SocketAddr) -> bool {
    let timeout = Duration::from_secs(3);
    let connection = tokio::time::timeout(timeout, TcpStream::connect(&socket_addr));
    matches!(connection.await, Ok(Ok(_stream)))
}
