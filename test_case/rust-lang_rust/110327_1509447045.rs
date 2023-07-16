plain
    Checking hashbrown v0.12.3
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.29.0
    Checking addr2line v0.17.0
error[E0599]: no method named `captured_envs` found for struct `sys::windows::process::Command` in the current scope
     |
1057 |         self.inner.captured_envs()
     |                    ^^^^^^^^^^^^^ method not found in `Command`
     |
     |
    ::: library/std/src/sys/windows/process.rs:158:1
     |
158  | pub struct Command {
     | ------------------ method `captured_envs` not found for this struct
error[E0308]: mismatched types
   --> library/std/src/sys_common/process.rs:101:24
    |
    |
101 |         CapturedEnvs { iter }
    |                        ^^^^ expected `IntoIter<OsString, OsString>`, found `IntoIter<EnvKey, OsString>`
    = note: expected struct `alloc_crate::collections::btree_map::IntoIter<OsString, _>`
    = note: expected struct `alloc_crate::collections::btree_map::IntoIter<OsString, _>`
               found struct `alloc_crate::collections::btree_map::IntoIter<EnvKey, _>`
Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:21
