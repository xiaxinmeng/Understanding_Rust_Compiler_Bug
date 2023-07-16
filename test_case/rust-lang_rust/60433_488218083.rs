plain
travis_time:end:196c055a:start=1556684851190764492,finish=1556684937113808972,duration=85923044480
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:17:16] 
[01:17:16] failures:
[01:17:16] 
[01:17:16] ---- /checkout/src/doc/edition-guide/src/rust-2018/data-types/inclusive-ranges.md - ___ (line 28) stdout ----
[01:17:16] thread '/checkout/src/doc/edition-guide/src/rust-2018/data-types/inclusive-ranges.md - ___ (line 28)' panicked at 'test compiled while it wasn't supposed to', src/librustdoc/test.rs:300:13
[01:17:16] 
[01:17:16] 
[01:17:16] failures:
[01:17:16]     /checkout/src/doc/edition-guide/src/rust-2018/data-types/inclusive-ranges.md - ___ (line 28)
---
[01:22:25] 
[01:22:25] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:22:25]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:25
[01:22:25]     |
[01:22:25] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:22:25] 
[01:22:25] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:22:25]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:49
[01:22:25]     |
[01:22:25]     |
[01:22:25] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:22:25] 
[01:22:25] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:22:25]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:610:23
[01:22:25]     |
---
[01:22:25] 
[01:22:26] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:22:26]    --> src/tools/clippy/clippy_lints/src/loops.rs:680:21
[01:22:26]     |
[01:22:26] 680 |         | ExprKind::Use(ref e) => never_loop_expr(e, main_loop_id),
[01:22:26] 
[01:22:28] error: aborting due to 8 previous errors
[01:22:28] 
[01:22:28] For more information about this error, try `rustc --explain E0599`.
---
[01:22:31] 
[01:22:31] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:22:31]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:25
[01:22:31]     |
[01:22:31] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:22:31] 
[01:22:31] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:22:31]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:49
[01:22:31]     |
[01:22:31]     |
[01:22:31] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:22:31] 
[01:22:31] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:22:31]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:610:23
[01:22:31]     |
---
[01:22:32] 
[01:22:32] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:22:32]    --> src/tools/clippy/clippy_lints/src/loops.rs:680:21
[01:22:32]     |
[01:22:32] 680 |         | ExprKind::Use(ref e) => never_loop_expr(e, main_loop_id),
[01:22:32] 
[01:22:34] error: aborting due to 8 previous errors
[01:22:34] 
[01:22:34] For more information about this error, try `rustc --explain E0599`.
---
[01:25:54] 
[01:25:54] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:25:54]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:25
[01:25:54]     |
[01:25:54] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:25:54] 
[01:25:54] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:25:54]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:49
[01:25:54]     |
[01:25:54]     |
[01:25:54] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:25:54] 
[01:25:54] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:25:54]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:610:23
[01:25:54]     |
---
[01:25:55]    Compiling rls-analysis v0.17.0
[01:25:56] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:25:56]    --> src/tools/clippy/clippy_lints/src/loops.rs:680:21
[01:25:56]     |
[01:25:56] 680 |         | ExprKind::Use(ref e) => never_loop_expr(e, main_loop_id),
[01:25:56] 
[01:25:57]    Compiling rustfix v0.4.4
[01:25:58] error: aborting due to 8 previous errors
[01:25:58] 
---
[01:35:35] +
[01:35:35] +
[01:35:35] 
[01:35:35] The actual stderr differed from the expected stderr.
[01:35:35] Actual stderr saved to /tmp/compiletest42LOJ9/async-fn.stderr
[01:35:35] To update references, run this command from build directory:
[01:35:35] tests/run-pass/update-references.sh '/tmp/compiletest42LOJ9' 'async-fn.rs'
[01:35:35] error: 1 errors occurred comparing output.
[01:35:35] status: exit code: 1
[01:35:35] thread '[ui] run-pass/async-fn.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:35:35] thread '[ui] run-pass/async-fn.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:35:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletest42LOJ9" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest42LOJ9/async-fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletest42LOJ9/async-fn.stage-id.aux" "-A" "unused"
[01:35:35] stdout:
[01:35:35] ------------------------------------------
[01:35:35] 
[01:35:35] ------------------------------------------
---
[01:35:40] +For more information about this error, try `rustc --explain E0080`.
[01:35:40] +
[01:35:40] 
[01:35:40] The actual stderr differed from the expected stderr.
[01:35:40] Actual stderr saved to /tmp/compiletest42LOJ9/hashmap.stderr
[01:35:40] To update references, run this command from build directory:
[01:35:40] tests/run-pass/update-references.sh '/tmp/compiletest42LOJ9' 'hashmap.rs'
[01:35:40] error: 1 errors occurred comparing output.
[01:35:40] status: exit code: 1
[01:35:40] status: exit code: 1
[01:35:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hashmap.rs" "-L" "/tmp/compiletest42LOJ9" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest42LOJ9/hashmap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-Zmiri-seed=0000000000000000" "-L" "/tmp/compiletest42LOJ9/hashmap.stage-id.aux" "-A" "unused"
[01:35:40] ------------------------------------------
[01:35:40] 
[01:35:40] ------------------------------------------
[01:35:40] stderr:
---
[01:35:50] Verifying status of rustfmt...
[01:35:50] Verifying status of clippy-driver...
[01:35:50] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:35:50] 
[01:35:50] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:35:50] 
[01:35:50] If you do intend to update 'clippy-driver', please check the error messages above and
[01:35:50] commit another update.
[01:35:50] 
[01:35:50] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:35:50] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:35:50] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:2ffaf413
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  1 06:04:55 UTC 2019
---
travis_time:end:00fbec1c:start=1556690696978841275,finish=1556690696983476957,duration=4635682
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2f26608c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0225dfad
travis_time:start:0225dfad
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:373c9e5a
$ dmesg | grep -i kill
