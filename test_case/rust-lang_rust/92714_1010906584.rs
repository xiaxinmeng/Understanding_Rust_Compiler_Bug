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
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error[E0658]: attributes on expressions are experimental
   --> src/tools/compiletest/src/header.rs:889:9
    |
889 |         #[cfg(bootstrap)]
    |
    = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
    = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable


error: removing an expression is not supported in this position
   --> src/tools/compiletest/src/header.rs:889:9
    |
889 |         #[cfg(bootstrap)]

For more information about this error, try `rustc --explain E0658`.
error: could not compile `compiletest` due to 2 previous errors

