plain
2020-02-07T20:44:11.9428782Z ========================== Starting Command Output ===========================
2020-02-07T20:44:11.9431570Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/29c9c919-fab2-494f-b768-776ab57020f0.sh
2020-02-07T20:44:11.9562723Z 
2020-02-07T20:44:11.9615424Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-07T20:44:11.9620361Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68938/merge to s
2020-02-07T20:44:11.9621709Z Task         : Get sources
2020-02-07T20:44:11.9621742Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T20:44:11.9621767Z Version      : 1.0.0
2020-02-07T20:44:11.9621792Z Author       : Microsoft
---
2020-02-07T20:44:12.7058706Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-07T20:44:12.7146937Z ##[command]git config gc.auto 0
2020-02-07T20:44:12.7224274Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-07T20:44:12.7444161Z ##[command]git config --get-all http.proxy
2020-02-07T20:44:12.7451101Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68938/merge:refs/remotes/pull/68938/merge
---
2020-02-07T21:33:01.6701042Z .................................................................................................... 1700/9603
2020-02-07T21:33:06.0858940Z .................................................................................................... 1800/9603
2020-02-07T21:33:17.0909596Z .............................i...................................................................... 1900/9603
2020-02-07T21:33:23.5234833Z .................................................................................................... 2000/9603
2020-02-07T21:33:35.8886272Z ...................iiiii............................................................................ 2100/9603
2020-02-07T21:33:45.0588250Z .................................................................................................... 2300/9603
2020-02-07T21:33:47.1901128Z .................................................................................................... 2400/9603
2020-02-07T21:33:51.4876128Z .................................................................................................... 2500/9603
2020-02-07T21:34:09.2340421Z .................................................................................................... 2600/9603
---
2020-02-07T21:36:21.9962580Z .....................................................................i...............i.............. 4900/9603
2020-02-07T21:36:28.5878356Z .................................................................................................... 5000/9603
2020-02-07T21:36:35.3166955Z .................................................................................................... 5100/9603
2020-02-07T21:36:39.1752239Z ............i....................................................................................... 5200/9603
2020-02-07T21:36:48.4287213Z ......................................................................................ii.ii........i 5300/9603
2020-02-07T21:36:54.9560936Z ........................i........................................................................... 5500/9603
2020-02-07T21:37:01.9023071Z .................................................................................................... 5600/9603
2020-02-07T21:37:08.9994124Z ..........................................................................i......................... 5700/9603
2020-02-07T21:37:15.3728941Z .................................................................................................... 5800/9603
2020-02-07T21:37:15.3728941Z .................................................................................................... 5800/9603
2020-02-07T21:37:20.7812609Z .................................................................................................... 5900/9603
2020-02-07T21:37:29.3172081Z .................................................................ii...i..ii...........i............. 6000/9603
2020-02-07T21:37:46.8239420Z .................................................................................................... 6200/9603
2020-02-07T21:37:52.9430732Z .................................................................................................... 6300/9603
2020-02-07T21:37:52.9430732Z .................................................................................................... 6300/9603
2020-02-07T21:37:59.2581605Z .............................................................................................i..ii.. 6400/9603
2020-02-07T21:38:25.1842760Z .................................................................................................... 6600/9603
2020-02-07T21:38:33.5615227Z ................................................................................i................... 6700/9603
2020-02-07T21:38:35.2290244Z .................................................................................................... 6800/9603
2020-02-07T21:38:37.0351770Z .......................................................................................i............ 6900/9603
---
2020-02-07T21:40:01.1002740Z .................................................................................................... 7600/9603
2020-02-07T21:40:05.0835161Z .................................................................................................... 7700/9603
2020-02-07T21:40:09.7360962Z .................................................................................................... 7800/9603
2020-02-07T21:40:17.2605673Z .................................................................................................... 7900/9603
2020-02-07T21:40:24.1401008Z ..................................................iiiiiii.i......................................... 8000/9603
2020-02-07T21:40:32.2681823Z ..........................................................................................i......i.. 8100/9603
2020-02-07T21:40:40.5524007Z .................................................................................................... 8300/9603
2020-02-07T21:40:53.2056172Z .................................................................................................... 8400/9603
2020-02-07T21:41:00.5081782Z .................................................................................................... 8500/9603
2020-02-07T21:41:06.5769175Z .................................................................................................... 8600/9603
---
2020-02-07T21:42:35.0197725Z 16 
2020-02-07T21:42:35.0198104Z + error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2020-02-07T21:42:35.0198434Z +   --> $DIR/shadowing.rs:6:14
2020-02-07T21:42:35.0198590Z +    |
2020-02-07T21:42:35.0198887Z + LL | trait Shadow<'a> {
2020-02-07T21:42:35.0199211Z +    |              -- first declared here
2020-02-07T21:42:35.0199392Z + LL |     //FIXME(#44265): The lifetime parameter shadowing should cause an error.
2020-02-07T21:42:35.0199701Z + LL |     type Bar<'a>;
2020-02-07T21:42:35.0200026Z +    |              ^^ lifetime 'a already in scope
2020-02-07T21:42:35.0200524Z + error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2020-02-07T21:42:35.0201481Z +   --> $DIR/shadowing.rs:15:14
2020-02-07T21:42:35.0201670Z +    |
2020-02-07T21:42:35.0201670Z +    |
2020-02-07T21:42:35.0201999Z + LL | impl<'a> NoShadow<'a> for &'a u32 {
2020-02-07T21:42:35.0202313Z +    |      -- first declared here
2020-02-07T21:42:35.0202491Z + LL |     //FIXME(#44265): The lifetime parameter shadowing should cause an error.
2020-02-07T21:42:35.0202778Z + LL |     type Bar<'a> = i32;
2020-02-07T21:42:35.0203099Z +    |              ^^ lifetime 'a already in scope
2020-02-07T21:42:35.0203584Z 17 error: type-generic associated types are not yet implemented
2020-02-07T21:42:35.0203908Z 18   --> $DIR/shadowing.rs:19:5
2020-02-07T21:42:35.0204093Z 19    |
2020-02-07T21:42:35.0204203Z 
2020-02-07T21:42:35.0204203Z 
2020-02-07T21:42:35.0204320Z 30    |
2020-02-07T21:42:35.0204780Z 31    = note: for more information, see ***/issues/44265
2020-02-07T21:42:35.0205253Z - error: aborting due to 4 previous errors
2020-02-07T21:42:35.0205435Z + error: aborting due to 6 previous errors
2020-02-07T21:42:35.0205574Z 34 
2020-02-07T21:42:35.0205887Z - For more information about this error, try `rustc --explain E0403`.
2020-02-07T21:42:35.0205887Z - For more information about this error, try `rustc --explain E0403`.
2020-02-07T21:42:35.0206069Z + Some errors have detailed explanations: E0403, E0496.
2020-02-07T21:42:35.0206381Z + For more information about an error, try `rustc --explain E0403`.
2020-02-07T21:42:35.0206553Z 36 
2020-02-07T21:42:35.0206658Z 
2020-02-07T21:42:35.0206763Z 
2020-02-07T21:42:35.0206885Z The actual stderr differed from the expected stderr.
2020-02-07T21:42:35.0207432Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/shadowing/shadowing.stderr
2020-02-07T21:42:35.0207863Z To update references, rerun the tests and pass the `--bless` flag
2020-02-07T21:42:35.0208297Z To only update this specific test, also pass `--test-args generic-associated-types/shadowing.rs`
2020-02-07T21:42:35.0208591Z error: 1 errors occurred comparing output.
2020-02-07T21:42:35.0208884Z status: exit code: 1
2020-02-07T21:42:35.0208884Z status: exit code: 1
2020-02-07T21:42:35.0209694Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/shadowing" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/shadowing/auxiliary" "-A" "unused"
2020-02-07T21:42:35.0210239Z ------------------------------------------
2020-02-07T21:42:35.0210385Z 
2020-02-07T21:42:35.0210721Z ------------------------------------------
2020-02-07T21:42:35.0210882Z stderr:
2020-02-07T21:42:35.0210882Z stderr:
2020-02-07T21:42:35.0211200Z ------------------------------------------
2020-02-07T21:42:35.0211602Z error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
2020-02-07T21:42:35.0211965Z   --> /checkout/src/test/ui/generic-associated-types/shadowing.rs:19:14
2020-02-07T21:42:35.0212162Z    |
2020-02-07T21:42:35.0212289Z LL | trait ShadowT<T> {
2020-02-07T21:42:35.0212591Z    |               - first use of `T`
2020-02-07T21:42:35.0212769Z LL |     type Bar<T>;
2020-02-07T21:42:35.0212896Z    |              ^ already used
2020-02-07T21:42:35.0213382Z error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
2020-02-07T21:42:35.0213755Z   --> /checkout/src/test/ui/generic-associated-types/shadowing.rs:30:14
2020-02-07T21:42:35.0213915Z    |
2020-02-07T21:42:35.0213915Z    |
2020-02-07T21:42:35.0214062Z LL | impl<T> NoShadowT<T> for Option<T> {
2020-02-07T21:42:35.0214359Z    |      - first use of `T`
2020-02-07T21:42:35.0214537Z LL |     type Bar<T> = i32;
2020-02-07T21:42:35.0214908Z    |              ^ already used
2020-02-07T21:42:35.0216977Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2020-02-07T21:42:35.0218381Z   --> /checkout/src/test/ui/generic-associated-types/shadowing.rs:6:14
2020-02-07T21:42:35.0219182Z    |
2020-02-07T21:42:35.0220163Z LL | trait Shadow<'a> {
2020-02-07T21:42:35.0220163Z LL | trait Shadow<'a> {
2020-02-07T21:42:35.0221530Z    |              -- first declared here
2020-02-07T21:42:35.0222284Z LL |     //FIXME(#44265): The lifetime parameter shadowing should cause an error.
2020-02-07T21:42:35.0223287Z LL |     type Bar<'a>;
2020-02-07T21:42:35.0224334Z    |              ^^ lifetime 'a already in scope
2020-02-07T21:42:35.0225583Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2020-02-07T21:42:35.0225954Z   --> /checkout/src/test/ui/generic-associated-types/shadowing.rs:15:14
2020-02-07T21:42:35.0226113Z    |
2020-02-07T21:42:35.0226113Z    |
2020-02-07T21:42:35.0249417Z LL | impl<'a> NoShadow<'a> for &'a u32 {
2020-02-07T21:42:35.0299746Z    |      -- first declared here
2020-02-07T21:42:35.0299853Z LL |     //FIXME(#44265): The lifetime parameter shadowing should cause an error.
2020-02-07T21:42:35.0300385Z LL |     type Bar<'a> = i32;
2020-02-07T21:42:35.0300593Z    |              ^^ lifetime 'a already in scope
2020-02-07T21:42:35.0300821Z error: type-generic associated types are not yet implemented
2020-02-07T21:42:35.0301058Z   --> /checkout/src/test/ui/generic-associated-types/shadowing.rs:19:5
2020-02-07T21:42:35.0301099Z    |
2020-02-07T21:42:35.0301166Z LL |     type Bar<T>;
2020-02-07T21:42:35.0301166Z LL |     type Bar<T>;
2020-02-07T21:42:35.0301221Z    |     ^^^^^^^^^^^^
2020-02-07T21:42:35.0301419Z    |
2020-02-07T21:42:35.0301776Z    = note: for more information, see ***/issues/44265
2020-02-07T21:42:35.0302053Z error: type-generic associated types are not yet implemented
2020-02-07T21:42:35.0302280Z   --> /checkout/src/test/ui/generic-associated-types/shadowing.rs:25:5
2020-02-07T21:42:35.0302339Z    |
2020-02-07T21:42:35.0302339Z    |
2020-02-07T21:42:35.0302485Z LL |     type Bar<U>; // OK
2020-02-07T21:42:35.0302575Z    |
2020-02-07T21:42:35.0302575Z    |
2020-02-07T21:42:35.0303040Z    = note: for more information, see ***/issues/44265
2020-02-07T21:42:35.0303110Z error: aborting due to 6 previous errors
2020-02-07T21:42:35.0303152Z 
2020-02-07T21:42:35.0303189Z Some errors have detailed explanations: E0403, E0496.
2020-02-07T21:42:35.0303719Z For more information about an error, try `rustc --explain E0403`.
---
2020-02-07T21:42:35.0306212Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-07T21:42:35.0306258Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-07T21:42:35.0306283Z 
2020-02-07T21:42:35.0306320Z 
2020-02-07T21:42:35.0307495Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-07T21:42:35.0307699Z 
2020-02-07T21:42:35.0307723Z 
2020-02-07T21:42:35.0307760Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-07T21:42:35.0307817Z Build completed unsuccessfully in 0:52:53
2020-02-07T21:42:35.0307817Z Build completed unsuccessfully in 0:52:53
2020-02-07T21:42:35.0307854Z == clock drift check ==
2020-02-07T21:42:35.0314529Z   local time: Fri Feb  7 21:42:35 UTC 2020
2020-02-07T21:42:35.1952748Z   network time: Fri, 07 Feb 2020 21:42:35 GMT
2020-02-07T21:42:35.1953640Z == end clock drift check ==
2020-02-07T21:42:35.5761318Z 
2020-02-07T21:42:35.5842315Z ##[error]Bash exited with code '1'.
2020-02-07T21:42:35.5872091Z ##[section]Finishing: Run build
2020-02-07T21:42:35.5896666Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68938/merge to s
2020-02-07T21:42:35.5898146Z Task         : Get sources
2020-02-07T21:42:35.5898181Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T21:42:35.5898217Z Version      : 1.0.0
2020-02-07T21:42:35.5898264Z Author       : Microsoft
2020-02-07T21:42:35.5898264Z Author       : Microsoft
2020-02-07T21:42:35.5898299Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-07T21:42:35.5898336Z ==============================================================================
2020-02-07T21:42:35.9604000Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-07T21:42:35.9633266Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68938/merge to s
2020-02-07T21:42:35.9724129Z Cleaning up task key
2020-02-07T21:42:35.9724739Z Start cleaning up orphan processes.
2020-02-07T21:42:35.9813097Z Terminate orphan process: pid (3726) (python)
2020-02-07T21:42:36.0009204Z ##[section]Finishing: Finalize Job
