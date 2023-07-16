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
   Compiling tracing v0.1.25
   Compiling tracing-subscriber v0.2.16
   Compiling rustfix v0.6.0
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
   --> src/tools/compiletest/src/header.rs:901:5
901 |     test::TestDesc {
901 |     test::TestDesc {
    |     ^^^^^^^^^^^^^^ missing `compile_fail` and `no_run`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0063`.
error: could not compile `compiletest`
