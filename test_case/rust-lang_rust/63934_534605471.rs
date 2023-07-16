plain
2019-09-24T14:03:42.4926302Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-24T14:03:43.1079919Z ##[command]git config gc.auto 0
2019-09-24T14:03:43.1084505Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-24T14:03:43.1086547Z ##[command]git config --get-all http.proxy
2019-09-24T14:03:43.1090656Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63934/merge:refs/remotes/pull/63934/merge
---
2019-09-24T15:02:25.9164519Z .................................................................................................... 1500/9042
2019-09-24T15:02:31.6069683Z .................................................................................................... 1600/9042
2019-09-24T15:02:43.2794851Z .........................................................................i...............i.......... 1700/9042
2019-09-24T15:02:49.9586619Z .................................................................................................... 1800/9042
2019-09-24T15:02:58.1114322Z ................................................................iiiii............................... 1900/9042
2019-09-24T15:03:16.2324507Z .................................................................................................... 2100/9042
2019-09-24T15:03:18.6085164Z .................................................................................................... 2200/9042
2019-09-24T15:03:21.6532979Z .................................................................................................... 2300/9042
2019-09-24T15:03:29.4550045Z .................................................................................................... 2400/9042
---
2019-09-24T15:06:18.7704826Z .......................................................i...............i............................ 4700/9042
2019-09-24T15:06:27.5079612Z .................................................................................................... 4800/9042
2019-09-24T15:06:35.5998390Z .................................................................................................... 4900/9042
2019-09-24T15:06:42.5630647Z .................................................................................................... 5000/9042
2019-09-24T15:06:51.7482190Z ..........................................ii.ii..................................................... 5100/9042
2019-09-24T15:07:01.2351606Z .................................................................................................... 5300/9042
2019-09-24T15:07:11.0478725Z .................................................................................................... 5400/9042
2019-09-24T15:07:17.9689348Z .......i............................................................................................ 5500/9042
2019-09-24T15:07:23.0960995Z .................................................................................................... 5600/9042
2019-09-24T15:07:23.0960995Z .................................................................................................... 5600/9042
2019-09-24T15:07:33.9594429Z .................................................................................................... 5700/9042
2019-09-24T15:07:45.9434672Z ..ii...i...ii..........i............................................................................ 5800/9042
2019-09-24T15:08:06.1848269Z .................................................................................................... 6000/9042
2019-09-24T15:08:14.5031020Z .................................................................................................... 6100/9042
2019-09-24T15:08:14.5031020Z .................................................................................................... 6100/9042
2019-09-24T15:08:27.3835054Z ....i..ii........................................................................................... 6200/9042
2019-09-24T15:08:45.7014536Z ................................................................i................................... 6400/9042
2019-09-24T15:08:47.0410623Z .................................................................................................... 6500/9042
2019-09-24T15:08:49.3289660Z ....................................i............................................................... 6600/9042
2019-09-24T15:08:53.1951269Z .................................................................................................... 6700/9042
---
2019-09-24T15:12:39.6802226Z 
2019-09-24T15:12:39.6803165Z ---- [ui] ui/impl-trait/negative-reasoning.rs stdout ----
2019-09-24T15:12:39.6804117Z diff of stderr:
2019-09-24T15:12:39.6804431Z 
2019-09-24T15:12:39.6804683Z 7 LL | impl AnotherTrait for D<OpaqueType> {
2019-09-24T15:12:39.6804942Z 8    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `D<OpaqueType>`
2019-09-24T15:12:39.6805212Z 9    |
2019-09-24T15:12:39.6805834Z -    = note: upstream crates may add new impl of trait `std::fmt::Debug` for type `OpaqueType` in future versions
2019-09-24T15:12:39.6806183Z +    = note: upstream crates may add a new impl of trait `std::fmt::Debug` for type `OpaqueType` in future versions
2019-09-24T15:12:39.6806719Z 12 error: aborting due to previous error
2019-09-24T15:12:39.6806986Z 13 
2019-09-24T15:12:39.6807200Z 
2019-09-24T15:12:39.6807564Z 
2019-09-24T15:12:39.6807564Z 
2019-09-24T15:12:39.6807762Z The actual stderr differed from the expected stderr.
2019-09-24T15:12:39.6808298Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/negative-reasoning/negative-reasoning.stderr
2019-09-24T15:12:39.6808763Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T15:12:39.6809302Z To only update this specific test, also pass `--test-args impl-trait/negative-reasoning.rs`
2019-09-24T15:12:39.6809758Z error: 1 errors occurred comparing output.
2019-09-24T15:12:39.6809988Z status: exit code: 1
2019-09-24T15:12:39.6809988Z status: exit code: 1
2019-09-24T15:12:39.6810838Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/negative-reasoning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/negative-reasoning" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/negative-reasoning/auxiliary" "-A" "unused"
2019-09-24T15:12:39.6811873Z ------------------------------------------
2019-09-24T15:12:39.6812116Z 
2019-09-24T15:12:39.6812571Z ------------------------------------------
2019-09-24T15:12:39.6812822Z stderr:
2019-09-24T15:12:39.6812822Z stderr:
2019-09-24T15:12:39.6813247Z ------------------------------------------
2019-09-24T15:12:39.6814000Z error[E0119]: conflicting implementations of trait `AnotherTrait` for type `D<OpaqueType>`:
2019-09-24T15:12:39.6814584Z   --> /checkout/src/test/ui/impl-trait/negative-reasoning.rs:18:1
2019-09-24T15:12:39.6814918Z    |
2019-09-24T15:12:39.6815186Z LL | impl<T: std::fmt::Debug> AnotherTrait for T { }
2019-09-24T15:12:39.6815711Z    | ------------------------------------------- first implementation here
2019-09-24T15:12:39.6816032Z ...
2019-09-24T15:12:39.6816273Z LL | impl AnotherTrait for D<OpaqueType> {
2019-09-24T15:12:39.6816536Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `D<OpaqueType>`
2019-09-24T15:12:39.6816808Z    |
2019-09-24T15:12:39.6817186Z    = note: upstream crates may add a new impl of trait `std::fmt::Debug` for type `OpaqueType` in future versions
2019-09-24T15:12:39.6817580Z error: aborting due to previous error
2019-09-24T15:12:39.6817758Z 
2019-09-24T15:12:39.6818201Z For more information about this error, try `rustc --explain E0119`.
2019-09-24T15:12:39.6818458Z 
---
2019-09-24T15:12:39.6838199Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-24T15:12:39.6838564Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-24T15:12:39.6854159Z 
2019-09-24T15:12:39.6857623Z 
2019-09-24T15:12:39.6859586Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-24T15:12:39.6859862Z 
2019-09-24T15:12:39.6859929Z 
2019-09-24T15:12:39.6867743Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-24T15:12:39.6868247Z Build completed unsuccessfully in 1:02:02
2019-09-24T15:12:39.6868247Z Build completed unsuccessfully in 1:02:02
2019-09-24T15:12:39.6925862Z == clock drift check ==
2019-09-24T15:12:39.6944166Z   local time: Tue Sep 24 15:12:39 UTC 2019
2019-09-24T15:12:39.7806512Z   network time: Tue, 24 Sep 2019 15:12:39 GMT
2019-09-24T15:12:39.7811038Z == end clock drift check ==
2019-09-24T15:12:40.5225621Z ##[error]Bash exited with code '1'.
2019-09-24T15:12:40.5260154Z ##[section]Starting: Checkout
2019-09-24T15:12:40.5261841Z ==============================================================================
2019-09-24T15:12:40.5261884Z Task         : Get sources
2019-09-24T15:12:40.5261943Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
