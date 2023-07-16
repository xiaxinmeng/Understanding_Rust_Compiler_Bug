 rust
use std::any::Any;
use std::sync::{Arc, Mutex};

pub type Wild = Any + 'static + Send;

pub fn with_wild(_: Option<Arc<Wild>>) { }

struct Modest;

fn main() {

    // Works
    with_wild(Some(Arc::new(Mutex::new(Modest))));

    let fluffy = Some(Arc::new(Mutex::new(Modest)));
    with_wild(fluffy.map(|x| x as Arc<Wild>));

}
