plain
2019-10-03T08:24:33.0473926Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T08:24:33.0666964Z ##[command]git config gc.auto 0
2019-10-03T08:24:33.0745944Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T08:24:33.0808767Z ##[command]git config --get-all http.proxy
2019-10-03T08:24:33.0965125Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64706/merge:refs/remotes/pull/64706/merge
---
2019-10-03T09:32:49.9934798Z .................................................................................................... 1500/9096
2019-10-03T09:32:57.6656923Z .................................................................................................... 1600/9096
2019-10-03T09:33:08.3906180Z .................................................................................................... 1700/9096
2019-10-03T09:33:18.4160597Z ...i...............i................................................................................ 1800/9096
2019-10-03T09:33:26.4830354Z ..............................................................................................iiiii. 1900/9096
2019-10-03T09:33:51.8483745Z .................................................................................................... 2100/9096
2019-10-03T09:33:54.6394843Z .................................................................................................... 2200/9096
2019-10-03T09:33:57.5658790Z .................................................................................................... 2300/9096
2019-10-03T09:34:04.8523449Z .................................................................................................... 2400/9096
---
2019-10-03T09:37:29.4431504Z .................................................................................i...............i.. 4700/9096
2019-10-03T09:37:38.7986925Z .................................................................................................... 4800/9096
2019-10-03T09:37:50.4910608Z .................................................................................................... 4900/9096
2019-10-03T09:37:57.1841372Z ..............................F..................................................................... 5000/9096
2019-10-03T09:38:10.4840531Z ..........................................................................ii.ii..................... 5100/9096
2019-10-03T09:38:21.9745211Z .................................................................................................... 5300/9096
2019-10-03T09:38:33.3701525Z .................................................................................................... 5400/9096
2019-10-03T09:38:41.4497155Z ........................................i........................................................... 5500/9096
2019-10-03T09:38:49.0891929Z .................................................................................................... 5600/9096
2019-10-03T09:38:49.0891929Z .................................................................................................... 5600/9096
2019-10-03T09:39:01.3028868Z .................................................................................................... 5700/9096
2019-10-03T09:39:14.3102350Z .....................................ii...i..ii...........i......................................... 5800/9096
2019-10-03T09:39:39.9315660Z .................................................................................................... 6000/9096
2019-10-03T09:39:50.6673270Z .................................................................................................... 6100/9096
2019-10-03T09:39:50.6673270Z .................................................................................................... 6100/9096
2019-10-03T09:40:08.9534691Z ..........................................i..ii..................................................... 6200/9096
2019-10-03T09:40:33.9548719Z .................................................................................................... 6400/9096
2019-10-03T09:40:36.4952911Z ..i................................................................................................. 6500/9096
2019-10-03T09:40:39.1619912Z ..........................................................................i......................... 6600/9096
2019-10-03T09:40:42.5624846Z .................................................................................................... 6700/9096
---
2019-10-03T09:45:23.9277775Z 
2019-10-03T09:45:23.9279298Z ---- [ui] ui/issues/issue-60218.rs stdout ----
2019-10-03T09:45:23.9279572Z diff of stderr:
2019-10-03T09:45:23.9279721Z 
2019-10-03T09:45:23.9280262Z 1 error[E0277]: the trait bound `for<'t> <std::iter::Map<<&'t _ as std::iter::IntoIterator>::IntoIter, _> as std::iter::Iterator>::Item: Foo` is not satisfied
2019-10-03T09:45:23.9280734Z 2   --> $DIR/issue-60218.rs:18:5
2019-10-03T09:45:23.9280923Z 3    |
2019-10-03T09:45:23.9281913Z - LL | / pub fn trigger_error<I, F>(iterable: I, functor: F)
2019-10-03T09:45:23.9282291Z - LL | | where
2019-10-03T09:45:23.9282682Z - LL | |     for<'t> &'t I: IntoIterator,
2019-10-03T09:45:23.9283123Z - LL | | for<'t> Map<<&'t I as IntoIterator>::IntoIter, F>: Iterator,
2019-10-03T09:45:23.9283549Z - LL | | for<'t> <Map<<&'t I as IntoIterator>::IntoIter, F> as Iterator>::Item: Foo,
2019-10-03T09:45:23.9283928Z - LL | | {
2019-10-03T09:45:23.9284621Z - LL | | }
2019-10-03T09:45:23.9285373Z -    | |_- required by `trigger_error`
2019-10-03T09:45:23.9285604Z + LL | pub fn trigger_error<I, F>(iterable: I, functor: F)
2019-10-03T09:45:23.9286137Z 12 ...
2019-10-03T09:45:23.9286137Z 12 ...
2019-10-03T09:45:23.9286530Z - LL |       trigger_error(vec![], |x: &u32| x)
2019-10-03T09:45:23.9287055Z -    |       ^^^^^^^^^^^^^ the trait `for<'t> Foo` is not implemented for `<std::iter::Map<<&'t _ as std::iter::IntoIterator>::IntoIter, _> as std::iter::Iterator>::Item`
2019-10-03T09:45:23.9287852Z + LL | for<'t> <Map<<&'t I as IntoIterator>::IntoIter, F> as Iterator>::Item: Foo,
2019-10-03T09:45:23.9288917Z +    |                                                                        --- required by this bound in `trigger_error`
2019-10-03T09:45:23.9289390Z + ...
2019-10-03T09:45:23.9289598Z + LL |     trigger_error(vec![], |x: &u32| x)
2019-10-03T09:45:23.9290112Z +    |     ^^^^^^^^^^^^^ the trait `for<'t> Foo` is not implemented for `<std::iter::Map<<&'t _ as std::iter::IntoIterator>::IntoIter, _> as std::iter::Iterator>::Item`
2019-10-03T09:45:23.9290489Z 16 error: aborting due to previous error
2019-10-03T09:45:23.9290632Z 17 
2019-10-03T09:45:23.9290779Z 
2019-10-03T09:45:23.9290903Z 
2019-10-03T09:45:23.9290903Z 
2019-10-03T09:45:23.9291047Z The actual stderr differed from the expected stderr.
2019-10-03T09:45:23.9291973Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60218/issue-60218.stderr
2019-10-03T09:45:23.9292557Z To update references, rerun the tests and pass the `--bless` flag
2019-10-03T09:45:23.9293126Z To only update this specific test, also pass `--test-args issues/issue-60218.rs`
2019-10-03T09:45:23.9294486Z error: 1 errors occurred comparing output.
2019-10-03T09:45:23.9294741Z status: exit code: 1
2019-10-03T09:45:23.9294741Z status: exit code: 1
2019-10-03T09:45:23.9295787Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60218.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60218" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60218/auxiliary" "-A" "unused"
2019-10-03T09:45:23.9296973Z ------------------------------------------
2019-10-03T09:45:23.9297014Z 
2019-10-03T09:45:23.9297255Z ------------------------------------------
2019-10-03T09:45:23.9297304Z stderr:
2019-10-03T09:45:23.9297304Z stderr:
2019-10-03T09:45:23.9297519Z ------------------------------------------
2019-10-03T09:45:23.9297855Z error[E0277]: the trait bound `for<'t> <std::iter::Map<<&'t _ as std::iter::IntoIterator>::IntoIter, _> as std::iter::Iterator>::Item: Foo` is not satisfied
2019-10-03T09:45:23.9298123Z   --> /checkout/src/test/ui/issues/issue-60218.rs:18:5
2019-10-03T09:45:23.9298175Z    |
2019-10-03T09:45:23.9298244Z LL | pub fn trigger_error<I, F>(iterable: I, functor: F)
2019-10-03T09:45:23.9298498Z ...
2019-10-03T09:45:23.9298498Z ...
2019-10-03T09:45:23.9298749Z LL | for<'t> <Map<<&'t I as IntoIterator>::IntoIter, F> as Iterator>::Item: Foo,
2019-10-03T09:45:23.9299065Z    |                                                                        --- required by this bound in `trigger_error`
2019-10-03T09:45:23.9299134Z ...
2019-10-03T09:45:23.9299183Z LL |     trigger_error(vec![], |x: &u32| x) //~ ERROR E0277
2019-10-03T09:45:23.9299542Z    |     ^^^^^^^^^^^^^ the trait `for<'t> Foo` is not implemented for `<std::iter::Map<<&'t _ as std::iter::IntoIterator>::IntoIter, _> as std::iter::Iterator>::Item`
2019-10-03T09:45:23.9299634Z error: aborting due to previous error
2019-10-03T09:45:23.9299690Z 
2019-10-03T09:45:23.9299934Z For more information about this error, try `rustc --explain E0277`.
2019-10-03T09:45:23.9299971Z 
---
2019-10-03T09:45:23.9337063Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-03T09:45:23.9337151Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-03T09:45:24.5393241Z 
2019-10-03T09:45:24.5393360Z 
2019-10-03T09:45:24.5396114Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-03T09:45:24.5396416Z 
2019-10-03T09:45:24.5396448Z 
2019-10-03T09:45:24.5396498Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-03T09:45:24.5396570Z Build completed unsuccessfully in 1:13:17
2019-10-03T09:45:24.5396570Z Build completed unsuccessfully in 1:13:17
2019-10-03T09:45:24.5396620Z == clock drift check ==
2019-10-03T09:45:24.5396669Z   local time: Thu Oct  3 09:45:23 UTC 2019
2019-10-03T09:45:24.5396745Z   network time: Thu, 03 Oct 2019 09:45:24 GMT
2019-10-03T09:45:24.5396802Z == end clock drift check ==
2019-10-03T09:45:25.0726506Z ##[error]Bash exited with code '1'.
2019-10-03T09:45:25.0769950Z ##[section]Starting: Checkout
2019-10-03T09:45:25.0772176Z ==============================================================================
2019-10-03T09:45:25.0772255Z Task         : Get sources
2019-10-03T09:45:25.0772305Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
