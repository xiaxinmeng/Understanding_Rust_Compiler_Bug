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
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0522]: definition of an unknown language item: `Context`
   --> library/core/src/task/wake.rs:177:23
    |
177 | #[cfg_attr(bootstrap, lang = "Context")]
    |                       ^^^^^^^^^^^^^^^^ definition of unknown language item `Context`
For more information about this error, try `rustc --explain E0522`.
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:00:14
