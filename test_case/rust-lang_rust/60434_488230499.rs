plain
travis_time:end:07d8ad88:start=1556691192000051083,finish=1556691263793005666,duration=71792954583
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:22:24] 
[01:22:24] failures:
[01:22:24] 
[01:22:24] ---- /checkout/src/doc/edition-guide/src/rust-2018/data-types/inclusive-ranges.md - ___ (line 28) stdout ----
[01:22:24] thread '/checkout/src/doc/edition-guide/src/rust-2018/data-types/inclusive-ranges.md - ___ (line 28)' panicked at 'test compiled while it wasn't supposed to', src/librustdoc/test.rs:300:13
[01:22:24] 
[01:22:24] 
[01:22:24] failures:
[01:22:24]     /checkout/src/doc/edition-guide/src/rust-2018/data-types/inclusive-ranges.md - ___ (line 28)
---
[01:28:05] 
[01:28:06] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:28:06]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:25
[01:28:06]     |
[01:28:06] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:28:06] 
[01:28:06] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:28:06]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:49
[01:28:06]     |
[01:28:06]     |
[01:28:06] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:28:06] 
[01:28:06] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:28:06]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:610:23
[01:28:06]     |
---
[01:28:06] 
[01:28:07] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:28:07]    --> src/tools/clippy/clippy_lints/src/loops.rs:680:21
[01:28:07]     |
[01:28:07] 680 |         | ExprKind::Use(ref e) => never_loop_expr(e, main_loop_id),
[01:28:07] 
[01:28:08] error: aborting due to 8 previous errors
[01:28:08] 
[01:28:08] For more information about this error, try `rustc --explain E0599`.
---
[01:28:09] 
[01:28:10] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:28:10]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:25
[01:28:10]     |
[01:28:10] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:28:10] 
[01:28:10] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:28:10]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:49
[01:28:10]     |
[01:28:10]     |
[01:28:10] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:28:10] 
[01:28:10] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:28:10]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:610:23
[01:28:10]     |
---
[01:28:10] 
[01:28:11] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:28:11]    --> src/tools/clippy/clippy_lints/src/loops.rs:680:21
[01:28:11]     |
[01:28:11] 680 |         | ExprKind::Use(ref e) => never_loop_expr(e, main_loop_id),
[01:28:11] 
[01:28:12] error: aborting due to 8 previous errors
[01:28:12] 
[01:28:12] For more information about this error, try `rustc --explain E0599`.
---
[01:32:04]    Compiling opener v0.3.2
[01:32:04] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:32:04]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:25
[01:32:04]     |
[01:32:04] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:32:04] 
[01:32:04] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:32:04]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:159:49
[01:32:04]     |
[01:32:04]     |
[01:32:04] 159 |             (&ExprKind::Use(ref le), &ExprKind::Use(ref re)) => self.eq_expr(le, re),
[01:32:04] 
[01:32:04] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:32:04]    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:610:23
[01:32:04]     |
---
[01:32:05] 
[01:32:06] error[E0599]: no variant or associated item named `Use` found for type `rustc::hir::ExprKind` in the current scope
[01:32:06]    --> src/tools/clippy/clippy_lints/src/loops.rs:680:21
[01:32:06]     |
[01:32:06] 680 |         | ExprKind::Use(ref e) => never_loop_expr(e, main_loop_id),
[01:32:06] 
[01:32:07]    Compiling rls-analysis v0.17.0
[01:32:09] error: aborting due to 8 previous errors
[01:32:09] 
---
[01:43:04] +
[01:43:04] +
[01:43:04] 
[01:43:04] The actual stderr differed from the expected stderr.
[01:43:04] Actual stderr saved to /tmp/compiletestVHWMBC/async-fn.stderr
[01:43:04] To update references, run this command from build directory:
[01:43:04] tests/run-pass/update-references.sh '/tmp/compiletestVHWMBC' 'async-fn.rs'
[01:43:04] error: 1 errors occurred comparing output.
[01:43:04] status: exit code: 1
[01:43:04] status: exit code: 1
[01:43:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestVHWMBC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestVHWMBC/async-fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestVHWMBC/async-fn.stage-id.aux" "-A" "unused"
[01:43:04] ------------------------------------------
[01:43:04] 
[01:43:04] ------------------------------------------
[01:43:04] stderr:
---
[01:43:09] +For more information about this error, try `rustc --explain E0080`.
[01:43:09] +
[01:43:09] 
[01:43:09] The actual stderr differed from the expected stderr.
[01:43:09] Actual stderr saved to /tmp/compiletestVHWMBC/hashmap.stderr
[01:43:09] To update references, run this command from build directory:
[01:43:09] tests/run-pass/update-references.sh '/tmp/compiletestVHWMBC' 'hashmap.rs'
[01:43:09] error: 1 errors occurred comparing output.
[01:43:09] status: exit code: 1
[01:43:09] status: exit code: 1
[01:43:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hashmap.rs" "-L" "/tmp/compiletestVHWMBC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestVHWMBC/hashmap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-Zmiri-seed=0000000000000000" "-L" "/tmp/compiletestVHWMBC/hashmap.stage-id.aux" "-A" "unused"
[01:43:09] ------------------------------------------
[01:43:09] 
[01:43:09] ------------------------------------------
[01:43:09] stderr:
---
[01:43:20] Verifying status of rustfmt...
[01:43:20] Verifying status of clippy-driver...
[01:43:20] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:43:20] 
[01:43:20] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:43:20] 
[01:43:20] If you do intend to update 'clippy-driver', please check the error messages above and
[01:43:20] commit another update.
[01:43:20] 
[01:43:20] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:43:20] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:43:20] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:244ddb16
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  1 07:57:53 UTC 2019
