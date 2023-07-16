plain
2019-07-27T16:57:20.0698590Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-27T16:57:20.0875945Z ##[command]git config gc.auto 0
2019-07-27T16:57:20.0945215Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-27T16:57:20.1002612Z ##[command]git config --get-all http.proxy
2019-07-27T16:57:20.1130654Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62855/merge:refs/remotes/pull/62855/merge
---
2019-07-27T16:57:52.9930332Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-27T16:57:52.9930502Z 
2019-07-27T16:57:52.9930797Z   git checkout -b <new-branch-name>
2019-07-27T16:57:52.9931237Z 
2019-07-27T16:57:52.9931383Z HEAD is now at a3c144a3b Merge 892b81f2001194973e3c88e1959a45457dadb0ac into 0e9b465d729d07101b29b4d096d83edf9be82df0
2019-07-27T16:57:53.0081491Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-27T16:57:53.0084224Z ==============================================================================
2019-07-27T16:57:53.0084305Z Task         : Bash
2019-07-27T16:57:53.0084516Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-27T17:54:55.3329620Z .................................................................................................... 700/5870
2019-07-27T17:54:59.1258035Z .................................................................................................... 800/5870
2019-07-27T17:55:04.2545862Z .................................................................................................... 900/5870
2019-07-27T17:55:09.1135425Z .................................................................................................... 1000/5870
2019-07-27T17:55:14.2246693Z i...........i....................................................................................... 1100/5870
2019-07-27T17:55:17.9245900Z ..............................iiiii................................................................. 1200/5870
2019-07-27T17:55:23.4359525Z .................................................................................................... 1400/5870
2019-07-27T17:55:25.8775956Z .................................................................................................... 1500/5870
2019-07-27T17:55:29.3379459Z .................................................................................................... 1600/5870
2019-07-27T17:55:31.8200903Z .................................................................................................... 1700/5870
---
2019-07-27T17:56:41.0408046Z .................................................................................................... 3400/5870
2019-07-27T17:56:45.7685867Z .................................................................................................... 3500/5870
2019-07-27T17:56:49.4376053Z ..........................i.....................................................F................... 3600/5870
2019-07-27T17:56:53.5625671Z .................................................................................................... 3700/5870
2019-07-27T17:56:56.9085508Z ....ii...i..ii...................................................................................... 3800/5870
2019-07-27T17:57:05.2894751Z .................................................................................................... 4000/5870
2019-07-27T17:57:08.8912927Z .......................ii........................................................................... 4100/5870
2019-07-27T17:57:11.0243491Z ............................................i....................................................... 4200/5870
2019-07-27T17:57:12.9131784Z .................................................................................................... 4300/5870
---
2019-07-27T17:58:30.2050690Z diff of stderr:
2019-07-27T17:58:30.2051385Z 
2019-07-27T17:58:30.2052331Z 18   --> $DIR/same-sequence-span.rs:20:1
2019-07-27T17:58:30.2053098Z 19    |
2019-07-27T17:58:30.2053799Z 20 LL | proc_macro_sequence::make_foo!();
2019-07-27T17:58:30.2054755Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not allowed after `expr` fragments
2019-07-27T17:58:30.2056660Z +    | |
2019-07-27T17:58:30.2056660Z +    | |
2019-07-27T17:58:30.2057644Z +    | not allowed after `expr` fragments
2019-07-27T17:58:30.2058512Z +    | in this macro invocation
2019-07-27T17:58:30.2061281Z 22    |
2019-07-27T17:58:30.2061506Z 23    = note: allowed there are: `=>`, `,` or `;`
2019-07-27T17:58:30.2061722Z 
2019-07-27T17:58:30.2062387Z 26   --> $DIR/same-sequence-span.rs:20:1
2019-07-27T17:58:30.2062548Z 27    |
2019-07-27T17:58:30.2062548Z 27    |
2019-07-27T17:58:30.2062656Z 28 LL | proc_macro_sequence::make_foo!();
2019-07-27T17:58:30.2063018Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not allowed after `expr` fragments
2019-07-27T17:58:30.2063268Z +    | |
2019-07-27T17:58:30.2063268Z +    | |
2019-07-27T17:58:30.2063389Z +    | not allowed after `expr` fragments
2019-07-27T17:58:30.2063492Z +    | in this macro invocation
2019-07-27T17:58:30.2063594Z 30    |
2019-07-27T17:58:30.2063717Z 31    = note: allowed there are: `=>`, `,` or `;`
2019-07-27T17:58:30.2063904Z 
2019-07-27T17:58:30.2064227Z 
2019-07-27T17:58:30.2064338Z The actual stderr differed from the expected stderr.
2019-07-27T17:58:30.2065017Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/same-sequence-span/same-sequence-span.stderr
2019-07-27T17:58:30.2065017Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/same-sequence-span/same-sequence-span.stderr
2019-07-27T17:58:30.2065433Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T17:58:30.2065803Z To only update this specific test, also pass `--test-args macros/same-sequence-span.rs`
2019-07-27T17:58:30.2066066Z error: 1 errors occurred comparing output.
2019-07-27T17:58:30.2066879Z status: exit code: 1
2019-07-27T17:58:30.2066879Z status: exit code: 1
2019-07-27T17:58:30.2067887Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/same-sequence-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/same-sequence-span" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/same-sequence-span/auxiliary" "-A" "unused"
2019-07-27T17:58:30.2068517Z ------------------------------------------
2019-07-27T17:58:30.2068673Z 
2019-07-27T17:58:30.2069175Z ------------------------------------------
2019-07-27T17:58:30.2069412Z stderr:
2019-07-27T17:58:30.2069412Z stderr:
2019-07-27T17:58:30.2069802Z ------------------------------------------
2019-07-27T17:58:30.2070141Z error: `$x:expr` may be followed by `$y:tt`, which is not allowed for `expr` fragments
2019-07-27T17:58:30.2070630Z    |
2019-07-27T17:58:30.2070630Z    |
2019-07-27T17:58:30.2070761Z LL |     (1 $x:expr $($y:tt,)*   //~ERROR `$x:expr` may be followed by `$y:tt`
2019-07-27T17:58:30.2070872Z    |                  ^^^^^ not allowed after `expr` fragments
2019-07-27T17:58:30.2070975Z    |
2019-07-27T17:58:30.2071097Z    = note: allowed there are: `=>`, `,` or `;`
2019-07-27T17:58:30.2071414Z 
2019-07-27T17:58:30.2071565Z error: `$x:expr` may be followed by `=`, which is not allowed for `expr` fragments
2019-07-27T17:58:30.2072683Z    |
2019-07-27T17:58:30.2072683Z    |
2019-07-27T17:58:30.2072828Z LL |                $(= $z:tt)*  //~ERROR `$x:expr` may be followed by `=`
2019-07-27T17:58:30.2072967Z    |                  ^ not allowed after `expr` fragments
2019-07-27T17:58:30.2073067Z    |
2019-07-27T17:58:30.2073374Z    = note: allowed there are: `=>`, `,` or `;`
2019-07-27T17:58:30.2073516Z 
2019-07-27T17:58:30.2073948Z error: `$x:expr` may be followed by `$y:tt`, which is not allowed for `expr` fragments
2019-07-27T17:58:30.2074913Z    |
2019-07-27T17:58:30.2074913Z    |
2019-07-27T17:58:30.2075033Z LL | proc_macro_sequence::make_foo!(); //~ERROR `$x:expr` may be followed by `$y:tt`
2019-07-27T17:58:30.2076031Z    | |
2019-07-27T17:58:30.2076031Z    | |
2019-07-27T17:58:30.2076972Z    | not allowed after `expr` fragments
2019-07-27T17:58:30.2077150Z    | in this macro invocation
2019-07-27T17:58:30.2077280Z    |
2019-07-27T17:58:30.2077410Z    = note: allowed there are: `=>`, `,` or `;`
2019-07-27T17:58:30.2077522Z 
2019-07-27T17:58:30.2077692Z error: `$x:expr` may be followed by `=`, which is not allowed for `expr` fragments
2019-07-27T17:58:30.2078388Z    |
2019-07-27T17:58:30.2078388Z    |
2019-07-27T17:58:30.2078528Z LL | proc_macro_sequence::make_foo!(); //~ERROR `$x:expr` may be followed by `$y:tt`
2019-07-27T17:58:30.2078806Z    | |
2019-07-27T17:58:30.2078806Z    | |
2019-07-27T17:58:30.2078954Z    | not allowed after `expr` fragments
2019-07-27T17:58:30.2079087Z    | in this macro invocation
2019-07-27T17:58:30.2079239Z    |
2019-07-27T17:58:30.2079368Z    = note: allowed there are: `=>`, `,` or `;`
2019-07-27T17:58:30.2079951Z error: aborting due to 4 previous errors
2019-07-27T17:58:30.2080042Z 
2019-07-27T17:58:30.2080127Z 
2019-07-27T17:58:30.2080452Z ------------------------------------------
---
2019-07-27T17:58:30.2082208Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-27T17:58:30.2082363Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-27T17:58:30.2082481Z 
2019-07-27T17:58:30.2082569Z 
2019-07-27T17:58:30.2084082Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-27T17:58:30.2084548Z 
2019-07-27T17:58:30.2084639Z 
2019-07-27T17:58:30.2086467Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-27T17:58:30.2086701Z Build completed unsuccessfully in 0:53:18
2019-07-27T17:58:30.2086701Z Build completed unsuccessfully in 0:53:18
2019-07-27T17:58:31.3911823Z ##[error]Bash exited with code '1'.
2019-07-27T17:58:31.3946637Z ##[section]Starting: Checkout
2019-07-27T17:58:31.3948279Z ==============================================================================
2019-07-27T17:58:31.3948345Z Task         : Get sources
2019-07-27T17:58:31.3948383Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
