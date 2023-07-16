plain
travis_time:end:144a1964:start=1560184012061041273,finish=1560184013352299141,duration=1291257868
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
[01:03:40] 
[01:03:40] running 144 tests
[01:03:43] i..iii.....iii...iiii....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:03:45] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:03:45] 
[01:03:45]  finished in 4.436
[01:03:45] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:47] 
[01:03:47] running 9 tests
[01:03:47] iiiiiiiii
[01:03:47] 
[01:03:47]  finished in 0.147
[01:03:47] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:02] 
[01:04:02] running 122 tests
[01:04:27] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:04:32] .i.i......iii.i.....ii
[01:04:32] 
[01:04:32]  finished in 29.751
[01:04:32] travis_fold:end:test_debuginfo

---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:32] 
[01:04:32] running 24 tests
[01:04:39] ......FF................
[01:04:39] 
[01:04:39] ---- [ui] ui-fulldeps/internal-lints/pass_ty_by_ref.rs stdout ----
[01:04:39] diff of stderr:
[01:04:39] 
[01:04:39] 
[01:04:39] 14   --> $DIR/pass_ty_by_ref.rs:15:18
[01:04:39] 15    |
[01:04:39] 16 LL |     ty_ctxt_ref: &TyCtxt<'_, '_>,
[01:04:39] -    |                  ^^^^^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] +    |                  ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] 19 error: passing `Ty<'_>` by reference
[01:04:39] 20   --> $DIR/pass_ty_by_ref.rs:19:28
[01:04:39] 
[01:04:39] 26   --> $DIR/pass_ty_by_ref.rs:19:55
[01:04:39] 26   --> $DIR/pass_ty_by_ref.rs:19:55
[01:04:39] 27    |
[01:04:39] 28 LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>) {}
[01:04:39] -    |                                                       ^^^^^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] +    |                                                       ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] 31 error: passing `Ty<'_>` by reference
[01:04:39] 32   --> $DIR/pass_ty_by_ref.rs:26:17
[01:04:39] 
[01:04:39] 38   --> $DIR/pass_ty_by_ref.rs:28:22
[01:04:39] 38   --> $DIR/pass_ty_by_ref.rs:28:22
[01:04:39] 39    |
[01:04:39] 40 LL |         ty_ctxt_ref: &TyCtxt<'_, '_>,
[01:04:39] -    |                      ^^^^^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] +    |                      ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] 43 error: passing `Ty<'_>` by reference
[01:04:39] 44   --> $DIR/pass_ty_by_ref.rs:31:41
[01:04:39] 
[01:04:39] 50   --> $DIR/pass_ty_by_ref.rs:31:68
[01:04:39] 50   --> $DIR/pass_ty_by_ref.rs:31:68
[01:04:39] 51    |
[01:04:39] 52 LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>);
[01:04:39] -    |                                                                    ^^^^^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] +    |                                                                    ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] 55 error: passing `Ty<'_>` by reference
[01:04:39] 56   --> $DIR/pass_ty_by_ref.rs:53:17
[01:04:39] 
[01:04:39] 62   --> $DIR/pass_ty_by_ref.rs:55:22
[01:04:39] 62   --> $DIR/pass_ty_by_ref.rs:55:22
[01:04:39] 63    |
[01:04:39] 64 LL |         ty_ctxt_ref: &TyCtxt<'_, '_>,
[01:04:39] -    |                      ^^^^^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] +    |                      ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] 67 error: passing `Ty<'_>` by reference
[01:04:39] 68   --> $DIR/pass_ty_by_ref.rs:59:38
[01:04:39] 
[01:04:39] 74   --> $DIR/pass_ty_by_ref.rs:59:65
[01:04:39] 74   --> $DIR/pass_ty_by_ref.rs:59:65
[01:04:39] 75    |
[01:04:39] 76 LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>) {}
[01:04:39] -    |                                                                 ^^^^^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] +    |                                                                 ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] 79 error: aborting due to 12 previous errors
[01:04:39] 80 
[01:04:39] 
[01:04:39] 
[01:04:39] 
[01:04:39] The actual stderr differed from the expected stderr.
[01:04:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/pass_ty_by_ref.stderr
[01:04:39] To update references, rerun the tests and pass the `--bless` flag
[01:04:39] To only update this specific test, also pass `--test-args internal-lints/pass_ty_by_ref.rs`
[01:04:39] error: 1 errors occurred comparing output.
[01:04:39] status: exit code: 1
[01:04:39] status: exit code: 1
[01:04:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/auxiliary" "-A" "unused"
[01:04:39] ------------------------------------------
[01:04:39] 
[01:04:39] ------------------------------------------
[01:04:39] stderr:
[01:04:39] stderr:
[01:04:39] ------------------------------------------
[01:04:39] error: passing `Ty<'_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:13:13
[01:04:39]    |
[01:04:39] LL |     ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
[01:04:39]    |             ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:04:39] note: lint level defined here
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:4:9
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL | #![deny(ty_pass_by_reference)]
[01:04:39] 
[01:04:39] error: passing `TyCtxt<'_, '_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:15:18
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL |     ty_ctxt_ref: &TyCtxt<'_, '_>, //~ ERROR passing `TyCtxt<'_, '_>` by reference
[01:04:39]    |                  ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] error: passing `Ty<'_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:19:28
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>) {}
[01:04:39]    |                            ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:04:39] error: passing `TyCtxt<'_, '_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:19:55
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>) {}
[01:04:39]    |                                                       ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] error: passing `Ty<'_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:26:17
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
[01:04:39]    |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:04:39] error: passing `TyCtxt<'_, '_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:28:22
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL |         ty_ctxt_ref: &TyCtxt<'_, '_>, //~ ERROR passing `TyCtxt<'_, '_>` by reference
[01:04:39]    |                      ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] error: passing `Ty<'_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:31:41
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>);
[01:04:39]    |                                         ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:04:39] error: passing `TyCtxt<'_, '_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:31:68
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>);
[01:04:39]    |                                                                    ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] error: passing `Ty<'_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:53:17
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
[01:04:39]    |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:04:39] error: passing `TyCtxt<'_, '_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:55:22
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL |         ty_ctxt_ref: &TyCtxt<'_, '_>, //~ ERROR passing `TyCtxt<'_, '_>` by reference
[01:04:39]    |                      ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] error: passing `Ty<'_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:59:38
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>) {}
[01:04:39]    |                                      ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:04:39] error: passing `TyCtxt<'_, '_>` by reference
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:59:65
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_, '_>) {}
[01:04:39]    |                                                                 ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_>`
[01:04:39] error: aborting due to 12 previous errors
[01:04:39] 
[01:04:39] 
[01:04:39] ------------------------------------------
[01:04:39] ------------------------------------------
[01:04:39] 
[01:04:39] 
[01:04:39] ---- [ui] ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs stdout ----
[01:04:39] diff of stderr:
[01:04:39] 
[01:04:39] 14   --> $DIR/qualified_ty_ty_ctxt.rs:27:16
[01:04:39] 15    |
[01:04:39] 16 LL |     ty_ctxt_q: ty::TyCtxt<'_, '_>,
[01:04:39] -    |                ^^^^^^^^^^^^^^^^^^^^^^ help: try using it unqualified: `TyCtxt<'_, '_>`
[01:04:39] +    |                ^^^^^^^^^^^^^^^^^^ help: try using it unqualified: `TyCtxt<'_, '_>`
[01:04:39] 19 error: aborting due to 2 previous errors
[01:04:39] 20 
[01:04:39] 
[01:04:39] 
[01:04:39] 
[01:04:39] The actual stderr differed from the expected stderr.
[01:04:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt/qualified_ty_ty_ctxt.stderr
[01:04:39] To update references, rerun the tests and pass the `--bless` flag
[01:04:39] To only update this specific test, also pass `--test-args internal-lints/qualified_ty_ty_ctxt.rs`
[01:04:39] error: 1 errors occurred comparing output.
[01:04:39] status: exit code: 1
[01:04:39] status: exit code: 1
[01:04:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt/auxiliary" "-A" "unused"
[01:04:39] ------------------------------------------
[01:04:39] 
[01:04:39] ------------------------------------------
[01:04:39] stderr:
[01:04:39] stderr:
[01:04:39] ------------------------------------------
[01:04:39] error: usage of qualified `ty::Ty<'_>`
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:25:11
[01:04:39]    |
[01:04:39] LL |     ty_q: ty::Ty<'_>, //~ ERROR usage of qualified `ty::Ty<'_>`
[01:04:39]    |           ^^^^^^^^^^ help: try using it unqualified: `Ty<'_>`
[01:04:39] note: lint level defined here
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:4:9
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL | #![deny(usage_of_qualified_ty)]
[01:04:39] 
[01:04:39] error: usage of qualified `ty::TyCtxt<'_, '_>`
[01:04:39]   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:27:16
[01:04:39]    |
[01:04:39]    |
[01:04:39] LL |     ty_ctxt_q: ty::TyCtxt<'_, '_>, //~ ERROR usage of qualified `ty::TyCtxt<'_, '_>`
[01:04:39]    |                ^^^^^^^^^^^^^^^^^^ help: try using it unqualified: `TyCtxt<'_, '_>`
[01:04:39] error: aborting due to 2 previous errors
[01:04:39] 
[01:04:39] 
[01:04:39] ------------------------------------------
---
[01:04:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:04:39] 
[01:04:39] 
[01:04:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:39] 
[01:04:39] 
[01:04:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:39] Build completed unsuccessfully in 0:59:54
---
travis_time:end:29a12336:start=1560187904996634838,finish=1560187905001537429,duration=4902591
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0783a9da
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16224ab0
travis_time:start:16224ab0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13c671a3
$ dmesg | grep -i kill
