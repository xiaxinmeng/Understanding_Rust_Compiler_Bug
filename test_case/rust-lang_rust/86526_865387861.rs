plain
   Compiling hashbrown v0.11.0
   Compiling object v0.22.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.14.0
error: cannot find macro `cfg_if` in this scope
    |
212 |             cfg_if! {
    |             ^^^^^^
    |
    |
    = note: consider importing this macro:
            cfg_if::cfg_if

error[E0425]: cannot find value `sun_len` in this scope
    |
    |
231 |             AddressKind::Pathname(OsStr::from_bytes(&path[..sun_len]).as_ref())

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
