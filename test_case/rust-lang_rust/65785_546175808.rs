plain
2019-10-25T01:20:00.4949159Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T01:20:01.3530067Z ##[command]git config gc.auto 0
2019-10-25T01:20:01.3532996Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T01:20:01.3537950Z ##[command]git config --get-all http.proxy
2019-10-25T01:20:01.3542946Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65785/merge:refs/remotes/pull/65785/merge
---
2019-10-25T02:24:55.0676132Z .................................................................................................... 1600/9240
2019-10-25T02:25:00.7386043Z .................................................................................................... 1700/9240
2019-10-25T02:25:13.8538329Z .....................................................i...............i.............................. 1800/9240
2019-10-25T02:25:22.0649521Z .................................................................................................... 1900/9240
2019-10-25T02:25:37.0584181Z ...........................................iiiii.................................................... 2000/9240
2019-10-25T02:25:48.1453054Z .................................................................................................... 2200/9240
2019-10-25T02:25:50.8694743Z .................................................................................................... 2300/9240
2019-10-25T02:25:55.2040071Z .................................................................................................... 2400/9240
2019-10-25T02:26:19.4714566Z .................................................................................................... 2500/9240
---
2019-10-25T02:29:17.9658543Z ..............................................i...............i..................................... 4800/9240
2019-10-25T02:29:27.4097397Z .................................................................................................... 4900/9240
2019-10-25T02:29:36.2854543Z .................................................................................................... 5000/9240
2019-10-25T02:29:43.3339981Z .................................................................................................... 5100/9240
2019-10-25T02:29:53.7072001Z ..............................................ii.ii................................................. 5200/9240
2019-10-25T02:30:04.1039632Z .................................................................................................... 5400/9240
2019-10-25T02:30:14.2968158Z .................................................................................................... 5500/9240
2019-10-25T02:30:22.0672748Z .............i...................................................................................... 5600/9240
2019-10-25T02:30:27.7176218Z ............................F....................................................................... 5700/9240
2019-10-25T02:30:27.7176218Z ............................F....................................................................... 5700/9240
2019-10-25T02:30:40.2045141Z .................................................................................................... 5800/9240
2019-10-25T02:30:52.6415187Z ..........ii...i..ii...........i.................................................................... 5900/9240
2019-10-25T02:31:15.3914189Z .................................................................................................... 6100/9240
2019-10-25T02:31:23.7034762Z .................................................................................................... 6200/9240
2019-10-25T02:31:23.7034762Z .................................................................................................... 6200/9240
2019-10-25T02:31:41.0504574Z ................................i..ii............................................................... 6300/9240
2019-10-25T02:32:02.7659464Z ..................................................................................................i. 6500/9240
2019-10-25T02:32:05.1168450Z .................................................................................................... 6600/9240
2019-10-25T02:32:07.5682770Z .........................................................................i.......................... 6700/9240
2019-10-25T02:32:10.6074418Z .................................................................................................... 6800/9240
---
2019-10-25T02:36:24.9955207Z 5    | ^^^^^^
2019-10-25T02:36:24.9955259Z 
2019-10-25T02:36:24.9955306Z 11    | ^^^^^^^^^^^^^^^^^
2019-10-25T02:36:24.9955352Z 12 
2019-10-25T02:36:24.9955420Z 13 error: malformed `ignore` attribute input
2019-10-25T02:36:24.9955926Z +   --> $DIR/malformed-regressions.rs:6:1
2019-10-25T02:36:24.9955977Z 15    |
2019-10-25T02:36:24.9955977Z 15    |
2019-10-25T02:36:24.9956045Z 16 LL | #[ignore()]
2019-10-25T02:36:24.9956148Z 
2019-10-25T02:36:24.9956194Z 23    | ^^^^^^^^^^^^^^^^^^^^
2019-10-25T02:36:24.9956259Z 24 
2019-10-25T02:36:24.9956259Z 24 
2019-10-25T02:36:24.9956309Z 25 error: malformed `inline` attribute input
2019-10-25T02:36:24.9957206Z +   --> $DIR/malformed-regressions.rs:7:1
2019-10-25T02:36:24.9957258Z 27    |
2019-10-25T02:36:24.9957258Z 27    |
2019-10-25T02:36:24.9957304Z 28 LL | #[inline = ""]
2019-10-25T02:36:24.9957404Z 
2019-10-25T02:36:24.9957715Z 35    | ^^^^^^^^^^^^^^^^^^^^^^^
2019-10-25T02:36:24.9957786Z 36 
2019-10-25T02:36:24.9957786Z 36 
2019-10-25T02:36:24.9957856Z 37 error: malformed `link` attribute input
2019-10-25T02:36:24.9958415Z +   --> $DIR/malformed-regressions.rs:8:1
2019-10-25T02:36:24.9958482Z 39    |
2019-10-25T02:36:24.9958482Z 39    |
2019-10-25T02:36:24.9958528Z 40 LL | #[link]
2019-10-25T02:36:24.9958774Z 41    | ^^^^^^^ help: must be of the form: `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ cfg = "...")]`
2019-10-25T02:36:24.9958912Z 42 
2019-10-25T02:36:24.9958912Z 42 
2019-10-25T02:36:24.9958961Z 43 error: malformed `link` attribute input
2019-10-25T02:36:24.9959529Z +   --> $DIR/malformed-regressions.rs:9:1
2019-10-25T02:36:24.9959580Z 45    |
2019-10-25T02:36:24.9959580Z 45    |
2019-10-25T02:36:24.9959627Z 46 LL | #[link = ""]
2019-10-25T02:36:24.9959710Z 47    | ^^^^^^^^^^^^ help: must be of the form: `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ cfg = "...")]`
2019-10-25T02:36:24.9959795Z 48 
2019-10-25T02:36:24.9959795Z 48 
2019-10-25T02:36:24.9959846Z 49 error: attribute must be of the form `#[test]`
2019-10-25T02:36:24.9960346Z +   --> $DIR/malformed-regressions.rs:1:1
2019-10-25T02:36:24.9960396Z 51    |
2019-10-25T02:36:24.9960396Z 51    |
2019-10-25T02:36:24.9960460Z 52 LL | #[test(foo)]
2019-10-25T02:36:24.9960539Z 
2019-10-25T02:36:24.9960578Z 
2019-10-25T02:36:24.9960649Z The actual stderr differed from the expected stderr.
2019-10-25T02:36:24.9961000Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions/malformed-regressions.stderr
2019-10-25T02:36:24.9961000Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions/malformed-regressions.stderr
2019-10-25T02:36:24.9961287Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T02:36:24.9961601Z To only update this specific test, also pass `--test-args malformed/malformed-regressions.rs`
2019-10-25T02:36:24.9961691Z error: 1 errors occurred comparing output.
2019-10-25T02:36:24.9961739Z status: exit code: 1
2019-10-25T02:36:24.9961739Z status: exit code: 1
2019-10-25T02:36:24.9963532Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/malformed/malformed-regressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions/auxiliary" "-A" "unused"
2019-10-25T02:36:24.9963947Z ------------------------------------------
2019-10-25T02:36:24.9963986Z 
2019-10-25T02:36:24.9964226Z ------------------------------------------
2019-10-25T02:36:24.9964295Z stderr:
2019-10-25T02:36:24.9964295Z stderr:
2019-10-25T02:36:24.9964530Z ------------------------------------------
2019-10-25T02:36:24.9964585Z error: malformed `doc` attribute input
2019-10-25T02:36:24.9964868Z   --> /checkout/src/test/ui/malformed/malformed-regressions.rs:5:1
2019-10-25T02:36:24.9964925Z    |
2019-10-25T02:36:24.9964980Z LL | #[doc] //~ ERROR malformed `doc` attribute input
2019-10-25T02:36:24.9965101Z help: the following are the possible correct uses
2019-10-25T02:36:24.9965150Z    |
2019-10-25T02:36:24.9965150Z    |
2019-10-25T02:36:24.9965215Z LL | #[doc(hidden|inline|...)] //~ ERROR malformed `doc` attribute input
2019-10-25T02:36:24.9965292Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-25T02:36:24.9965347Z LL | #[doc = "string"] //~ ERROR malformed `doc` attribute input
2019-10-25T02:36:24.9965636Z 
2019-10-25T02:36:24.9965636Z 
2019-10-25T02:36:24.9965684Z error: malformed `ignore` attribute input
2019-10-25T02:36:24.9966464Z    |
2019-10-25T02:36:24.9966464Z    |
2019-10-25T02:36:24.9966518Z LL | #[ignore()] //~ ERROR malformed `ignore` attribute input
2019-10-25T02:36:24.9966623Z help: the following are the possible correct uses
2019-10-25T02:36:24.9966693Z    |
2019-10-25T02:36:24.9966693Z    |
2019-10-25T02:36:24.9966744Z LL | #[ignore] //~ ERROR malformed `ignore` attribute input
2019-10-25T02:36:24.9966795Z    | ^^^^^^^^^
2019-10-25T02:36:24.9966868Z LL | #[ignore = "reason"] //~ ERROR malformed `ignore` attribute input
2019-10-25T02:36:24.9967109Z 
2019-10-25T02:36:24.9967109Z 
2019-10-25T02:36:24.9967438Z error: malformed `inline` attribute input
2019-10-25T02:36:24.9968534Z    |
2019-10-25T02:36:24.9968534Z    |
2019-10-25T02:36:24.9968587Z LL | #[inline = ""] //~ ERROR malformed `inline` attribute input
2019-10-25T02:36:24.9968718Z help: the following are the possible correct uses
2019-10-25T02:36:24.9968765Z    |
2019-10-25T02:36:24.9968765Z    |
2019-10-25T02:36:24.9968835Z LL | #[inline] //~ ERROR malformed `inline` attribute input
2019-10-25T02:36:24.9968885Z    | ^^^^^^^^^
2019-10-25T02:36:24.9968939Z LL | #[inline(always|never)] //~ ERROR malformed `inline` attribute input
2019-10-25T02:36:24.9969044Z 
2019-10-25T02:36:24.9969044Z 
2019-10-25T02:36:24.9969090Z error: malformed `link` attribute input
2019-10-25T02:36:24.9969569Z    |
2019-10-25T02:36:24.9969569Z    |
2019-10-25T02:36:24.9969630Z LL | #[link] //~ ERROR malformed `link` attribute input
2019-10-25T02:36:24.9969695Z    | ^^^^^^^ help: must be of the form: `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ cfg = "...")]`
2019-10-25T02:36:24.9969767Z 
2019-10-25T02:36:24.9969821Z error: malformed `link` attribute input
2019-10-25T02:36:24.9970152Z    |
2019-10-25T02:36:24.9970152Z    |
2019-10-25T02:36:24.9970222Z LL | #[link = ""] //~ ERROR malformed `link` attribute input
2019-10-25T02:36:24.9970288Z    | ^^^^^^^^^^^^ help: must be of the form: `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ cfg = "...")]`
2019-10-25T02:36:24.9970328Z 
2019-10-25T02:36:24.9970395Z error: attribute must be of the form `#[test]`
2019-10-25T02:36:24.9970718Z    |
2019-10-25T02:36:24.9970718Z    |
2019-10-25T02:36:24.9970799Z LL | #[test(foo)] //~ ERROR attribute must be of the form `#[test]`
2019-10-25T02:36:24.9970898Z    |
2019-10-25T02:36:24.9970898Z    |
2019-10-25T02:36:24.9970949Z    = note: `#[deny(ill_formed_attribute_input)]` on by default
2019-10-25T02:36:24.9971043Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-10-25T02:36:24.9971533Z    = note: for more information, see issue #57571 <***/issues/57571>
2019-10-25T02:36:24.9971691Z error: aborting due to 6 previous errors
2019-10-25T02:36:24.9971722Z 
2019-10-25T02:36:24.9971749Z 
2019-10-25T02:36:24.9972008Z ------------------------------------------
---
2019-10-25T02:36:25.0022615Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-25T02:36:25.0023198Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T02:36:25.0037404Z 
2019-10-25T02:36:25.0042238Z 
2019-10-25T02:36:25.0044996Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-25T02:36:25.0045334Z 
2019-10-25T02:36:25.0045368Z 
2019-10-25T02:36:25.0058011Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-25T02:36:25.0058097Z Build completed unsuccessfully in 1:09:48
2019-10-25T02:36:25.0058097Z Build completed unsuccessfully in 1:09:48
2019-10-25T02:36:25.0120654Z == clock drift check ==
2019-10-25T02:36:25.0141346Z   local time: Fri Oct 25 02:36:25 UTC 2019
2019-10-25T02:36:25.1130912Z   network time: Fri, 25 Oct 2019 02:36:25 GMT
2019-10-25T02:36:25.1131411Z == end clock drift check ==
2019-10-25T02:36:26.1909312Z 
2019-10-25T02:36:26.2029554Z ##[error]Bash exited with code '1'.
2019-10-25T02:36:26.2079676Z ##[section]Starting: Checkout
2019-10-25T02:36:26.2081642Z ==============================================================================
2019-10-25T02:36:26.2081699Z Task         : Get sources
2019-10-25T02:36:26.2081747Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
