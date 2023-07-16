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
    Finished release [optimized] target(s) in 11.33s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: /checkout/library/core/src/convert/num.rs:49: tab character
tidy error: /checkout/library/core/src/convert/num.rs:50: tab character
tidy error: /checkout/library/core/src/convert/num.rs:51: tab character
tidy error: /checkout/library/core/src/convert/num.rs:389: tab character
tidy error: /checkout/library/core/src/convert/num.rs:390: tab character
tidy error: /checkout/library/core/src/convert/num.rs:391: tab character
tidy error: /checkout/library/core/src/convert/num.rs:460: tab character
tidy error: /checkout/library/core/src/convert/num.rs:461: tab character
tidy error: /checkout/library/core/src/convert/num.rs:462: tab character
tidy error: /checkout/library/core/src/convert/num.rs:501: tab character
tidy error: /checkout/library/core/src/convert/num.rs:502: tab character
tidy error: /checkout/library/core/src/convert/num.rs:503: tab character
* highest error code: E0781
Found 516 error codes
Found 0 error codes with no tests
Done!
Done!
* 326 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:13
