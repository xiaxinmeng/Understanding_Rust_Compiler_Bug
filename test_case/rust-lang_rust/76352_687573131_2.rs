rust
fn main() {
    let mut hmap = std::collections::HashMap::<u64, u64>::new();
    hmap.insert(1, 2);
    zzz(); // breakpointing here
    println!("Hello, world! {:?}", hmap);
}

#[no_mangle] extern "C" fn zzz() {}
