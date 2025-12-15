use futures::StreamExt;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::time::Duration;

use crate::model::Port;
use crate::model::Subdomain;

pub async fn scan_top_100_ports(concurrency: usize, subdomain: Subdomain) -> Subdomain {
    const TOP_100_PORTS: &[u16] = &[
        80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111, 995,
        993, 5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179,
        1026, 2000, 8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666,
        646, 5000, 5631, 631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513,
        990, 5357, 427, 49156, 543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070,
        5190, 3000, 5432, 1900, 3986, 13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899,
        9100, 119, 37
    ];

    let mut ret = subdomain.clone();

    // Resolve domain to socket address
    // - 1337 is a dummy port to satisfy the `to_socket_addrs`
    print!("{:?}", subdomain);
    let socket_addrs: Vec<SocketAddr> = format!("{}:1337", subdomain.domain)
        .to_socket_addrs()
        .expect("Creating Socket Address")
        .collect();

    // If subdomain is unresolvable, return the original object
    if socket_addrs.is_empty() {
        return ret;
    }

    let socket_addr = socket_addrs[0];
    debug_assert!(socket_addr.port() == 1337);

    // Create channels
    let (input_tx, input_rx) = tokio::sync::mpsc::channel(concurrency);
    let (output_tx, output_rx) = tokio::sync::mpsc::channel(concurrency);
    let input_rx_stream = tokio_stream::wrappers::ReceiverStream::new(input_rx);
    let output_rx_stream = tokio_stream::wrappers::ReceiverStream::new(output_rx);

    // Send TOP_100_PORTS to input channel
    tokio::spawn(async move {
        for port in TOP_100_PORTS {
            input_tx.send(*port).await.unwrap();
        }
    });

    // Consume input channel - scan ports and send open ports to output channel
    input_rx_stream
        .for_each_concurrent(concurrency, |port| {
            let _output_tx = output_tx.clone();
            async move {
                let port = scan_port(socket_addr, port).await;
                if port.is_open {
                    _output_tx.send(port).await.unwrap();
                }
            }
        })
        .await;

    // Explicitly close the output channel to collect the results
    drop(output_tx);

    ret.open_ports = output_rx_stream.collect().await;
    ret.open_ports.sort_by_key(|p| p.port);

    ret
}

async fn scan_port(mut socket_addr: SocketAddr, port: u16) -> Port {
    socket_addr.set_port(port);

    let timeout = Duration::from_secs(3);
    let connection = tokio::time::timeout(timeout, tokio::net::TcpStream::connect(&socket_addr));

    let is_open = matches!(connection.await, Ok(Ok(_stream)));

    Port { port, is_open }
}
