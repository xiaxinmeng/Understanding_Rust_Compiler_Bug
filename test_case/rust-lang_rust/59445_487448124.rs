plain
travis_time:end:00f6016e:start=1556507286783688122,finish=1556507287671260336,duration=887572214
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:11:49] .................................................................................................... 3000/5477
[01:11:52] .................................................................................................... 3100/5477
[01:11:57] .................................................................................................... 3200/5477
[01:12:00] ....................................................................i............................... 3300/5477
[01:12:04] .........................................................................F.......................... 3400/5477
[01:12:12] .................................................................................................... 3600/5477
[01:12:15] .................................................................................................... 3700/5477
[01:12:18] .................................................ii................................................. 3800/5477
[01:12:21] ...................................................................i................................ 3900/5477
[01:12:21] ...................................................................i................................ 3900/5477
[01:12:23] .................................................................................................... 4000/5477
[01:12:25] ...........................i........................................................................ 4100/5477
[01:12:28] .................F.................................................................................. 4200/5477
[01:12:43] .................................................................................................... 4400/5477
[01:12:46] .................................................................................................... 4500/5477
[01:12:50] .................................................................................................... 4600/5477
[01:12:56] .................................................................................................... 4700/5477
[01:12:56] .................................................................................................... 4700/5477
[01:13:00] .................................................................................................... 4800/5477
[01:13:03] .................................................................................................... 4900/5477
[01:13:08] .................................................................................................... 5000/5477
[01:13:12] .............................F..............................................................F....... 5100/5477
[01:13:19] .................................................................................................... 5300/5477
[01:13:21] .................................................................................................... 5400/5477
[01:13:24] ...............i.............................................................
[01:13:24] failures:
[01:13:24] failures:
[01:13:24] 
[01:13:24] ---- [ui] ui/maybe-bounds.rs stdout ----
[01:13:24] diff of stderr:
[01:13:24] 
[01:13:24] 6    |
[01:13:24] 7    = note: traits are `?Sized` by default
[01:13:24] 8 
[01:13:24] - error: `?Trait` is not permitted in trait object types
[01:13:24] -   --> $DIR/maybe-bounds.rs:3:16
[01:13:24] -    |
[01:13:24] - LL | type A1 = Tr + (?Sized);
[01:13:24] - 
[01:13:24] - 
[01:13:24] - error: `?Trait` is not permitted in trait object types
[01:13:24] -   --> $DIR/maybe-bounds.rs:4:24
[01:13:24] -    |
[01:13:24] - LL | type A2 = for<'a> Tr + (?Sized);
[01:13:24] - 
[01:13:24] - error: aborting due to 3 previous errors
[01:13:24] + error: aborting due to previous error
[01:13:24] 22 
[01:13:24] 22 
[01:13:24] 23 
[01:13:24] 
[01:13:24] 
[01:13:24] The actual stderr differed from the expected stderr.
[01:13:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/maybe-bounds/maybe-bounds.stderr
[01:13:24] To update references, rerun the tests and pass the `--bless` flag
[01:13:24] To only update this specific test, also pass `--test-args maybe-bounds.rs`
[01:13:24] error: 1 errors occurred comparing output.
[01:13:24] status: exit code: 1
[01:13:24] status: exit code: 1
[01:13:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/maybe-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/maybe-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/maybe-bounds/auxiliary" "-A" "unused"
[01:13:24] ------------------------------------------
[01:13:24] 
[01:13:24] ------------------------------------------
[01:13:24] stderr:
[01:13:24] stderr:
[01:13:24] ------------------------------------------
[01:13:24] error: `?Trait` is not permitted in supertraits
[01:13:24]    |
[01:13:24]    |
[01:13:24] LL | trait Tr: ?Sized {} //~ ERROR `?Trait` is not permitted in supertraits
[01:13:24]    |
[01:13:24]    |
[01:13:24]    = note: traits are `?Sized` by default
[01:13:24] error: aborting due to previous error
[01:13:24] 
[01:13:24] 
[01:13:24] ------------------------------------------
[01:13:24] ------------------------------------------
[01:13:24] 
[01:13:24] 
[01:13:24] ---- [ui] ui/parser/trait-object-trait-parens.rs stdout ----
[01:13:24] diff of stderr:
[01:13:24] 
[01:13:24] - error: `?Trait` is not permitted in trait object types
[01:13:24] -    |
[01:13:24] -    |
[01:13:24] - LL |     let _: Box<(Copy) + (?Sized) + (for<'a> Trait<'a>)>;
[01:13:24] - 
[01:13:24] - 
[01:13:24] - error: `?Trait` is not permitted in trait object types
[01:13:24] -    |
[01:13:24] -    |
[01:13:24] - LL |     let _: Box<(for<'a> Trait<'a>) + (Copy) + (?Sized)>;
[01:13:24] - 
[01:13:24] 13 error[E0261]: use of undeclared lifetime name `'a`
[01:13:24] 14   --> $DIR/trait-object-trait-parens.rs:9:31
[01:13:24] 15    |
[01:13:24] 15    |
[01:13:24] 
[01:13:24] 16 LL |     let _: Box<(for<'a> Trait<'a>) + (Copy) + (?Sized)>;
[01:13:24] 18 
[01:13:24] - error: aborting due to 3 previous errors
[01:13:24] + error: aborting due to previous error
[01:13:24] 20 
[01:13:24] 20 
[01:13:24] 21 For more information about this error, try `rustc --explain E0261`.
[01:13:24] 22 
[01:13:24] 
[01:13:24] 
[01:13:24] The actual stderr differed from the expected stderr.
[01:13:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-trait-parens/trait-object-trait-parens.stderr
[01:13:24] To update references, rerun the tests and pass the `--bless` flag
[01:13:24] To only update this specific test, also pass `--test-args parser/trait-object-trait-parens.rs`
[01:13:24] error: 1 errors occurred comparing output.
[01:13:24] status: exit code: 1
[01:13:24] status: exit code: 1
[01:13:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/trait-object-trait-parens.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-trait-parens/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-trait-parens/auxiliary" "-A" "unused"
[01:13:24] ------------------------------------------
[01:13:24] 
[01:13:24] ------------------------------------------
[01:13:24] stderr:
[01:13:24] stderr:
[01:13:24] ------------------------------------------
[01:13:24] error[E0261]: use of undeclared lifetime name `'a`
[01:13:24]   --> /checkout/src/test/ui/parser/trait-object-trait-parens.rs:9:31
[01:13:24]    |
[01:13:24] LL |     let _: Box<(for<'a> Trait<'a>) + (Copy) + (?Sized)>;
[01:13:24] 
[01:13:24] error: aborting due to previous error
[01:13:24] 
[01:13:24] For more information about this error, try `rustc --explain E0261`.
---
[01:13:24] normalized stderr:
[01:13:24] error[E0224]: at least one non-builtin trait is required for an object type
[01:13:24]   --> $DIR/trait-alias-maybe-bound.rs:12:12
[01:13:24]    |
[01:13:24] LL | type _T0 = dyn _1;
[01:13:24] 
[01:13:24] error[E0224]: at least one non-builtin trait is required for an object type
[01:13:24]   --> $DIR/trait-alias-maybe-bound.rs:24:12
[01:13:24]    |
[01:13:24]    |
[01:13:24] LL | type _T3 = dyn _2;
[01:13:24] 
[01:13:24] error: aborting due to 2 previous errors
[01:13:24] 
[01:13:24] 
[01:13:24] 
[01:13:24] 
[01:13:24] 
[01:13:24] The actual stderr differed from the expected stderr.
[01:13:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-maybe-bound/trait-alias-maybe-bound.stderr
[01:13:24] To update references, rerun the tests and pass the `--bless` flag
[01:13:24] To only update this specific test, also pass `--test-args traits/trait-alias/trait-alias-maybe-bound.rs`
[01:13:24] error: 1 errors occurred comparing output.
[01:13:24] status: exit code: 1
[01:13:24] status: exit code: 1
[01:13:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias/trait-alias-maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-maybe-bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-maybe-bound/auxiliary" "-A" "unused"
[01:13:24] ------------------------------------------
[01:13:24] 
[01:13:24] ------------------------------------------
[01:13:24] stderr:
[01:13:24] stderr:
[01:13:24] ------------------------------------------
[01:13:24] error[E0224]: at least one non-builtin trait is required for an object type
[01:13:24]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-maybe-bound.rs:12:12
[01:13:24]    |
[01:13:24] LL | type _T0 = dyn _1;
[01:13:24] 
[01:13:24] error[E0224]: at least one non-builtin trait is required for an object type
[01:13:24]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-maybe-bound.rs:24:12
[01:13:24]    |
[01:13:24]    |
[01:13:24] LL | type _T3 = dyn _2;
[01:13:24] 
[01:13:24] error: aborting due to 2 previous errors
[01:13:24] 
[01:13:24] 
---
[01:13:24] normalized stderr:
[01:13:24] error[E0224]: at least one non-builtin trait is required for an object type
[01:13:24]   --> $DIR/wf-trait-object-maybe-bound.rs:4:11
[01:13:24]    |
[01:13:24] LL | type _0 = dyn ?Sized;
[01:13:24] 
[01:13:24] error: aborting due to previous error
[01:13:24] 
[01:13:24] 
[01:13:24] 
[01:13:24] 
[01:13:24] 
[01:13:24] The actual stderr differed from the expected stderr.
[01:13:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-maybe-bound/wf-trait-object-maybe-bound.stderr
[01:13:24] To update references, rerun the tests and pass the `--bless` flag
[01:13:24] To only update this specific test, also pass `--test-args traits/wf-trait-object-maybe-bound.rs`
[01:13:24] error: 1 errors occurred comparing output.
[01:13:24] status: exit code: 1
[01:13:24] status: exit code: 1
[01:13:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/wf-trait-object-maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-maybe-bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-maybe-bound/auxiliary" "-A" "unused"
[01:13:24] ------------------------------------------
[01:13:24] 
[01:13:24] ------------------------------------------
[01:13:24] stderr:
[01:13:24] stderr:
[01:13:24] ------------------------------------------
[01:13:24] error[E0224]: at least one non-builtin trait is required for an object type
[01:13:24]   --> /checkout/src/test/ui/traits/wf-trait-object-maybe-bound.rs:4:11
[01:13:24]    |
[01:13:24] LL | type _0 = dyn ?Sized;
[01:13:24] 
[01:13:24] error: aborting due to previous error
[01:13:24] 
[01:13:24] 
---
[01:13:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:13:24] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:24] 
[01:13:24] 
[01:13:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:24] 
[01:13:24] 
[01:13:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:24] Build completed unsuccessfully in 0:04:25
[01:13:24] Build completed unsuccessfully in 0:04:25
[01:13:24] make: *** [check] Error 1
[01:13:24] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00712320
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 29 04:21:43 UTC 2019
