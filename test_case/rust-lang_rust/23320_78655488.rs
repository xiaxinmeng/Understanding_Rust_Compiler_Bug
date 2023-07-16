 rust
use std::fmt::Debug;
use std::hash::{hash, Hash, SipHasher};
use quickcheck::{Arbitrary, quickcheck};

fn prop_sane_hash<T: Arbitrary + Debug + Hash + PartialEq>(a: T, b: T) -> bool {
    a != b || hash::<_, SipHasher>(&a) == hash::<_, SipHasher>(&b)
}
fn main() {
    quickcheck(prop_sane_hash as fn(Foo, Foo) -> bool);
}
