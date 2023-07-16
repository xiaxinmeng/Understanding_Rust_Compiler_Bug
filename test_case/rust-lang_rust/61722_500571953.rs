plain
travis_time:end:05d429da:start=1560192817584885527,finish=1560192818838945300,duration=1254059773
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:01] 
[01:05:01] running 144 tests
[01:05:04] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:05:05] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:05:05] 
[01:05:05]  finished in 4.586
[01:05:05] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:08] 
[01:05:08] running 9 tests
[01:05:08] iiiiiiiii
[01:05:08] 
[01:05:08]  finished in 0.149
[01:05:08] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:23] 
[01:05:23] running 122 tests
[01:05:48] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:05:53] .i.i......iii.i.....ii
[01:05:53] 
[01:05:53]  finished in 30.231
[01:05:53] travis_fold:end:test_debuginfo

---
[01:05:54] 
[01:05:54] running 24 tests
[01:06:01] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:06:01] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:01] .....F..................
[01:06:01] 
[01:06:01] ---- [ui] ui-fulldeps/internal-lints/pass_ty_by_ref.rs stdout ----
[01:06:01] diff of stderr:
[01:06:01] 
[01:06:01] 
[01:06:01] 62   --> $DIR/pass_ty_by_ref.rs:55:22
[01:06:01] 63    |
[01:06:01] 64 LL |         ty_ctxt_ref: &TyCtxt<'_, '_>,
[01:06:01] -    |                      ^^^^^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:06:01] +    |                      ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:06:01] 67 error: passing `Ty<'_>` by reference
[01:06:01] 68   --> $DIR/pass_ty_by_ref.rs:59:38
[01:06:01] 
[01:06:01] 
[01:06:01] 
[01:06:01] The actual stderr differed from the expected stderr.
[01:06:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/pass_ty_by_ref.stderr
[01:06:01] To update references, rerun the tests and pass the `--bless` flag
[01:06:01] To only update this specific test, also pass `--test-args internal-lints/pass_ty_by_ref.rs`
[01:06:01] error: 1 errors occurred comparing output.
[01:06:01] status: exit code: 1
[01:06:01] status: exit code: 1
[01:06:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/auxiliary" "-A" "unused"
[01:06:01] ------------------------------------------
[01:06:01] 
[01:06:01] ------------------------------------------
[01:06:01] stderr:
[01:06:01] stderr:
[01:06:01] ------------------------------------------
[01:06:01] error: passing `Ty<'_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:13:13
[01:06:01]    |
[01:06:01] LL |     ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
[01:06:01]    |             ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:06:01] note: lint level defined here
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:4:9
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL | #![deny(ty_pass_by_reference)]
[01:06:01] 
[01:06:01] error: passing `TyCtxt<'_, '_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:15:18
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL |     ty_ctxt_ref: &TyCtxt<'_, '_>, //~ ERROR passing `TyCtxt<'_, '_>` by reference
[01:06:01]    |                  ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:06:01] error: passing `Ty<'_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:19:28
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>) {}
[01:06:01]    |                            ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:06:01] error: passing `TyCtxt<'_, '_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:19:55
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>) {}
[01:06:01]    |                                                       ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:06:01] error: passing `Ty<'_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:26:17
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
[01:06:01]    |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:06:01] error: passing `TyCtxt<'_, '_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:28:22
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL |         ty_ctxt_ref: &TyCtxt<'_, '_>, //~ ERROR passing `TyCtxt<'_, '_>` by reference
[01:06:01]    |                      ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:06:01] error: passing `Ty<'_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:31:41
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>);
[01:06:01]    |                                         ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:06:01] error: passing `TyCtxt<'_, '_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:31:68
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>);
[01:06:01]    |                                                                    ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:06:01] error: passing `Ty<'_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:53:17
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
[01:06:01]    |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:06:01] error: passing `TyCtxt<'_, '_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:55:22
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL |         ty_ctxt_ref: &TyCtxt<'_, '_>, //~ ERROR passing `TyCtxt<'_, '_>` by reference
[01:06:01]    |                      ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:06:01] error: passing `Ty<'_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:59:38
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>) {}
[01:06:01]    |                                      ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:06:01] error: passing `TyCtxt<'_, '_>` by reference
[01:06:01]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:59:65
[01:06:01]    |
[01:06:01]    |
[01:06:01] LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>) {}
[01:06:01]    |                                                                 ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:06:01] error: aborting due to 12 previous errors
[01:06:01] 
[01:06:01] 
[01:06:01] ------------------------------------------
---
[01:06:01] test result: FAILED. 23 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:01] 
[01:06:01] 
[01:06:01] 
[01:06:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:01] 
[01:06:01] 
[01:06:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:01] Build completed unsuccessfully in 1:01:12
---
travis_time:end:04b358eb:start=1560196792658297079,finish=1560196792663197208,duration=4900129
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00975837
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:38453904
travis_time:start:38453904
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04fb8926
$ dmesg | grep -i kill
