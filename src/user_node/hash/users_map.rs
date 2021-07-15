use std::{collections::HashMap};
use sha2::{Digest, Sha512};
use std::convert::TryInto;

pub struct UsersMap {
    users: HashMap<String, [u8; 64]>
}

impl UsersMap {
    pub fn new() -> UsersMap {
        UsersMap {
            users: HashMap::new()
        }
    }

    pub fn add_user(&mut self, username: String) -> Result<(), u8> {
        let user_hash = Sha512::digest(username.clone().as_str().as_bytes());
        let user_hash: [u8; 64] = user_hash.as_slice().try_into().expect("Wrong length");

        if !(self.user_exist(username.clone())) {
            self.users.insert(username, user_hash);
            return Ok(());
        } else {
            return Err(1);
        }
    }

    fn user_exist(&mut self, username: String) -> bool {
        match self.users.get(&username) {
            Some(_) => return false,
            None => return true
        };
    }
}