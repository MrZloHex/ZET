mod hash;
use hash::users_map::UsersMap;


use libp2p::{PeerId, floodsub::Topic, identity::Keypair};


pub struct User {
    user_map: UsersMap,
    username: String,
    keys: Keypair,
    peer_id: PeerId,
    topic: Topic,
    balance: u128
}

impl User {
    pub fn new(username: String, key_pair: Keypair, tp: Topic) -> User {
        User {
            user_map: UsersMap::new(),
            username: username.to_string(),
            keys: key_pair.clone(),
            peer_id: PeerId::from(key_pair.public()),
            topic: tp,
            balance: 0
        }
    }

    pub fn get_keys(&mut self) -> Keypair {
        self.keys.clone()
    }

    pub fn invite() {

    }

    pub fn augment() {

    }

    pub fn transfer() {

    }

    pub fn request() {

    }
}