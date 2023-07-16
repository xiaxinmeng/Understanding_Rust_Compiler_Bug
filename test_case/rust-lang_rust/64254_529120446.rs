plain
2019-09-07T14:41:16.3437670Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-07T14:41:16.3652338Z ##[command]git config gc.auto 0
2019-09-07T14:41:16.3717122Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-07T14:41:16.3800673Z ##[command]git config --get-all http.proxy
2019-09-07T14:41:16.4345869Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64254/merge:refs/remotes/pull/64254/merge
---
2019-09-07T15:48:28.7207101Z .................................................................................................... 1500/9007
2019-09-07T15:48:34.9592045Z .................................................................................................... 1600/9007
2019-09-07T15:48:48.5026160Z ......................................................i...............i............................. 1700/9007
2019-09-07T15:48:56.9863930Z .................................................................................................... 1800/9007
2019-09-07T15:49:12.8308884Z .............................................iiiii.................................................. 1900/9007
2019-09-07T15:49:24.5129828Z .................................................................................................... 2100/9007
2019-09-07T15:49:27.3666432Z .................................................................................................... 2200/9007
2019-09-07T15:49:31.2819533Z .................................................................................................... 2300/9007
2019-09-07T15:49:39.8087118Z .................................................................................................... 2400/9007
---
2019-09-07T15:52:50.2579706Z ..................................i...............i................................................. 4700/9007
2019-09-07T15:53:02.6264730Z .................................................................................................... 4800/9007
2019-09-07T15:53:09.4831578Z .................................................................................................... 4900/9007
2019-09-07T15:53:21.0174627Z .................................................................................................... 5000/9007
2019-09-07T15:53:27.4588281Z ................ii.ii............................................................................... 5100/9007
2019-09-07T15:53:38.9128878Z .................................................................................................... 5300/9007
2019-09-07T15:53:49.8941028Z ...............................................................................i.................... 5400/9007
2019-09-07T15:53:58.2347264Z .................................................................................................... 5500/9007
2019-09-07T15:54:04.7354390Z .................................................................................................... 5600/9007
2019-09-07T15:54:04.7354390Z .................................................................................................... 5600/9007
2019-09-07T15:54:16.1358361Z .........................................................................ii...i..ii............i.... 5700/9007
2019-09-07T15:54:43.1035230Z .................................................................................................... 5900/9007
2019-09-07T15:54:54.0249174Z .................................................................................................... 6000/9007
2019-09-07T15:54:54.0249174Z .................................................................................................... 6000/9007
2019-09-07T15:55:01.9394619Z ...........................................................................i..ii.................... 6100/9007
2019-09-07T15:55:33.6289593Z .................................................................................................... 6300/9007
2019-09-07T15:55:35.8961945Z ..................................i................................................................. 6400/9007
2019-09-07T15:55:38.2507125Z .................................................................................................... 6500/9007
2019-09-07T15:55:41.0724145Z ......i............................................................................................. 6600/9007
---
2019-09-07T16:00:48.3423736Z  finished in 22.865
2019-09-07T16:00:48.3632184Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-07T16:00:48.5385203Z 
2019-09-07T16:00:48.5385806Z running 150 tests
2019-09-07T16:00:52.0251224Z i....iii......iii..iiii....i...........FF................i..i..................i....i.........ii.i.i 100/150
2019-09-07T16:00:54.1066564Z ..iiii..............i.........iii.i.......ii.FF...
2019-09-07T16:00:54.1069149Z 
2019-09-07T16:00:54.1069781Z ---- [codegen] codegen/i686-macosx-deployment-target.rs stdout ----
2019-09-07T16:00:54.1070191Z 
2019-09-07T16:00:54.1070385Z error: compilation failed!
2019-09-07T16:00:54.1070385Z error: compilation failed!
2019-09-07T16:00:54.1070541Z status: exit code: 1
2019-09-07T16:00:54.1072069Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/i686-macosx-deployment-target.rs" "-Zthreads=1" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/i686-macosx-deployment-target" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "--target=i686-apple-darwin" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/i686-macosx-deployment-target/auxiliary" "--emit=llvm-ir"
2019-09-07T16:00:54.1072975Z ------------------------------------------
2019-09-07T16:00:54.1073156Z 
2019-09-07T16:00:54.1073590Z ------------------------------------------
2019-09-07T16:00:54.1073773Z stderr:
2019-09-07T16:00:54.1073773Z stderr:
2019-09-07T16:00:54.1074132Z ------------------------------------------
2019-09-07T16:00:54.1074354Z error: Error loading target specification: failed to get macosx SDK path: No such file or directory (os error 2)
2019-09-07T16:00:54.1074501Z   |
2019-09-07T16:00:54.1075232Z   = help: Use `--print target-list` for a list of built-in targets
2019-09-07T16:00:54.1075625Z 
2019-09-07T16:00:54.1076017Z ------------------------------------------
2019-09-07T16:00:54.1076219Z 
2019-09-07T16:00:54.1076337Z 
2019-09-07T16:00:54.1076337Z 
2019-09-07T16:00:54.1076735Z ---- [codegen] codegen/i686-no-macosx-deployment-target.rs stdout ----
2019-09-07T16:00:54.1077178Z 
2019-09-07T16:00:54.1077324Z error: compilation failed!
2019-09-07T16:00:54.1077460Z status: exit code: 1
2019-09-07T16:00:54.1078500Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/i686-no-macosx-deployment-target.rs" "-Zthreads=1" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/i686-no-macosx-deployment-target" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "--target=i686-apple-darwin" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/i686-no-macosx-deployment-target/auxiliary" "--emit=llvm-ir"
2019-09-07T16:00:54.1079129Z ------------------------------------------
2019-09-07T16:00:54.1079296Z 
2019-09-07T16:00:54.1079656Z ------------------------------------------
2019-09-07T16:00:54.1079851Z stderr:
2019-09-07T16:00:54.1079851Z stderr:
2019-09-07T16:00:54.1080204Z ------------------------------------------
2019-09-07T16:00:54.1080424Z error: Error loading target specification: failed to get macosx SDK path: No such file or directory (os error 2)
2019-09-07T16:00:54.1080568Z   |
2019-09-07T16:00:54.1081068Z   = help: Use `--print target-list` for a list of built-in targets
2019-09-07T16:00:54.1081390Z 
2019-09-07T16:00:54.1082113Z ------------------------------------------
2019-09-07T16:00:54.1082307Z 
2019-09-07T16:00:54.1082458Z 
2019-09-07T16:00:54.1082458Z 
2019-09-07T16:00:54.1082866Z ---- [codegen] codegen/x86_64-macosx-deployment-target.rs stdout ----
2019-09-07T16:00:54.1083032Z 
2019-09-07T16:00:54.1083191Z error: compilation failed!
2019-09-07T16:00:54.1083336Z status: exit code: 1
2019-09-07T16:00:54.1084372Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/x86_64-macosx-deployment-target.rs" "-Zthreads=1" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/x86_64-macosx-deployment-target" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "--target=x86_64-apple-darwin" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/x86_64-macosx-deployment-target/auxiliary" "--emit=llvm-ir"
2019-09-07T16:00:54.1085086Z ------------------------------------------
2019-09-07T16:00:54.1085280Z 
2019-09-07T16:00:54.1085658Z ------------------------------------------
2019-09-07T16:00:54.1085842Z stderr:
2019-09-07T16:00:54.1085842Z stderr:
2019-09-07T16:00:54.1086227Z ------------------------------------------
2019-09-07T16:00:54.1086446Z error: Error loading target specification: failed to get macosx SDK path: No such file or directory (os error 2)
2019-09-07T16:00:54.1086628Z   |
2019-09-07T16:00:54.1087029Z   = help: Use `--print target-list` for a list of built-in targets
2019-09-07T16:00:54.1087318Z 
2019-09-07T16:00:54.1087702Z ------------------------------------------
2019-09-07T16:00:54.1087880Z 
2019-09-07T16:00:54.1088001Z 
2019-09-07T16:00:54.1088001Z 
2019-09-07T16:00:54.1088496Z ---- [codegen] codegen/x86_64-no-macosx-deployment-target.rs stdout ----
2019-09-07T16:00:54.1088674Z 
2019-09-07T16:00:54.1088815Z error: compilation failed!
2019-09-07T16:00:54.1088963Z status: exit code: 1
2019-09-07T16:00:54.1089898Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/x86_64-no-macosx-deployment-target.rs" "-Zthreads=1" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/x86_64-no-macosx-deployment-target" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "--target=x86_64-apple-darwin" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/x86_64-no-macosx-deployment-target/auxiliary" "--emit=llvm-ir"
2019-09-07T16:00:54.1090549Z ------------------------------------------
2019-09-07T16:00:54.1090887Z 
2019-09-07T16:00:54.1091750Z ------------------------------------------
2019-09-07T16:00:54.1092035Z stderr:
2019-09-07T16:00:54.1092035Z stderr:
2019-09-07T16:00:54.1092551Z ------------------------------------------
2019-09-07T16:00:54.1092993Z error: Error loading target specification: failed to get macosx SDK path: No such file or directory (os error 2)
2019-09-07T16:00:54.1093165Z   |
2019-09-07T16:00:54.1093594Z   = help: Use `--print target-list` for a list of built-in targets
2019-09-07T16:00:54.1093925Z 
2019-09-07T16:00:54.1094300Z ------------------------------------------
2019-09-07T16:00:54.1094470Z 
2019-09-07T16:00:54.1094617Z 
---
2019-09-07T16:00:54.1099486Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-07T16:00:54.1099825Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-07T16:00:54.1099966Z 
2019-09-07T16:00:54.1100081Z 
2019-09-07T16:00:54.1102362Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-07T16:00:54.1102984Z 
2019-09-07T16:00:54.1103110Z 
2019-09-07T16:00:54.1106428Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-07T16:00:54.1106768Z Build completed unsuccessfully in 1:12:04
2019-09-07T16:00:54.1106768Z Build completed unsuccessfully in 1:12:04
2019-09-07T16:00:54.1173714Z == clock drift check ==
2019-09-07T16:00:54.1191224Z   local time: Sat Sep  7 16:00:54 UTC 2019
2019-09-07T16:00:54.2091131Z   network time: Sat, 07 Sep 2019 16:00:54 GMT
2019-09-07T16:00:54.2092524Z == end clock drift check ==
2019-09-07T16:00:57.2162633Z ##[error]Bash exited with code '1'.
2019-09-07T16:00:57.2216065Z ##[section]Starting: Checkout
2019-09-07T16:00:57.2218213Z ==============================================================================
2019-09-07T16:00:57.2218293Z Task         : Get sources
2019-09-07T16:00:57.2218345Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
