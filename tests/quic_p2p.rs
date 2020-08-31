use quic_p2p::QuicP2p;
use quic_p2p::Config;
use std::net::{IpAddr, Ipv4Addr};
use bytes::Bytes;

#[tokio::test]
async fn connection() {
    let mut config = Config::default();
    config.ip = Some(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let quic_p2p = QuicP2p::with_config(Some(config), Default::default(), false).unwrap();
    let endpoint_a = quic_p2p.new_endpoint().unwrap();
    println!("{}", endpoint_a.local_address());
    let endpoint_b = quic_p2p.new_endpoint().unwrap();
    println!("{}", endpoint_b.local_address());
    
    let connection = endpoint_a.connect_to(&endpoint_b.local_address()).await.unwrap();
    let (mut send, recv) = connection.send(Bytes::from(vec![1, 2, 3, 4])).await.unwrap();
    send.write_all(&vec![1, 2, 3, 4]).await.unwrap();
    
    let mut incoming_connections = endpoint_b.listen().unwrap();
    let message = incoming_connections.next().await;
    let mut messages = message.unwrap();
    let msg = messages.next().await.unwrap();
    dbg!(msg);
}