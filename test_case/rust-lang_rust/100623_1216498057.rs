
use fnv::FnvHasher;
use std::hash::{Hash, Hasher};
use std::net::Ipv4Addr;

fn main() {
    fn hash<T: Hash>(value: T) -> u64 {
        let mut hasher = FnvHasher::default();
        value.hash(&mut hasher);
        hasher.finish()
    }

    assert_eq!(
        hash(Ipv4Addr::new(127, 0, 0, 1)),
        7785339717392670637,
    );
}
