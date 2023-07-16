rust
use std::collections::HashMap;

fn map() {
    let map = HashMap::<u32, u32>::new();
    let _ = map.iter().rev();
}
