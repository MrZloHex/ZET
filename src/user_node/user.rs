mod hash;
use std::{collections::hash_map::Keys, future::Ready};

use hash::users_map::UsersMap;


pub struct User {
    user_map: UsersMap,
    username: String,
    balance: u128
}

impl User {
    pub fn new(username: &str) -> User {
        User {
            user_map: UsersMap::new(),
            username: username.to_string(),
            balance: 0
        }
    }

    pub fn invite(&mut self, username: &str) -> User {
        match self.user_map.add_user(username.clone().to_string()) {
            Ok(_) => (),
            Err(_) => panic!("This user is invited")
        };
        User::new(username)
    }

    pub fn my_friends(&mut self) {
        for user in self.user_map.get_users_map().keys() {
            println!("{}", user);
        }
    }

    pub fn send_message(&mut self, addressat: &str, message: &str) -> Result<(), u16> {
        
    }
}