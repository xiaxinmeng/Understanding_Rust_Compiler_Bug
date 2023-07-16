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
    Finished release [optimized] target(s) in 13.69s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: following path contains more than 939 entries, you should move the test to some relevant subdirectory (current: 940): /checkout/src/test/ui
* highest error code: E0791
* highest error code: E0791
/checkout/src/test/ui/deployment-target/invalid-target.rs: revision [unspecified] should specify `needs-llvm-components:` as it has `--target` set
Found 0 error(s) in error codes
Done!
* 392 features
some tidy checks failed
