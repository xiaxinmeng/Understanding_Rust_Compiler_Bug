plain
travis_time:end:048cd080:start=1560468441115487915,finish=1560468443698068261,duration=2582580346
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    99% |███████████████████████████████▉| 5.5MB 74.9MB/s eta 0:00:01
    99% |████████████████████████████████| 5.5MB 74.3MB/s eta 0:00:01
    99% |████████████████████████████████| 5.5MB 74.8MB/s eta 0:00:01
    100% |████████████████████████████████| 5.5MB 4.1MB/s 
Requirement already satisfied: PyYAML<=5.1,>=3.10; python_version != "2.6" in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
  Downloading https://files.pythonhosted.org/packages/e1/ae/baedc9cb175552e95f3395c43055a6a5e125ae4d48a1d7a924baca83e92e/rsa-3.4.2-py2.py3-none-any.whl (46kB)
    21% |███████                         | 10kB 21.9MB/s eta 0:00:01
    43% |██████████████                  | 20kB 26.7MB/s eta 0:00:01
    65% |█████████████████████           | 30kB 31.7MB/s eta 0:00:01
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:38] 
[01:07:38] running 9 tests
[01:07:38] iiiiiiiii
[01:07:38] 
[01:07:38]  finished in 0.155
[01:07:38] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:54] 
[01:07:54] running 122 tests
[01:08:20] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:08:25] .i.i......iii.i.....ii
[01:08:25] 
[01:08:25]  finished in 30.875
[01:08:25] travis_fold:end:test_debuginfo

