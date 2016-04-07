use std::collections::hash_map::{HashMap, Entry};

type UidGidHashMap = HashMap<u32, String>;

struct UidGidHash { mapper: UidGidHashMap }

impl UidGidHash {

    fn search(&mut self, key: u32) -> String {
        match self.mapper.entry(key.to_owned()) {
            // Cache miss - lookup user_name and store
            Entry::Vacant(entry) => {
                let user_name = lookup_user(key);
                entry.insert(user_name.to_owned());
                return user_name
            }
            // Cache hit - return value
            Entry::Occupied(entry) => {
                return entry.get().to_owned()
            }
        }
    }

}


fn main() {

    // Gets "Jenny" - a direct lookup is working fine
    let mut uid = 503;
    let myuser = lookup_user(uid);
    println!("{:?}", myuser);

    // Create struct with hashmap of uids->usernames
    let mut table = UidGidHash { mapper: UidGidHashMap::new() };

    // Look for Jenny in the cache. As it is empty, insert the K:V pair, getting V from lookup_user(uid: u32)
    uid = 503;
    let user = table.search(uid);
    println!("User is {:?}", user);

    uid = 502;
    let user = table.search(uid);

    println!("HashMap contains:\n{:#?}", table.mapper);

}


fn lookup_user(uid: u32) -> String {
    let user = match uid {
        0 => "root",
        502 => "Matt",
        503 => "Jenny",
        _ => "UNKNOWN!"
    };
    user.to_string()
}





// --
