plain
[00:17:31]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:17:31]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:17:31]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:17:55]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:17:55] error: expected one of `.`, `;`, `?`, or an operator, found `if`
[00:17:55]     |
[00:17:55] 347 |                 }
[00:17:55] 347 |                 }
[00:17:55]     |                  - expected one of `.`, `;`, `?`, or an operator here
[00:17:55] ...
[00:17:55] 352 |                 if e.span.ctxt().outer().expn_info()
[00:17:55]     |                 ^^ unexpected token
[00:17:56]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:17:57] error: aborting due to previous error
[00:17:57] 
[00:17:57] error: Could not compile `rustc_lint`.
[00:17:57] error: Could not compile `rustc_lint`.
[00:17:57] warning: build failed, waiting for other jobs to finish...
[00:18:17] error: build failed
[00:18:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:17] expected success, got: exit code: 101
[00:18:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:18:17] travis_fold:end:stage0-rustc

[00:18:17] travis_time:end:stage0-rustc:start=1538498205618374939,finish=1538498984837921510,duration=779219546571


[00:18:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:17] Build completed unsuccessfully in 0:13:55
[00:18:17] Makefile:28: recipe for target 'all' failed
[00:18:17] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:091a445c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:04a80a85:start=1538498985674509624,finish=1538498985679749402,duration=5239778
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:251f6a37
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2c2291dd
travis_time:start:2c2291dd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
travis_time:end:2c2291dd:start=1538498985700383398,finish=1538498985705587315,duration=5203917
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c875cc4
