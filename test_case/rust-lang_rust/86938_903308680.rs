plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.18
error[E0599]: no associated item named `CapacityOverflow` found for struct `TryReserveError` in the current scope
    |
    |
65  | pub struct TryReserveError {
    | -------------------------- associated item `CapacityOverflow` not found for this
...
117 |             TryReserveError::CapacityOverflow => capacity_overflow(),
    |                              ^^^^^^^^^^^^^^^^ associated item not found in `TryReserveError`
error[E0223]: ambiguous associated type
   --> library/alloc/src/collections/mod.rs:118:13
    |
    |
118 |             TryReserveError::AllocError { layout, non_exhaustive: () } => {
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<TryReserveError as Trait>::AllocError`
error[E0223]: ambiguous associated type
   --> library/alloc/src/raw_vec.rs:242:34
    |
    |
242 |             Err(_) => return Err(TryReserveError::AllocError { layout, non_exhaustive: () }),
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<TryReserveError as Trait>::AllocError`
error[E0223]: ambiguous associated type
    --> library/alloc/src/sync.rs:1207:26
     |
     |
1207 |             .map_err(|_| TryReserveError::AllocError { layout, non_exhaustive: () })
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<TryReserveError as Trait>::AllocError`

error[E0599]: no associated item named `CapacityOverflow` found for struct `TryReserveError` in the current scope
    |
    |
105 |             Err(TryReserveError::CapacityOverflow)
    |                                  ^^^^^^^^^^^^^^^^ associated item not found in `TryReserveError`
   ::: library/alloc/src/collections/mod.rs:65:1
    |
    |
65  | pub struct TryReserveError {
    | -------------------------- associated item `CapacityOverflow` not found for this
Some errors have detailed explanations: E0223, E0599.
For more information about an error, try `rustc --explain E0223`.
error: could not compile `alloc` due to 5 previous errors
Build completed unsuccessfully in 0:01:16
