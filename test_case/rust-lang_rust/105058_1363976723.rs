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
   Compiling ignore v0.4.18
   Compiling cargo-platform v0.1.2
   Compiling cargo_metadata v0.14.0
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
error: unused `Result` in tuple element 1 that must be used
  ::: src/tools/tidy/src/no_merge.rs:22:5
   |
   |
22 |       dbg!(output.status, String::from_utf8(output.stderr), String::from_utf8(output.stdout),);
   |
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `-D unused-must-use` implied by `-D warnings`

error: unused `Result` in tuple element 2 that must be used
  ::: src/tools/tidy/src/no_merge.rs:22:5
   |
   |
22 |       dbg!(output.status, String::from_utf8(output.stderr), String::from_utf8(output.stdout),);
   |
   |
   = note: this `Result` may be an `Err` variant, which should be handled
error: could not compile `tidy` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:11
