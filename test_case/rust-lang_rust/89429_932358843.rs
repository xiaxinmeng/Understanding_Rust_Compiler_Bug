rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct User {
    name: String,
    age: u32,
    cool: bool,
}

struct Boozer {
    name: String,
    age: u32,
    cool: bool,
}

impl Hash for Boozer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.age.hash(state);
        self.cool.hash(state);
    }
}

fn hash<H: Hash>(h: H) {
    let mut hasher = DefaultHasher::new();
    h.hash(&mut hasher);
    println!("{}", hasher.finish());
}

fn main() {
    let user = User {
        name: "Jack".to_string(),
        age: 8,
        cool: true,
    };
    let boozer = Boozer {
        name: "Jack".to_string(),
        age: 8,
        cool: true,
    };
    let tuple = ("Jack".to_string(), 8, true);

    hash(user);
    hash(boozer);
    hash(tuple);
}
