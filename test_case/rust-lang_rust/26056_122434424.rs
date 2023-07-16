 rust
use std::collections::HashMap;

trait Map: MapLookup<<Self as Map>::Key> {
    type Key;
    type Value;
}

impl<K, V> Map for HashMap<K, V> {
    type Key = K;
    type Value = V;
}

trait MapLookup<Q> {
    type MapValue;
}

impl<K, V> MapLookup<K> for HashMap<K, V> {
    type MapValue = V;
}

fn main() {
    let _ = &HashMap::new()
        as &Map<Key = u32, Value = u16, MapValue = u16>;
}
