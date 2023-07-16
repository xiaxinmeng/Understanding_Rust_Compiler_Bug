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
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
error: unused variable: `filestr`
  --> src/tools/tidy/src/edition.rs:21:17
   |
21 |             let filestr = file.to_string_lossy().replace("\\", "/");
   |                 ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_filestr`
   |
   = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `tidy` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:00:08
