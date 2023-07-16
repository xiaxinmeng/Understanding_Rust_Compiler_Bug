plain
    |
137 |                 offset as i64,
    |                 ^^^^^^^^^^^^^ expected `i32`, found `i64`
    |
help: you can convert an `i64` to an `i32` and panic if the converted value doesn't fit
    |
137 |                 (offset as i64).try_into().unwrap(),
    |                 +             +++++++++++++++++++++
error[E0308]: mismatched types
   --> library/std/src/sys/unix/fd.rs:217:17
    |
217 |                 offset as i64,
217 |                 offset as i64,
    |                 ^^^^^^^^^^^^^ expected `i32`, found `i64`
    |
help: you can convert an `i64` to an `i32` and panic if the converted value doesn't fit
    |
217 |                 (offset as i64).try_into().unwrap(),
    |                 +             +++++++++++++++++++++
For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:45
