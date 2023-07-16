plain
2019-10-04T19:50:10.4158558Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-04T19:50:10.4424274Z ##[command]git config gc.auto 0
2019-10-04T19:50:10.4513400Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-04T19:50:10.4573286Z ##[command]git config --get-all http.proxy
2019-10-04T19:50:10.4712472Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64623/merge:refs/remotes/pull/64623/merge
---
2019-10-04T20:58:23.9953030Z .................................................................................................... 1500/9107
2019-10-04T20:58:31.6863574Z .................................................................................................... 1600/9107
2019-10-04T20:58:41.8406804Z .................................................................................................... 1700/9107
2019-10-04T20:58:52.2243378Z .......i...............i............................................................................ 1800/9107
2019-10-04T20:59:00.2286763Z ..................................................................................................ii 1900/9107
2019-10-04T20:59:18.6646335Z iii................................................................................................. 2000/9107
2019-10-04T20:59:28.4400643Z .................................................................................................... 2200/9107
2019-10-04T20:59:31.3826357Z .................................................................................................... 2300/9107
2019-10-04T20:59:38.3619987Z .................................................................................................... 2400/9107
2019-10-04T20:59:44.6144517Z .................................................................................................... 2500/9107
---
2019-10-04T21:03:03.9335971Z .......................................................................................i............ 4700/9107
2019-10-04T21:03:12.8986169Z ...i................................................................................................ 4800/9107
2019-10-04T21:03:25.0106536Z .................................................................................................... 4900/9107
2019-10-04T21:03:31.4869218Z .................................................................................................... 5000/9107
2019-10-04T21:03:45.7463071Z ................................................................................ii.ii............... 5100/9107
2019-10-04T21:03:56.9256357Z .................................................................................................... 5300/9107
2019-10-04T21:04:08.5555366Z .................................................................................................... 5400/9107
2019-10-04T21:04:16.7202513Z ..............................................i..................................................... 5500/9107
2019-10-04T21:04:24.8833702Z .................................................................................................... 5600/9107
2019-10-04T21:04:24.8833702Z .................................................................................................... 5600/9107
2019-10-04T21:04:37.3028798Z .................................................................................................... 5700/9107
2019-10-04T21:04:46.3077043Z ...........................................ii...i..ii...........i................................... 5800/9107
2019-10-04T21:05:15.9407191Z .................................................................................................... 6000/9107
2019-10-04T21:05:26.5517748Z .................................................................................................... 6100/9107
2019-10-04T21:05:26.5517748Z .................................................................................................... 6100/9107
2019-10-04T21:05:42.3666292Z .................................................i..ii.............................................. 6200/9107
2019-10-04T21:06:10.3318355Z .................................................................................................... 6400/9107
2019-10-04T21:06:12.8623231Z .........i.......................................................................................... 6500/9107
2019-10-04T21:06:15.4617625Z .................................................................................i.................. 6600/9107
2019-10-04T21:06:18.5499138Z .................................................................................................... 6700/9107
---
2019-10-04T21:11:04.4438757Z diff of stderr:
2019-10-04T21:11:04.4438805Z 
2019-10-04T21:11:04.4447065Z 5    |           ^^^^^ method not found in `&()`
2019-10-04T21:11:04.4447228Z 6    |
2019-10-04T21:11:04.4450100Z 7    = help: items from traits can only be used if the trait is in scope
2019-10-04T21:11:04.4450939Z - help: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-10-04T21:11:04.4451428Z + help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-10-04T21:11:04.4452437Z 10 LL | use std::ops::Deref;
2019-10-04T21:11:04.4452483Z 11    |
2019-10-04T21:11:04.4454241Z 
2019-10-04T21:11:04.4454307Z 17    |               ^^^^^^^^^ method not found in `&mut ()`
2019-10-04T21:11:04.4454307Z 17    |               ^^^^^^^^^ method not found in `&mut ()`
2019-10-04T21:11:04.4454352Z 18    |
2019-10-04T21:11:04.4454417Z 19    = help: items from traits can only be used if the trait is in scope
2019-10-04T21:11:04.4454892Z - help: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-10-04T21:11:04.4454953Z + help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-10-04T21:11:04.4455060Z 22 LL | use std::ops::DerefMut;
2019-10-04T21:11:04.4455100Z 23    |
2019-10-04T21:11:04.4455145Z 
2019-10-04T21:11:04.4455169Z 
2019-10-04T21:11:04.4455169Z 
2019-10-04T21:11:04.4455211Z The actual stderr differed from the expected stderr.
2019-10-04T21:11:04.4455512Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene/hygiene.stderr
2019-10-04T21:11:04.4455924Z To update references, rerun the tests and pass the `--bless` flag
2019-10-04T21:11:04.4456366Z To only update this specific test, also pass `--test-args underscore-imports/hygiene.rs`
2019-10-04T21:11:04.4457151Z error: 1 errors occurred comparing output.
2019-10-04T21:11:04.4457197Z status: exit code: 1
2019-10-04T21:11:04.4457197Z status: exit code: 1
2019-10-04T21:11:04.4458253Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-imports/hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene/auxiliary" "-A" "unused"
2019-10-04T21:11:04.4458617Z ------------------------------------------
2019-10-04T21:11:04.4458669Z 
2019-10-04T21:11:04.4458896Z ------------------------------------------
2019-10-04T21:11:04.4458941Z stderr:
2019-10-04T21:11:04.4458941Z stderr:
2019-10-04T21:11:04.4459171Z ------------------------------------------
2019-10-04T21:11:04.4465029Z error[E0599]: no method named `deref` found for type `&()` in the current scope
2019-10-04T21:11:04.4465452Z   --> /checkout/src/test/ui/underscore-imports/hygiene.rs:38:11
2019-10-04T21:11:04.4465534Z    |
2019-10-04T21:11:04.4465584Z LL |     (&()).deref();              //~ ERROR no method named `deref`
2019-10-04T21:11:04.4465635Z    |           ^^^^^ method not found in `&()`
2019-10-04T21:11:04.4465745Z    = help: items from traits can only be used if the trait is in scope
2019-10-04T21:11:04.4465745Z    = help: items from traits can only be used if the trait is in scope
2019-10-04T21:11:04.4465799Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-10-04T21:11:04.4466259Z LL | use std::ops::Deref;
2019-10-04T21:11:04.4466306Z    |
2019-10-04T21:11:04.4467211Z 
2019-10-04T21:11:04.4467296Z error[E0599]: no method named `deref_mut` found for type `&mut ()` in the current scope
2019-10-04T21:11:04.4467296Z error[E0599]: no method named `deref_mut` found for type `&mut ()` in the current scope
2019-10-04T21:11:04.4467685Z   --> /checkout/src/test/ui/underscore-imports/hygiene.rs:39:15
2019-10-04T21:11:04.4467736Z    |
2019-10-04T21:11:04.4467786Z LL |     (&mut ()).deref_mut();      //~ ERROR no method named `deref_mut`
2019-10-04T21:11:04.4467855Z    |               ^^^^^^^^^ method not found in `&mut ()`
2019-10-04T21:11:04.4467944Z    = help: items from traits can only be used if the trait is in scope
2019-10-04T21:11:04.4467944Z    = help: items from traits can only be used if the trait is in scope
2019-10-04T21:11:04.4468019Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-10-04T21:11:04.4468108Z LL | use std::ops::DerefMut;
2019-10-04T21:11:04.4468166Z    |
2019-10-04T21:11:04.4468193Z 
2019-10-04T21:11:04.4468237Z error: aborting due to 2 previous errors
---
2019-10-04T21:11:04.4502434Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-04T21:11:04.4502557Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-04T21:11:04.4526484Z 
2019-10-04T21:11:04.4526584Z 
2019-10-04T21:11:04.4529489Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-04T21:11:04.4529788Z 
2019-10-04T21:11:04.4529822Z 
2019-10-04T21:11:04.4535192Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-04T21:11:04.4535267Z Build completed unsuccessfully in 1:13:07
2019-10-04T21:11:04.4535267Z Build completed unsuccessfully in 1:13:07
2019-10-04T21:11:04.4603407Z == clock drift check ==
2019-10-04T21:11:04.4615695Z   local time: Fri Oct  4 21:11:04 UTC 2019
2019-10-04T21:11:04.5608058Z   network time: Fri, 04 Oct 2019 21:11:04 GMT
2019-10-04T21:11:04.5612707Z == end clock drift check ==
2019-10-04T21:11:05.9729302Z ##[error]Bash exited with code '1'.
2019-10-04T21:11:05.9795738Z ##[section]Starting: Checkout
2019-10-04T21:11:05.9797981Z ==============================================================================
2019-10-04T21:11:05.9798072Z Task         : Get sources
2019-10-04T21:11:05.9798119Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
