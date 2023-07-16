plain
2019-08-27T06:03:48.7344486Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-27T06:03:48.7555127Z ##[command]git config gc.auto 0
2019-08-27T06:03:48.7629593Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-27T06:03:48.7687025Z ##[command]git config --get-all http.proxy
2019-08-27T06:03:48.7830464Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63938/merge:refs/remotes/pull/63938/merge
---
2019-08-27T06:04:12.7341361Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-27T06:04:12.7341432Z 
2019-08-27T06:04:12.7341944Z   git checkout -b <new-branch-name>
2019-08-27T06:04:12.7341981Z 
2019-08-27T06:04:12.7342285Z HEAD is now at be4b8e9fa Merge f99ff5a4c1600f9d09b0fc04467beb2df5d720e3 into 0444b9f66acb5da23dc816e0d8eb59623ba9ea50
2019-08-27T06:04:12.7499398Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-27T06:04:12.7502618Z ==============================================================================
2019-08-27T06:04:12.7502679Z Task         : Bash
2019-08-27T06:04:12.7502728Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-27T07:06:25.3050821Z .................................................................................................... 1500/8967
2019-08-27T07:06:31.1174658Z .................................................................................................... 1600/8967
2019-08-27T07:06:43.8184584Z .............................................i...............i...................................... 1700/8967
2019-08-27T07:06:51.9660279Z .................................................................................................... 1800/8967
2019-08-27T07:07:06.0944969Z ....................................iiiii........................................................... 1900/8967
2019-08-27T07:07:16.6303837Z .................................................................................................... 2100/8967
2019-08-27T07:07:19.1446024Z .................................................................................................... 2200/8967
2019-08-27T07:07:23.3853340Z .................................................................................................... 2300/8967
2019-08-27T07:07:30.4684902Z .................................................................................................... 2400/8967
---
2019-08-27T07:10:29.4296111Z .......................i...............i............................................................ 4700/8967
2019-08-27T07:10:40.8215261Z .................................................................................................... 4800/8967
2019-08-27T07:10:46.7869891Z .................................................................................................... 4900/8967
2019-08-27T07:10:57.4373145Z .................................................................................................... 5000/8967
2019-08-27T07:11:02.5194378Z ....ii.ii........................................................................................... 5100/8967
2019-08-27T07:11:16.1096612Z .................................................................................................... 5300/8967
2019-08-27T07:11:23.2177184Z ..................................................................i................................. 5400/8967
2019-08-27T07:11:30.2405831Z .................................................................................................... 5500/8967
2019-08-27T07:11:37.0888161Z .................................................................................................... 5600/8967
2019-08-27T07:11:37.0888161Z .................................................................................................... 5600/8967
2019-08-27T07:11:46.9948860Z ............................................................ii...i..i.i..........i.................. 5700/8967
2019-08-27T07:12:10.1510285Z .................................................................................................... 5900/8967
2019-08-27T07:12:14.9922101Z .................................................................................................... 6000/8967
2019-08-27T07:12:14.9922101Z .................................................................................................... 6000/8967
2019-08-27T07:12:21.6531963Z .............................................................i..ii.................................. 6100/8967
2019-08-27T07:12:49.0892217Z .................................................................................................... 6300/8967
2019-08-27T07:12:51.1314039Z ................i................................................................................... 6400/8967
2019-08-27T07:12:53.2195587Z ........................................................................................i........... 6500/8967
2019-08-27T07:12:55.7999282Z .................................................................................................... 6600/8967
---
2019-08-27T07:16:51.3284790Z 10 
2019-08-27T07:16:51.3284886Z 
2019-08-27T07:16:51.3285003Z 
2019-08-27T07:16:51.3285120Z The actual stderr differed from the expected stderr.
2019-08-27T07:16:51.3285567Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/while-parsing-this-or-pattern/while-parsing-this-or-pattern.stderr
2019-08-27T07:16:51.3285967Z To update references, rerun the tests and pass the `--bless` flag
2019-08-27T07:16:51.3286372Z To only update this specific test, also pass `--test-args or-patterns/while-parsing-this-or-pattern.rs`
2019-08-27T07:16:51.3286663Z error: 1 errors occurred comparing output.
2019-08-27T07:16:51.3286777Z status: exit code: 1
2019-08-27T07:16:51.3286777Z status: exit code: 1
2019-08-27T07:16:51.3287768Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/while-parsing-this-or-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/while-parsing-this-or-pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/while-parsing-this-or-pattern/auxiliary" "-A" "unused"
2019-08-27T07:16:51.3288623Z ------------------------------------------
2019-08-27T07:16:51.3288760Z 
2019-08-27T07:16:51.3289053Z ------------------------------------------
2019-08-27T07:16:51.3289643Z stderr:
2019-08-27T07:16:51.3289643Z stderr:
2019-08-27T07:16:51.3290053Z ------------------------------------------
2019-08-27T07:16:51.3290240Z error: expected pattern, found `.`
2019-08-27T07:16:51.3290859Z    |
2019-08-27T07:16:51.3290859Z    |
2019-08-27T07:16:51.3291027Z LL |         Some(42) | .=. => {} //~ ERROR expected pattern, found `.`
2019-08-27T07:16:51.3291382Z    |         --------   ^ expected pattern
2019-08-27T07:16:51.3291943Z    |         while parsing this or-pattern starting here
2019-08-27T07:16:51.3292099Z 
2019-08-27T07:16:51.3292251Z error: aborting due to previous error
2019-08-27T07:16:51.3292385Z 
---
2019-08-27T07:16:51.3315579Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-27T07:16:51.3316039Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-27T07:16:51.3330215Z 
2019-08-27T07:16:51.3330299Z 
2019-08-27T07:16:51.3337508Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-27T07:16:51.3338126Z 
2019-08-27T07:16:51.3338161Z 
2019-08-27T07:16:51.3349468Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-27T07:16:51.3350012Z Build completed unsuccessfully in 1:06:02
2019-08-27T07:16:51.3350012Z Build completed unsuccessfully in 1:06:02
2019-08-27T07:16:51.3401636Z == clock drift check ==
2019-08-27T07:16:51.3415942Z   local time: Tue Aug 27 07:16:51 UTC 2019
2019-08-27T07:16:51.4954392Z   network time: Tue, 27 Aug 2019 07:16:51 GMT
2019-08-27T07:16:51.4957548Z == end clock drift check ==
2019-08-27T07:16:52.3645923Z ##[error]Bash exited with code '1'.
2019-08-27T07:16:52.3687474Z ##[section]Starting: Checkout
2019-08-27T07:16:52.3689688Z ==============================================================================
2019-08-27T07:16:52.3689945Z Task         : Get sources
2019-08-27T07:16:52.3689994Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
