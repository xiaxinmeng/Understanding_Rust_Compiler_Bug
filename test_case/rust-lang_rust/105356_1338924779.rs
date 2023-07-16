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
    Finished release [optimized] target(s) in 8.51s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: the following output file is not associated with any mir-opt test, you can remove it: /checkout/src/test/mir-opt/building/custom/enums.unwrap.built.after.mir
tidy error: the following output file is not associated with any mir-opt test, you can remove it: /checkout/src/test/mir-opt/building/custom/enums.unwrap_deref.built.after.mir
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
Done!
