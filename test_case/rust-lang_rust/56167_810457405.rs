rust
// - Just a single T instead of (K, V).
// - No bounds on T.
// - Hashing and equality checking is completely user-controlled.
// - A hasher is provided by the caller for functions that might cause a table resize.

pub fn get_mut(
    &mut self,
    hash: u64,
    eq: impl FnMut(&T) -> bool
) -> Option<&mut T>;

pub fn insert_entry(
    &mut self,
    hash: u64,
    value: T,
    hasher: impl Fn(&T) -> u64
) -> &mut T;
