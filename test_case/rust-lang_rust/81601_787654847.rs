plain
    Finished release [optimized] target(s) in 1m 08s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] StdLink { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target_compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.001
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 68.649
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/bootstrap/native.rs:123:43
failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test --stage 2
Build completed unsuccessfully in 0:02:19
Build completed unsuccessfully in 0:02:19
make: *** [check] Error 1
Makefile:42: recipe for target 'check' failed

command did not execute successfully: "make" "check"
expected success, got: exit code: 2

