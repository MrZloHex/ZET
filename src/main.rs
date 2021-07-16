#![allow(non_snake_case)]

mod user_node;
use user_node::user::User;

use crate::user_node::user;


use libp2p::{
    PeerId,
    Transport,
    identity,
    noise::{Keypair, NoiseConfig, X25519Spec},
    tcp::TokioTcpConfig,
    core::upgrade,
    mplex
};


#[tokio::main]
async fn main() {
    let username = if let Some(uname) = std::env::args().nth(1) {
        println!("Creating user {}", uname);
        uname
    } else {
        "user".to_string()
    };

    let keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(keys.public());

    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&keys)
        .expect("Can create  auth keys");

    let transport = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();
}
