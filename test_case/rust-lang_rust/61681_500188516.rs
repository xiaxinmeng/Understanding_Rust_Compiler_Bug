plain
travis_time:end:07be68ae:start=1560057708277261204,finish=1560057796193974615,duration=87916713411
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:59:01] .................................................................................................... 4900/5666
[00:59:05] .................................................................................................... 5000/5666
[00:59:08] .................................................................................................... 5100/5666
[00:59:13] .................................................................................................... 5200/5666
[00:59:16] .....................F..............................FF.F...........................F................ 5300/5666
[00:59:23] .................................................................................................... 5500/5666
[00:59:26] .................................................................................................... 5600/5666
[00:59:28] ....i.............................................................
[00:59:28] failures:
[00:59:28] failures:
[00:59:28] 
[00:59:28] ---- [ui] ui/traits/trait-alias/trait-alias-only-maybe-bound.rs stdout ----
[00:59:28] diff of stderr:
[00:59:28] 
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] + error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28] 3    |
[00:59:28] 3    |
[00:59:28] 4 LL | type _T0 = dyn _1;
[00:59:28] 5    |            ^^^^^^
[00:59:28] 6 
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] + error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28] 9    |
[00:59:28] 9    |
[00:59:28] 10 LL | type _T1 = dyn _2;
[00:59:28] 
[00:59:28] The actual stderr differed from the expected stderr.
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-only-maybe-bound/trait-alias-only-maybe-bound.stderr
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-only-maybe-bound/trait-alias-only-maybe-bound.stderr
[00:59:28] To update references, rerun the tests and pass the `--bless` flag
[00:59:28] To only update this specific test, also pass `--test-args traits/trait-alias/trait-alias-only-maybe-bound.rs`
[00:59:28] error: 1 errors occurred comparing output.
[00:59:28] status: exit code: 1
[00:59:28] status: exit code: 1
[00:59:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias/trait-alias-only-maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-only-maybe-bound" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-only-maybe-bound/auxiliary" "-A" "unused"
[00:59:28] ------------------------------------------
[00:59:28] 
[00:59:28] ------------------------------------------
[00:59:28] stderr:
[00:59:28] stderr:
[00:59:28] ------------------------------------------
[00:59:28] error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL | type _T0 = dyn _1;
[00:59:28] 
[00:59:28] 
[00:59:28] error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL | type _T1 = dyn _2;
[00:59:28] 
[00:59:28] error: aborting due to 2 previous errors
[00:59:28] 
[00:59:28] 
[00:59:28] 
[00:59:28] ------------------------------------------
[00:59:28] 
[00:59:28] 
[00:59:28] ---- [ui] ui/traits/trait-object-macro-matcher.rs stdout ----
[00:59:28] diff of stderr:
[00:59:28] 
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] + error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28] 3    |
[00:59:28] 3    |
[00:59:28] 4 LL |     m!(dyn 'static +);
[00:59:28] 
[00:59:28] The actual stderr differed from the expected stderr.
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-macro-matcher/trait-object-macro-matcher.stderr
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-macro-matcher/trait-object-macro-matcher.stderr
[00:59:28] To update references, rerun the tests and pass the `--bless` flag
[00:59:28] To only update this specific test, also pass `--test-args traits/trait-object-macro-matcher.rs`
[00:59:28] error: 1 errors occurred comparing output.
[00:59:28] status: exit code: 1
[00:59:28] status: exit code: 1
[00:59:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-macro-matcher.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-macro-matcher" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-macro-matcher/auxiliary" "-A" "unused"
[00:59:28] ------------------------------------------
[00:59:28] 
[00:59:28] ------------------------------------------
[00:59:28] stderr:
[00:59:28] stderr:
[00:59:28] ------------------------------------------
[00:59:28] error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL |     m!(dyn 'static +); //~ ERROR at least one non-builtin trait is required for an object type
[00:59:28] 
[00:59:28] error[E0038]: the trait `std::marker::Copy` cannot be made into an object
[00:59:28]   --> /checkout/src/test/ui/traits/trait-object-macro-matcher.rs:8:8
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL |     m!(dyn Copy + Send + 'static);
[00:59:28]    |        ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` cannot be made into an object
[00:59:28]    = note: the trait cannot require that `Self : Sized`
[00:59:28] 
[00:59:28] error: aborting due to 2 previous errors
[00:59:28] 
---
[00:59:28] 
[00:59:28] ---- [ui] ui/traits/trait-object-vs-lifetime-2.rs stdout ----
[00:59:28] diff of stderr:
[00:59:28] 
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] + error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28] 3    |
[00:59:28] 3    |
[00:59:28] 4 LL |     dyn 'static +: 'static + Copy,
[00:59:28] 
[00:59:28] The actual stderr differed from the expected stderr.
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime-2/trait-object-vs-lifetime-2.stderr
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime-2/trait-object-vs-lifetime-2.stderr
[00:59:28] To update references, rerun the tests and pass the `--bless` flag
[00:59:28] To only update this specific test, also pass `--test-args traits/trait-object-vs-lifetime-2.rs`
[00:59:28] error: 1 errors occurred comparing output.
[00:59:28] status: exit code: 1
[00:59:28] status: exit code: 1
[00:59:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-vs-lifetime-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime-2/auxiliary" "-A" "unused"
[00:59:28] ------------------------------------------
[00:59:28] 
[00:59:28] ------------------------------------------
[00:59:28] stderr:
[00:59:28] stderr:
[00:59:28] ------------------------------------------
[00:59:28] error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL |     dyn 'static +: 'static + Copy,
[00:59:28] 
[00:59:28] error: aborting due to previous error
[00:59:28] 
[00:59:28] 
[00:59:28] 
[00:59:28] ------------------------------------------
[00:59:28] 
[00:59:28] 
[00:59:28] ---- [ui] ui/traits/trait-object-vs-lifetime.rs stdout ----
[00:59:28] diff of stderr:
[00:59:28] 
[00:59:28] 4 LL |     let _: S<dyn 'static +, 'static>;
[00:59:28] 6 
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] + error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28] 9    |
[00:59:28] 9    |
[00:59:28] 10 LL |     let _: S<'static, dyn 'static +>;
[00:59:28] 
[00:59:28] 22 LL |     let _: S<'static, 'static>;
[00:59:28] 24 
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] + error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28] 27    |
[00:59:28] 27    |
[00:59:28] 28 LL |     let _: S<dyn 'static +, 'static>;
[00:59:28] 
[00:59:28] The actual stderr differed from the expected stderr.
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime/trait-object-vs-lifetime.stderr
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime/trait-object-vs-lifetime.stderr
[00:59:28] To update references, rerun the tests and pass the `--bless` flag
[00:59:28] To only update this specific test, also pass `--test-args traits/trait-object-vs-lifetime.rs`
[00:59:28] error: 1 errors occurred comparing output.
[00:59:28] status: exit code: 1
[00:59:28] status: exit code: 1
[00:59:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-vs-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime/auxiliary" "-A" "unused"
[00:59:28] ------------------------------------------
[00:59:28] 
[00:59:28] ------------------------------------------
[00:59:28] stderr:
[00:59:28] stderr:
[00:59:28] ------------------------------------------
[00:59:28] error: lifetime arguments must be declared prior to type arguments
[00:59:28]   --> /checkout/src/test/ui/traits/trait-object-vs-lifetime.rs:14:29
[00:59:28]    |
[00:59:28] LL |     let _: S<dyn 'static +, 'static>;
[00:59:28] 
[00:59:28] 
[00:59:28] error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL |     let _: S<'static, dyn 'static +>;
[00:59:28] 
[00:59:28] error[E0107]: wrong number of lifetime arguments: expected 1, found 2
[00:59:28]   --> /checkout/src/test/ui/traits/trait-object-vs-lifetime.rs:11:23
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL |     let _: S<'static, 'static>;
[00:59:28] 
[00:59:28] error[E0107]: wrong number of type arguments: expected 1, found 0
[00:59:28]   --> /checkout/src/test/ui/traits/trait-object-vs-lifetime.rs:11:12
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL |     let _: S<'static, 'static>;
[00:59:28] 
[00:59:28] 
[00:59:28] error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL |     let _: S<dyn 'static +, 'static>;
[00:59:28] 
[00:59:28] error: aborting due to 5 previous errors
[00:59:28] 
[00:59:28] For more information about this error, try `rustc --explain E0107`.
[00:59:28] For more information about this error, try `rustc --explain E0107`.
[00:59:28] 
[00:59:28] ------------------------------------------
[00:59:28] 
[00:59:28] 
[00:59:28] ---- [ui] ui/traits/wf-trait-object-only-maybe-bound.rs stdout ----
[00:59:28] diff of stderr:
[00:59:28] 
[00:59:28] 4 LL | type _0 = dyn ?Sized;
[00:59:28] 6 
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] - error[E0224]: at least one non-builtin trait is required for an object type
[00:59:28] + error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28] 8   --> $DIR/wf-trait-object-only-maybe-bound.rs:3:11
[00:59:28] 9    |
[00:59:28] 10 LL | type _0 = dyn ?Sized;
[00:59:28] 
[00:59:28] The actual stderr differed from the expected stderr.
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-only-maybe-bound/wf-trait-object-only-maybe-bound.stderr
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-only-maybe-bound/wf-trait-object-only-maybe-bound.stderr
[00:59:28] To update references, rerun the tests and pass the `--bless` flag
[00:59:28] To only update this specific test, also pass `--test-args traits/wf-trait-object-only-maybe-bound.rs`
[00:59:28] error: 1 errors occurred comparing output.
[00:59:28] status: exit code: 1
[00:59:28] status: exit code: 1
[00:59:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/wf-trait-object-only-maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-only-maybe-bound" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-only-maybe-bound/auxiliary" "-A" "unused"
[00:59:28] ------------------------------------------
[00:59:28] 
[00:59:28] ------------------------------------------
[00:59:28] stderr:
[00:59:28] stderr:
[00:59:28] ------------------------------------------
[00:59:28] error: `?Trait` is not permitted in trait object types
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL | type _0 = dyn ?Sized;
[00:59:28] 
[00:59:28] 
[00:59:28] error[E0224]: at least one non-builtin trait, outside of &send, is required for an object type
[00:59:28]    |
[00:59:28]    |
[00:59:28] LL | type _0 = dyn ?Sized;
[00:59:28] 
[00:59:28] error: aborting due to 2 previous errors
[00:59:28] 
[00:59:28] 
---
[00:59:28] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:59:28] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:28] 
[00:59:28] 
[00:59:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:28] 
[00:59:28] 
[00:59:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:28] Build completed unsuccessfully in 0:55:20
---
travis_time:end:007936f2:start=1560061374717631045,finish=1560061374722580360,duration=4949315
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:229aca18
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:276c84c2
travis_time:start:276c84c2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bf3e060
$ dmesg | grep -i kill
