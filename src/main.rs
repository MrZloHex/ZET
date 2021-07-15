#![allow(non_snake_case)]

pub mod hash;

use hash::users;

fn main() {
    let mut users = users::Users::new();

    match users.add_user("MrZlo".to_string()) {
        Err(_) => panic!("This user existing"),
        _ => ()
    }
}
