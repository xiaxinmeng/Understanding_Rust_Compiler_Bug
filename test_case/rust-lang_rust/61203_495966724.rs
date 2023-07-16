plain
travis_time:end:0861ca65:start=1558838741118634141,finish=1558838743462634679,duration=2344000538
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:12:21] ..................i................................................................................. 3400/5586
[01:12:25] ............................................................................................ii...i.. 3500/5586
[01:12:28] ii.................................................................................................. 3600/5586
[01:12:32] .................................................................................................... 3700/5586
[01:12:36] ...........F.....................................................................................ii. 3800/5586
[01:12:41] ..................i................................................................................. 4000/5586
[01:12:43] ..................................................................................i................. 4100/5586
[01:12:46] .................................................................................................... 4200/5586
[01:12:51] .................................................................................................... 4300/5586
---
[01:13:48] 
[01:13:48] 154                T,
[01:13:48] 155            ]
[01:13:48] 156 
[01:13:48] - error: failed to remove $TEST_BUILD_DIR/nll/ty-outlives/projection-one-region-trait-bound-static-closure/projection-one-region-trait-bound-static-closure.projection_one_region_trait_bound_static_closure.7rcbfp3g-cgu.0.rcgu.o: The system cannot find the path specified. (os error 3)
[01:13:48] - 
[01:13:48] - error: failed to remove $TEST_BUILD_DIR/nll/ty-outlives/projection-one-region-trait-bound-static-closure/projection-one-region-trait-bound-static-closure.projection_one_region_trait_bound_static_closure.7rcbfp3g-cgu.1.rcgu.o: The system cannot find the path specified. (os error 3)
[01:13:48] - error: aborting due to 2 previous errors
[01:13:48] - 
[01:13:48] 163 
[01:13:48] 
[01:13:48] 
[01:13:48] 
[01:13:48] The actual stderr differed from the expected stderr.
[01:13:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure/projection-one-region-trait-bound-static-closure.stderr
[01:13:48] To update references, rerun the tests and pass the `--bless` flag
[01:13:48] To only update this specific test, also pass `--test-args nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs`
[01:13:48] error: 1 errors occurred comparing output.
[01:13:48] status: exit code: 0
[01:13:48] status: exit code: 0
[01:13:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure/auxiliary" "-A" "unused"
[01:13:48] ------------------------------------------
[01:13:48] 
[01:13:48] ------------------------------------------
[01:13:48] stderr:
[01:13:48] stderr:
[01:13:48] ------------------------------------------
[01:13:48] note: No external requirements
[01:13:48]   --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs:36:29
[01:13:48]    |
[01:13:48] LL |     with_signature(cell, t, |cell, t| require(cell, t));
[01:13:48]    |
[01:13:48]    |
[01:13:48]    = note: defining type: DefId(0:28 ~ projection_one_region_trait_bound_static_closure[317d]::no_relationships_late[0]::{{closure}}[0]) with closure substs [
[01:13:48]                '_#1r,
[01:13:48]                T,
[01:13:48]                i32,
[01:13:48]                extern "rust-call" fn((std::cell::Cell<&'_#2r ()>, T)),
[01:13:48]            ]
[01:13:48]    = note: late-bound region is '_#3r
[01:13:48] note: No external requirements
[01:13:48]   --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs:32:1
[01:13:48]    |
[01:13:48]    |
[01:13:48] LL | / fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
[01:13:48] LL | | where
[01:13:48] LL | |     T: Anything<'b>,
[01:13:48] LL | | {
[01:13:48] LL | |     with_signature(cell, t, |cell, t| require(cell, t));
[01:13:48]    | |_^
[01:13:48]    |
[01:13:48]    |
[01:13:48]    = note: defining type: DefId(0:24 ~ projection_one_region_trait_bound_static_closure[317d]::no_relationships_late[0]) with substs [
[01:13:48]                '_#1r,
[01:13:48]                T,
[01:13:48] 
[01:13:48] note: No external requirements
[01:13:48]   --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs:45:29
[01:13:48]    |
[01:13:48]    |
[01:13:48] LL |     with_signature(cell, t, |cell, t| require(cell, t));
[01:13:48]    |
[01:13:48]    |
[01:13:48]    = note: defining type: DefId(0:33 ~ projection_one_region_trait_bound_static_closure[317d]::no_relationships_early[0]::{{closure}}[0]) with closure substs [
[01:13:48]                '_#1r,
[01:13:48]                '_#2r,
[01:13:48]                T,
[01:13:48]                i32,
[01:13:48]                extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
[01:13:48] 
[01:13:48] note: No external requirements
[01:13:48]   --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs:40:1
[01:13:48]    |
[01:13:48]    |
[01:13:48] LL | / fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
[01:13:48] LL | | where
[01:13:48] LL | |     T: Anything<'b>,
[01:13:48] LL | |     'a: 'a,
[01:13:48] LL | | {
[01:13:48] LL | |     with_signature(cell, t, |cell, t| require(cell, t));
[01:13:48]    | |_^
[01:13:48]    |
[01:13:48]    |
[01:13:48]    = note: defining type: DefId(0:29 ~ projection_one_region_trait_bound_static_closure[317d]::no_relationships_early[0]) with substs [
[01:13:48]                '_#1r,
[01:13:48]                '_#2r,
[01:13:48]                T,
[01:13:48] 
[01:13:48] note: No external requirements
[01:13:48]   --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs:64:29
[01:13:48]    |
[01:13:48]    |
[01:13:48] LL |     with_signature(cell, t, |cell, t| require(cell, t));
[01:13:48]    |
[01:13:48]    |
[01:13:48]    = note: defining type: DefId(0:38 ~ projection_one_region_trait_bound_static_closure[317d]::projection_outlives[0]::{{closure}}[0]) with closure substs [
[01:13:48]                '_#1r,
[01:13:48]                '_#2r,
[01:13:48]                T,
[01:13:48]                i32,
[01:13:48]                extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
[01:13:48] 
[01:13:48] note: No external requirements
[01:13:48]   --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs:49:1
[01:13:48]    |
[01:13:48]    |
[01:13:48] LL | / fn projection_outlives<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
[01:13:48] LL | | where
[01:13:48] LL | |     T: Anything<'b>,
[01:13:48] LL | |     T::AssocType: 'a,
[01:13:48] ...  |
[01:13:48] LL | |     with_signature(cell, t, |cell, t| require(cell, t));
[01:13:48]    | |_^
[01:13:48]    |
[01:13:48]    |
[01:13:48]    = note: defining type: DefId(0:34 ~ projection_one_region_trait_bound_static_closure[317d]::projection_outlives[0]) with substs [
[01:13:48]                '_#1r,
[01:13:48]                '_#2r,
[01:13:48]                T,
[01:13:48] 
[01:13:48] note: No external requirements
[01:13:48]   --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs:73:29
[01:13:48]    |
[01:13:48]    |
[01:13:48] LL |     with_signature(cell, t, |cell, t| require(cell, t));
[01:13:48]    |
[01:13:48]    |
[01:13:48]    = note: defining type: DefId(0:43 ~ projection_one_region_trait_bound_static_closure[317d]::elements_outlive[0]::{{closure}}[0]) with closure substs [
[01:13:48]                '_#1r,
[01:13:48]                '_#2r,
[01:13:48]                T,
[01:13:48]                i32,
[01:13:48]                extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
[01:13:48] 
[01:13:48] note: No external requirements
[01:13:48]   --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs:68:1
[01:13:48]    |
[01:13:48]    |
[01:13:48] LL | / fn elements_outlive<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
[01:13:48] LL | | where
[01:13:48] LL | |     T: Anything<'b>,
[01:13:48] LL | |     'b: 'a,
[01:13:48] LL | | {
[01:13:48] LL | |     with_signature(cell, t, |cell, t| require(cell, t));
[01:13:48]    | |_^
[01:13:48]    |
[01:13:48]    |
[01:13:48]    = note: defining type: DefId(0:39 ~ projection_one_region_trait_bound_static_closure[317d]::elements_outlive[0]) with substs [
[01:13:48]                '_#1r,
[01:13:48]                '_#2r,
[01:13:48]                T,
[01:13:48] 
[01:13:48] note: No external requirements
[01:13:48]   --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs:85:29
[01:13:48]    |
[01:13:48]    |
[01:13:48] LL |     with_signature(cell, t, |cell, t| require(cell, t));
[01:13:48]    |
[01:13:48]    |
[01:13:48]    = note: defining type: DefId(0:47 ~ projection_one_region_trait_bound_static_closure[317d]::one_region[0]::{{closure}}[0]) with closure substs [
[01:13:48]                '_#1r,
[01:13:48]                T,
[01:13:48]                i32,
[01:13:48]                extern "rust-call" fn((std::cell::Cell<&'_#2r ()>, T)),
[01:13:48] 
[01:13:48] note: No external requirements
[01:13:48]   --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-static-closure.rs:77:1
[01:13:48]    |
[01:13:48]    |
[01:13:48] LL | / fn one_region<'a, T>(cell: Cell<&'a ()>, t: T)
[01:13:48] LL | | where
[01:13:48] LL | |     T: Anything<'a>,
[01:13:48] LL | | {
[01:13:48] ...  |
[01:13:48] LL | |     with_signature(cell, t, |cell, t| require(cell, t));
[01:13:48]    | |_^
[01:13:48]    |
[01:13:48]    |
[01:13:48]    = note: defining type: DefId(0:44 ~ projection_one_region_trait_bound_static_closure[317d]::one_region[0]) with substs [
[01:13:48]                '_#1r,
[01:13:48]                T,
[01:13:48] 
[01:13:48] 
[01:13:48] ------------------------------------------
[01:13:48] 
---
[01:13:48] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:13:48] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:48] 
[01:13:48] 
[01:13:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:48] 
[01:13:48] 
[01:13:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:48] Build completed unsuccessfully in 0:04:50
[01:13:48] Build completed unsuccessfully in 0:04:50
[01:13:48] make: *** [check] Error 1
[01:13:48] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:039f6bb8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 26 03:59:42 UTC 2019
