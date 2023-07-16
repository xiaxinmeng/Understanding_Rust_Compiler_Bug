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
error: removing an expression is not supported in this position
   --> src/tools/compiletest/src/header.rs:883:17
    |
883 |                 #[cfg(not(bootstrap))]

error[E0425]: cannot find value `ignore_message` in this scope
   --> src/tools/compiletest/src/header.rs:884:17
    |
    |
884 |                 ignore_message = config.parse_name_value_directive(ln, "ignore");

For more information about this error, try `rustc --explain E0425`.
error: could not compile `compiletest` due to 2 previous errors

