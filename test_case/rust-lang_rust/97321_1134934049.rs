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
    Finished release [optimized] target(s) in 8.82s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: library/{core_name} and library/{std_name} have different contents
* 629 error codes
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
