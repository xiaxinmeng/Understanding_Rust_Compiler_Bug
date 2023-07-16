plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
...iiiiii...................................F............i.............................. 1232/1233
.
failures:

---- src/thread/mod.rs - thread::current_cpu (line 1660) stdout ----
error[E0658]: use of unstable library feature 'current_cpu'
  |
  |
7 |     let cpu = thread::current_cpu()?.get();
  |
  |
  = help: add `#![feature(current_cpu)]` to the crate attributes to enable
error[E0599]: no method named `get` found for type `u32` in the current scope
 --> src/thread/mod.rs:1665:38
  |
  |
7 |     let cpu = thread::current_cpu()?.get();
  |                                      ^^^ method not found in `u32`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0658.
For more information about an error, try `rustc --explain E0599`.
