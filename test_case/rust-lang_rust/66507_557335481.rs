plain
2019-11-21T23:24:47.3008807Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-21T23:24:48.0725402Z ##[command]git config gc.auto 0
2019-11-21T23:24:48.0729585Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-21T23:24:48.0734229Z ##[command]git config --get-all http.proxy
2019-11-21T23:24:48.0739881Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66507/merge:refs/remotes/pull/66507/merge
---
2019-11-22T00:25:30.1450355Z .................................................................................................... 1600/9278
2019-11-22T00:25:35.0259555Z .................................................................................................... 1700/9278
2019-11-22T00:25:48.1920285Z ...........................i........................................................................ 1800/9278
2019-11-22T00:25:54.9704915Z .................................................................................................... 1900/9278
2019-11-22T00:26:09.4140055Z ............iiiii................................................................................... 2000/9278
2019-11-22T00:26:19.0124743Z .................................................................................................... 2200/9278
2019-11-22T00:26:21.5187022Z .................................................................................................... 2300/9278
2019-11-22T00:26:26.6422144Z .................................................................................................... 2400/9278
2019-11-22T00:26:48.2036519Z .................................................................................................... 2500/9278
---
2019-11-22T00:29:28.8651495Z ............i...............i....................................................................... 4800/9278
2019-11-22T00:29:39.6432697Z .................................................................................................... 4900/9278
2019-11-22T00:29:45.1443444Z .................................................................................................... 5000/9278
2019-11-22T00:29:54.7092063Z .................................................................................................... 5100/9278
2019-11-22T00:30:00.8071050Z .................ii.ii...........i.................................................................. 5200/9278
2019-11-22T00:30:10.0652922Z .................................................................................................... 5400/9278
2019-11-22T00:30:20.9805431Z ...............F...................................................................................i 5500/9278
2019-11-22T00:30:28.9067102Z .................................................................................................... 5600/9278
2019-11-22T00:30:34.7351969Z .................................................................................................... 5700/9278
2019-11-22T00:30:34.7351969Z .................................................................................................... 5700/9278
2019-11-22T00:30:45.2815902Z .....................................................................................ii...i..ii..... 5800/9278
2019-11-22T00:31:08.1491749Z .................................................................................................... 6000/9278
2019-11-22T00:31:16.2418734Z .................................................................................................... 6100/9278
2019-11-22T00:31:24.7287135Z .................................................................................................... 6200/9278
2019-11-22T00:31:24.7287135Z .................................................................................................... 6200/9278
2019-11-22T00:31:40.4345569Z ........i..ii....................................................................................... 6300/9278
2019-11-22T00:31:59.7907216Z ............................................................................i....................... 6500/9278
2019-11-22T00:32:02.2219295Z .................................................................................................... 6600/9278
2019-11-22T00:32:04.5895657Z .................................................................i.................................. 6700/9278
2019-11-22T00:32:07.6714622Z .................................................................................................... 6800/9278
---
2019-11-22T00:36:56.7331437Z diff of stderr:
2019-11-22T00:36:56.7331474Z 
2019-11-22T00:36:56.7331522Z 29    |             ^^^^^
2019-11-22T00:36:56.7331567Z 30 
2019-11-22T00:36:56.7331671Z 31 error[E0571]: `break` with value from a `while` loop
2019-11-22T00:36:56.7332906Z +   --> $DIR/loop-break-value.rs:36:12
2019-11-22T00:36:56.7332988Z 33    |
2019-11-22T00:36:56.7333035Z 34 LL |         if break () {
2019-11-22T00:36:56.7333035Z 34 LL |         if break () {
2019-11-22T00:36:56.7333093Z 35    |            ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7333199Z 40    |            ^^^^^
2019-11-22T00:36:56.7333243Z 41 
2019-11-22T00:36:56.7333243Z 41 
2019-11-22T00:36:56.7333293Z 42 error[E0571]: `break` with value from a `while` loop
2019-11-22T00:36:56.7333825Z +   --> $DIR/loop-break-value.rs:41:9
2019-11-22T00:36:56.7333874Z 44    |
2019-11-22T00:36:56.7333936Z 45 LL |         break None;
2019-11-22T00:36:56.7333936Z 45 LL |         break None;
2019-11-22T00:36:56.7333993Z 46    |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7334073Z 51    |         ^^^^^
2019-11-22T00:36:56.7334134Z 52 
2019-11-22T00:36:56.7334134Z 52 
2019-11-22T00:36:56.7334182Z 53 error[E0571]: `break` with value from a `while` loop
2019-11-22T00:36:56.7334728Z +   --> $DIR/loop-break-value.rs:47:13
2019-11-22T00:36:56.7334777Z 55    |
2019-11-22T00:36:56.7334777Z 55    |
2019-11-22T00:36:56.7335034Z 56 LL |             break 'while_let_loop "nope";
2019-11-22T00:36:56.7335113Z 57    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7335182Z 
2019-11-22T00:36:56.7335231Z The actual stderr differed from the expected stderr.
2019-11-22T00:36:56.7335594Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/loop-break-value.stderr
2019-11-22T00:36:56.7335594Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/loop-break-value.stderr
2019-11-22T00:36:56.7335884Z To update references, rerun the tests and pass the `--bless` flag
2019-11-22T00:36:56.7336188Z To only update this specific test, also pass `--test-args loops/loop-break-value.rs`
2019-11-22T00:36:56.7336294Z error: 1 errors occurred comparing output.
2019-11-22T00:36:56.7336344Z status: exit code: 1
2019-11-22T00:36:56.7336344Z status: exit code: 1
2019-11-22T00:36:56.7337570Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/loops/loop-break-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/auxiliary" "-A" "unused"
2019-11-22T00:36:56.7337982Z ------------------------------------------
2019-11-22T00:36:56.7338023Z 
2019-11-22T00:36:56.7338275Z ------------------------------------------
2019-11-22T00:36:56.7338324Z stderr:
2019-11-22T00:36:56.7338324Z stderr:
2019-11-22T00:36:56.7338587Z ------------------------------------------
2019-11-22T00:36:56.7338640Z warning: denote infinite loops with `loop { ... }`
2019-11-22T00:36:56.7338998Z    |
2019-11-22T00:36:56.7338998Z    |
2019-11-22T00:36:56.7339283Z LL |     'while_loop: while true { //~ WARN denote infinite loops with
2019-11-22T00:36:56.7339340Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: use `loop`
2019-11-22T00:36:56.7339403Z    |
2019-11-22T00:36:56.7339452Z    = note: `#[warn(while_true)]` on by default
2019-11-22T00:36:56.7339485Z 
2019-11-22T00:36:56.7339534Z error[E0571]: `break` with value from a `while` loop
2019-11-22T00:36:56.7339875Z    |
2019-11-22T00:36:56.7339875Z    |
2019-11-22T00:36:56.7339927Z LL |         break (); //~ ERROR `break` with value from a `while` loop
2019-11-22T00:36:56.7340001Z    |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7340053Z    |
2019-11-22T00:36:56.7340109Z help: instead, use `break` on its own without a value inside this `while` loop
2019-11-22T00:36:56.7340175Z    |
2019-11-22T00:36:56.7340245Z LL |         break; //~ ERROR `break` with value from a `while` loop
2019-11-22T00:36:56.7340341Z 
2019-11-22T00:36:56.7340341Z 
2019-11-22T00:36:56.7340390Z error[E0571]: `break` with value from a `while` loop
2019-11-22T00:36:56.7340728Z    |
2019-11-22T00:36:56.7340989Z LL |             break 'while_loop 123;
2019-11-22T00:36:56.7340989Z LL |             break 'while_loop 123;
2019-11-22T00:36:56.7341048Z    |             ^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7341099Z    |
2019-11-22T00:36:56.7341167Z help: instead, use `break` on its own without a value inside this `while` loop
2019-11-22T00:36:56.7341262Z LL |             break;
2019-11-22T00:36:56.7341325Z    |             ^^^^^
2019-11-22T00:36:56.7341355Z 
2019-11-22T00:36:56.7341355Z 
2019-11-22T00:36:56.7341404Z error[E0571]: `break` with value from a `while` loop
2019-11-22T00:36:56.7341760Z    |
2019-11-22T00:36:56.7341760Z    |
2019-11-22T00:36:56.7341813Z LL |         if break () { //~ ERROR `break` with value from a `while` loop
2019-11-22T00:36:56.7342155Z    |            ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7342231Z    |
2019-11-22T00:36:56.7342284Z help: instead, use `break` on its own without a value inside this `while` loop
2019-11-22T00:36:56.7342332Z    |
2019-11-22T00:36:56.7342399Z LL |         if break { //~ ERROR `break` with value from a `while` loop
2019-11-22T00:36:56.7342479Z 
2019-11-22T00:36:56.7342479Z 
2019-11-22T00:36:56.7342528Z error[E0571]: `break` with value from a `while` loop
2019-11-22T00:36:56.7342929Z    |
2019-11-22T00:36:56.7342975Z LL |         break None;
2019-11-22T00:36:56.7342975Z LL |         break None;
2019-11-22T00:36:56.7343044Z    |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7343225Z    |
2019-11-22T00:36:56.7343336Z help: instead, use `break` on its own without a value inside this `while` loop
2019-11-22T00:36:56.7343452Z LL |         break;
2019-11-22T00:36:56.7343499Z    |         ^^^^^
2019-11-22T00:36:56.7343529Z 
2019-11-22T00:36:56.7343529Z 
2019-11-22T00:36:56.7343593Z error[E0571]: `break` with value from a `while` loop
2019-11-22T00:36:56.7343957Z    |
2019-11-22T00:36:56.7343957Z    |
2019-11-22T00:36:56.7344227Z LL |             break 'while_let_loop "nope";
2019-11-22T00:36:56.7344290Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7344344Z    |
2019-11-22T00:36:56.7344420Z help: instead, use `break` on its own without a value inside this `while` loop
2019-11-22T00:36:56.7344514Z LL |             break;
2019-11-22T00:36:56.7344559Z    |             ^^^^^
2019-11-22T00:36:56.7344615Z 
2019-11-22T00:36:56.7344615Z 
2019-11-22T00:36:56.7344671Z error[E0571]: `break` with value from a `for` loop
2019-11-22T00:36:56.7345019Z    |
2019-11-22T00:36:56.7345019Z    |
2019-11-22T00:36:56.7345071Z LL |         break (); //~ ERROR `break` with value from a `for` loop
2019-11-22T00:36:56.7345130Z    |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7345193Z    |
2019-11-22T00:36:56.7345246Z help: instead, use `break` on its own without a value inside this `for` loop
2019-11-22T00:36:56.7345295Z    |
2019-11-22T00:36:56.7345403Z LL |         break; //~ ERROR `break` with value from a `for` loop
2019-11-22T00:36:56.7345500Z 
2019-11-22T00:36:56.7345500Z 
2019-11-22T00:36:56.7345548Z error[E0571]: `break` with value from a `for` loop
2019-11-22T00:36:56.7345892Z    |
2019-11-22T00:36:56.7345892Z    |
2019-11-22T00:36:56.7345937Z LL |         break [()];
2019-11-22T00:36:56.7346008Z    |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7346072Z    |
2019-11-22T00:36:56.7346125Z help: instead, use `break` on its own without a value inside this `for` loop
2019-11-22T00:36:56.7346234Z LL |         break;
2019-11-22T00:36:56.7346280Z    |         ^^^^^
2019-11-22T00:36:56.7346309Z 
2019-11-22T00:36:56.7346309Z 
2019-11-22T00:36:56.7346357Z error[E0571]: `break` with value from a `for` loop
2019-11-22T00:36:56.7346704Z    |
2019-11-22T00:36:56.7346952Z LL |             break 'for_loop Some(17);
2019-11-22T00:36:56.7346952Z LL |             break 'for_loop Some(17);
2019-11-22T00:36:56.7347028Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-11-22T00:36:56.7347079Z    |
2019-11-22T00:36:56.7347130Z help: instead, use `break` on its own without a value inside this `for` loop
2019-11-22T00:36:56.7347241Z LL |             break;
2019-11-22T00:36:56.7347302Z    |             ^^^^^
2019-11-22T00:36:56.7347332Z 
2019-11-22T00:36:56.7347393Z error[E0308]: mismatched types
2019-11-22T00:36:56.7347393Z error[E0308]: mismatched types
2019-11-22T00:36:56.7347665Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:2:31
2019-11-22T00:36:56.7347715Z    |
2019-11-22T00:36:56.7347778Z LL |     let val: ! = loop { break break; };
2019-11-22T00:36:56.7347833Z    |                               ^^^^^ expected `!`, found `()`
2019-11-22T00:36:56.7347940Z    = note:   expected type `!`
2019-11-22T00:36:56.7347989Z            found unit type `()`
2019-11-22T00:36:56.7348020Z 
2019-11-22T00:36:56.7348064Z error[E0308]: mismatched types
---
2019-11-22T00:36:56.7348561Z 
2019-11-22T00:36:56.7348700Z error[E0308]: mismatched types
2019-11-22T00:36:56.7349079Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:14:15
2019-11-22T00:36:56.7349157Z    |
2019-11-22T00:36:56.7349206Z LL |         break "asdf"; //~ ERROR mismatched types
2019-11-22T00:36:56.7349259Z    |               ^^^^^^ expected `i32`, found `&str`
2019-11-22T00:36:56.7349355Z error[E0308]: mismatched types
2019-11-22T00:36:56.7349658Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:19:31
2019-11-22T00:36:56.7349709Z    |
2019-11-22T00:36:56.7349709Z    |
2019-11-22T00:36:56.7350003Z LL |             break 'outer_loop "nope"; //~ ERROR mismatched types
2019-11-22T00:36:56.7350062Z    |                               ^^^^^^ expected `i32`, found `&str`
2019-11-22T00:36:56.7350143Z error[E0308]: mismatched types
2019-11-22T00:36:56.7350431Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:71:26
2019-11-22T00:36:56.7350481Z    |
2019-11-22T00:36:56.7350481Z    |
2019-11-22T00:36:56.7350749Z LL |                 break 'c 123; //~ ERROR mismatched types
2019-11-22T00:36:56.7350841Z    |                          ^^^ expected `()`, found integer
2019-11-22T00:36:56.7350920Z error[E0308]: mismatched types
2019-11-22T00:36:56.7351206Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:78:15
2019-11-22T00:36:56.7351256Z    |
2019-11-22T00:36:56.7351256Z    |
2019-11-22T00:36:56.7351306Z LL |         break (break, break); //~ ERROR mismatched types
2019-11-22T00:36:56.7351360Z    |               ^^^^^^^^^^^^^^ expected `()`, found tuple
2019-11-22T00:36:56.7351468Z    = note: expected unit type `()`
2019-11-22T00:36:56.7351468Z    = note: expected unit type `()`
2019-11-22T00:36:56.7351518Z                   found tuple `(!, !)`
2019-11-22T00:36:56.7351613Z error[E0308]: mismatched types
2019-11-22T00:36:56.7352296Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:83:15
2019-11-22T00:36:56.7352359Z    |
2019-11-22T00:36:56.7352427Z LL |         break 2; //~ ERROR mismatched types
---
2019-11-22T00:36:56.7353297Z    |
2019-11-22T00:36:56.7353346Z LL |         break; //~ ERROR mismatched types
2019-11-22T00:36:56.7353412Z    |         ^^^^^
2019-11-22T00:36:56.7353457Z    |         |
2019-11-22T00:36:56.7353506Z    |         expected integer, found `()`
2019-11-22T00:36:56.7353575Z    |         help: give it a value of the expected type: `break value`
2019-11-22T00:36:56.7353658Z error: aborting due to 16 previous errors
2019-11-22T00:36:56.7353690Z 
2019-11-22T00:36:56.7353756Z Some errors have detailed explanations: E0308, E0571.
2019-11-22T00:36:56.7354288Z For more information about an error, try `rustc --explain E0308`.
---
2019-11-22T00:36:56.7361281Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-22T00:36:56.7361617Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-22T00:36:56.7376513Z 
2019-11-22T00:36:56.7376782Z 
2019-11-22T00:36:56.7379312Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-22T00:36:56.7380087Z 
2019-11-22T00:36:56.7380246Z 
2019-11-22T00:36:56.7385411Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-22T00:36:56.7385647Z Build completed unsuccessfully in 1:05:48
2019-11-22T00:36:56.7385647Z Build completed unsuccessfully in 1:05:48
2019-11-22T00:36:56.7436453Z == clock drift check ==
2019-11-22T00:36:56.7456543Z   local time: Fri Nov 22 00:36:56 UTC 2019
2019-11-22T00:36:57.0280288Z   network time: Fri, 22 Nov 2019 00:36:57 GMT
2019-11-22T00:36:57.0285926Z == end clock drift check ==
2019-11-22T00:36:57.8599199Z 
2019-11-22T00:36:57.8730299Z ##[error]Bash exited with code '1'.
2019-11-22T00:36:57.8767554Z ##[section]Starting: Checkout
2019-11-22T00:36:57.8769322Z ==============================================================================
2019-11-22T00:36:57.8769374Z Task         : Get sources
2019-11-22T00:36:57.8769434Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
