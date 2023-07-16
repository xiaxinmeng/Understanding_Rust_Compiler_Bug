plain
[RUSTC-TIMING] core test:false 25.817
error[E0308]: mismatched types
   --> library/std/src/net/addr.rs:286:19
    |
286 |                 ..unsafe { mem::zeroed() }
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `sockaddr_in`, found inferred type
    = note: expected struct `sockaddr_in`
                 found type `_`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> library/std/src/net/addr.rs:387:19
    |
387 |                 ..unsafe { mem::zeroed() }
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `sockaddr_in6`, found inferred type
    = note: expected struct `sockaddr_in6`
                 found type `_`

For more information about this error, try `rustc --explain E0308`.