---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:25] 
[01:08:25] running 24 tests
[01:08:32] ......FF................
[01:08:32] 
[01:08:32] ---- [ui] ui-fulldeps/internal-lints/pass_ty_by_ref.rs stdout ----
[01:08:32] diff of stderr:
[01:08:32] 
[01:08:32] 
[01:08:32] 14   --> $DIR/pass_ty_by_ref.rs:15:18
[01:08:32] 15    |
[01:08:32] 16 LL |     ty_ctxt_ref: &TyCtxt<'_>,
[01:08:32] -    |                  ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] +    |                  ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] 19 error: passing `Ty<'_>` by reference
[01:08:32] 20   --> $DIR/pass_ty_by_ref.rs:19:28
[01:08:32] 
[01:08:32] 26   --> $DIR/pass_ty_by_ref.rs:19:55
[01:08:32] 26   --> $DIR/pass_ty_by_ref.rs:19:55
[01:08:32] 27    |
[01:08:32] 28 LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
[01:08:32] -    |                                                       ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] +    |                                                       ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] 31 error: passing `Ty<'_>` by reference
[01:08:32] 32   --> $DIR/pass_ty_by_ref.rs:26:17
[01:08:32] 
[01:08:32] 38   --> $DIR/pass_ty_by_ref.rs:28:22
[01:08:32] 38   --> $DIR/pass_ty_by_ref.rs:28:22
[01:08:32] 39    |
[01:08:32] 40 LL |         ty_ctxt_ref: &TyCtxt<'_>,
[01:08:32] -    |                      ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] +    |                      ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] 43 error: passing `Ty<'_>` by reference
[01:08:32] 44   --> $DIR/pass_ty_by_ref.rs:31:41
[01:08:32] 
[01:08:32] 50   --> $DIR/pass_ty_by_ref.rs:31:68
[01:08:32] 50   --> $DIR/pass_ty_by_ref.rs:31:68
[01:08:32] 51    |
[01:08:32] 52 LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);
[01:08:32] -    |                                                                    ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] +    |                                                                    ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] 55 error: passing `Ty<'_>` by reference
[01:08:32] 56   --> $DIR/pass_ty_by_ref.rs:53:17
[01:08:32] 
[01:08:32] 62   --> $DIR/pass_ty_by_ref.rs:55:22
[01:08:32] 62   --> $DIR/pass_ty_by_ref.rs:55:22
[01:08:32] 63    |
[01:08:32] 64 LL |         ty_ctxt_ref: &TyCtxt<'_>,
[01:08:32] -    |                      ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] +    |                      ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] 67 error: passing `Ty<'_>` by reference
[01:08:32] 68   --> $DIR/pass_ty_by_ref.rs:59:38
[01:08:32] 
[01:08:32] 74   --> $DIR/pass_ty_by_ref.rs:59:65
[01:08:32] 74   --> $DIR/pass_ty_by_ref.rs:59:65
[01:08:32] 75    |
[01:08:32] 76 LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
[01:08:32] -    |                                                                 ^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] +    |                                                                 ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] 79 error: aborting due to 12 previous errors
[01:08:32] 80 
[01:08:32] 
[01:08:32] 
[01:08:32] 
[01:08:32] The actual stderr differed from the expected stderr.
[01:08:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/pass_ty_by_ref.stderr
[01:08:32] To update references, rerun the tests and pass the `--bless` flag
[01:08:32] To only update this specific test, also pass `--test-args internal-lints/pass_ty_by_ref.rs`
[01:08:32] error: 1 errors occurred comparing output.
[01:08:32] status: exit code: 1
[01:08:32] status: exit code: 1
[01:08:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/auxiliary" "-A" "unused"
[01:08:32] ------------------------------------------
[01:08:32] 
[01:08:32] ------------------------------------------
[01:08:32] stderr:
[01:08:32] stderr:
[01:08:32] ------------------------------------------
[01:08:32] error: passing `Ty<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:13:13
[01:08:32]    |
[01:08:32] LL |     ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
[01:08:32]    |             ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:08:32] note: lint level defined here
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:4:9
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL | #![deny(ty_pass_by_reference)]
[01:08:32] 
[01:08:32] error: passing `TyCtxt<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:15:18
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL |     ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference
[01:08:32]    |                  ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] error: passing `Ty<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:19:28
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
[01:08:32]    |                            ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:08:32] error: passing `TyCtxt<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:19:55
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
[01:08:32]    |                                                       ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] error: passing `Ty<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:26:17
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
[01:08:32]    |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:08:32] error: passing `TyCtxt<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:28:22
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL |         ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference
[01:08:32]    |                      ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] error: passing `Ty<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:31:41
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);
[01:08:32]    |                                         ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:08:32] error: passing `TyCtxt<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:31:68
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);
[01:08:32]    |                                                                    ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] error: passing `Ty<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:53:17
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
[01:08:32]    |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:08:32] error: passing `TyCtxt<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:55:22
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL |         ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference
[01:08:32]    |                      ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] error: passing `Ty<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:59:38
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
[01:08:32]    |                                      ^^^^^^^ help: try passing by value: `Ty<'_>`
[01:08:32] error: passing `TyCtxt<'_>` by reference
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:59:65
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
[01:08:32]    |                                                                 ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
[01:08:32] error: aborting due to 12 previous errors
[01:08:32] 
[01:08:32] 
[01:08:32] ------------------------------------------
[01:08:32] ------------------------------------------
[01:08:32] 
[01:08:32] 
[01:08:32] ---- [ui] ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs stdout ----
[01:08:32] diff of stderr:
[01:08:32] 
[01:08:32] 14   --> $DIR/qualified_ty_ty_ctxt.rs:27:16
[01:08:32] 15    |
[01:08:32] 16 LL |     ty_ctxt_q: ty::TyCtxt<'_>,
[01:08:32] -    |                ^^^^^^^^^^^^^^^^^^ help: try using it unqualified: `TyCtxt<'_>`
[01:08:32] +    |                ^^^^^^^^^^^^^^ help: try using it unqualified: `TyCtxt<'_>`
[01:08:32] 19 error: aborting due to 2 previous errors
[01:08:32] 20 
[01:08:32] 
[01:08:32] 
[01:08:32] 
[01:08:32] The actual stderr differed from the expected stderr.
[01:08:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt/qualified_ty_ty_ctxt.stderr
[01:08:32] To update references, rerun the tests and pass the `--bless` flag
[01:08:32] To only update this specific test, also pass `--test-args internal-lints/qualified_ty_ty_ctxt.rs`
[01:08:32] error: 1 errors occurred comparing output.
[01:08:32] status: exit code: 1
[01:08:32] status: exit code: 1
[01:08:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt/auxiliary" "-A" "unused"
[01:08:32] ------------------------------------------
[01:08:32] 
[01:08:32] ------------------------------------------
[01:08:32] stderr:
[01:08:32] stderr:
[01:08:32] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:08:32] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:32] ------------------------------------------
[01:08:32] error: usage of qualified `ty::Ty<'_>`
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:25:11
[01:08:32]    |
[01:08:32] LL |     ty_q: ty::Ty<'_>, //~ ERROR usage of qualified `ty::Ty<'_>`
[01:08:32]    |           ^^^^^^^^^^ help: try using it unqualified: `Ty<'_>`
[01:08:32] note: lint level defined here
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:4:9
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL | #![deny(usage_of_qualified_ty)]
[01:08:32] 
[01:08:32] error: usage of qualified `ty::TyCtxt<'_>`
[01:08:32]   --> /checkout/src/test/ui-fulldeps/internal-lints/qualified_ty_ty_ctxt.rs:27:16
[01:08:32]    |
[01:08:32]    |
[01:08:32] LL |     ty_ctxt_q: ty::TyCtxt<'_>, //~ ERROR usage of qualified `ty::TyCtxt<'_>`
[01:08:32]    |                ^^^^^^^^^^^^^^ help: try using it unqualified: `TyCtxt<'_>`
[01:08:32] error: aborting due to 2 previous errors
[01:08:32] 
[01:08:32] 
[01:08:32] ------------------------------------------
---
[01:08:32] test result: FAILED. 22 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:32] 
[01:08:32] 
[01:08:32] 
[01:08:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:32] 
[01:08:32] 
[01:08:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:32] Build completed unsuccessfully in 1:03:48
