#![allow(non_snake_case)]

mod user_node;
use std::usize;

use user_node::user::User;

use crate::user_node::user;


use libp2p::{
    PeerId,
    Transport,
    identity,
    noise::{Keypair, NoiseConfig, X25519Spec},
    tcp::TokioTcpConfig,
    core::upgrade,
    mplex,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    mdns::{TokioMdns},
    NetworkBehaviour
};

use tokio::{
    fs,
    io::AsyncBufRead,
    sync::mpsc
};


struct Message {
    id: usize,
    theme: String,
    data: String,
    public: bool
}


enum ListMode {
    ALL,
    One(String),
}


struct ListResponse {
    mode: ListMode,
    data: Message,
    receiver: String,
}

#[derive(NetworkBehaviour)]
struct MessageBehaviour {
    floodsub: Floodsub,
    mdns: TokioMdns,
    #[behaviour(ignore)]
    response_sender: mpsc::UnboundedSender<ListResponse>
}






#[tokio::main]
async fn main() {
    let username = if let Some(uname) = std::env::args().nth(1) {
        println!("Creating user {}", uname);
        uname
    } else {
        "user".to_string()
    };

    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();

    let keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(keys.public());
    let topic = Topic::new("message");

    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&keys)
        .expect("Can create  auth keys");

    let transport = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let mut behavior = MessageBehaviour {
        floodsub: Floodsub::new(peer_id.clone()),
        mdns: TokioMdns::new().expect("Can create mdns"),
        response_sender,
    };

    behavior.floodsub.subscribe(topic.clone());


}
