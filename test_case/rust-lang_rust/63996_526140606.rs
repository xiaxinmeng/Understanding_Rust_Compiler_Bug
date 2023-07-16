plain
2019-08-29T11:02:51.8321688Z test [ui] ui/consts/const-eval/strlen.rs ... ok
2019-08-29T11:02:51.8322142Z test [ui] ui/consts/const-eval/ub-ref.rs ... ok
2019-08-29T11:02:51.8322794Z test [ui] ui/consts/const-eval/ub-uninhabit.rs ... ok
2019-08-29T11:02:51.8323408Z test [ui] ui/consts/const-eval/ub-upvars.rs ... ok
2019-08-29T11:02:51.8323839Z test [ui] ui/consts/const-eval/ub-wide-ptr.rs ... ok
2019-08-29T11:02:51.8325013Z test [ui] ui/consts/const-eval/union-ice.rs ... ok
2019-08-29T11:02:51.8354056Z test [ui] ui/consts/const-eval/union-ub.rs ... ok
2019-08-29T11:02:51.8354521Z test [ui] ui/consts/const-eval/unused-broken-const.rs ... ok
2019-08-29T11:02:51.8354983Z test [ui] ui/consts/const-eval/union_promotion.rs ... ok
---
2019-08-29T11:10:31.6378291Z test [ui] ui/rfc-2166-underscore-imports/duplicate.rs ... ok
2019-08-29T11:10:31.7151125Z test [ui] ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs ... ok
2019-08-29T11:10:31.8202970Z test [ui] ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs ... ok
2019-08-29T11:10:31.8605223Z test [ui] ui/rfc-2497-if-let-chains/ast-pretty-check.rs ... ok
2019-08-29T11:10:32.0706560Z ERROR 2019-08-29T11:10:32Z: compiletest::runtest: fatal error, panic: "failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/dbg-macro-expected-behavior.run.stderr`: No such file or directory (os error 2)"
2019-08-29T11:10:32.1951486Z test [ui] ui/rfc-2497-if-let-chains/feature-gate.rs ... ok
2019-08-29T11:10:32.2277762Z test [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs ... ok
2019-08-29T11:10:32.2712314Z test [ui] ui/rfc-2565-param-attrs/param-attrs-2018.rs ... ok
2019-08-29T11:10:32.3503128Z test [ui] ui/rfc-2565-param-attrs/param-attrs-allowed.rs ... ok
---
2019-08-29T11:13:15.4250974Z 
2019-08-29T11:13:15.4252994Z ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
2019-08-29T11:13:15.4253137Z diff of run.stderr:
2019-08-29T11:13:15.4253184Z 
2019-08-29T11:13:15.4253596Z - [$DIR/dbg-macro-expected-behavior.rs:20] Unit = Unit
2019-08-29T11:13:15.4253860Z - [$DIR/dbg-macro-expected-behavior.rs:21] a = Unit
2019-08-29T11:13:15.4254155Z - [$DIR/dbg-macro-expected-behavior.rs:27] Point{x: 42, y: 24,} = Point {
2019-08-29T11:13:15.4254376Z -     x: 42,
2019-08-29T11:13:15.4254594Z -     y: 24,
2019-08-29T11:13:15.4254991Z - }
2019-08-29T11:13:15.4255360Z - [$DIR/dbg-macro-expected-behavior.rs:28] b = Point {
2019-08-29T11:13:15.4255538Z -     x: 42,
2019-08-29T11:13:15.4255711Z -     y: 24,
2019-08-29T11:13:15.4255858Z - }
2019-08-29T11:13:15.4256048Z - [$DIR/dbg-macro-expected-behavior.rs:36]
2019-08-29T11:13:15.4256243Z - [$DIR/dbg-macro-expected-behavior.rs:40] &a = NoCopy(
2019-08-29T11:13:15.4256748Z - )
2019-08-29T11:13:15.4256748Z - )
2019-08-29T11:13:15.4256945Z - [$DIR/dbg-macro-expected-behavior.rs:40] dbg!(& a) = NoCopy(
2019-08-29T11:13:15.4257254Z - )
2019-08-29T11:13:15.4257254Z - )
2019-08-29T11:13:15.4257615Z - [$DIR/dbg-macro-expected-behavior.rs:45] f(&42) = 42
2019-08-29T11:13:15.4257790Z - before
2019-08-29T11:13:15.4258013Z - [$DIR/dbg-macro-expected-behavior.rs:50] { foo += 1; eprintln!("before"); 7331 } = 7331
2019-08-29T11:13:15.4258233Z - [$DIR/dbg-macro-expected-behavior.rs:58] ("Yeah",) = (
2019-08-29T11:13:15.4258395Z -     "Yeah",
2019-08-29T11:13:15.4258558Z - )
2019-08-29T11:13:15.4258741Z - [$DIR/dbg-macro-expected-behavior.rs:61] 1 = 1
2019-08-29T11:13:15.4258954Z - [$DIR/dbg-macro-expected-behavior.rs:61] 2 = 2
2019-08-29T11:13:15.4259147Z - [$DIR/dbg-macro-expected-behavior.rs:65] 1u8 = 1
2019-08-29T11:13:15.4259538Z - [$DIR/dbg-macro-expected-behavior.rs:65] 2u32 = 2
2019-08-29T11:13:15.4259739Z - [$DIR/dbg-macro-expected-behavior.rs:65] "Yeah" = "Yeah"
2019-08-29T11:13:15.4259958Z 
2019-08-29T11:13:15.4259987Z 
2019-08-29T11:13:15.4259987Z 
2019-08-29T11:13:15.4260500Z error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/dbg-macro-expected-behavior.run.stderr`: No such file or directory (os error 2)
2019-08-29T11:13:15.4260825Z thread '[ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2236:9
2019-08-29T11:13:15.4260959Z 
2019-08-29T11:13:15.4261001Z 
2019-08-29T11:13:15.4261045Z failures:
2019-08-29T11:13:15.4261618Z     [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs
2019-08-29T11:13:15.4261618Z     [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs
2019-08-29T11:13:15.4261677Z 
2019-08-29T11:13:15.4262021Z test result: FAILED. 8639 passed; 1 failed; 333 ignored; 0 measured; 0 filtered out
2019-08-29T11:13:15.4262082Z 
2019-08-29T11:13:15.4306883Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-29T11:13:15.4320559Z 
2019-08-29T11:13:15.4320708Z 
2019-08-29T11:13:15.4327742Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-29T11:13:15.4328362Z 
2019-08-29T11:13:15.4328397Z 
2019-08-29T11:13:15.4342027Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-08-29T11:13:15.4342176Z Build completed unsuccessfully in 1:24:49
2019-08-29T11:13:15.4342176Z Build completed unsuccessfully in 1:24:49
2019-08-29T11:13:15.4405849Z == clock drift check ==
2019-08-29T11:13:15.4430004Z   local time: Thu Aug 29 11:13:15 UTC 2019
2019-08-29T11:13:15.5516679Z   network time: Thu, 29 Aug 2019 11:13:15 GMT
2019-08-29T11:13:15.5520416Z == end clock drift check ==
2019-08-29T11:13:16.5865677Z ##[error]Bash exited with code '1'.
2019-08-29T11:13:16.5906574Z ##[section]Starting: Upload CPU usage statistics
2019-08-29T11:13:16.5913755Z ==============================================================================
2019-08-29T11:13:16.5913869Z Task         : Bash
2019-08-29T11:13:16.5913955Z Description  : Run a Bash script on macOS, Linux, or Windows
