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
    Finished release [optimized] target(s) in 8.77s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
/checkout/src/test/ui/layout/thumb-enum.rs: revision [unspecified] should specify `needs-llvm-components:` as it has `--target` set
* highest error code: E0784
Found 500 error codes
Found 0 error codes with no tests
Done!
Done!
* 346 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:16
