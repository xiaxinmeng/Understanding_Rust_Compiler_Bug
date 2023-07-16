plain
2019-09-14T11:19:28.8555855Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-14T11:19:28.8772323Z ##[command]git config gc.auto 0
2019-09-14T11:19:28.8882210Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-14T11:19:28.8928976Z ##[command]git config --get-all http.proxy
2019-09-14T11:19:28.9119595Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64419/merge:refs/remotes/pull/64419/merge
---
2019-09-14T12:25:13.7053610Z .................................................................................................... 1500/9013
2019-09-14T12:25:20.0072112Z .................................................................................................... 1600/9013
2019-09-14T12:25:33.2927931Z ...........................................................i...............i........................ 1700/9013
2019-09-14T12:25:41.2976475Z .................................................................................................... 1800/9013
2019-09-14T12:25:57.5269174Z ..................................................iiiii............................................. 1900/9013
2019-09-14T12:26:09.2769545Z .................................................................................................... 2100/9013
2019-09-14T12:26:11.9859892Z .................................................................................................... 2200/9013
2019-09-14T12:26:15.6850814Z .................................................................................................... 2300/9013
2019-09-14T12:26:24.3280818Z .................................................................................................... 2400/9013
---
2019-09-14T12:29:33.8842429Z .....................................i...............i.............................................. 4700/9013
2019-09-14T12:29:45.8604211Z .................................................................................................... 4800/9013
2019-09-14T12:29:53.6457690Z .................................................................................................... 4900/9013
2019-09-14T12:30:05.1025403Z .................................................................................................... 5000/9013
2019-09-14T12:30:11.7149659Z ....................ii.ii........................................................................... 5100/9013
2019-09-14T12:30:22.9964732Z .................................................................................................... 5300/9013
2019-09-14T12:30:33.7957239Z ....................................................................................i............... 5400/9013
2019-09-14T12:30:42.4265952Z .................................................................................................... 5500/9013
2019-09-14T12:30:48.3070698Z .................................................................................................... 5600/9013
2019-09-14T12:30:48.3070698Z .................................................................................................... 5600/9013
2019-09-14T12:30:59.2247628Z ...............................................................................ii...i..ii........... 5700/9013
2019-09-14T12:31:15.7393259Z i................................................................................................... 5800/9013
2019-09-14T12:31:37.2414329Z .................................................................................................... 6000/9013
2019-09-14T12:31:37.2414329Z .................................................................................................... 6000/9013
2019-09-14T12:31:42.5287798Z .................................................................................i..ii.............. 6100/9013
2019-09-14T12:32:14.7185500Z .................................................................................................... 6300/9013
2019-09-14T12:32:17.2326714Z ........................................i........................................................... 6400/9013
2019-09-14T12:32:19.6735410Z .................................................................................................... 6500/9013
2019-09-14T12:32:22.5315663Z ............i....................................................................................... 6600/9013
---
2019-09-14T12:36:44.7282218Z  finished in 1.536
2019-09-14T12:36:44.7497262Z Check compiletest suite=run-fail mode=run-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-14T12:36:44.9204743Z 
2019-09-14T12:36:44.9205062Z running 144 tests
2019-09-14T12:36:55.8943860Z ...................................................................................FFFF...F.FFF..... 100/144
2019-09-14T12:37:01.9636464Z failures:
2019-09-14T12:37:01.9636668Z 
2019-09-14T12:37:01.9637216Z ---- [run-fail] run-fail/overflowing-lsh-1.rs stdout ----
2019-09-14T12:37:01.9638420Z 
2019-09-14T12:37:01.9638420Z 
2019-09-14T12:37:01.9638690Z error: compilation failed!
2019-09-14T12:37:01.9638852Z status: exit code: 1
2019-09-14T12:37:01.9639934Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-lsh-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-1/auxiliary"
2019-09-14T12:37:01.9640659Z ------------------------------------------
2019-09-14T12:37:01.9641140Z 
2019-09-14T12:37:01.9641634Z ------------------------------------------
2019-09-14T12:37:01.9641867Z stderr:
2019-09-14T12:37:01.9641867Z stderr:
2019-09-14T12:37:01.9642308Z ------------------------------------------
2019-09-14T12:37:01.9642515Z error: attempt to shift left with overflow
2019-09-14T12:37:01.9642980Z  --> /checkout/src/test/run-fail/overflowing-lsh-1.rs:7:14
2019-09-14T12:37:01.9643188Z   |
2019-09-14T12:37:01.9643353Z 7 |     let _x = 1_i32 << 32;
2019-09-14T12:37:01.9643694Z   |
2019-09-14T12:37:01.9643852Z   = note: `#[deny(const_err)]` on by default
2019-09-14T12:37:01.9644012Z 
2019-09-14T12:37:01.9644163Z error: aborting due to previous error
---
2019-09-14T12:37:01.9647137Z ---- [run-fail] run-fail/overflowing-lsh-3.rs stdout ----
2019-09-14T12:37:01.9647368Z 
2019-09-14T12:37:01.9647526Z error: compilation failed!
2019-09-14T12:37:01.9647697Z status: exit code: 1
2019-09-14T12:37:01.9649040Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-lsh-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-3/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-3/auxiliary"
2019-09-14T12:37:01.9649801Z ------------------------------------------
2019-09-14T12:37:01.9649986Z 
2019-09-14T12:37:01.9650417Z ------------------------------------------
2019-09-14T12:37:01.9650614Z stderr:
2019-09-14T12:37:01.9650614Z stderr:
2019-09-14T12:37:01.9651007Z ------------------------------------------
2019-09-14T12:37:01.9651245Z error: attempt to shift left with overflow
2019-09-14T12:37:01.9651670Z  --> /checkout/src/test/run-fail/overflowing-lsh-3.rs:7:14
2019-09-14T12:37:01.9651893Z   |
2019-09-14T12:37:01.9652057Z 7 |     let _x = 1_u64 << 64;
2019-09-14T12:37:01.9652384Z   |
2019-09-14T12:37:01.9652539Z   = note: `#[deny(const_err)]` on by default
2019-09-14T12:37:01.9652673Z 
2019-09-14T12:37:01.9652846Z error: aborting due to previous error
---
2019-09-14T12:37:01.9654559Z ---- [run-fail] run-fail/overflowing-lsh-2.rs stdout ----
2019-09-14T12:37:01.9654747Z 
2019-09-14T12:37:01.9654924Z error: compilation failed!
2019-09-14T12:37:01.9655080Z status: exit code: 1
2019-09-14T12:37:01.9656032Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-lsh-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-2/auxiliary"
2019-09-14T12:37:01.9656749Z ------------------------------------------
2019-09-14T12:37:01.9656942Z 
2019-09-14T12:37:01.9657352Z ------------------------------------------
2019-09-14T12:37:01.9657570Z stderr:
2019-09-14T12:37:01.9657570Z stderr:
2019-09-14T12:37:01.9657967Z ------------------------------------------
2019-09-14T12:37:01.9658166Z error: attempt to shift left with overflow
2019-09-14T12:37:01.9662346Z  --> /checkout/src/test/run-fail/overflowing-lsh-2.rs:7:14
2019-09-14T12:37:01.9664435Z 7 |     let _x = 1 << -1;
2019-09-14T12:37:01.9664682Z   |              ^^^^^^^
2019-09-14T12:37:01.9664727Z   |
2019-09-14T12:37:01.9664799Z   = note: `#[deny(const_err)]` on by default
---
2019-09-14T12:37:01.9665601Z ---- [run-fail] run-fail/overflowing-lsh-4.rs stdout ----
2019-09-14T12:37:01.9665650Z 
2019-09-14T12:37:01.9665697Z error: compilation failed!
2019-09-14T12:37:01.9665745Z status: exit code: 1
2019-09-14T12:37:01.9666535Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-lsh-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-4/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-4/auxiliary"
2019-09-14T12:37:01.9666911Z ------------------------------------------
2019-09-14T12:37:01.9666947Z 
2019-09-14T12:37:01.9667189Z ------------------------------------------
2019-09-14T12:37:01.9667256Z stderr:
2019-09-14T12:37:01.9667256Z stderr:
2019-09-14T12:37:01.9667496Z ------------------------------------------
2019-09-14T12:37:01.9667557Z error: attempt to shift left with overflow
2019-09-14T12:37:01.9667822Z   --> /checkout/src/test/run-fail/overflowing-lsh-4.rs:11:13
2019-09-14T12:37:01.9667895Z    |
2019-09-14T12:37:01.9667944Z 11 |     let x = 1_i8 << 17;
2019-09-14T12:37:01.9668058Z    |
2019-09-14T12:37:01.9668108Z    = note: `#[deny(const_err)]` on by default
2019-09-14T12:37:01.9668142Z 
2019-09-14T12:37:01.9668204Z error: aborting due to previous error
---
2019-09-14T12:37:01.9669860Z ---- [run-fail] run-fail/overflowing-rsh-1.rs stdout ----
2019-09-14T12:37:01.9669899Z 
2019-09-14T12:37:01.9669947Z error: compilation failed!
2019-09-14T12:37:01.9670018Z status: exit code: 1
2019-09-14T12:37:01.9670973Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-rsh-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-1/auxiliary"
2019-09-14T12:37:01.9671418Z ------------------------------------------
2019-09-14T12:37:01.9671455Z 
2019-09-14T12:37:01.9671714Z ------------------------------------------
2019-09-14T12:37:01.9671763Z stderr:
2019-09-14T12:37:01.9671763Z stderr:
2019-09-14T12:37:01.9672002Z ------------------------------------------
2019-09-14T12:37:01.9672056Z error: attempt to shift right with overflow
2019-09-14T12:37:01.9672338Z  --> /checkout/src/test/run-fail/overflowing-rsh-1.rs:7:14
2019-09-14T12:37:01.9672392Z   |
2019-09-14T12:37:01.9672631Z 7 |     let _x = -1_i32 >> 32;
2019-09-14T12:37:01.9672756Z   |
2019-09-14T12:37:01.9672806Z   = note: `#[deny(const_err)]` on by default
2019-09-14T12:37:01.9672839Z 
2019-09-14T12:37:01.9672903Z error: aborting due to previous error
---
2019-09-14T12:37:01.9673541Z ---- [run-fail] run-fail/overflowing-rsh-2.rs stdout ----
2019-09-14T12:37:01.9673694Z 
2019-09-14T12:37:01.9673763Z error: compilation failed!
2019-09-14T12:37:01.9673810Z status: exit code: 1
2019-09-14T12:37:01.9674618Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-rsh-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-2/auxiliary"
2019-09-14T12:37:01.9674988Z ------------------------------------------
2019-09-14T12:37:01.9675023Z 
2019-09-14T12:37:01.9675352Z ------------------------------------------
2019-09-14T12:37:01.9675401Z stderr:
2019-09-14T12:37:01.9675401Z stderr:
2019-09-14T12:37:01.9675648Z ------------------------------------------
2019-09-14T12:37:01.9675718Z error: attempt to shift right with overflow
2019-09-14T12:37:01.9675978Z  --> /checkout/src/test/run-fail/overflowing-rsh-2.rs:7:14
2019-09-14T12:37:01.9676031Z   |
2019-09-14T12:37:01.9676277Z 7 |     let _x = -1_i32 >> -1;
2019-09-14T12:37:01.9676373Z   |
2019-09-14T12:37:01.9676437Z   = note: `#[deny(const_err)]` on by default
2019-09-14T12:37:01.9676469Z 
2019-09-14T12:37:01.9676526Z error: aborting due to previous error
---
2019-09-14T12:37:01.9677154Z ---- [run-fail] run-fail/overflowing-rsh-3.rs stdout ----
2019-09-14T12:37:01.9677206Z 
2019-09-14T12:37:01.9677252Z error: compilation failed!
2019-09-14T12:37:01.9677300Z status: exit code: 1
2019-09-14T12:37:01.9678073Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-rsh-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-3/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-3/auxiliary"
2019-09-14T12:37:01.9679279Z ------------------------------------------
2019-09-14T12:37:01.9679319Z 
2019-09-14T12:37:01.9679564Z ------------------------------------------
2019-09-14T12:37:01.9679614Z stderr:
2019-09-14T12:37:01.9679614Z stderr:
2019-09-14T12:37:01.9679867Z ------------------------------------------
2019-09-14T12:37:01.9679921Z error: attempt to shift right with overflow
2019-09-14T12:37:01.9680181Z  --> /checkout/src/test/run-fail/overflowing-rsh-3.rs:7:14
2019-09-14T12:37:01.9680265Z   |
2019-09-14T12:37:01.9680498Z 7 |     let _x = -1_i64 >> 64;
2019-09-14T12:37:01.9680609Z   |
2019-09-14T12:37:01.9680659Z   = note: `#[deny(const_err)]` on by default
2019-09-14T12:37:01.9680691Z 
2019-09-14T12:37:01.9680738Z error: aborting due to previous error
---
2019-09-14T12:37:01.9681395Z ---- [run-fail] run-fail/overflowing-rsh-4.rs stdout ----
2019-09-14T12:37:01.9681431Z 
2019-09-14T12:37:01.9681478Z error: compilation failed!
2019-09-14T12:37:01.9681525Z status: exit code: 1
2019-09-14T12:37:01.9682303Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-rsh-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-4/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-4/auxiliary"
2019-09-14T12:37:01.9682824Z ------------------------------------------
2019-09-14T12:37:01.9682861Z 
2019-09-14T12:37:01.9683101Z ------------------------------------------
2019-09-14T12:37:01.9683169Z stderr:
2019-09-14T12:37:01.9683169Z stderr:
2019-09-14T12:37:01.9683420Z ------------------------------------------
2019-09-14T12:37:01.9683474Z error: attempt to shift right with overflow
2019-09-14T12:37:01.9683755Z   --> /checkout/src/test/run-fail/overflowing-rsh-4.rs:11:13
2019-09-14T12:37:01.9683810Z    |
2019-09-14T12:37:01.9683859Z 11 |     let x = 2_i8 >> 17;
2019-09-14T12:37:01.9683971Z    |
2019-09-14T12:37:01.9684020Z    = note: `#[deny(const_err)]` on by default
2019-09-14T12:37:01.9684064Z 
2019-09-14T12:37:01.9684127Z error: aborting due to previous error
---
2019-09-14T12:37:01.9687268Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-14T12:37:01.9687332Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-14T12:37:01.9687369Z 
2019-09-14T12:37:01.9687398Z 
2019-09-14T12:37:01.9689457Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-14T12:37:01.9689764Z 
2019-09-14T12:37:01.9689815Z 
2019-09-14T12:37:01.9689866Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-14T12:37:01.9689921Z Build completed unsuccessfully in 1:10:03
2019-09-14T12:37:01.9689921Z Build completed unsuccessfully in 1:10:03
2019-09-14T12:37:01.9706260Z == clock drift check ==
2019-09-14T12:37:01.9722373Z   local time: Sat Sep 14 12:37:01 UTC 2019
2019-09-14T12:37:02.0731001Z   network time: Sat, 14 Sep 2019 12:37:02 GMT
2019-09-14T12:37:02.0733791Z == end clock drift check ==
2019-09-14T12:37:02.8492648Z ##[error]Bash exited with code '1'.
2019-09-14T12:37:02.8532974Z ##[section]Starting: Checkout
2019-09-14T12:37:02.8535105Z ==============================================================================
2019-09-14T12:37:02.8535166Z Task         : Get sources
2019-09-14T12:37:02.8535220Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
