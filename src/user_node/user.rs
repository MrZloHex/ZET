mod hash;
use hash::users_map::UsersMap;


pub struct User {
    user_map: UsersMap,
    username: String
}

impl User {
    pub fn new(username: &str) -> User {
        User {
            user_map: UsersMap::new(username.clone().to_string()),
            username: username.to_string()
        }
    }
}