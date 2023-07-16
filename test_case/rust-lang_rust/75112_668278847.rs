rust
use std::collections::HashMap;

pub fn find<'a>(lookup: &HashMap<u32, usize>, items: &'a mut Vec<u32>, id: u32) -> Option<&'a mut u32> {
    if let Some(token) = lookup.get(&id) {
        return items.get_mut(*token);
    }

    if let Some(token) = lookup.get(&id.wrapping_add(1)) {
        let item = items.get_mut(*token)?; // <- 2. Therefore this lifetime of this borrow of `items` is exactly 'a, not a shorter, temporary lifetime.
        if *item == id {
            return Some(item); // <- 1. This line requires item: 'a
        }
    }

    if let Some(token) = lookup.get(&(id.wrapping_sub(1))) {
        let item = items.get_mut(*token)?; // <- 3. Because 2 already requires the lifetime to be borrowed for entirety of 'a, this is no longer allowed.
        if *item == id {
            return Some(item);
        }
    }

    None
}
