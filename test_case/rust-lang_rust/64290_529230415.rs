plain
2019-09-08T17:28:18.9948195Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-08T17:28:19.9776615Z ##[command]git config gc.auto 0
2019-09-08T17:28:19.9782990Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-08T17:28:19.9787413Z ##[command]git config --get-all http.proxy
2019-09-08T17:28:19.9791419Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64290/merge:refs/remotes/pull/64290/merge
---
2019-09-08T18:29:27.4303421Z .................................................................................................... 1500/9008
2019-09-08T18:29:33.6296543Z .................................................................................................... 1600/9008
2019-09-08T18:29:46.8057450Z ......................................................i...............i............................. 1700/9008
2019-09-08T18:29:54.9195143Z .................................................................................................... 1800/9008
2019-09-08T18:30:09.3275449Z .............................................iiiii.................................................. 1900/9008
2019-09-08T18:30:20.3638109Z .................................................................................................... 2100/9008
2019-09-08T18:30:22.8441710Z .................................................................................................... 2200/9008
2019-09-08T18:30:26.3520129Z .................................................................................................... 2300/9008
2019-09-08T18:30:34.6418396Z .................................................................................................... 2400/9008
---
2019-09-08T18:33:35.7754289Z ..................................i...............i................................................. 4700/9008
2019-09-08T18:33:47.5834212Z .................................................................................................... 4800/9008
2019-09-08T18:33:53.7616091Z .................................................................................................... 4900/9008
2019-09-08T18:34:04.3778187Z .................................................................................................... 5000/9008
2019-09-08T18:34:10.3616359Z ................ii.ii............................................................................... 5100/9008
2019-09-08T18:34:20.8849546Z .................................................................................................... 5300/9008
2019-09-08T18:34:30.9063846Z ...............................................................................i.................... 5400/9008
2019-09-08T18:34:38.6513929Z .................................................................................................... 5500/9008
2019-09-08T18:34:44.4441787Z .................................................................................................... 5600/9008
2019-09-08T18:34:44.4441787Z .................................................................................................... 5600/9008
2019-09-08T18:34:54.9274362Z .........................................................................ii...i..ii...........i..... 5700/9008
2019-09-08T18:35:20.5558887Z .................................................................................................... 5900/9008
2019-09-08T18:35:30.8882260Z .................................................................................................... 6000/9008
2019-09-08T18:35:30.8882260Z .................................................................................................... 6000/9008
2019-09-08T18:35:36.5472940Z ...........................................................................i..ii.................... 6100/9008
2019-09-08T18:36:06.5542966Z .................................................................................................... 6300/9008
2019-09-08T18:36:08.5785698Z ..................................i................................................................. 6400/9008
2019-09-08T18:36:10.7297451Z .................................................................................................... 6500/9008
2019-09-08T18:36:13.3227454Z ......i............................................................................................. 6600/9008
---
2019-09-08T18:40:23.4867579Z 1 error: incorrect close delimiter: `}`
2019-09-08T18:40:23.4867986Z -   --> $DIR/unclosed_delim_mod.rs:5:1
2019-09-08T18:40:23.4868611Z +   --> $DIR/unclosed_delim_mod.rs:7:1
2019-09-08T18:40:23.4868832Z 3    |
2019-09-08T18:40:23.4869502Z 4 LL | pub fn new() -> Result<Value, ()> {
2019-09-08T18:40:23.4869985Z 5    |                                   - close delimiter possibly meant for this
2019-09-08T18:40:23.4870247Z 
2019-09-08T18:40:23.4870403Z The actual stderr differed from the expected stderr.
2019-09-08T18:40:23.4870403Z The actual stderr differed from the expected stderr.
2019-09-08T18:40:23.4870879Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep/unclosed-delimiter-in-dep.stderr
2019-09-08T18:40:23.4871307Z To update references, rerun the tests and pass the `--bless` flag
2019-09-08T18:40:23.4871745Z To only update this specific test, also pass `--test-args parser/unclosed-delimiter-in-dep.rs`
2019-09-08T18:40:23.4872044Z error: 1 errors occurred comparing output.
2019-09-08T18:40:23.4872178Z status: exit code: 1
2019-09-08T18:40:23.4872178Z status: exit code: 1
2019-09-08T18:40:23.4873202Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep/auxiliary" "-A" "unused"
2019-09-08T18:40:23.4874029Z ------------------------------------------
2019-09-08T18:40:23.4874178Z 
2019-09-08T18:40:23.4874517Z ------------------------------------------
2019-09-08T18:40:23.4874699Z stderr:
2019-09-08T18:40:23.4874699Z stderr:
2019-09-08T18:40:23.4875026Z ------------------------------------------
2019-09-08T18:40:23.4875185Z error: incorrect close delimiter: `}`
2019-09-08T18:40:23.4875548Z   --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:7:1
2019-09-08T18:40:23.4875710Z    |
2019-09-08T18:40:23.4876050Z LL | pub fn new() -> Result<Value, ()> {
2019-09-08T18:40:23.4876470Z    |                                   - close delimiter possibly meant for this
2019-09-08T18:40:23.4876891Z LL |     Ok(Value {
2019-09-08T18:40:23.4877713Z    |       - un-closed delimiter
2019-09-08T18:40:23.4878452Z LL | }
2019-09-08T18:40:23.4878513Z    | ^ incorrect close delimiter
2019-09-08T18:40:23.4878545Z 
2019-09-08T18:40:23.4878590Z error[E0308]: mismatched types
2019-09-08T18:40:23.4878590Z error[E0308]: mismatched types
2019-09-08T18:40:23.4878913Z   --> /checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs:4:20
2019-09-08T18:40:23.4878981Z    |
2019-09-08T18:40:23.4879225Z LL |     let _: usize = unclosed_delim_mod::new();
2019-09-08T18:40:23.4879287Z    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected usize, found enum `std::result::Result`
2019-09-08T18:40:23.4879353Z    |
2019-09-08T18:40:23.4879397Z    = note: expected type `usize`
2019-09-08T18:40:23.4879460Z               found type `std::result::Result<unclosed_delim_mod::Value, ()>`
2019-09-08T18:40:23.4879558Z error: aborting due to 2 previous errors
2019-09-08T18:40:23.4879587Z 
2019-09-08T18:40:23.4879884Z For more information about this error, try `rustc --explain E0308`.
2019-09-08T18:40:23.4879921Z 
---
2019-09-08T18:40:23.4901196Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-08T18:40:23.4901281Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-08T18:40:23.4920199Z 
2019-09-08T18:40:23.4920567Z 
2019-09-08T18:40:23.4922423Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-08T18:40:23.4923049Z 
2019-09-08T18:40:23.4923105Z 
2019-09-08T18:40:23.4927323Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-08T18:40:23.4927416Z Build completed unsuccessfully in 1:04:58
2019-09-08T18:40:23.4927416Z Build completed unsuccessfully in 1:04:58
2019-09-08T18:40:23.4982449Z == clock drift check ==
2019-09-08T18:40:23.5000681Z   local time: Sun Sep  8 18:40:23 UTC 2019
2019-09-08T18:40:23.5892168Z   network time: Sun, 08 Sep 2019 18:40:23 GMT
2019-09-08T18:40:23.5895632Z == end clock drift check ==
2019-09-08T18:40:24.2791447Z ##[error]Bash exited with code '1'.
2019-09-08T18:40:24.2832937Z ##[section]Starting: Checkout
2019-09-08T18:40:24.2835035Z ==============================================================================
2019-09-08T18:40:24.2835093Z Task         : Get sources
2019-09-08T18:40:24.2835163Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
