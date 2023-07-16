plain
2020-02-09T23:49:06.4732820Z ========================== Starting Command Output ===========================
2020-02-09T23:49:06.4735469Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/27782079-9eca-4014-a074-5a0218370543.sh
2020-02-09T23:49:06.4735663Z 
2020-02-09T23:49:06.4739264Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-09T23:49:06.4744787Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69008/merge to s
2020-02-09T23:49:06.4746179Z Task         : Get sources
2020-02-09T23:49:06.4746207Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T23:49:06.4746234Z Version      : 1.0.0
2020-02-09T23:49:06.4746271Z Author       : Microsoft
---
2020-02-09T23:49:07.3693124Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-09T23:49:07.3801212Z ##[command]git config gc.auto 0
2020-02-09T23:49:07.3885365Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-09T23:49:07.3947636Z ##[command]git config --get-all http.proxy
2020-02-09T23:49:07.4112454Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69008/merge:refs/remotes/pull/69008/merge
---
2020-02-10T00:44:13.9839723Z .................................................................................................... 1700/9622
2020-02-10T00:44:18.6432411Z .................................................................................................... 1800/9622
2020-02-10T00:44:30.1578796Z ............................i....................................................................... 1900/9622
2020-02-10T00:44:36.6499611Z .................................................................................................... 2000/9622
2020-02-10T00:44:49.6924062Z ..................iiiii............................................................................. 2100/9622
2020-02-10T00:44:58.6300249Z .................................................................................................... 2300/9622
2020-02-10T00:45:00.9632910Z .................................................................................................... 2400/9622
2020-02-10T00:45:05.6954445Z .................................................................................................... 2500/9622
2020-02-10T00:45:25.1307720Z .................................................................................................... 2600/9622
---
2020-02-10T00:47:55.2910400Z .....................................................................i...............i.............. 4900/9622
2020-02-10T00:48:03.0119854Z .................................................................................................... 5000/9622
2020-02-10T00:48:10.8433096Z .................................................................................................... 5100/9622
2020-02-10T00:48:15.6325823Z ...........i........................................................................................ 5200/9622
2020-02-10T00:48:26.3436093Z .....................................................................................ii.ii........i. 5300/9622
2020-02-10T00:48:30.0207279Z ..i................................................................................................. 5400/9622
2020-02-10T00:48:41.2424683Z .................................................................................................... 5600/9622
2020-02-10T00:48:49.3334145Z .........................................................................i.......................... 5700/9622
2020-02-10T00:48:56.1268574Z .................................................................................................... 5800/9622
2020-02-10T00:49:02.1335641Z .................................................................................................... 5900/9622
2020-02-10T00:49:02.1335641Z .................................................................................................... 5900/9622
2020-02-10T00:49:11.7242253Z .................................................................ii...i..ii...........i............. 6000/9622
2020-02-10T00:49:31.6669889Z .................................................................................................... 6200/9622
2020-02-10T00:49:38.8349738Z .................................................................................................... 6300/9622
2020-02-10T00:49:38.8349738Z .................................................................................................... 6300/9622
2020-02-10T00:49:46.3098870Z .............................................................................................i..ii.. 6400/9622
2020-02-10T00:50:09.4006811Z .................................................................................................... 6600/9622
2020-02-10T00:50:18.5334529Z ................................................................................i................... 6700/9622
2020-02-10T00:50:20.5411855Z .................................................................................................... 6800/9622
2020-02-10T00:50:22.6287283Z .......................................................................................i............ 6900/9622
---
2020-02-10T00:51:53.1952645Z .................................................................................................... 7600/9622
2020-02-10T00:51:56.9763571Z .................................................................................................... 7700/9622
2020-02-10T00:52:01.8898255Z .................................................................................................... 7800/9622
2020-02-10T00:52:09.7477141Z .................................................................................................... 7900/9622
2020-02-10T00:52:18.3905194Z ...............................................................iiiiiii.i............................ 8000/9622
2020-02-10T00:52:32.0637087Z ...i......i......................................................................................... 8200/9622
2020-02-10T00:52:37.1437636Z .................................................................................................... 8300/9622
2020-02-10T00:52:49.9254001Z .................................................................................................... 8400/9622
2020-02-10T00:52:57.7148387Z .................................................................................................... 8500/9622
---
2020-02-10T00:54:46.8485041Z - error: aborting due to 4 previous errors
2020-02-10T00:54:46.8485193Z + error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8485519Z +   --> $DIR/bindings.rs:5:16
2020-02-10T00:54:46.8485661Z +    |
2020-02-10T00:54:46.8485791Z + LL | fn a<T: Clone>(x: T) {
2020-02-10T00:54:46.8486110Z +    |      -- help: consider further restricting this bound: `T: std::clone::Clone +`
2020-02-10T00:54:46.8486259Z + LL |     const foo: impl Clone = x;
2020-02-10T00:54:46.8486513Z +    |
2020-02-10T00:54:46.8486638Z +    = note: the return type of a function must have a statically known size
2020-02-10T00:54:46.8486774Z 34 
2020-02-10T00:54:46.8487077Z - For more information about this error, try `rustc --explain E0435`.
2020-02-10T00:54:46.8487077Z - For more information about this error, try `rustc --explain E0435`.
2020-02-10T00:54:46.8487226Z + error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8487520Z +   --> $DIR/bindings.rs:11:20
2020-02-10T00:54:46.8487662Z +    |
2020-02-10T00:54:46.8487793Z + LL | fn b<T: Clone>(x: T) {
2020-02-10T00:54:46.8488126Z +    |      -- help: consider further restricting this bound: `T: std::clone::Clone +`
2020-02-10T00:54:46.8488286Z + LL |     let _ = move || {
2020-02-10T00:54:46.8488595Z + LL |         const foo: impl Clone = x;
2020-02-10T00:54:46.8488836Z +    |
2020-02-10T00:54:46.8488983Z +    = note: the return type of a function must have a statically known size
2020-02-10T00:54:46.8489275Z + 
2020-02-10T00:54:46.8489402Z + error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8489402Z + error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8489963Z +   --> $DIR/bindings.rs:18:20
2020-02-10T00:54:46.8490111Z +    |
2020-02-10T00:54:46.8490231Z + LL | trait Foo<T: Clone> {
2020-02-10T00:54:46.8490843Z +    |           -- help: consider further restricting this bound: `T: std::clone::Clone +`
2020-02-10T00:54:46.8491015Z + LL |     fn a(x: T) {
2020-02-10T00:54:46.8491159Z + LL |         const foo: impl Clone = x;
2020-02-10T00:54:46.8491424Z +    |
2020-02-10T00:54:46.8491563Z +    = note: the return type of a function must have a statically known size
2020-02-10T00:54:46.8491685Z + 
2020-02-10T00:54:46.8491805Z + error[E0277]: the trait bound `Self: Foo<T>` is not satisfied
2020-02-10T00:54:46.8491805Z + error[E0277]: the trait bound `Self: Foo<T>` is not satisfied
2020-02-10T00:54:46.8492122Z +   --> $DIR/bindings.rs:18:20
2020-02-10T00:54:46.8492270Z +    |
2020-02-10T00:54:46.8492549Z + LL |     fn a(x: T) {
2020-02-10T00:54:46.8493236Z +    |               - help: consider further restricting `Self`: `where Self: Foo<T>`
2020-02-10T00:54:46.8493433Z + LL |         const foo: impl Clone = x;
2020-02-10T00:54:46.8493575Z +    |                    ^^^^^^^^^^ the trait `Foo<T>` is not implemented for `Self`
2020-02-10T00:54:46.8493810Z +    = note: the return type of a function must have a statically known size
2020-02-10T00:54:46.8493941Z + 
2020-02-10T00:54:46.8494060Z + error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8494499Z +   --> $DIR/bindings.rs:25:20
2020-02-10T00:54:46.8494499Z +   --> $DIR/bindings.rs:25:20
2020-02-10T00:54:46.8494661Z +    |
2020-02-10T00:54:46.8494777Z + LL | impl<T: Clone> Foo<T> for i32 {
2020-02-10T00:54:46.8495097Z +    |      -- help: consider further restricting this bound: `T: std::clone::Clone +`
2020-02-10T00:54:46.8495265Z + LL |     fn a(x: T) {
2020-02-10T00:54:46.8495379Z + LL |         const foo: impl Clone = x;
2020-02-10T00:54:46.8495648Z +    |
2020-02-10T00:54:46.8495764Z +    = note: the return type of a function must have a statically known size
2020-02-10T00:54:46.8495896Z + 
2020-02-10T00:54:46.8496008Z + error: aborting due to 9 previous errors
2020-02-10T00:54:46.8496008Z + error: aborting due to 9 previous errors
2020-02-10T00:54:46.8496117Z + 
2020-02-10T00:54:46.8496249Z + Some errors have detailed explanations: E0277, E0435.
2020-02-10T00:54:46.8496581Z + For more information about an error, try `rustc --explain E0277`.
2020-02-10T00:54:46.8496738Z 36 
2020-02-10T00:54:46.8496857Z 
2020-02-10T00:54:46.8496956Z 
2020-02-10T00:54:46.8497072Z The actual stderr differed from the expected stderr.
2020-02-10T00:54:46.8497419Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings/bindings.stderr
2020-02-10T00:54:46.8497772Z To update references, rerun the tests and pass the `--bless` flag
2020-02-10T00:54:46.8498107Z To only update this specific test, also pass `--test-args impl-trait/bindings.rs`
2020-02-10T00:54:46.8498385Z error: 1 errors occurred comparing output.
2020-02-10T00:54:46.8498497Z status: exit code: 1
2020-02-10T00:54:46.8498497Z status: exit code: 1
2020-02-10T00:54:46.8499271Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings/auxiliary" "-A" "unused"
2020-02-10T00:54:46.8499774Z ------------------------------------------
2020-02-10T00:54:46.8499905Z 
2020-02-10T00:54:46.8500194Z ------------------------------------------
2020-02-10T00:54:46.8500354Z stderr:
---
2020-02-10T00:54:46.8507638Z 
2020-02-10T00:54:46.8507756Z error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8508074Z   --> /checkout/src/test/ui/impl-trait/bindings.rs:5:16
2020-02-10T00:54:46.8508215Z    |
2020-02-10T00:54:46.8508344Z LL | fn a<T: Clone>(x: T) {
2020-02-10T00:54:46.8508684Z    |      -- help: consider further restricting this bound: `T: std::clone::Clone +`
2020-02-10T00:54:46.8509540Z LL |     const foo: impl Clone = x;
2020-02-10T00:54:46.8509644Z    |
2020-02-10T00:54:46.8509682Z    = note: the return type of a function must have a statically known size
2020-02-10T00:54:46.8509829Z 
2020-02-10T00:54:46.8509870Z error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8509870Z error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8510149Z   --> /checkout/src/test/ui/impl-trait/bindings.rs:11:20
2020-02-10T00:54:46.8510189Z    |
2020-02-10T00:54:46.8510241Z LL | fn b<T: Clone>(x: T) {
2020-02-10T00:54:46.8510451Z    |      -- help: consider further restricting this bound: `T: std::clone::Clone +`
2020-02-10T00:54:46.8510492Z LL |     let _ = move || {
2020-02-10T00:54:46.8510545Z LL |         const foo: impl Clone = x;
2020-02-10T00:54:46.8510630Z    |
2020-02-10T00:54:46.8510685Z    = note: the return type of a function must have a statically known size
2020-02-10T00:54:46.8510710Z 
2020-02-10T00:54:46.8510747Z error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8510747Z error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8510939Z   --> /checkout/src/test/ui/impl-trait/bindings.rs:18:20
2020-02-10T00:54:46.8510992Z    |
2020-02-10T00:54:46.8511025Z LL | trait Foo<T: Clone> {
2020-02-10T00:54:46.8511361Z    |           -- help: consider further restricting this bound: `T: std::clone::Clone +`
2020-02-10T00:54:46.8511423Z LL |     fn a(x: T) {
2020-02-10T00:54:46.8511459Z LL |         const foo: impl Clone = x;
2020-02-10T00:54:46.8511554Z    |
2020-02-10T00:54:46.8511591Z    = note: the return type of a function must have a statically known size
2020-02-10T00:54:46.8511616Z 
2020-02-10T00:54:46.8511653Z error[E0277]: the trait bound `Self: Foo<T>` is not satisfied
2020-02-10T00:54:46.8511653Z error[E0277]: the trait bound `Self: Foo<T>` is not satisfied
2020-02-10T00:54:46.8511873Z   --> /checkout/src/test/ui/impl-trait/bindings.rs:18:20
2020-02-10T00:54:46.8511910Z    |
2020-02-10T00:54:46.8511942Z LL |     fn a(x: T) {
2020-02-10T00:54:46.8512165Z    |               - help: consider further restricting `Self`: `where Self: Foo<T>`
2020-02-10T00:54:46.8512207Z LL |         const foo: impl Clone = x;
2020-02-10T00:54:46.8512247Z    |                    ^^^^^^^^^^ the trait `Foo<T>` is not implemented for `Self`
2020-02-10T00:54:46.8512532Z    = note: the return type of a function must have a statically known size
2020-02-10T00:54:46.8512560Z 
2020-02-10T00:54:46.8512598Z error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2020-02-10T00:54:46.8512850Z   --> /checkout/src/test/ui/impl-trait/bindings.rs:25:20
2020-02-10T00:54:46.8512850Z   --> /checkout/src/test/ui/impl-trait/bindings.rs:25:20
2020-02-10T00:54:46.8512887Z    |
2020-02-10T00:54:46.8512922Z LL | impl<T: Clone> Foo<T> for i32 {
2020-02-10T00:54:46.8513146Z    |      -- help: consider further restricting this bound: `T: std::clone::Clone +`
2020-02-10T00:54:46.8513266Z LL |     fn a(x: T) {
2020-02-10T00:54:46.8513301Z LL |         const foo: impl Clone = x;
2020-02-10T00:54:46.8513398Z    |
2020-02-10T00:54:46.8513435Z    = note: the return type of a function must have a statically known size
2020-02-10T00:54:46.8513476Z 
2020-02-10T00:54:46.8513512Z error: aborting due to 9 previous errors
---
2020-02-10T00:54:46.8520309Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-10T00:54:46.8520390Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-10T00:54:46.8533936Z 
2020-02-10T00:54:46.8534363Z 
2020-02-10T00:54:46.8539235Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-10T00:54:46.8539441Z 
2020-02-10T00:54:46.8539468Z 
2020-02-10T00:54:46.8545593Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-10T00:54:46.8545711Z Build completed unsuccessfully in 0:59:34
2020-02-10T00:54:46.8545711Z Build completed unsuccessfully in 0:59:34
2020-02-10T00:54:46.8594707Z == clock drift check ==
2020-02-10T00:54:46.8615860Z   local time: Mon Feb 10 00:54:46 UTC 2020
2020-02-10T00:54:47.0237568Z   network time: Mon, 10 Feb 2020 00:54:47 GMT
2020-02-10T00:54:47.0237634Z == end clock drift check ==
2020-02-10T00:54:47.4182833Z 
2020-02-10T00:54:47.4293827Z ##[error]Bash exited with code '1'.
2020-02-10T00:54:47.4304476Z ##[section]Finishing: Run build
2020-02-10T00:54:47.4323430Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69008/merge to s
2020-02-10T00:54:47.4324983Z Task         : Get sources
2020-02-10T00:54:47.4325021Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T00:54:47.4325058Z Version      : 1.0.0
2020-02-10T00:54:47.4325107Z Author       : Microsoft
2020-02-10T00:54:47.4325107Z Author       : Microsoft
2020-02-10T00:54:47.4325143Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-10T00:54:47.4325320Z ==============================================================================
2020-02-10T00:54:47.8190247Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-10T00:54:47.8224534Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69008/merge to s
2020-02-10T00:54:47.8329352Z Cleaning up task key
2020-02-10T00:54:47.8329991Z Start cleaning up orphan processes.
2020-02-10T00:54:47.8432234Z Terminate orphan process: pid (3592) (python)
2020-02-10T00:54:47.8694745Z ##[section]Finishing: Finalize Job
