rust
use std::collections::HashMap;

pub trait T {}

static CALLBACKS: HashMap<*const dyn T, dyn FnMut(&mut _) + 'static> = HashMap::new();

pub fn main() {}
