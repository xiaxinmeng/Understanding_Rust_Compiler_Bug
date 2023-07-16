plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking addr2line v0.16.0
error[E0433]: failed to resolve: use of undeclared crate or module `sync`
  --> library/std/src/sys/windows/rand.rs:31:19
   |
31 |     static VALUE: sync::OnceLock<HashMapRng> = sync::OnceLock::new();
   |                   ^^^^ use of undeclared crate or module `sync`
error[E0433]: failed to resolve: use of undeclared crate or module `sync`
  --> library/std/src/sys/windows/rand.rs:31:54
   |
   |
31 |     static VALUE: sync::OnceLock<HashMapRng> = sync::OnceLock::new();
   |
help: consider importing this struct
   |
1  | use crate::sync::OnceLock;
1  | use crate::sync::OnceLock;
   |
help: if you import `OnceLock`, refer to it directly
   |
31 -     static VALUE: sync::OnceLock<HashMapRng> = sync::OnceLock::new();
31 +     static VALUE: sync::OnceLock<HashMapRng> = OnceLock::new();

error: unused import: `crate::lazy`
 --> library/std/src/sys/windows/rand.rs:2:5
  |
  |
2 | use crate::lazy;
  |     ^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `std` due to 3 previous errors
Build completed unsuccessfully in 0:00:20
