plain
    Checking addr2line v0.19.0
error[E0308]: mismatched types
   --> library/std/src/sys_common/process.rs:101:24
    |
101 |         CapturedEnvs { iter }
    |                        ^^^^ expected `IntoIter<OsString, OsString>`, found `IntoIter<EnvKey, OsString>`
    = note: expected struct `alloc_crate::collections::btree_map::IntoIter<OsString, _>`
    = note: expected struct `alloc_crate::collections::btree_map::IntoIter<OsString, _>`
               found struct `alloc_crate::collections::btree_map::IntoIter<EnvKey, _>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:12
