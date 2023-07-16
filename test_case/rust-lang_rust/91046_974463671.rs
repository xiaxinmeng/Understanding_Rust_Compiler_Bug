rust
use std::collections::HashMap;
use std::hash::Hash;

pub fn get<'a, K: Eq + Hash, V, const N: usize>(
    map: &'a mut HashMap<K, V>,
    mut_index: &K,
    ref_indices: [&K; N],
) -> Option<(&'a mut V, [Option<&'a V>; N])> {
    if ref_indices.contains(&&mut_index) {
        panic!("Attempted to obtain a mutable and immutable reference to same item");
    }

    let mut_item = map.get_mut(mut_index)? as *mut V;

    let map = &*map; // <- add this line

    let ref_items = ref_indices.map(|i| map.get(i));
    // SAFETY:
    // - aliasing: we got passed a unique reference to the map and made sure none of our immutable references point to mut_item
    // - lifetime: the reference has lifetime 'a which matches the HashMap it points into
    let mut_item = unsafe { &mut *mut_item as &'a mut V };

    Some((mut_item, ref_items))
}
