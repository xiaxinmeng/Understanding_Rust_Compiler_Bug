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
   |
70 | mod test;
   | ^^^^^^^^^
   |
   = help: to create the module `test`, create file "compiler/rustc_errors/src/test.rs" or "compiler/rustc_errors/src/test/mod.rs"
For more information about this error, try `rustc --explain E0583`.
error: could not compile `rustc_errors` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:07
