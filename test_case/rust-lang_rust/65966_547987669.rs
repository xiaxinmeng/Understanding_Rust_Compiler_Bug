plain
2019-10-30T15:07:22.1247893Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-30T15:07:22.1465740Z ##[command]git config gc.auto 0
2019-10-30T15:07:22.1545812Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-30T15:07:22.1607390Z ##[command]git config --get-all http.proxy
2019-10-30T15:07:23.0904834Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65966/merge:refs/remotes/pull/65966/merge
---
2019-10-30T16:03:59.0170834Z .................................................................................................... 1600/9264
2019-10-30T16:04:04.6604452Z .................................................................................................... 1700/9264
2019-10-30T16:04:16.8301466Z ..........................................................i...............i......................... 1800/9264
2019-10-30T16:04:24.5154269Z .................................................................................................... 1900/9264
2019-10-30T16:04:38.5334674Z ................................................iiiii............................................... 2000/9264
2019-10-30T16:04:48.8693802Z .................................................................................................... 2200/9264
2019-10-30T16:04:51.2677968Z .................................................................................................... 2300/9264
2019-10-30T16:04:54.8486060Z .................................................................................................... 2400/9264
2019-10-30T16:05:17.2455848Z .................................................................................................... 2500/9264
---
2019-10-30T16:08:03.9978042Z .................................................i...............i.................................. 4800/9264
2019-10-30T16:08:12.4827580Z .................................................................................................... 4900/9264
2019-10-30T16:08:20.8072694Z .................................................................................................... 5000/9264
2019-10-30T16:08:26.7545294Z .................................................................................................... 5100/9264
2019-10-30T16:08:36.7313096Z ..................................................ii.ii...........i................................. 5200/9264
2019-10-30T16:08:46.2195004Z .................................................................................................... 5400/9264
2019-10-30T16:08:55.8784217Z .................................................................................................... 5500/9264
2019-10-30T16:09:03.1307304Z .......................i............................................................................ 5600/9264
2019-10-30T16:09:09.4105144Z .................................................................................................... 5700/9264
2019-10-30T16:09:09.4105144Z .................................................................................................... 5700/9264
2019-10-30T16:09:20.9068584Z .................................................................................................... 5800/9264
2019-10-30T16:09:32.8419770Z ........ii...i..ii...........i...................................................................... 5900/9264
2019-10-30T16:09:53.5562498Z .................................................................................................... 6100/9264
2019-10-30T16:09:59.0290938Z .................................................................................................... 6200/9264
2019-10-30T16:09:59.0290938Z .................................................................................................... 6200/9264
2019-10-30T16:10:12.6666456Z ...........................i..ii.................................................................... 6300/9264
2019-10-30T16:10:32.1406871Z .............................................................................................i...... 6500/9264
2019-10-30T16:10:34.2102838Z ..........................................................................................FF........ 6600/9264
2019-10-30T16:10:36.3181608Z .....................................................................i.............................. 6700/9264
2019-10-30T16:10:39.0187205Z .................................................................................................... 6800/9264
---
2019-10-30T16:15:24.3334956Z diff of stderr:
2019-10-30T16:15:24.3335082Z 
2019-10-30T16:15:24.3335228Z 34    |
2019-10-30T16:15:24.3335399Z 35 
2019-10-30T16:15:24.3335530Z 36 error: expected one of `.`, `?`, `{`, or an operator, found `}`
2019-10-30T16:15:24.3336335Z -   --> $DIR/issue-62973.rs:8:1
2019-10-30T16:15:24.3336766Z +   --> $DIR/issue-62973.rs:8:2
2019-10-30T16:15:24.3336941Z 38    |
2019-10-30T16:15:24.3337106Z 39 LL | fn p() { match s { v, E { [) {) }
2019-10-30T16:15:24.3337474Z 40    |          ----- while parsing this match expression
2019-10-30T16:15:24.3337774Z 
2019-10-30T16:15:24.3338120Z The actual stderr differed from the expected stderr.
2019-10-30T16:15:24.3338635Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62973/issue-62973.stderr
2019-10-30T16:15:24.3338635Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62973/issue-62973.stderr
2019-10-30T16:15:24.3339081Z To update references, rerun the tests and pass the `--bless` flag
2019-10-30T16:15:24.3339700Z To only update this specific test, also pass `--test-args parser/issue-62973.rs`
2019-10-30T16:15:24.3339999Z error: 1 errors occurred comparing output.
2019-10-30T16:15:24.3340140Z status: exit code: 1
2019-10-30T16:15:24.3340140Z status: exit code: 1
2019-10-30T16:15:24.3340946Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-62973.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62973" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62973/auxiliary" "-A" "unused"
2019-10-30T16:15:24.3341498Z ------------------------------------------
2019-10-30T16:15:24.3341643Z 
2019-10-30T16:15:24.3341954Z ------------------------------------------
2019-10-30T16:15:24.3342133Z stderr:
2019-10-30T16:15:24.3342133Z stderr:
2019-10-30T16:15:24.3342441Z ------------------------------------------
2019-10-30T16:15:24.3342784Z error: this file contains an un-closed delimiter
2019-10-30T16:15:24.3343350Z   --> /checkout/src/test/ui/parser/issue-62973.rs:8:2
2019-10-30T16:15:24.3343514Z    |
2019-10-30T16:15:24.3343639Z LL | fn p() { match s { v, E { [) {) }
2019-10-30T16:15:24.3343977Z    |        -         - un-closed delimiter
2019-10-30T16:15:24.3344432Z    |        un-closed delimiter
2019-10-30T16:15:24.3344615Z LL | 
2019-10-30T16:15:24.3344734Z LL | 
2019-10-30T16:15:24.3344852Z    |  ^
2019-10-30T16:15:24.3344852Z    |  ^
2019-10-30T16:15:24.3344980Z 
2019-10-30T16:15:24.3345115Z error: expected one of `,` or `}`, found `{`
2019-10-30T16:15:24.3345447Z   --> /checkout/src/test/ui/parser/issue-62973.rs:6:25
2019-10-30T16:15:24.3345632Z    |
2019-10-30T16:15:24.3346231Z LL | fn p() { match s { v, E { [) {) }
2019-10-30T16:15:24.3346634Z    |                -        ^ expected one of `,` or `}` here
2019-10-30T16:15:24.3346992Z    |                while parsing this struct
2019-10-30T16:15:24.3347110Z 
2019-10-30T16:15:24.3347289Z error: struct literals are not allowed here
2019-10-30T16:15:24.3347655Z   --> /checkout/src/test/ui/parser/issue-62973.rs:6:16
2019-10-30T16:15:24.3347655Z   --> /checkout/src/test/ui/parser/issue-62973.rs:6:16
2019-10-30T16:15:24.3347875Z    |
2019-10-30T16:15:24.3348019Z LL |   fn p() { match s { v, E { [) {) }
2019-10-30T16:15:24.3348309Z LL | |
2019-10-30T16:15:24.3348444Z LL | |
2019-10-30T16:15:24.3348575Z    | |_^
2019-10-30T16:15:24.3348724Z    |
2019-10-30T16:15:24.3348724Z    |
2019-10-30T16:15:24.3348865Z help: surround the struct literal with parentheses
2019-10-30T16:15:24.3349017Z    |
2019-10-30T16:15:24.3349181Z LL | fn p() { match (s { v, E { [) {) }
2019-10-30T16:15:24.3349615Z LL | )
2019-10-30T16:15:24.3349759Z    |
2019-10-30T16:15:24.3349865Z 
2019-10-30T16:15:24.3349865Z 
2019-10-30T16:15:24.3349989Z error: expected one of `.`, `?`, `{`, or an operator, found `}`
2019-10-30T16:15:24.3350348Z   --> /checkout/src/test/ui/parser/issue-62973.rs:8:2
2019-10-30T16:15:24.3350511Z    |
2019-10-30T16:15:24.3350634Z LL | fn p() { match s { v, E { [) {) }
2019-10-30T16:15:24.3350986Z    |          ----- while parsing this match expression
2019-10-30T16:15:24.3351284Z LL | 
2019-10-30T16:15:24.3351284Z LL | 
2019-10-30T16:15:24.3351440Z    |  ^ expected one of `.`, `?`, `{`, or an operator here
2019-10-30T16:15:24.3351682Z error: incorrect close delimiter: `)`
2019-10-30T16:15:24.3352059Z   --> /checkout/src/test/ui/parser/issue-62973.rs:6:28
2019-10-30T16:15:24.3352226Z    |
2019-10-30T16:15:24.3352226Z    |
2019-10-30T16:15:24.3352356Z LL | fn p() { match s { v, E { [) {) }
2019-10-30T16:15:24.3352835Z    |                           -^ incorrect close delimiter
2019-10-30T16:15:24.3353406Z    |                           un-closed delimiter
2019-10-30T16:15:24.3353580Z 
2019-10-30T16:15:24.3353710Z error: incorrect close delimiter: `)`
2019-10-30T16:15:24.3354063Z   --> /checkout/src/test/ui/parser/issue-62973.rs:6:31
2019-10-30T16:15:24.3354063Z   --> /checkout/src/test/ui/parser/issue-62973.rs:6:31
2019-10-30T16:15:24.3354246Z    |
2019-10-30T16:15:24.3354440Z LL | fn p() { match s { v, E { [) {) }
2019-10-30T16:15:24.3354868Z    |                              -^ incorrect close delimiter
2019-10-30T16:15:24.3356274Z    |                              un-closed delimiter
2019-10-30T16:15:24.3356320Z 
2019-10-30T16:15:24.3357155Z error: aborting due to 6 previous errors
2019-10-30T16:15:24.3357192Z 
---
2019-10-30T16:15:24.3357902Z diff of stderr:
2019-10-30T16:15:24.3357932Z 
2019-10-30T16:15:24.3357980Z 23    |        `..` must be at the end and cannot have a trailing comma
2019-10-30T16:15:24.3358027Z 24 
2019-10-30T16:15:24.3358091Z 25 error: expected `[`, found `}`
2019-10-30T16:15:24.3358299Z -   --> $DIR/issue-63135.rs:3:15
2019-10-30T16:15:24.3358504Z +   --> $DIR/issue-63135.rs:3:16
2019-10-30T16:15:24.3358709Z 27    |
2019-10-30T16:15:24.3358752Z 28 LL | fn i(n{...,f #
2019-10-30T16:15:24.3358799Z 29    |                ^ expected `[`
2019-10-30T16:15:24.3358888Z 30 
2019-10-30T16:15:24.3358888Z 30 
2019-10-30T16:15:24.3358933Z 31 error: expected one of `:` or `|`, found `)`
2019-10-30T16:15:24.3359176Z -   --> $DIR/issue-63135.rs:3:15
2019-10-30T16:15:24.3359403Z +   --> $DIR/issue-63135.rs:3:16
2019-10-30T16:15:24.3359450Z 33    |
2019-10-30T16:15:24.3359699Z 34 LL | fn i(n{...,f #
2019-10-30T16:15:24.3359752Z 35    |                ^ expected one of `:` or `|` here
2019-10-30T16:15:24.3359824Z 
2019-10-30T16:15:24.3359864Z The actual stderr differed from the expected stderr.
2019-10-30T16:15:24.3360130Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-63135/issue-63135.stderr
2019-10-30T16:15:24.3360130Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-63135/issue-63135.stderr
2019-10-30T16:15:24.3360376Z To update references, rerun the tests and pass the `--bless` flag
2019-10-30T16:15:24.3360641Z To only update this specific test, also pass `--test-args parser/issue-63135.rs`
2019-10-30T16:15:24.3360738Z error: 1 errors occurred comparing output.
2019-10-30T16:15:24.3360782Z status: exit code: 1
2019-10-30T16:15:24.3360782Z status: exit code: 1
2019-10-30T16:15:24.3361447Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-63135.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-63135" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-63135/auxiliary" "-A" "unused"
2019-10-30T16:15:24.3362209Z ------------------------------------------
2019-10-30T16:15:24.3362272Z 
2019-10-30T16:15:24.3362487Z ------------------------------------------
2019-10-30T16:15:24.3362543Z stderr:
2019-10-30T16:15:24.3362543Z stderr:
2019-10-30T16:15:24.3362769Z ------------------------------------------
2019-10-30T16:15:24.3362985Z error: this file contains an un-closed delimiter
2019-10-30T16:15:24.3363207Z   --> /checkout/src/test/ui/parser/issue-63135.rs:3:16
2019-10-30T16:15:24.3363252Z    |
2019-10-30T16:15:24.3363312Z LL | fn i(n{...,f #
2019-10-30T16:15:24.3363496Z    |     - -        ^
2019-10-30T16:15:24.3363744Z    |     | un-closed delimiter
2019-10-30T16:15:24.3364166Z    |     un-closed delimiter
2019-10-30T16:15:24.3364206Z 
2019-10-30T16:15:24.3364249Z error: expected field pattern, found `...`
2019-10-30T16:15:24.3364249Z error: expected field pattern, found `...`
2019-10-30T16:15:24.3364520Z   --> /checkout/src/test/ui/parser/issue-63135.rs:3:8
2019-10-30T16:15:24.3364565Z    |
2019-10-30T16:15:24.3364604Z LL | fn i(n{...,f #
2019-10-30T16:15:24.3364670Z    |        ^^^ help: to omit remaining fields, use one fewer `.`: `..`
2019-10-30T16:15:24.3364704Z 
2019-10-30T16:15:24.3364748Z error: expected `}`, found `,`
2019-10-30T16:15:24.3365237Z   --> /checkout/src/test/ui/parser/issue-63135.rs:3:11
2019-10-30T16:15:24.3365304Z    |
2019-10-30T16:15:24.3365344Z LL | fn i(n{...,f #
2019-10-30T16:15:24.3365525Z    |        ---^
2019-10-30T16:15:24.3365629Z    |        |  expected `}`
2019-10-30T16:15:24.3365676Z    |        `..` must be at the end and cannot have a trailing comma
2019-10-30T16:15:24.3366078Z 
2019-10-30T16:15:24.3366078Z 
2019-10-30T16:15:24.3366323Z error: expected `[`, found `}`
2019-10-30T16:15:24.3366602Z   --> /checkout/src/test/ui/parser/issue-63135.rs:3:16
2019-10-30T16:15:24.3366650Z    |
2019-10-30T16:15:24.3366712Z LL | fn i(n{...,f #
2019-10-30T16:15:24.3366758Z    |                ^ expected `[`
2019-10-30T16:15:24.3366788Z 
2019-10-30T16:15:24.3366831Z error: expected one of `:` or `|`, found `)`
2019-10-30T16:15:24.3367086Z   --> /checkout/src/test/ui/parser/issue-63135.rs:3:16
2019-10-30T16:15:24.3367132Z    |
2019-10-30T16:15:24.3367173Z LL | fn i(n{...,f #
2019-10-30T16:15:24.3367360Z    |                ^ expected one of `:` or `|` here
2019-10-30T16:15:24.3367436Z error: aborting due to 5 previous errors
2019-10-30T16:15:24.3367465Z 
2019-10-30T16:15:24.3367490Z 
2019-10-30T16:15:24.3367751Z ------------------------------------------
---
2019-10-30T16:15:24.3387945Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-30T16:15:24.3388062Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-30T16:15:24.3404558Z 
2019-10-30T16:15:24.3404640Z 
2019-10-30T16:15:24.3407056Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-30T16:15:24.3407327Z 
2019-10-30T16:15:24.3407357Z 
2019-10-30T16:15:24.3413731Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-30T16:15:24.3413940Z Build completed unsuccessfully in 1:01:51
2019-10-30T16:15:24.3413940Z Build completed unsuccessfully in 1:01:51
2019-10-30T16:15:24.3466220Z == clock drift check ==
2019-10-30T16:15:24.3478739Z   local time: Wed Oct 30 16:15:24 UTC 2019
2019-10-30T16:15:24.3831616Z   network time: Wed, 30 Oct 2019 16:15:24 GMT
2019-10-30T16:15:24.3837037Z == end clock drift check ==
2019-10-30T16:15:25.2973236Z 
2019-10-30T16:15:25.3078761Z ##[error]Bash exited with code '1'.
2019-10-30T16:15:25.3117032Z ##[section]Starting: Checkout
2019-10-30T16:15:25.3118807Z ==============================================================================
2019-10-30T16:15:25.3118864Z Task         : Get sources
2019-10-30T16:15:25.3118912Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
