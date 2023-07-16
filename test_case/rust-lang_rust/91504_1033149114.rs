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
    Finished release [optimized] target(s) in 9.64s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: following path contains more than 982 entries, you should move the test to some relevant subdirectory (current: 985): /checkout/src/test/ui
* 629 error codes
* highest error code: E0787
Found 503 error codes
Found 0 error codes with no tests
Found 0 error codes with no tests
Done!
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: Zero }', src/tools/tidy/src/features.rs:387:32
Build completed unsuccessfully in 0:00:12
