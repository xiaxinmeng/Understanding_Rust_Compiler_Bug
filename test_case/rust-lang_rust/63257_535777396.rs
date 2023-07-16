plain
2019-09-27T02:50:55.9933181Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T02:50:56.0147876Z ##[command]git config gc.auto 0
2019-09-27T02:50:56.0205653Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T02:50:56.0285543Z ##[command]git config --get-all http.proxy
2019-09-27T02:50:56.0428631Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63257/merge:refs/remotes/pull/63257/merge
---
2019-09-27T03:56:24.2849937Z .................................................................................................... 1500/9046
2019-09-27T03:56:30.5662858Z .................................................................................................... 1600/9046
2019-09-27T03:56:43.1884708Z .........................................................................i...............i.......... 1700/9046
2019-09-27T03:56:50.3067042Z .................................................................................................... 1800/9046
2019-09-27T03:56:59.0260726Z ................................................................iiiii............................... 1900/9046
2019-09-27T03:57:18.9896798Z .................................................................................................... 2100/9046
2019-09-27T03:57:21.6465937Z .................................................................................................... 2200/9046
2019-09-27T03:57:25.0112242Z .................................................................................................... 2300/9046
2019-09-27T03:57:33.7248795Z .................................................................................................... 2400/9046
---
2019-09-27T04:00:36.9386522Z ....................................................i...............i............................... 4700/9046
2019-09-27T04:00:46.4946581Z .................................................................................................... 4800/9046
2019-09-27T04:00:55.1206951Z .................................................................................................... 4900/9046
2019-09-27T04:01:02.8208008Z .................................................................................................... 5000/9046
2019-09-27T04:01:12.8275872Z ........................................ii.ii....................................................... 5100/9046
2019-09-27T04:01:23.0100302Z ..........................................................................................F......... 5300/9046
2019-09-27T04:01:33.7333257Z .................................................................................................... 5400/9046
2019-09-27T04:01:41.4025164Z .....i.............................................................................................. 5500/9046
2019-09-27T04:01:46.7762244Z .................................................................................................... 5600/9046
2019-09-27T04:01:46.7762244Z .................................................................................................... 5600/9046
2019-09-27T04:01:58.8427511Z .................................................................................................... 5700/9046
2019-09-27T04:02:12.2026799Z ii...i..ii...........i.............................................................................. 5800/9046
2019-09-27T04:02:34.7593385Z .................................................................................................... 6000/9046
2019-09-27T04:02:41.5310617Z .................................................................................................... 6100/9046
2019-09-27T04:02:41.5310617Z .................................................................................................... 6100/9046
2019-09-27T04:02:56.1828361Z ...i..ii............................................................................................ 6200/9046
2019-09-27T04:03:15.7297484Z ...............................................................i.................................... 6400/9046
2019-09-27T04:03:17.9909766Z .................................................................................................... 6500/9046
2019-09-27T04:03:20.6306743Z ...................................i................................................................ 6600/9046
2019-09-27T04:03:24.8779573Z .................................................................................................... 6700/9046
---
2019-09-27T04:07:39.6460687Z 
2019-09-27T04:07:39.6463199Z ---- [ui] ui/lint/use_suggestion_json.rs stdout ----
2019-09-27T04:07:39.6463269Z diff of stderr:
2019-09-27T04:07:39.6463339Z 
2019-09-27T04:07:39.6463410Z 108 {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"\u001b[0m\u001b[1m\u001b[38;5;9merror\u001b[0m\u001b[0m\u001b[1m: aborting due to previous error\u001b[0m
2019-09-27T04:07:39.6463528Z 110 "}
2019-09-27T04:07:39.6463528Z 110 "}
2019-09-27T04:07:39.6464477Z - {"message":"For more information about this error, try `rustc --explain E0412`.","code":null,"level":"","spans":[],"children":[],"rendered":"\u001b[0m\u001b[1mFor more information about this error, try `rustc --explain E0412`.\u001b[0m
2019-09-27T04:07:39.6464938Z + {"message":"For more information about this error, try `rustc --explain E0412`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"\u001b[0m\u001b[1mFor more information about this error, try `rustc --explain E0412`.\u001b[0m
2019-09-27T04:07:39.6465018Z 112 "}
2019-09-27T04:07:39.6465090Z 
2019-09-27T04:07:39.6465117Z 
2019-09-27T04:07:39.6465414Z The actual stderr differed from the expected stderr.
2019-09-27T04:07:39.6465785Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json/use_suggestion_json.stderr
2019-09-27T04:07:39.6465785Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json/use_suggestion_json.stderr
2019-09-27T04:07:39.6466030Z To update references, rerun the tests and pass the `--bless` flag
2019-09-27T04:07:39.6466343Z To only update this specific test, also pass `--test-args lint/use_suggestion_json.rs`
2019-09-27T04:07:39.6466432Z error: 1 errors occurred comparing output.
2019-09-27T04:07:39.6466481Z status: exit code: 1
2019-09-27T04:07:39.6466481Z status: exit code: 1
2019-09-27T04:07:39.6467550Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/use_suggestion_json.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "json" "--json=diagnostic-rendered-ansi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json/auxiliary" "-A" "unused"
2019-09-27T04:07:39.6467909Z ------------------------------------------
2019-09-27T04:07:39.6467942Z 
2019-09-27T04:07:39.6468138Z ------------------------------------------
2019-09-27T04:07:39.6468197Z stderr:
2019-09-27T04:07:39.6468197Z stderr:
2019-09-27T04:07:39.6468388Z ------------------------------------------
2019-09-27T04:07:39.6468625Z error[E0412]: cannot find type `Iter` in this scope
2019-09-27T04:07:39.6468899Z   --> /checkout/src/test/ui/lint/use_suggestion_json.rs:12:12
2019-09-27T04:07:39.6469100Z    |
2019-09-27T04:07:39.6469330Z LL |     let x: Iter;
2019-09-27T04:07:39.6469924Z help: possible candidates are found in other modules, you can import them into scope
2019-09-27T04:07:39.6470125Z    |
2019-09-27T04:07:39.6470394Z LL | use std::collections::binary_heap::Iter;
2019-09-27T04:07:39.6470587Z    |
---
2019-09-27T04:07:39.6502841Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-27T04:07:39.6502921Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-27T04:07:39.6520701Z 
2019-09-27T04:07:39.6520775Z 
2019-09-27T04:07:39.6523019Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-27T04:07:39.6523318Z 
2019-09-27T04:07:39.6523348Z 
2019-09-27T04:07:39.6530066Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-27T04:07:39.6530163Z Build completed unsuccessfully in 1:08:59
2019-09-27T04:07:39.6530163Z Build completed unsuccessfully in 1:08:59
2019-09-27T04:07:39.6583009Z == clock drift check ==
2019-09-27T04:07:39.6611189Z   local time: Fri Sep 27 04:07:39 UTC 2019
2019-09-27T04:07:39.9430777Z   network time: Fri, 27 Sep 2019 04:07:39 GMT
2019-09-27T04:07:39.9433881Z == end clock drift check ==
2019-09-27T04:07:41.0881550Z ##[error]Bash exited with code '1'.
2019-09-27T04:07:41.0935507Z ##[section]Starting: Checkout
2019-09-27T04:07:41.0937568Z ==============================================================================
2019-09-27T04:07:41.0937644Z Task         : Get sources
2019-09-27T04:07:41.0937692Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
