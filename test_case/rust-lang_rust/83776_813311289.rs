plain
[TIMING] Tidy { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.000
tidy check
Checking which error codes lack tests...
* 623 error codes
thread '<unnamed>' panicked at 'File::open(entry.path()) failed on /checkout/obj/build/tmp/distcheck/src/tidy-test-file with No such file or directory (os error 2)', src/tools/tidy/src/lib.rs:88:12
* highest error code: E0781
* 324 features
* 324 features
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any', src/tools/tidy/src/main.rs:108:6


command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout/obj/build/tmp/distcheck" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/tmp/distcheck/build" "16"


failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test --stage 2
Build completed unsuccessfully in 0:01:08
Build completed unsuccessfully in 0:01:08
make: *** [check] Error 1
Makefile:42: recipe for target 'check' failed

command did not execute successfully: "make" "check"
expected success, got: exit code: 2

