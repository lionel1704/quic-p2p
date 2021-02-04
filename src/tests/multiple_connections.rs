use crate::{Endpoint, IncomingMessages};

use super::{new_qp2p, random_msg};
use anyhow::Result;

#[tokio::test(core_threads = 20)]
async fn multiple_connections() -> Result<()> {
    let qp2p = new_qp2p()?;
    // creating 4 peers
    let (peer1, _peer1_incoming_conns, mut peer1_incoming_messages, _peer1_disconnections) = qp2p.new_endpoint().await?;
    let (peer2, mut peer2_incoming_conns, mut peer2_incoming_messages, _peer2_disconnections) = qp2p.new_endpoint().await?;
    let (peer3, mut peer3_incoming_conns, mut peer3_incoming_messages, _peer3_disconnections) = qp2p.new_endpoint().await?;
    let (peer4, mut peer4_incoming_conns, mut peer4_incoming_messages, _peer4_disconnections) = qp2p.new_endpoint().await?;

    let peer1_addr = peer1.socket_addr();
    let peer2_addr = peer2.socket_addr();
    let peer3_addr = peer3.socket_addr();
    let peer4_addr = peer4.socket_addr();

    // Peer 1 connects to the other peers
    peer1.connect_to(&peer2_addr).await?;
    peer1.connect_to(&peer3_addr).await?;
    peer1.connect_to(&peer4_addr).await?;

    // All the peers should receive connection event
    let incoming_conn_at_peer2 = peer2_incoming_conns.next().await;
    assert_eq!(incoming_conn_at_peer2, Some(peer1_addr));

    let incoming_conn_at_peer3 = peer3_incoming_conns.next().await;
    assert_eq!(incoming_conn_at_peer3, Some(peer1_addr));

    let incoming_conn_at_peer4 = peer4_incoming_conns.next().await;
    assert_eq!(incoming_conn_at_peer4, Some(peer1_addr));


    exchange_message(&peer1, &mut peer1_incoming_messages, &peer2, &mut peer2_incoming_messages).await?;
    exchange_message(&peer1, &mut peer1_incoming_messages, &peer3, &mut peer3_incoming_messages).await?;
    exchange_message(&peer1, &mut peer1_incoming_messages, &peer4, &mut peer4_incoming_messages).await?;

    Ok(())
}

async fn exchange_message(peer_a: &Endpoint, peer_a_message_channel: &mut IncomingMessages, peer_b: &Endpoint, peer_b_message_channel: &mut IncomingMessages) -> Result<()> {
    let msg1 = random_msg();
    peer_a.send_message(msg1.clone(), &peer_b.socket_addr()).await?;

    let msg2 = random_msg();
    peer_b.send_message(msg2.clone(), &peer_a.socket_addr()).await?;

    
    let received= peer_b_message_channel.next().await;
    assert_eq!(received, Some((peer_a.socket_addr(), msg1)));

    let received= peer_a_message_channel.next().await;
    assert_eq!(received, Some((peer_b.socket_addr(), msg2)));

    Ok(())
}