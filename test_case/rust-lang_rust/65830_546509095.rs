plain
2019-10-25T19:51:08.1226491Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T19:51:08.1409057Z ##[command]git config gc.auto 0
2019-10-25T19:51:08.6617625Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T19:51:08.6619757Z ##[command]git config --get-all http.proxy
2019-10-25T19:51:08.6622404Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65830/merge:refs/remotes/pull/65830/merge
---
2019-10-25T20:19:17.6833916Z    Compiling rustc-rayon v0.3.0
2019-10-25T20:19:22.6824116Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-10-25T20:19:24.0627499Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-10-25T20:19:26.3974485Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-25T20:19:26.8256250Z warning: function is never used: `query`
2019-10-25T20:19:26.8256511Z   --> src/librustc_macros/src/query.rs:16:26
2019-10-25T20:19:26.8256683Z    |
2019-10-25T20:19:26.8256911Z 16 |     syn::custom_keyword!(query);
2019-10-25T20:19:26.8257612Z    |
2019-10-25T20:19:26.8257832Z    = note: `#[warn(dead_code)]` on by default
2019-10-25T20:19:26.8257862Z 
2019-10-25T20:19:26.8257862Z 
2019-10-25T20:19:26.8258061Z warning: function is never used: `Keywords`
2019-10-25T20:19:26.8258488Z   --> src/librustc_macros/src/symbols.rs:13:26
2019-10-25T20:19:26.8258651Z    |
2019-10-25T20:19:26.8258852Z 13 |     syn::custom_keyword!(Keywords);
2019-10-25T20:19:26.8259098Z 
2019-10-25T20:19:26.8259275Z warning: function is never used: `Symbols`
2019-10-25T20:19:26.8259465Z   --> src/librustc_macros/src/symbols.rs:14:26
2019-10-25T20:19:26.8259639Z    |
2019-10-25T20:19:26.8259639Z    |
2019-10-25T20:19:26.8259839Z 14 |     syn::custom_keyword!(Symbols);
2019-10-25T20:19:26.8260087Z 
2019-10-25T20:19:35.5004722Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-25T20:19:41.9399252Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-25T20:19:59.8413025Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
---
2019-10-25T20:42:12.2803396Z .................................................................................................... 1600/9252
2019-10-25T20:42:17.0069048Z .................................................................................................... 1700/9252
2019-10-25T20:42:27.6995349Z ........................................................i........................................... 1800/9252
2019-10-25T20:42:34.5268519Z .................................................................................................... 1900/9252
2019-10-25T20:42:46.5121632Z ........................................iiiii....................................................... 2000/9252
2019-10-25T20:42:55.4274914Z .................................................................................................... 2200/9252
2019-10-25T20:42:57.6272846Z .................................................................................................... 2300/9252
2019-10-25T20:43:01.2977870Z .................................................................................................... 2400/9252
2019-10-25T20:43:20.2958610Z .................................................................................................... 2500/9252
---
2019-10-25T20:45:49.9623761Z .........................................i...............i.......................................... 4800/9252
2019-10-25T20:45:59.0474925Z .................................................................................................... 4900/9252
2019-10-25T20:46:05.0872038Z .................................................................................................... 5000/9252
2019-10-25T20:46:11.4195215Z .................................................................................................... 5100/9252
2019-10-25T20:46:19.5577203Z ..........................................ii.ii...........i......................................... 5200/9252
2019-10-25T20:46:28.1088412Z .................................................................................................... 5400/9252
2019-10-25T20:46:36.1473797Z .................................................................................................... 5500/9252
2019-10-25T20:46:42.8367263Z ....................i............................................................................... 5600/9252
2019-10-25T20:46:47.7486265Z .................................................................................................... 5700/9252
2019-10-25T20:46:47.7486265Z .................................................................................................... 5700/9252
2019-10-25T20:46:57.9773237Z .................................................................................................... 5800/9252
2019-10-25T20:47:07.5783975Z .................ii...i..ii...........i............................................................. 5900/9252
2019-10-25T20:47:26.3796230Z .................................................................................................... 6100/9252
2019-10-25T20:47:31.9175318Z .................................................................................................... 6200/9252
2019-10-25T20:47:31.9175318Z .................................................................................................... 6200/9252
2019-10-25T20:47:42.6238157Z ........................................i..ii....................................................... 6300/9252
2019-10-25T20:48:01.2189499Z .................................................................................................... 6500/9252
2019-10-25T20:48:03.1984926Z ......i............................................................................................. 6600/9252
2019-10-25T20:48:05.1565766Z .................................................................................i.................. 6700/9252
2019-10-25T20:48:07.4331001Z .................................................................................................... 6800/9252
---
2019-10-25T20:51:37.9336740Z 17 LL | struct Struct {
2019-10-25T20:51:37.9336865Z 18    |        ^^^^^^
2019-10-25T20:51:37.9336964Z 
2019-10-25T20:51:37.9337070Z 19 
2019-10-25T20:51:37.9337177Z 20 warning: function is never used: `func`
2019-10-25T20:51:37.9337813Z +   --> $DIR/unused-warning-point-at-identifier.rs:19:4
2019-10-25T20:51:37.9337958Z 22    |
2019-10-25T20:51:37.9338252Z 23 LL | fn func() -> usize {
2019-10-25T20:51:37.9338399Z 24    |    ^^^^
2019-10-25T20:51:37.9338399Z 24    |    ^^^^
2019-10-25T20:51:37.9338495Z 
2019-10-25T20:51:37.9338623Z 25 
2019-10-25T20:51:37.9338742Z 26 warning: function is never used: `func_complete_span`
2019-10-25T20:51:37.9339385Z +   --> $DIR/unused-warning-point-at-identifier.rs:24:1
2019-10-25T20:51:37.9339533Z 28    |
2019-10-25T20:51:37.9339663Z 29 LL | func_complete_span()
2019-10-25T20:51:37.9339775Z 30    | ^^^^^^^^^^^^^^^^^^
2019-10-25T20:51:37.9339775Z 30    | ^^^^^^^^^^^^^^^^^^
2019-10-25T20:51:37.9339871Z 
2019-10-25T20:51:37.9339963Z 
2019-10-25T20:51:37.9340094Z The actual stderr differed from the expected stderr.
2019-10-25T20:51:37.9340468Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/unused-warning-point-at-identifier/unused-warning-point-at-identifier.stderr
2019-10-25T20:51:37.9340830Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T20:51:37.9341243Z To only update this specific test, also pass `--test-args span/unused-warning-point-at-identifier.rs`
2019-10-25T20:51:37.9341538Z error: 1 errors occurred comparing output.
2019-10-25T20:51:37.9341764Z status: exit code: 0
2019-10-25T20:51:37.9341764Z status: exit code: 0
2019-10-25T20:51:37.9342533Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/unused-warning-point-at-identifier.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/unused-warning-point-at-identifier/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/unused-warning-point-at-identifier/auxiliary"
2019-10-25T20:51:37.9343054Z ------------------------------------------
2019-10-25T20:51:37.9343219Z 
2019-10-25T20:51:37.9343527Z ------------------------------------------
2019-10-25T20:51:37.9343679Z stderr:
2019-10-25T20:51:37.9343679Z stderr:
2019-10-25T20:51:37.9344001Z ------------------------------------------
2019-10-25T20:51:37.9344185Z warning: enum is never used: `Enum`
2019-10-25T20:51:37.9344540Z   --> /checkout/src/test/ui/span/unused-warning-point-at-identifier.rs:5:6
2019-10-25T20:51:37.9344701Z    |
2019-10-25T20:51:37.9344824Z LL | enum Enum { //~ WARN enum is never used
2019-10-25T20:51:37.9345072Z    |
2019-10-25T20:51:37.9345185Z note: lint level defined here
2019-10-25T20:51:37.9345534Z   --> /checkout/src/test/ui/span/unused-warning-point-at-identifier.rs:3:9
2019-10-25T20:51:37.9345688Z    |
2019-10-25T20:51:37.9345688Z    |
2019-10-25T20:51:37.9345801Z LL | #![warn(unused)]
2019-10-25T20:51:37.9345938Z    |         ^^^^^^
2019-10-25T20:51:37.9346055Z    = note: `#[warn(dead_code)]` implied by `#[warn(unused)]`
2019-10-25T20:51:37.9346155Z 
2019-10-25T20:51:37.9346290Z warning: struct is never constructed: `Struct`
2019-10-25T20:51:37.9346611Z   --> /checkout/src/test/ui/span/unused-warning-point-at-identifier.rs:12:8
2019-10-25T20:51:37.9346763Z    |
2019-10-25T20:51:37.9346901Z LL | struct Struct { //~ WARN struct is never constructed
2019-10-25T20:51:37.9347245Z 
2019-10-25T20:51:37.9347245Z 
2019-10-25T20:51:37.9347383Z warning: function is never used: `func`
2019-10-25T20:51:37.9347891Z    |
2019-10-25T20:51:37.9347891Z    |
2019-10-25T20:51:37.9348228Z LL | fn func() -> usize { //~ WARN function is never used
2019-10-25T20:51:37.9348483Z 
2019-10-25T20:51:37.9348483Z 
2019-10-25T20:51:37.9348620Z warning: function is never used: `func_complete_span`
2019-10-25T20:51:37.9349090Z    |
2019-10-25T20:51:37.9349090Z    |
2019-10-25T20:51:37.9349237Z LL | func_complete_span() //~ WARN function is never used
2019-10-25T20:51:37.9349449Z 
2019-10-25T20:51:37.9349565Z 
2019-10-25T20:51:37.9349864Z ------------------------------------------
2019-10-25T20:51:37.9349999Z 
---
2019-10-25T20:51:37.9362828Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-25T20:51:37.9363051Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T20:51:37.9374142Z 
2019-10-25T20:51:37.9374315Z 
2019-10-25T20:51:37.9379879Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-25T20:51:37.9381354Z 
2019-10-25T20:51:37.9381409Z 
2019-10-25T20:51:37.9388304Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-25T20:51:37.9388805Z Build completed unsuccessfully in 0:54:01
2019-10-25T20:51:37.9388805Z Build completed unsuccessfully in 0:54:01
2019-10-25T20:51:37.9431946Z == clock drift check ==
2019-10-25T20:51:37.9445927Z   local time: Fri Oct 25 20:51:37 UTC 2019
2019-10-25T20:51:38.0307624Z   network time: Fri, 25 Oct 2019 20:51:38 GMT
2019-10-25T20:51:38.0310501Z == end clock drift check ==
2019-10-25T20:51:39.3383968Z 
2019-10-25T20:51:39.3475914Z ##[error]Bash exited with code '1'.
2019-10-25T20:51:39.3505170Z ##[section]Starting: Checkout
2019-10-25T20:51:39.3506633Z ==============================================================================
2019-10-25T20:51:39.3506674Z Task         : Get sources
2019-10-25T20:51:39.3506725Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
