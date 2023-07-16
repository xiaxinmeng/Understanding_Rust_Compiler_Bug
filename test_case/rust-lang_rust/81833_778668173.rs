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
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 10.43s
tidy check
Checking which error codes lack tests...
thread '<unnamed>' panicked at 'Deleted temp file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/tidy/src/bins.rs:43:46
* 624 error codes
* highest error code: E0781
Found 436 error codes
Found 0 error codes with no tests
Found 0 error codes with no tests
Done!
* 319 features
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any', src/tools/tidy/src/main.rs:103:6


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:13
