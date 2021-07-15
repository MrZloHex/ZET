#![allow(non_snake_case)]

mod user_node;
use user_node::user::User;

fn main() {
    let mut MrZloHex = User::new("MrZlohex");

    let mut Arkasha = MrZloHex.invite("Arkasha");

    MrZloHex.my_friends();
}
