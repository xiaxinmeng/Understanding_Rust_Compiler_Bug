rust
use std::borrow::Borrow;
use std::collections::hash_map::RawEntryMut;
use std::hash::{BuildHasher, Hash, Hasher};

fn get_mut_or_insert_with<'a, K, V, Q, F>(
    map: &'a mut HashMap<K, V>,
    key: &Q,
    default: F,
) -> &'a mut V
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ToOwned<Owned = K>,
    F: FnOnce() -> V,
{
    let mut hasher = map.hasher().build_hasher();
    key.hash(&mut hasher);
    let hash = hasher.finish();

    match map.raw_entry_mut().from_key_hashed_nocheck(hash, key) {
        RawEntryMut::Occupied(entry) => entry.into_mut(),
        RawEntryMut::Vacant(entry) => {
            entry
                .insert_hashed_nocheck(hash, key.to_owned(), default())
                .1
        }
    }
}
