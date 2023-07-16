plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking core v0.0.0 (/checkout/library/core)
error[E0425]: cannot find value `c4` in this scope
   --> library/core/tests/slice.rs:756:14
    |
756 |     let c4 = c4.array_windows::<0>();
    |              ^^ help: a local variable with a similar name exists: `c`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
