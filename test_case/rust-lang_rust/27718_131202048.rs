 rust
#![feature(arc_unique,arc_counts)]

use std::sync::Arc;
use std::hash::{Hash,Hasher};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ArcHashKey<T>(pub Arc<T>);
impl<T: PartialEq> PartialEq for ArcHashKey<T> {
    fn eq(&self, other: &ArcHashKey<T>) -> bool { self.0.eq(&other.0) }
}
impl<T: Eq> Eq for ArcHashKey<T> { }
impl<T: Hash> Hash for ArcHashKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state) }
}

fn main() {
    let mut map=HashMap::new();
    let mut key=ArcHashKey(Arc::new("key"));
    map.insert(key.clone(),"value");
    // hand out some references to key, thread some things
    if Arc::strong_count(&key.0)==2 {
        map.remove(&key);
        println!("all threads are done with the arc: {:?}",Arc::get_mut(&mut key.0));
    }
}
