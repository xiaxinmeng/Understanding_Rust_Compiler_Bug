 rust
use std::mem;
struct Bucket<K,V> { x: uint, y: K, z: V }
fn main() {
     println!("{} {}",
              mem::size_of::<Bucket<~str, ~str>>(),
              mem::size_of::<Option<Bucket<~str, ~str>>>())
}
