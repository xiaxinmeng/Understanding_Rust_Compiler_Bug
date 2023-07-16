plain
2019-09-20T08:26:30.1401766Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-20T08:26:30.1597609Z ##[command]git config gc.auto 0
2019-09-20T08:26:30.1691988Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-20T08:26:30.1755016Z ##[command]git config --get-all http.proxy
2019-09-20T08:26:30.1904634Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62330/merge:refs/remotes/pull/62330/merge
---
2019-09-20T09:33:14.6644554Z .................................................................................................... 1500/9026
2019-09-20T09:33:21.3068860Z .................................................................................................... 1600/9026
2019-09-20T09:33:34.9511974Z ..................................................................i...............i................. 1700/9026
2019-09-20T09:33:42.7821650Z .................................................................................................... 1800/9026
2019-09-20T09:33:59.5298914Z .........................................................iiiii...................................... 1900/9026
2019-09-20T09:34:12.5972432Z .................................................................................................... 2100/9026
2019-09-20T09:34:15.4131779Z .................................................................................................... 2200/9026
2019-09-20T09:34:19.1016564Z .................................................................................................... 2300/9026
2019-09-20T09:34:28.3367596Z .................................................................................................... 2400/9026
---
2019-09-20T09:37:42.6586362Z .............................................i...............i...................................... 4700/9026
2019-09-20T09:37:54.1318533Z .................................................................................................... 4800/9026
2019-09-20T09:38:02.1739195Z .................................................................................................... 4900/9026
2019-09-20T09:38:12.6451799Z .................................................................................................... 5000/9026
2019-09-20T09:38:21.0279204Z .............................ii.ii.................................................................. 5100/9026
2019-09-20T09:38:31.9728352Z .................................................................................................... 5300/9026
2019-09-20T09:38:43.4285769Z .............................................................................................i...... 5400/9026
2019-09-20T09:38:52.4976306Z .................................................................................................... 5500/9026
2019-09-20T09:38:57.6690000Z .................................................................................................... 5600/9026
2019-09-20T09:38:57.6690000Z .................................................................................................... 5600/9026
2019-09-20T09:39:09.1573947Z ........................................................................................ii...i..ii.. 5700/9026
2019-09-20T09:39:37.4715782Z .................................................................................................... 5900/9026
2019-09-20T09:39:48.2102436Z .................................................................................................... 6000/9026
2019-09-20T09:39:48.2102436Z .................................................................................................... 6000/9026
2019-09-20T09:39:57.7238760Z ..........................................................................................i..ii..... 6100/9026
2019-09-20T09:40:28.1313701Z .................................................................................................... 6300/9026
2019-09-20T09:40:33.0133488Z ..................................................i................................................. 6400/9026
2019-09-20T09:40:35.4398125Z .................................................................................................... 6500/9026
2019-09-20T09:40:38.1453255Z .....................i.............................................................................. 6600/9026
---
2019-09-20T09:45:06.4479542Z diff of stderr:
2019-09-20T09:45:06.4479578Z 
2019-09-20T09:45:06.4479847Z 13    | ----------- method `clone` not found for this
2019-09-20T09:45:06.4479900Z 14 ...
2019-09-20T09:45:06.4479963Z 15 LL |     let w = u.clone();
2019-09-20T09:45:06.4480222Z -    |               ^^^^^ method not found in `U4<CloneNoCopy>`
2019-09-20T09:45:06.4480288Z +    |               ^^^^^ method not found in `U5<CloneNoCopy>`
2019-09-20T09:45:06.4480459Z 18    = note: the method `clone` exists but the following trait bounds were not satisfied:
2019-09-20T09:45:06.4480459Z 18    = note: the method `clone` exists but the following trait bounds were not satisfied:
2019-09-20T09:45:06.4480520Z 19            `U5<CloneNoCopy> : std::clone::Clone`
2019-09-20T09:45:06.4480600Z 
2019-09-20T09:45:06.4480651Z The actual stderr differed from the expected stderr.
2019-09-20T09:45:06.4481017Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone/union-derive-clone.stderr
2019-09-20T09:45:06.4481017Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone/union-derive-clone.stderr
2019-09-20T09:45:06.4481320Z To update references, rerun the tests and pass the `--bless` flag
2019-09-20T09:45:06.4481613Z To only update this specific test, also pass `--test-args union/union-derive-clone.rs`
2019-09-20T09:45:06.4481718Z error: 1 errors occurred comparing output.
2019-09-20T09:45:06.4481769Z status: exit code: 1
2019-09-20T09:45:06.4481769Z status: exit code: 1
2019-09-20T09:45:06.4482535Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-derive-clone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone/auxiliary" "-A" "unused"
2019-09-20T09:45:06.4483192Z ------------------------------------------
2019-09-20T09:45:06.4483249Z 
2019-09-20T09:45:06.4483500Z ------------------------------------------
2019-09-20T09:45:06.4483552Z stderr:
2019-09-20T09:45:06.4483552Z stderr:
2019-09-20T09:45:06.4483791Z ------------------------------------------
2019-09-20T09:45:06.4483865Z error[E0277]: the trait bound `U1: std::marker::Copy` is not satisfied
2019-09-20T09:45:06.4484138Z   --> /checkout/src/test/ui/union/union-derive-clone.rs:5:10
2019-09-20T09:45:06.4484208Z    |
2019-09-20T09:45:06.4484276Z LL | #[derive(Clone)] //~ ERROR the trait bound `U1: std::marker::Copy` is not satisfied
2019-09-20T09:45:06.4484470Z    |          ^^^^^ the trait `std::marker::Copy` is not implemented for `U1`
2019-09-20T09:45:06.4484811Z    |
2019-09-20T09:45:06.4484888Z    = note: required by `std::clone::AssertParamIsCopy`
2019-09-20T09:45:06.4484921Z 
2019-09-20T09:45:06.4484974Z error[E0599]: no method named `clone` found for type `U5<CloneNoCopy>` in the current scope
2019-09-20T09:45:06.4485411Z    |
2019-09-20T09:45:06.4485411Z    |
2019-09-20T09:45:06.4485456Z LL | union U5<T> {
2019-09-20T09:45:06.4485705Z    | ----------- method `clone` not found for this
2019-09-20T09:45:06.4485755Z ...
2019-09-20T09:45:06.4485810Z LL |     let w = u.clone(); //~ ERROR no method named `clone` found for type `U5<CloneNoCopy>`
2019-09-20T09:45:06.4485880Z    |               ^^^^^ method not found in `U5<CloneNoCopy>`
2019-09-20T09:45:06.4485995Z    = note: the method `clone` exists but the following trait bounds were not satisfied:
2019-09-20T09:45:06.4485995Z    = note: the method `clone` exists but the following trait bounds were not satisfied:
2019-09-20T09:45:06.4486056Z            `U5<CloneNoCopy> : std::clone::Clone`
2019-09-20T09:45:06.4486126Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-09-20T09:45:06.4486186Z    = note: the following trait defines an item `clone`, perhaps you need to implement it:
2019-09-20T09:45:06.4486242Z            candidate #1: `std::clone::Clone`
2019-09-20T09:45:06.4486334Z error: aborting due to 2 previous errors
2019-09-20T09:45:06.4486365Z 
2019-09-20T09:45:06.4486412Z Some errors have detailed explanations: E0277, E0599.
2019-09-20T09:45:06.4486685Z For more information about an error, try `rustc --explain E0277`.
---
2019-09-20T09:45:06.4527732Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-20T09:45:06.4528501Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-20T09:45:06.4550835Z 
2019-09-20T09:45:06.4550973Z 
2019-09-20T09:45:06.4557777Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-20T09:45:06.4558347Z 
2019-09-20T09:45:06.4558384Z 
2019-09-20T09:45:06.4569036Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-20T09:45:06.4569159Z Build completed unsuccessfully in 1:11:16
2019-09-20T09:45:06.4569159Z Build completed unsuccessfully in 1:11:16
2019-09-20T09:45:06.4638096Z == clock drift check ==
2019-09-20T09:45:06.4653156Z   local time: Fri Sep 20 09:45:06 UTC 2019
2019-09-20T09:45:06.6160758Z   network time: Fri, 20 Sep 2019 09:45:06 GMT
2019-09-20T09:45:06.6161273Z == end clock drift check ==
2019-09-20T09:45:07.4247635Z ##[error]Bash exited with code '1'.
2019-09-20T09:45:07.4286132Z ##[section]Starting: Checkout
2019-09-20T09:45:07.4288091Z ==============================================================================
2019-09-20T09:45:07.4288149Z Task         : Get sources
2019-09-20T09:45:07.4288200Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
