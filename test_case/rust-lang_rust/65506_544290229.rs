plain
2019-10-20T19:20:10.8072094Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-20T19:20:10.8275533Z ##[command]git config gc.auto 0
2019-10-20T19:20:10.8362611Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-20T19:20:10.8430024Z ##[command]git config --get-all http.proxy
2019-10-20T19:20:10.8585918Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65506/merge:refs/remotes/pull/65506/merge
---
2019-10-20T20:25:40.3383039Z .................................................................................................... 1600/9306
2019-10-20T20:25:45.8937192Z .................................................................................................... 1700/9306
2019-10-20T20:25:59.3639723Z ............................................i...............i....................................... 1800/9306
2019-10-20T20:26:08.0284985Z .................................................................................................... 1900/9306
2019-10-20T20:26:23.3499718Z ..................................iiiii............................................................. 2000/9306
2019-10-20T20:26:34.7775745Z .................................................................................................... 2200/9306
2019-10-20T20:26:37.4710774Z .................................................................................................... 2300/9306
2019-10-20T20:26:42.3233078Z .................................................................................................... 2400/9306
2019-10-20T20:27:05.9977136Z .................................................................................................... 2500/9306
---
2019-10-20T20:30:09.1274748Z .......................................................i...............i............................ 4800/9306
2019-10-20T20:30:19.0118802Z .................................................................................................... 4900/9306
2019-10-20T20:30:28.3632727Z .................................................................................................... 5000/9306
2019-10-20T20:30:34.9822090Z .................................................................................................... 5100/9306
2019-10-20T20:30:46.4566712Z ..........................................................ii.ii..................................... 5200/9306
2019-10-20T20:30:56.7007980Z .................................................................................................... 5400/9306
2019-10-20T20:31:07.3088961Z .................................................................................................... 5500/9306
2019-10-20T20:31:14.9378949Z ...............................i.................................................................... 5600/9306
2019-10-20T20:31:21.7462064Z .................................................................................................... 5700/9306
2019-10-20T20:31:21.7462064Z .................................................................................................... 5700/9306
2019-10-20T20:31:33.2637643Z .................................................................................................... 5800/9306
2019-10-20T20:31:41.4348182Z ..............................................ii...i...ii..........i................................ 5900/9306
2019-10-20T20:32:08.9263039Z .................................................................................................... 6100/9306
2019-10-20T20:32:18.8406294Z .................................................................................................... 6200/9306
2019-10-20T20:32:18.8406294Z .................................................................................................... 6200/9306
2019-10-20T20:32:26.4642061Z .....................................................................i..ii.......................... 6300/9306
2019-10-20T20:32:57.3064484Z .................................................................................................... 6500/9306
2019-10-20T20:33:03.9286788Z ....................................................................i............................... 6600/9306
2019-10-20T20:33:06.2494398Z .................................................................................................... 6700/9306
2019-10-20T20:33:08.8528555Z ..........................................i......................................................... 6800/9306
---
2019-10-20T20:37:33.6987115Z normalized stderr:
2019-10-20T20:37:33.6987310Z warning: trait objects without an explicit `dyn` are deprecated
2019-10-20T20:37:33.6987714Z   --> $DIR/issue-20971.rs:16:22
2019-10-20T20:37:33.6987867Z    |
2019-10-20T20:37:33.6988218Z LL | pub fn many() -> Box<Parser<Input = <() as Parser>::Input> + 'static> {
2019-10-20T20:37:33.6988637Z    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Parser<Input = <() as Parser>::Input> + 'static`
2019-10-20T20:37:33.6988953Z    = note: `#[warn(bare_trait_objects)]` on by default
2019-10-20T20:37:33.6989056Z 
2019-10-20T20:37:33.6989174Z 
2019-10-20T20:37:33.6989275Z 
2019-10-20T20:37:33.6989275Z 
2019-10-20T20:37:33.6989373Z 
2019-10-20T20:37:33.6989492Z The actual stderr differed from the expected stderr.
2019-10-20T20:37:33.6989876Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20971/issue-20971.stderr
2019-10-20T20:37:33.6990218Z To update references, rerun the tests and pass the `--bless` flag
2019-10-20T20:37:33.6990605Z To only update this specific test, also pass `--test-args issues/issue-20971.rs`
2019-10-20T20:37:33.6990873Z error: 1 errors occurred comparing output.
2019-10-20T20:37:33.6991004Z status: exit code: 0
2019-10-20T20:37:33.6991004Z status: exit code: 0
2019-10-20T20:37:33.6991681Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20971.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20971/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20971/auxiliary" "-A" "unused"
2019-10-20T20:37:33.6992192Z ------------------------------------------
2019-10-20T20:37:33.6992339Z 
2019-10-20T20:37:33.6992805Z ------------------------------------------
2019-10-20T20:37:33.6993669Z stderr:
2019-10-20T20:37:33.6993669Z stderr:
2019-10-20T20:37:33.6994252Z ------------------------------------------
2019-10-20T20:37:33.6994466Z warning: trait objects without an explicit `dyn` are deprecated
2019-10-20T20:37:33.6994859Z   --> /checkout/src/test/ui/issues/issue-20971.rs:16:22
2019-10-20T20:37:33.6995065Z    |
2019-10-20T20:37:33.6995466Z LL | pub fn many() -> Box<Parser<Input = <() as Parser>::Input> + 'static> {
2019-10-20T20:37:33.6995981Z    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Parser<Input = <() as Parser>::Input> + 'static`
2019-10-20T20:37:33.6996325Z    = note: `#[warn(bare_trait_objects)]` on by default
2019-10-20T20:37:33.6996453Z 
2019-10-20T20:37:33.6996579Z 
2019-10-20T20:37:33.6997091Z ------------------------------------------
2019-10-20T20:37:33.6997091Z ------------------------------------------
2019-10-20T20:37:33.6997222Z 
2019-10-20T20:37:33.6997340Z 
2019-10-20T20:37:33.6997631Z ---- [ui] ui/issues/issue-2444.rs stdout ----
2019-10-20T20:37:33.6998131Z normalized stderr:
2019-10-20T20:37:33.6998305Z warning: type `e` should have an upper camel case name
2019-10-20T20:37:33.6998945Z   --> $DIR/issue-2444.rs:6:6
2019-10-20T20:37:33.6999122Z    |
2019-10-20T20:37:33.6999244Z LL | enum e<T> {
2019-10-20T20:37:33.6999366Z    |      ^ help: convert the identifier to upper camel case: `E`
2019-10-20T20:37:33.6999657Z    = note: `#[warn(non_camel_case_types)]` on by default
2019-10-20T20:37:33.6999764Z 
2019-10-20T20:37:33.6999889Z warning: variant `ee` should have an upper camel case name
2019-10-20T20:37:33.7000425Z   --> $DIR/issue-2444.rs:7:5
2019-10-20T20:37:33.7000425Z   --> $DIR/issue-2444.rs:7:5
2019-10-20T20:37:33.7000599Z    |
2019-10-20T20:37:33.7000722Z LL |     ee(Arc<T>),
2019-10-20T20:37:33.7000849Z    |     ^^ help: convert the identifier to upper camel case: `Ee`
2019-10-20T20:37:33.7001078Z 
2019-10-20T20:37:33.7001185Z 
2019-10-20T20:37:33.7001291Z 
2019-10-20T20:37:33.7001438Z The actual stderr differed from the expected stderr.
2019-10-20T20:37:33.7001438Z The actual stderr differed from the expected stderr.
2019-10-20T20:37:33.7001985Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2444/issue-2444.stderr
2019-10-20T20:37:33.7002375Z To update references, rerun the tests and pass the `--bless` flag
2019-10-20T20:37:33.7003758Z To only update this specific test, also pass `--test-args issues/issue-2444.rs`
2019-10-20T20:37:33.7004149Z error: 1 errors occurred comparing output.
2019-10-20T20:37:33.7004293Z status: exit code: 0
2019-10-20T20:37:33.7004293Z status: exit code: 0
2019-10-20T20:37:33.7005197Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2444.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2444/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2444/auxiliary" "-A" "unused"
2019-10-20T20:37:33.7005852Z ------------------------------------------
2019-10-20T20:37:33.7006013Z 
2019-10-20T20:37:33.7006378Z ------------------------------------------
2019-10-20T20:37:33.7006556Z stderr:
2019-10-20T20:37:33.7006556Z stderr:
2019-10-20T20:37:33.7007232Z ------------------------------------------
2019-10-20T20:37:33.7007577Z warning: type `e` should have an upper camel case name
2019-10-20T20:37:33.7007912Z   --> /checkout/src/test/ui/issues/issue-2444.rs:6:6
2019-10-20T20:37:33.7008057Z    |
2019-10-20T20:37:33.7008170Z LL | enum e<T> {
2019-10-20T20:37:33.7008286Z    |      ^ help: convert the identifier to upper camel case: `E`
2019-10-20T20:37:33.7008551Z    = note: `#[warn(non_camel_case_types)]` on by default
2019-10-20T20:37:33.7008650Z 
2019-10-20T20:37:33.7008769Z warning: variant `ee` should have an upper camel case name
2019-10-20T20:37:33.7009064Z   --> /checkout/src/test/ui/issues/issue-2444.rs:7:5
2019-10-20T20:37:33.7009064Z   --> /checkout/src/test/ui/issues/issue-2444.rs:7:5
2019-10-20T20:37:33.7009215Z    |
2019-10-20T20:37:33.7009348Z LL |     ee(Arc<T>),
2019-10-20T20:37:33.7009474Z    |     ^^ help: convert the identifier to upper camel case: `Ee`
2019-10-20T20:37:33.7009697Z 
2019-10-20T20:37:33.7009984Z ------------------------------------------
2019-10-20T20:37:33.7010111Z 
2019-10-20T20:37:33.7010212Z 
---
2019-10-20T20:37:33.7017350Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-20T20:37:33.7017720Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-20T20:37:33.7035687Z 
2019-10-20T20:37:33.7036161Z 
2019-10-20T20:37:33.7038147Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-20T20:37:33.7038839Z 
2019-10-20T20:37:33.7038951Z 
2019-10-20T20:37:33.7093902Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-20T20:37:33.7094820Z Build completed unsuccessfully in 1:10:45
2019-10-20T20:37:33.7094820Z Build completed unsuccessfully in 1:10:45
2019-10-20T20:37:33.7101043Z == clock drift check ==
2019-10-20T20:37:33.7114835Z   local time: Sun Oct 20 20:37:33 UTC 2019
2019-10-20T20:37:33.8497494Z   network time: Sun, 20 Oct 2019 20:37:33 GMT
2019-10-20T20:37:33.8498260Z == end clock drift check ==
2019-10-20T20:37:34.9216416Z 
2019-10-20T20:37:34.9345610Z ##[error]Bash exited with code '1'.
2019-10-20T20:37:34.9402034Z ##[section]Starting: Checkout
2019-10-20T20:37:34.9404277Z ==============================================================================
2019-10-20T20:37:34.9404354Z Task         : Get sources
2019-10-20T20:37:34.9404400Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
