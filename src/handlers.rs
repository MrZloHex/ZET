use std::collections::HashSet;
use libp2p::Swarm;

use crate::rules::MessageBehaviour;

pub async fn handle_list_peers(swarm: &mut Swarm<MessageBehaviour>) {
    println!("Discovered Peers:");
    let nodes = swarm.mdns.discovered_nodes();
    let mut unique_peers = HashSet::new();
    for peer in nodes {
        unique_peers.insert(peer);
    }
    unique_peers.iter().for_each(|p| println!("{}", p));
}

pub async fn handle_send_message(swarm: &mut Swarm<MessageBehaviour>) {
    
}