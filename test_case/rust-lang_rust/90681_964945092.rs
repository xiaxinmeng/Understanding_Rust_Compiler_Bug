plain
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.027
error[E0405]: cannot find trait `Send` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.107/src/wasi.rs:61:13
   |
61 | unsafe impl Send for clockid_t {}
   |
help: consider importing this trait
   |
1  | use core::marker::Send;
1  | use core::marker::Send;
   |

error[E0405]: cannot find trait `Sync` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.107/src/wasi.rs:62:13
   |
62 | unsafe impl Sync for clockid_t {}
   |
help: consider importing this trait
   |
1  | use core::marker::Sync;
