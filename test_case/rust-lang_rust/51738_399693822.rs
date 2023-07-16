plain
[00:03:28]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:03:41]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:03:41]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:03:41]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:41] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`
[00:03:41]    --> liballoc/boxed.rs:431:1
[00:03:41]     |
[00:03:41] 429 |     /// This conversion is more expensive because the <T> is not already on the heap.
[00:03:41]     |                                                                                      - expected one of 8 possible tokens here
[00:03:41] 431 | }
[00:03:41] 431 | }
[00:03:41]     | ^ unexpected token
[00:03:41] 
[00:03:41] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`
[00:03:41]    --> liballoc/boxed.rs:448:1
[00:03:41]     |
[00:03:41] 446 |     /// and the pointer moves along
[00:03:41]     |                                    - expected one of 8 possible tokens here
[00:03:41] 448 | }
[00:03:41] 448 | }
[00:03:41]     | ^ unexpected token
[00:03:41] 
[00:03:41] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`
[00:03:41]    --> liballoc/boxed.rs:466:1
[00:03:41] 463 |     /// on the heap.
[00:03:41] 463 |     /// on the heap.
[00:03:41]     |                     - expected one of 8 possible tokens here
[00:03:41] 466 | }
[00:03:41] 466 | }
[00:03:41]     | ^ unexpected token
[00:03:41] 
[00:03:41] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`
[00:03:41]    --> liballoc/boxed.rs:482:1
[00:03:41]     |
[00:03:41] 481 |     /// This function is unsafe. Data can't be moved from this reference.
[00:03:41]     |                                                                          - expected one of 8 possible tokens here
[00:03:41] 482 | }
[00:03:41]     | ^ unexpected token
[00:03:41] 
[00:03:41] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`
[00:03:41]    --> liballoc/boxed.rs:904:1
[00:03:41] 903 |     /// heap.
[00:03:41] 903 |     /// heap.
[00:03:41]     |              - expected one of 8 possible tokens here
[00:03:41] 904 | }
[00:03:41]     | ^ unexpected token
[00:03:41] 
[00:03:41] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`
[00:03:41]    --> liballoc/boxed.rs:916:1
[00:03:41] 915 |     /// heap.
[00:03:41] 915 |     /// heap.
[00:03:41]     |              - expected one of 8 possible tokens here
[00:03:41] 916 | }
[00:03:41]     | ^ unexpected token
[00:03:41] 
[00:03:41] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`
[00:03:41]     --> liballoc/boxed.rs:1012:1
[00:03:41]      |
[00:03:41] 1010 |     /// This conversion is cheaper because the PinBox is already on the heap.
[00:03:41]      |                                                                              - expected one of 8 possible tokens here
[00:03:41] 1012 | }
[00:03:41] 1012 | }
[00:03:41]      | ^ unexpected token
[00:03:41] 
[00:03:41] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`
[00:03:41]     --> liballoc/boxed.rs:1024:1
[00:03:41]      |
[00:03:41] 1022 |     /// This conversion is cheaper because the Box is already on the heap.
[00:03:41]      |                                                                           - expected one of 8 possible tokens here
[00:03:41] 1024 | }
[00:03:41] 1024 | }
[00:03:41]      | ^ unexpected token
[00:03:42]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:42]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:43] error: aborting due to 8 previous errors
[00:03:43] 
[00:03:43] 
[00:03:43] error: Could not compile `alloc`.
[00:03:43] 
[00:03:43] Caused by:
[00:03:43]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=d6cd5f8b78fddf12 -C extra-filename=-d6cd5f8b78fddf12 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-e9cdce497aae9e81.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-90a13cda2e54742f.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-ffd422941bf53e42/out` (exit code: 101)
[00:03:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:43] expected success, got: exit code: 101
[00:03:43] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:43] travis_fold:end:stage0-std

[00:03:43] travis_time:end:stage0-std:start=1529773670313722367,finish=1529773700848827912,duration=30535105545


[00:03:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:43] Build completed unsuccessfully in 0:00:31
[00:03:43] Makefile:79: recipe for target 'tidy' failed
[00:03:43] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0db9de90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00124c06:start=1529773701354458889,finish=1529773701362190494,duration=7731605
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:003dbd86
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:089aadc4
$ dmesg | grep -i kill
