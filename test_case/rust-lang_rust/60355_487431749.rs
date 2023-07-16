plain
[01:25:25] [RUSTC-TIMING] semver test:false 2.804
[01:25:43] [RUSTC-TIMING] toml test:false 20.403
[01:25:50] [RUSTC-TIMING] cargo_metadata test:false 24.558
[01:25:50]    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
[01:26:01] error: usage of qualified `ty::Ty<'tcx>`
[01:26:01]    |
[01:26:01] 71 |     item_type: ty::Ty<'tcx>,
[01:26:01] 71 |     item_type: ty::Ty<'tcx>,
[01:26:01]    |                ^^^^^^^^^^^^ help: try using it unqualified: `Ty<'tcx>`
[01:26:01] note: lint level defined here
[01:26:01]   --> src/tools/clippy/clippy_lints/src/lib.rs:11:9
[01:26:01]    |
[01:26:01] 11 | #![deny(internal)]
[01:26:01] 11 | #![deny(internal)]
[01:26:01]    |         ^^^^^^^^
[01:26:01]    = note: #[deny(usage_of_qualified_ty)] implied by #[deny(internal)]
[01:26:01] 
[01:26:01] error: usage of qualified `ty::Ty<'tcx>`
[01:26:01]     |
[01:26:01] 112 |     item_type: ty::Ty<'tcx>,
[01:26:01] 112 |     item_type: ty::Ty<'tcx>,
[01:26:01]     |                ^^^^^^^^^^^^ help: try using it unqualified: `Ty<'tcx>`
[01:26:01] 
[01:26:01] error: usage of qualified `ty::Ty<'tcx>`
[01:26:01]    |
[01:26:01] 71 |     item_type: ty::Ty<'tcx>,
[01:26:01] 71 |     item_type: ty::Ty<'tcx>,
[01:26:01]    |                ^^^^^^^^^^^^ help: try using it unqualified: `Ty<'tcx>`
[01:26:01] note: lint level defined here
[01:26:01]   --> src/tools/clippy/clippy_lints/src/lib.rs:11:9
[01:26:01]    |
[01:26:01] 11 | #![deny(internal)]
[01:26:01] 11 | #![deny(internal)]
[01:26:01]    |         ^^^^^^^^
[01:26:01]    = note: #[deny(usage_of_qualified_ty)] implied by #[deny(internal)]
[01:26:01] 
[01:26:01] error: usage of qualified `ty::Ty<'tcx>`
[01:26:01]     |
[01:26:01] 112 |     item_type: ty::Ty<'tcx>,
[01:26:01] 112 |     item_type: ty::Ty<'tcx>,
[01:26:01]     |                ^^^^^^^^^^^^ help: try using it unqualified: `Ty<'tcx>`
[01:26:02] error: aborting due to 2 previous errors
[01:26:02] 
[01:26:02] error: aborting due to 2 previous errors
[01:26:02] 
---
[01:29:43] [RUSTC-TIMING] lsp_types test:false 27.784
[01:29:44]    Compiling rls-analysis v0.17.0
[01:29:47] [RUSTC-TIMING] rand_pcg test:false 0.411
[01:29:47] [RUSTC-TIMING] rand_chacha test:false 0.778
[01:29:56] error: usage of qualified `ty::Ty<'tcx>`
[01:29:56]    |
[01:29:56] 71 |     item_type: ty::Ty<'tcx>,
[01:29:56] 71 |     item_type: ty::Ty<'tcx>,
[01:29:56]    |                ^^^^^^^^^^^^ help: try using it unqualified: `Ty<'tcx>`
[01:29:56] note: lint level defined here
[01:29:56]   --> src/tools/clippy/clippy_lints/src/lib.rs:11:9
[01:29:56]    |
[01:29:56] 11 | #![deny(internal)]
[01:29:56] 11 | #![deny(internal)]
[01:29:56]    |         ^^^^^^^^
[01:29:56]    = note: #[deny(usage_of_qualified_ty)] implied by #[deny(internal)]
[01:29:56] 
[01:29:56] error: usage of qualified `ty::Ty<'tcx>`
[01:29:56]     |
[01:29:56] 112 |     item_type: ty::Ty<'tcx>,
[01:29:56] 112 |     item_type: ty::Ty<'tcx>,
[01:29:56]     |                ^^^^^^^^^^^^ help: try using it unqualified: `Ty<'tcx>`
[01:29:56] error: aborting due to 2 previous errors
[01:29:56] 
[01:29:56] [RUSTC-TIMING] clippy_lints test:false 18.364
[01:29:56] error: Could not compile `clippy_lints`.
---
[01:40:02] +
[01:40:02] +
[01:40:02] 
[01:40:02] The actual stderr differed from the expected stderr.
[01:40:02] Actual stderr saved to /tmp/compiletestBTP9Iq/async-fn.stderr
[01:40:02] To update references, run this command from build directory:
[01:40:02] tests/run-pass/update-references.sh '/tmp/compiletestBTP9Iq' 'async-fn.rs'
[01:40:02] error: 1 errors occurred comparing output.
[01:40:02] status: exit code: 1
[01:40:02] status: exit code: 1
[01:40:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestBTP9Iq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestBTP9Iq/async-fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestBTP9Iq/async-fn.stage-id.aux" "-A" "unused"
[01:40:02] ------------------------------------------
[01:40:02] 
[01:40:02] ------------------------------------------
[01:40:02] stderr:
---
[01:40:07] +For more information about this error, try `rustc --explain E0080`.
[01:40:07] +
[01:40:07] 
[01:40:07] The actual stderr differed from the expected stderr.
[01:40:07] Actual stderr saved to /tmp/compiletestBTP9Iq/hashmap.stderr
[01:40:07] To update references, run this command from build directory:
[01:40:07] tests/run-pass/update-references.sh '/tmp/compiletestBTP9Iq' 'hashmap.rs'
[01:40:07] error: 1 errors occurred comparing output.
[01:40:07] status: exit code: 1
[01:40:07] status: exit code: 1
[01:40:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hashmap.rs" "-L" "/tmp/compiletestBTP9Iq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestBTP9Iq/hashmap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-Zmiri-seed=0000000000000000" "-L" "/tmp/compiletestBTP9Iq/hashmap.stage-id.aux" "-A" "unused"
[01:40:07] ------------------------------------------
[01:40:07] 
[01:40:07] ------------------------------------------
[01:40:07] stderr:
---
[01:40:17] Verifying status of rustfmt...
[01:40:17] Verifying status of clippy-driver...
[01:40:17] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:40:17] 
[01:40:17] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:40:17] 
[01:40:17] If you do intend to update 'clippy-driver', please check the error messages above and
[01:40:17] commit another update.
[01:40:17] 
[01:40:17] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:40:17] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:40:17] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:18ee9400
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 29 01:14:31 UTC 2019
---
travis_time:end:01310fa4:start=1556500473085397601,finish=1556500473094488404,duration=9090803
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:288ca47f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2bfacbd5
travis_time:start:2bfacbd5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f7f84b0
$ dmesg | grep -i kill
