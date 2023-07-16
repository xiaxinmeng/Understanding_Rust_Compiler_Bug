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
    Checking hashbrown v0.12.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0432]: unresolved imports `crate::sys::mutex`, `crate::sys::mutex`
  |
  |
3 | use crate::sys::mutex::{self, Mutex};
  |                 ^^^^^   ^^^^ no `mutex` in `sys`
  |                 |
  |                 could not find `mutex` in `sys`

error[E0425]: cannot find function `dur2timeout` in module `super`
   |
   |
34 |             super::dur2timeout(dur),
   |                    ^^^^^^^^^^^ not found in `super`
help: consider importing this function
   |
   |
1  | use crate::sys::dur2timeout;

Some errors have detailed explanations: E0425, E0432.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `std` due to 2 previous errors
