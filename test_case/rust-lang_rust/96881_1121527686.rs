plain
   Compiling addr2line v0.16.0
error[E0308]: mismatched types
    --> library/std/src/ffi/os_str.rs:1233:28
     |
1233 |         suffix.iter().fold(first, |mut a, b| {
     |                            ^^^^^ expected struct `OsString`, found `&S`
     = note: expected struct `OsString`
             found reference `&S`

For more information about this error, try `rustc --explain E0308`.
