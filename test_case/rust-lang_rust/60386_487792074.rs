plain
travis_time:end:0ad9ec0c:start=1556581155141154719,finish=1556581234140342218,duration=78999187499
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:07:36] .................................................................................................... 2300/5470
[01:07:39] .................................................................................................... 2400/5470
[01:07:44] .................................................................................................... 2500/5470
[01:07:47] .................................................................................................... 2600/5470
[01:07:51] .....................F.............................................................................. 2700/5470
[01:07:59] .................................................................................................... 2900/5470
[01:08:03] .................................................................................................... 3000/5470
[01:08:06] .................................................................................................... 3100/5470
[01:08:06] .................................................................................................... 3100/5470
[01:08:10] ...F................................................................................................ 3200/5470
[01:08:17] .................................................................................................... 3400/5470
[01:08:20] .............................................ii...i..ii............................................. 3500/5470
[01:08:24] .................................................................................................... 3600/5470
[01:08:28] .................................................................................................... 3700/5470
---
[01:08:51] .................................................................................................... 4300/5470
[01:08:54] .................................................................................................... 4400/5470
[01:08:57] .................................................................................................... 4500/5470
[01:09:01] .................................................................................................... 4600/5470
[01:09:07] ........................................................................F........................... 4700/5470
[01:09:14] .................................................................................................... 4900/5470
[01:09:19] .................................................................................................... 5000/5470
[01:09:22] .................................................................................................... 5100/5470
[01:09:25] .................................................................................................... 5200/5470
---
[01:09:33] 1 error[E0599]: no method named `exec` found for type `&mut std::process::Command` in the current scope
[01:09:33] -   --> $DIR/issue-39175.rs:14:39
[01:09:33] +   --> $DIR/issue-39175.rs:15:39
[01:09:33] 3    |
[01:09:33] 4 LL |     Command::new("echo").arg("hello").exec();
[01:09:33] 
[01:09:33] 
[01:09:33] The actual stderr differed from the expected stderr.
[01:09:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/issue-39175.stderr
[01:09:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/issue-39175.stderr
[01:09:33] To update references, rerun the tests and pass the `--bless` flag
[01:09:33] To only update this specific test, also pass `--test-args issues/issue-39175.rs`
[01:09:33] error: 1 errors occurred comparing output.
[01:09:33] status: exit code: 1
[01:09:33] status: exit code: 1
[01:09:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39175.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/auxiliary" "-A" "unused"
[01:09:33] ------------------------------------------
[01:09:33] 
[01:09:33] ------------------------------------------
[01:09:33] stderr:
[01:09:33] stderr:
[01:09:33] ------------------------------------------
[01:09:33] error[E0599]: no method named `exec` found for type `&mut std::process::Command` in the current scope
[01:09:33]   --> /checkout/src/test/ui/issues/issue-39175.rs:15:39
[01:09:33]    |
[01:09:33] LL |     Command::new("echo").arg("hello").exec();
[01:09:33]    |
[01:09:33]    = help: items from traits can only be used if the trait is in scope
[01:09:33] help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:09:33]    |
---
[01:09:33] 
[01:09:33] ---- [ui] ui/linkage2.rs stdout ----
[01:09:33] diff of stderr:
[01:09:33] 
[01:09:33] 1 error: must have type `*const T` or `*mut T`
[01:09:33] +   --> $DIR/linkage2.rs:9:32
[01:09:33] 3    |
[01:09:33] 3    |
[01:09:33] 4 LL |     #[linkage = "extern_weak"] static foo: i32;
[01:09:33] 
[01:09:33] 
[01:09:33] The actual stderr differed from the expected stderr.
[01:09:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage2/linkage2.stderr
[01:09:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage2/linkage2.stderr
[01:09:33] To update references, rerun the tests and pass the `--bless` flag
[01:09:33] To only update this specific test, also pass `--test-args linkage2.rs`
[01:09:33] error: 1 errors occurred comparing output.
[01:09:33] status: exit code: 1
[01:09:33] status: exit code: 1
[01:09:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/linkage2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage2/auxiliary" "-A" "unused"
[01:09:33] ------------------------------------------
[01:09:33] 
[01:09:33] ------------------------------------------
[01:09:33] stderr:
[01:09:33] stderr:
[01:09:33] ------------------------------------------
[01:09:33] error: must have type `*const T` or `*mut T`
[01:09:33]    |
[01:09:33]    |
[01:09:33] LL |     #[linkage = "extern_weak"] static foo: i32;
[01:09:33] 
[01:09:33] error: aborting due to previous error
[01:09:33] 
[01:09:33] 
---
[01:09:33] ------------------------------------------
[01:09:33] stderr:
[01:09:33] ------------------------------------------
[01:09:33] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:09:33]   left: `[":22] Unit = Unit", ":23] a = Unit", ":29] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24,", "}", ":30] b = Point {", "    x: 42,", "    y: 24,", "}", ":38]", ":42] &a = NoCopy(", "    1337,", ")", ":42] dbg!(& a) = NoCopy(", "    1337,", ")", ":47] f(&42) = 42", "before", ":52] { foo += 1; eprintln!(\"before\"); 7331 } = 7331", ":60] (\"Yeah\",) = (", "    \"Yeah\",", ")", ":63] 1 = 1", ":63] 2 = 2", ":67] 1u8 = 1", ":67] 2u32 = 2", ":67] \"Yeah\" = \"Yeah\""]`,
[01:09:33]  right: `[":21] Unit = Unit", ":22] a = Unit", ":28] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24,", "}", ":29] b = Point {", "    x: 42,", "    y: 24,", "}", ":37]", ":41] &a = NoCopy(", "    1337,", ")", ":41] dbg!(& a) = NoCopy(", "    1337,", ")", ":46] f(&42) = 42", "before", ":51] { foo += 1; eprintln!(\"before\"); 7331 } = 7331", ":59] (\"Yeah\",) = (", "    \"Yeah\",", ")", ":62] 1 = 1", ":62] 2 = 2", ":66] 1u8 = 1", ":66] 2u32 = 2", ":66] \"Yeah\" = \"Yeah\""]`', /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:72:5
[01:09:33] 
[01:09:33] ------------------------------------------
[01:09:33] 
[01:09:33] 
---
[01:09:33] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:09:33] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:09:33] 
[01:09:33] 
[01:09:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:33] 
[01:09:33] 
[01:09:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:33] Build completed unsuccessfully in 0:04:13
[01:09:33] Build completed unsuccessfully in 0:04:13
[01:09:33] make: *** [check] Error 1
[01:09:33] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1a035d80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 00:50:16 UTC 2019
