plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error: duplicate diagnostic item found: `LocalKey`.
    |
    |
100 | pub struct LocalKey<T: 'static> {
    |
    = note: the diagnostic item is first defined in crate `std`.

error: could not compile `std` due to previous error
