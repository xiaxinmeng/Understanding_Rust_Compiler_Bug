rust
use std::collections::BTreeSet;
use std::iter;

fn main() {
    let mut junks = None;
    for (id, value) in iter::repeat(("junk", 99)).take(10) {
        if id == "junk" {
            junks.get_or_insert_with(BTreeSet::default).insert(value);
        }
    }
    println!("{:?}", junks);
}
