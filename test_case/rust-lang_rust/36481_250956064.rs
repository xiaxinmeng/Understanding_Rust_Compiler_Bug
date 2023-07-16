 rust
use std::collections::HashMap;
fn main() {
    let first_map: HashMap<u64, _> = (0..900000).map(|i| (i, ())).collect();
    let second_map: HashMap<u64, _> = (900000..1800000).map(|i| (i, ())).collect();

    let mut merged = first_map;
    println!("-->");
    merged.extend(second_map);
    println!("{}", merged.len());
}
