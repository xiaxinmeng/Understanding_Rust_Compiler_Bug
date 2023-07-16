
 error[E0605]: non-primitive cast: `std::collections::HashSet<*mut arena::HeaderUnTyped>` as `*mut _`
   --> src/gc_logic.rs:705:25
    |
705 |                         ts.gc_arenas_full as *mut _ as *mut HashSet<*mut u8>,
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait

error[E0605]: non-primitive cast: `std::collections::HashSet<*mut arena::HeaderUnTyped>` as `*mut _`
   --> src/gc_logic.rs:705:25
    |
705 |                         ts.gc_arenas_full as *mut _ as *mut HashSet<*mut u8>,
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait

error: aborting due to previous error; 1 warning emitted
