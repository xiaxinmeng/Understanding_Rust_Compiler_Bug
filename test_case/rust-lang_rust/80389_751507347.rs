rust
use std::collections::HashMap;

pub fn test<'a, 'b>(map: &'a HashMap<&'static (), ()>, key: &'b ()) -> Option<&'a ()> {
    map.get(&key)
}
