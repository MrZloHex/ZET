use std::{collections::HashMap};
use sha2::{Digest, Sha512};
use std::convert::TryInto;

pub struct UsersMap {
    users_map: HashMap<String, [u8; 64]>
}

impl UsersMap {
    pub fn new(username: String) -> UsersMap {
        let mut u_m = UsersMap {
            users_map: HashMap::new()
        };
        u_m.add_user(username);
        u_m
    }

    pub fn add_user(&mut self, username: String) -> Result<(), u8> {
        let user_hash = Sha512::digest(username.clone().as_str().as_bytes());
        let user_hash: [u8; 64] = user_hash.as_slice().try_into().expect("Wrong length");

        if !(self.user_exist(username.clone())) {
            self.users_map.insert(username, user_hash);
            return Ok(());
        } else {
            return Err(1);
        }
    }

    fn user_exist(&mut self, username: String) -> bool {
        match self.users_map.get(&username) {
            Some(_) => return false,
            None => return true
        };
    }
}