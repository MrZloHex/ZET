use std::collections::HashSet;
use libp2p::Swarm;

use crate::{rules::MessageBehaviour, user_node::user::User};

pub async fn handle_list_peers(swarm: &mut Swarm<MessageBehaviour>) {
    println!("Discovered Peers:");
    let nodes = swarm.mdns.discovered_nodes();
    let mut unique_peers = HashSet::new();
    for peer in nodes {
        unique_peers.insert(peer);
    }
    unique_peers.iter().for_each(|p| println!("{}", p));
}

pub async fn handle_send_message(cmd: &str, swarm: &mut Swarm<MessageBehaviour>, user: &mut User) {
    let rest = cmd.strip_prefix("send ").unwrap();
    let rest_c: Vec<&str> = rest.split(" ").collect();
    let peer_recipient = rest_c[1];
    let message = rest_c[0];

    println!("Recepient: {}, Message: {}", peer_recipient, message);

    swarm.floodsub.publish(user.get_topic(), message.as_bytes());
}