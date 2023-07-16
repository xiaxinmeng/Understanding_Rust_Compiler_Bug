plain
2019-10-25T14:37:37.6958749Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T14:37:37.7186501Z ##[command]git config gc.auto 0
2019-10-25T14:37:37.7234194Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T14:37:37.7282182Z ##[command]git config --get-all http.proxy
2019-10-25T14:37:37.7410019Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65811/merge:refs/remotes/pull/65811/merge
---
2019-10-25T15:33:53.8143440Z .................................................................................................... 1600/9254
2019-10-25T15:33:59.1679053Z .................................................................................................... 1700/9254
2019-10-25T15:34:10.9466486Z ........................................................i...............i........................... 1800/9254
2019-10-25T15:34:18.0798647Z .................................................................................................... 1900/9254
2019-10-25T15:34:32.0208395Z ..............................................iiiii................................................. 2000/9254
2019-10-25T15:34:42.1387644Z .................................................................................................... 2200/9254
2019-10-25T15:34:44.5706269Z .................................................................................................... 2300/9254
2019-10-25T15:34:48.1862552Z .................................................................................................... 2400/9254
2019-10-25T15:35:09.8671313Z .................................................................................................... 2500/9254
---
2019-10-25T15:37:53.5823790Z .................................................i...............i.................................. 4800/9254
2019-10-25T15:38:02.1788087Z .................................................................................................... 4900/9254
2019-10-25T15:38:10.2603874Z .................................................................................................... 5000/9254
2019-10-25T15:38:16.1847444Z .................................................................................................... 5100/9254
2019-10-25T15:38:25.9665886Z ..................................................ii.ii...........i................................. 5200/9254
2019-10-25T15:38:35.1673052Z .................................................................................................... 5400/9254
2019-10-25T15:38:43.9281766Z .................................................................................................... 5500/9254
2019-10-25T15:38:51.4893614Z ....................i............................................................................... 5600/9254
2019-10-25T15:38:56.7239208Z .................................................................................................... 5700/9254
2019-10-25T15:38:56.7239208Z .................................................................................................... 5700/9254
2019-10-25T15:39:08.2940941Z .................................................................................................... 5800/9254
2019-10-25T15:39:19.3779029Z .................ii...i..ii...........i............................................................. 5900/9254
2019-10-25T15:39:40.2492199Z .................................................................................................... 6100/9254
2019-10-25T15:39:46.1935635Z .................................................................................................... 6200/9254
2019-10-25T15:39:46.1935635Z .................................................................................................... 6200/9254
2019-10-25T15:39:58.3070677Z ........................................i..ii....................................................... 6300/9254
2019-10-25T15:40:19.0654287Z .................................................................................................... 6500/9254
2019-10-25T15:40:21.0792047Z ......i............................................................................................. 6600/9254
2019-10-25T15:40:23.2280813Z ......FFF...........................................................................i............... 6700/9254
2019-10-25T15:40:25.7532318Z .................................................................................................... 6800/9254
---
2019-10-25T15:44:18.2139182Z 1 error: invalid variable declaration
2019-10-25T15:44:18.2139445Z -   --> $DIR/issue-65257--auto-instead-of-let-recovery.rs:4:5
2019-10-25T15:44:18.2140054Z +   --> $DIR/issue-65257--auto-instead-of-let-recovery.rs:3:5
2019-10-25T15:44:18.2140095Z 3    |
2019-10-25T15:44:18.2140151Z 4 LL |     auto n = 0;
2019-10-25T15:44:18.2140203Z 5    |     ^^^^ help: to introduce a variable, write `let` instead of `auto`
2019-10-25T15:44:18.2140282Z 6 
2019-10-25T15:44:18.2140317Z 7 error: invalid variable declaration
2019-10-25T15:44:18.2140521Z -   --> $DIR/issue-65257--auto-instead-of-let-recovery.rs:6:5
2019-10-25T15:44:18.2140876Z +   --> $DIR/issue-65257--auto-instead-of-let-recovery.rs:5:5
2019-10-25T15:44:18.2140876Z +   --> $DIR/issue-65257--auto-instead-of-let-recovery.rs:5:5
2019-10-25T15:44:18.2140930Z 9    |
2019-10-25T15:44:18.2140964Z 10 LL |     auto m;
2019-10-25T15:44:18.2141004Z 11    |     ^^^^ help: to introduce a variable, write `let` instead of `auto`
2019-10-25T15:44:18.2141073Z 
2019-10-25T15:44:18.2141108Z The actual stderr differed from the expected stderr.
2019-10-25T15:44:18.2141108Z The actual stderr differed from the expected stderr.
2019-10-25T15:44:18.2141552Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-65257--auto-instead-of-let-recovery/issue-65257--auto-instead-of-let-recovery.stderr
2019-10-25T15:44:18.2141779Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T15:44:18.2142044Z To only update this specific test, also pass `--test-args parser/issue-65257--auto-instead-of-let-recovery.rs`
2019-10-25T15:44:18.2142127Z error: 1 errors occurred comparing output.
2019-10-25T15:44:18.2142163Z status: exit code: 1
2019-10-25T15:44:18.2142163Z status: exit code: 1
2019-10-25T15:44:18.2142817Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-65257--auto-instead-of-let-recovery.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-65257--auto-instead-of-let-recovery" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-65257--auto-instead-of-let-recovery/auxiliary" "-A" "unused"
2019-10-25T15:44:18.2143106Z ------------------------------------------
2019-10-25T15:44:18.2143240Z 
2019-10-25T15:44:18.2143455Z ------------------------------------------
2019-10-25T15:44:18.2143494Z stderr:
2019-10-25T15:44:18.2143494Z stderr:
2019-10-25T15:44:18.2143693Z ------------------------------------------
2019-10-25T15:44:18.2143732Z error: invalid variable declaration
2019-10-25T15:44:18.2143946Z   --> /checkout/src/test/ui/parser/issue-65257--auto-instead-of-let-recovery.rs:3:5
2019-10-25T15:44:18.2144002Z    |
2019-10-25T15:44:18.2144042Z LL |     auto n = 0;//~ ERROR invalid variable declaration
2019-10-25T15:44:18.2144083Z    |     ^^^^ help: to introduce a variable, write `let` instead of `auto`
2019-10-25T15:44:18.2144161Z error: invalid variable declaration
2019-10-25T15:44:18.2144381Z   --> /checkout/src/test/ui/parser/issue-65257--auto-instead-of-let-recovery.rs:5:5
2019-10-25T15:44:18.2144445Z    |
2019-10-25T15:44:18.2144445Z    |
2019-10-25T15:44:18.2144482Z LL |     auto m;//~ ERROR invalid variable declaration
2019-10-25T15:44:18.2144530Z    |     ^^^^ help: to introduce a variable, write `let` instead of `auto`
2019-10-25T15:44:18.2144607Z error: aborting due to 2 previous errors
2019-10-25T15:44:18.2144630Z 
2019-10-25T15:44:18.2144651Z 
2019-10-25T15:44:18.2144837Z ------------------------------------------
---
2019-10-25T15:44:18.2145214Z 1 error: invalid variable declaration
2019-10-25T15:44:18.2145412Z -   --> $DIR/issue-65257--var-instead-of-let-recovery.rs:4:5
2019-10-25T15:44:18.2145787Z +   --> $DIR/issue-65257--var-instead-of-let-recovery.rs:3:5
2019-10-25T15:44:18.2145922Z 3    |
2019-10-25T15:44:18.2146330Z 4 LL |     var n = 0;
2019-10-25T15:44:18.2146382Z 5    |     ^^^ help: to introduce a variable, write `let` instead of `var`
2019-10-25T15:44:18.2146472Z 6 
2019-10-25T15:44:18.2146525Z 7 error: invalid variable declaration
2019-10-25T15:44:18.2146837Z -   --> $DIR/issue-65257--var-instead-of-let-recovery.rs:6:5
2019-10-25T15:44:18.2147096Z +   --> $DIR/issue-65257--var-instead-of-let-recovery.rs:5:5
2019-10-25T15:44:18.2147096Z +   --> $DIR/issue-65257--var-instead-of-let-recovery.rs:5:5
2019-10-25T15:44:18.2147143Z 9    |
2019-10-25T15:44:18.2147184Z 10 LL |     var m;
2019-10-25T15:44:18.2147249Z 11    |     ^^^ help: to introduce a variable, write `let` instead of `var`
2019-10-25T15:44:18.2147308Z 
2019-10-25T15:44:18.2147353Z The actual stderr differed from the expected stderr.
2019-10-25T15:44:18.2147353Z The actual stderr differed from the expected stderr.
2019-10-25T15:44:18.2147725Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-65257--var-instead-of-let-recovery/issue-65257--var-instead-of-let-recovery.stderr
2019-10-25T15:44:18.2147995Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T15:44:18.2148302Z To only update this specific test, also pass `--test-args parser/issue-65257--var-instead-of-let-recovery.rs`
2019-10-25T15:44:18.2148412Z error: 1 errors occurred comparing output.
2019-10-25T15:44:18.2148460Z status: exit code: 1
2019-10-25T15:44:18.2148460Z status: exit code: 1
2019-10-25T15:44:18.2149293Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-65257--var-instead-of-let-recovery.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-65257--var-instead-of-let-recovery" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-65257--var-instead-of-let-recovery/auxiliary" "-A" "unused"
2019-10-25T15:44:18.2149979Z ------------------------------------------
2019-10-25T15:44:18.2150171Z 
2019-10-25T15:44:18.2150340Z ------------------------------------------
2019-10-25T15:44:18.2150390Z stderr:
2019-10-25T15:44:18.2150390Z stderr:
2019-10-25T15:44:18.2150648Z ------------------------------------------
2019-10-25T15:44:18.2150694Z error: invalid variable declaration
2019-10-25T15:44:18.2150914Z   --> /checkout/src/test/ui/parser/issue-65257--var-instead-of-let-recovery.rs:3:5
2019-10-25T15:44:18.2150973Z    |
2019-10-25T15:44:18.2151011Z LL |     var n = 0;//~ ERROR invalid variable declaration
2019-10-25T15:44:18.2151052Z    |     ^^^ help: to introduce a variable, write `let` instead of `var`
2019-10-25T15:44:18.2151125Z error: invalid variable declaration
2019-10-25T15:44:18.2151329Z   --> /checkout/src/test/ui/parser/issue-65257--var-instead-of-let-recovery.rs:5:5
2019-10-25T15:44:18.2151383Z    |
2019-10-25T15:44:18.2151383Z    |
2019-10-25T15:44:18.2151419Z LL |     var m;//~ ERROR invalid variable declaration
2019-10-25T15:44:18.2151466Z    |     ^^^ help: to introduce a variable, write `let` instead of `var`
2019-10-25T15:44:18.2151540Z error: aborting due to 2 previous errors
2019-10-25T15:44:18.2151563Z 
2019-10-25T15:44:18.2151588Z 
2019-10-25T15:44:18.2151760Z ------------------------------------------
---
2019-10-25T15:44:18.2153686Z 11    |     ^^^ help: missing `let`
2019-10-25T15:44:18.2153708Z 
2019-10-25T15:44:18.2153728Z 
2019-10-25T15:44:18.2153761Z The actual stderr differed from the expected stderr.
2019-10-25T15:44:18.2154048Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-65257-mut-instead-of-let-recovery/issue-65257-mut-instead-of-let-recovery.stderr
2019-10-25T15:44:18.2154246Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T15:44:18.2154471Z To only update this specific test, also pass `--test-args parser/issue-65257-mut-instead-of-let-recovery.rs`
2019-10-25T15:44:18.2154557Z error: 1 errors occurred comparing output.
2019-10-25T15:44:18.2154590Z status: exit code: 1
2019-10-25T15:44:18.2154590Z status: exit code: 1
2019-10-25T15:44:18.2155231Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-65257-mut-instead-of-let-recovery.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-65257-mut-instead-of-let-recovery" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-65257-mut-instead-of-let-recovery/auxiliary" "-A" "unused"
2019-10-25T15:44:18.2155499Z ------------------------------------------
2019-10-25T15:44:18.2155524Z 
2019-10-25T15:44:18.2155872Z ------------------------------------------
2019-10-25T15:44:18.2155930Z stderr:
2019-10-25T15:44:18.2155930Z stderr:
2019-10-25T15:44:18.2156986Z ------------------------------------------
2019-10-25T15:44:18.2157051Z error: invalid variable declaration
2019-10-25T15:44:18.2157493Z   --> /checkout/src/test/ui/parser/issue-65257-mut-instead-of-let-recovery.rs:3:5
2019-10-25T15:44:18.2157556Z    |
2019-10-25T15:44:18.2157606Z LL |     mut n = 0;//~ ERROR invalid variable declaration
2019-10-25T15:44:18.2157706Z 
2019-10-25T15:44:18.2157747Z error: invalid variable declaration
2019-10-25T15:44:18.2158041Z   --> /checkout/src/test/ui/parser/issue-65257-mut-instead-of-let-recovery.rs:5:5
2019-10-25T15:44:18.2158107Z    |
2019-10-25T15:44:18.2158107Z    |
2019-10-25T15:44:18.2158154Z LL |     mut var;//~ ERROR invalid variable declaration
2019-10-25T15:44:18.2158229Z 
2019-10-25T15:44:18.2158291Z error: aborting due to 2 previous errors
2019-10-25T15:44:18.2158321Z 
2019-10-25T15:44:18.2158346Z 
---
2019-10-25T15:44:18.2169060Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-25T15:44:18.2169174Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T15:44:18.2188054Z 
2019-10-25T15:44:18.2188141Z 
2019-10-25T15:44:18.2196448Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-25T15:44:18.2196935Z 
2019-10-25T15:44:18.2197164Z 
2019-10-25T15:44:18.2198373Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-25T15:44:18.2198435Z Build completed unsuccessfully in 1:00:06
2019-10-25T15:44:18.2198435Z Build completed unsuccessfully in 1:00:06
2019-10-25T15:44:18.2249955Z == clock drift check ==
2019-10-25T15:44:18.2261505Z   local time: Fri Oct 25 15:44:18 UTC 2019
2019-10-25T15:44:18.3694627Z   network time: Fri, 25 Oct 2019 15:44:18 GMT
2019-10-25T15:44:18.3695127Z == end clock drift check ==
2019-10-25T15:44:19.3144731Z 
2019-10-25T15:44:19.3269996Z ##[error]Bash exited with code '1'.
2019-10-25T15:44:19.3309395Z ##[section]Starting: Checkout
2019-10-25T15:44:19.3311119Z ==============================================================================
2019-10-25T15:44:19.3311191Z Task         : Get sources
2019-10-25T15:44:19.3311251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
