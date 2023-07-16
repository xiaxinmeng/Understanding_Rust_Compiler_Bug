plain
2019-10-29T16:23:48.2704877Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-29T16:23:48.2901021Z ##[command]git config gc.auto 0
2019-10-29T16:23:48.2981064Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-29T16:23:48.3052160Z ##[command]git config --get-all http.proxy
2019-10-29T16:23:49.0716978Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65937/merge:refs/remotes/pull/65937/merge
---
2019-10-29T17:23:26.1095970Z .................................................................................................... 1600/9261
2019-10-29T17:23:31.8928525Z .................................................................................................... 1700/9261
2019-10-29T17:23:44.1822376Z ..........................................................i...............i......................... 1800/9261
2019-10-29T17:23:51.6662892Z .................................................................................................... 1900/9261
2019-10-29T17:24:06.1948099Z ................................................iiiii............................................... 2000/9261
2019-10-29T17:24:16.7663128Z .................................................................................................... 2200/9261
2019-10-29T17:24:19.2919257Z .................................................................................................... 2300/9261
2019-10-29T17:24:23.0000723Z .................................................................................................... 2400/9261
2019-10-29T17:24:46.1145038Z .................................................................................................... 2500/9261
---
2019-10-29T17:27:35.4910937Z .................................................i...............i.................................. 4800/9261
2019-10-29T17:27:44.2761776Z .................................................................................................... 4900/9261
2019-10-29T17:27:52.6230187Z .................................................................................................... 5000/9261
2019-10-29T17:27:58.8344463Z .................................................................................................... 5100/9261
2019-10-29T17:28:09.0704065Z ..................................................ii.ii...........i................................. 5200/9261
2019-10-29T17:28:18.5986327Z .................................................................................................... 5400/9261
2019-10-29T17:28:27.7674418Z .................................................................................................... 5500/9261
2019-10-29T17:28:35.2267698Z ......................i............................................................................. 5600/9261
2019-10-29T17:28:41.2342135Z .................................................................................................... 5700/9261
2019-10-29T17:28:41.2342135Z .................................................................................................... 5700/9261
2019-10-29T17:28:52.8291886Z .................................................................................................... 5800/9261
2019-10-29T17:29:04.7620263Z .......ii...i..ii...........i....................................................................... 5900/9261
2019-10-29T17:29:26.2281476Z .................................................................................................... 6100/9261
2019-10-29T17:29:32.6752596Z .................................................................................................... 6200/9261
2019-10-29T17:29:32.6752596Z .................................................................................................... 6200/9261
2019-10-29T17:29:46.6542867Z ..........................i..ii..................................................................... 6300/9261
2019-10-29T17:30:06.6757950Z ............................................................................................i....... 6500/9261
2019-10-29T17:30:08.9299880Z .................................................................................................... 6600/9261
2019-10-29T17:30:11.1955985Z ...................................................................i................................ 6700/9261
2019-10-29T17:30:14.1383330Z .................................................................................................... 6800/9261
---
2019-10-29T17:34:16.0604397Z 57    |
2019-10-29T17:34:16.0604463Z 58 LL |     async fn foo() {}
2019-10-29T17:34:16.0604513Z 59    |     ^^^^^^^^^^^^^^^^^
2019-10-29T17:34:16.0604559Z +    |
2019-10-29T17:34:16.0604632Z +    = note: Due to technical restrictions rust does not currently support `async` trait fns.
2019-10-29T17:34:16.0605162Z +    = note: Consider using the `async-trait` crate in the meantime until further notice.
2019-10-29T17:34:16.0605311Z 61 error: aborting due to 10 previous errors
2019-10-29T17:34:16.0605360Z 62 
2019-10-29T17:34:16.0605391Z 
2019-10-29T17:34:16.0605419Z 
2019-10-29T17:34:16.0605419Z 
2019-10-29T17:34:16.0605484Z The actual stderr differed from the expected stderr.
2019-10-29T17:34:16.0605904Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015/edition-deny-async-fns-2015.stderr
2019-10-29T17:34:16.0606185Z To update references, rerun the tests and pass the `--bless` flag
2019-10-29T17:34:16.0606539Z To only update this specific test, also pass `--test-args async-await/edition-deny-async-fns-2015.rs`
2019-10-29T17:34:16.0606630Z error: 1 errors occurred comparing output.
2019-10-29T17:34:16.0606680Z status: exit code: 1
2019-10-29T17:34:16.0606680Z status: exit code: 1
2019-10-29T17:34:16.0607578Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015/auxiliary" "-A" "unused"
2019-10-29T17:34:16.0607949Z ------------------------------------------
2019-10-29T17:34:16.0608108Z 
2019-10-29T17:34:16.0608381Z ------------------------------------------
2019-10-29T17:34:16.0608455Z stderr:
2019-10-29T17:34:16.0608455Z stderr:
2019-10-29T17:34:16.0608696Z ------------------------------------------
2019-10-29T17:34:16.0608754Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0609067Z   --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:3:1
2019-10-29T17:34:16.0609126Z    |
2019-10-29T17:34:16.0609180Z LL | async fn foo() {} //~ ERROR `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0609285Z 
2019-10-29T17:34:16.0609335Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0609645Z   --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:5:12
2019-10-29T17:34:16.0609700Z    |
2019-10-29T17:34:16.0609700Z    |
2019-10-29T17:34:16.0609757Z LL | fn baz() { async fn foo() {} } //~ ERROR `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0609868Z 
2019-10-29T17:34:16.0609918Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0610203Z   --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:7:1
2019-10-29T17:34:16.0610274Z    |
2019-10-29T17:34:16.0610274Z    |
2019-10-29T17:34:16.0610328Z LL | async fn async_baz() { //~ ERROR `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0610419Z 
2019-10-29T17:34:16.0610490Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0610777Z   --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:8:5
2019-10-29T17:34:16.0610829Z    |
2019-10-29T17:34:16.0610829Z    |
2019-10-29T17:34:16.0610901Z LL |     async fn bar() {} //~ ERROR `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0610983Z 
2019-10-29T17:34:16.0611049Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0611335Z   --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:14:5
2019-10-29T17:34:16.0611398Z    |
2019-10-29T17:34:16.0611398Z    |
2019-10-29T17:34:16.0611452Z LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0611552Z 
2019-10-29T17:34:16.0611600Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0611988Z   --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:18:5
2019-10-29T17:34:16.0612049Z    |
2019-10-29T17:34:16.0612049Z    |
2019-10-29T17:34:16.0612102Z LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0612202Z 
2019-10-29T17:34:16.0612252Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0612562Z   --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:36:9
2019-10-29T17:34:16.0612632Z    |
2019-10-29T17:34:16.0612632Z    |
2019-10-29T17:34:16.0612686Z LL |         async fn bar() {} //~ ERROR `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0612780Z 
2019-10-29T17:34:16.0612847Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0613575Z   --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:26:9
2019-10-29T17:34:16.0613643Z    |
2019-10-29T17:34:16.0613643Z    |
2019-10-29T17:34:16.0613732Z LL |         async fn foo() {} //~ ERROR `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0613817Z 
2019-10-29T17:34:16.0613883Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0614184Z   --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:31:13
2019-10-29T17:34:16.0614237Z    |
2019-10-29T17:34:16.0614237Z    |
2019-10-29T17:34:16.0614292Z LL |             async fn bar() {} //~ ERROR `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0614397Z 
2019-10-29T17:34:16.0614397Z 
2019-10-29T17:34:16.0614446Z error[E0706]: trait fns cannot be declared `async`
2019-10-29T17:34:16.0614913Z   --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:18:5
2019-10-29T17:34:16.0614969Z    |
2019-10-29T17:34:16.0615021Z LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in the 2015 edition
2019-10-29T17:34:16.0615136Z    |
2019-10-29T17:34:16.0615136Z    |
2019-10-29T17:34:16.0615200Z    = note: Due to technical restrictions rust does not currently support `async` trait fns.
2019-10-29T17:34:16.0615501Z    = note: Consider using the `async-trait` crate in the meantime until further notice.
2019-10-29T17:34:16.0615608Z error: aborting due to 10 previous errors
2019-10-29T17:34:16.0615641Z 
2019-10-29T17:34:16.0615912Z For more information about this error, try `rustc --explain E0670`.
2019-10-29T17:34:16.0615968Z 
---
2019-10-29T17:34:16.0655362Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-29T17:34:16.0655546Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-29T17:34:16.0674252Z 
2019-10-29T17:34:16.0674737Z 
2019-10-29T17:34:16.0676697Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-29T17:34:16.0677179Z 
2019-10-29T17:34:16.0677305Z 
2019-10-29T17:34:16.0690270Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-29T17:34:16.0690340Z Build completed unsuccessfully in 1:03:46
2019-10-29T17:34:16.0690340Z Build completed unsuccessfully in 1:03:46
2019-10-29T17:34:16.0748728Z == clock drift check ==
2019-10-29T17:34:16.0762367Z   local time: Tue Oct 29 17:34:16 UTC 2019
2019-10-29T17:34:16.3600745Z   network time: Tue, 29 Oct 2019 17:34:16 GMT
2019-10-29T17:34:16.3603855Z == end clock drift check ==
2019-10-29T17:34:17.5731587Z 
2019-10-29T17:34:17.5836007Z ##[error]Bash exited with code '1'.
2019-10-29T17:34:17.5935462Z ##[section]Starting: Checkout
2019-10-29T17:34:17.5937283Z ==============================================================================
2019-10-29T17:34:17.5937346Z Task         : Get sources
2019-10-29T17:34:17.5937398Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
