plain
2019-12-23T14:12:48.1150414Z ---- [ui] ui/typeck/issue-66868-closure-typeck.rs stdout ----
2019-12-23T14:12:48.1150618Z diff of stderr:
2019-12-23T14:12:48.1150669Z 
2019-12-23T14:12:48.1150736Z 12    |
2019-12-23T14:12:48.1151244Z 13    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::RwLockWriteGuard<'_, issue_66868_closure_typeck::S>`
2019-12-23T14:12:48.1152561Z 14    = note: required because it appears within the type `for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}`
2019-12-23T14:12:48.1154206Z -    = note: required because it appears within the type `[static generator@DefId(15:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]`
2019-12-23T14:12:48.1156581Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@DefId(15:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]>`
2019-12-23T14:12:48.1158370Z +    = note: required because it appears within the type `[static generator@DefId(14:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]`
2019-12-23T14:12:48.1160035Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@DefId(14:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]>`
2019-12-23T14:12:48.1160724Z 18    = note: required because it appears within the type `impl std::future::Future`
2019-12-23T14:12:48.1160821Z 19 
2019-12-23T14:12:48.1160864Z 
2019-12-23T14:12:48.1160927Z 
2019-12-23T14:12:48.1160927Z 
2019-12-23T14:12:48.1161167Z The actual stderr differed from the expected stderr.
2019-12-23T14:12:48.1161683Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-66868-closure-typeck/issue-66868-closure-typeck.stderr
2019-12-23T14:12:48.1162089Z To update references, rerun the tests and pass the `--bless` flag
2019-12-23T14:12:48.1162478Z To only update this specific test, also pass `--test-args typeck/issue-66868-closure-typeck.rs`
2019-12-23T14:12:48.1162647Z error: 1 errors occurred comparing output.
2019-12-23T14:12:48.1162749Z status: exit code: 1
2019-12-23T14:12:48.1162749Z status: exit code: 1
2019-12-23T14:12:48.1163927Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-66868-closure-typeck.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-66868-closure-typeck" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-66868-closure-typeck/auxiliary" "-A" "unused"
2019-12-23T14:12:48.1164722Z ------------------------------------------
2019-12-23T14:12:48.1164781Z 
2019-12-23T14:12:48.1165080Z ------------------------------------------
2019-12-23T14:12:48.1165161Z stderr:
2019-12-23T14:12:48.1165161Z stderr:
2019-12-23T14:12:48.1165458Z ------------------------------------------
2019-12-23T14:12:48.1165856Z error[E0277]: `std::sync::RwLockWriteGuard<'_, issue_66868_closure_typeck::S>` cannot be sent between threads safely
2019-12-23T14:12:48.1166331Z    |
2019-12-23T14:12:48.1166331Z    |
2019-12-23T14:12:48.1166423Z LL | pub fn g<T>(task: T)
2019-12-23T14:12:48.1166772Z LL | where
2019-12-23T14:12:48.1166863Z LL |     T: Send,
2019-12-23T14:12:48.1167162Z    |        ---- required by this bound in `g`
2019-12-23T14:12:48.1167267Z ...
2019-12-23T14:12:48.1167267Z ...
2019-12-23T14:12:48.1167371Z LL |     g(issue_66868_closure_typeck::f()); //~ ERROR: cannot be sent between threads safely
2019-12-23T14:12:48.1167810Z    |     ^ `std::sync::RwLockWriteGuard<'_, issue_66868_closure_typeck::S>` cannot be sent between threads safely
2019-12-23T14:12:48.1167917Z    |
2019-12-23T14:12:48.1168382Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::RwLockWriteGuard<'_, issue_66868_closure_typeck::S>`
2019-12-23T14:12:48.1169647Z    = note: required because it appears within the type `for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}`
2019-12-23T14:12:48.1171380Z    = note: required because it appears within the type `[static generator@DefId(14:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]`
2019-12-23T14:12:48.1173043Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@DefId(14:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]>`
2019-12-23T14:12:48.1173591Z    = note: required because it appears within the type `impl std::future::Future`
2019-12-23T14:12:48.1173669Z 
2019-12-23T14:12:48.1173759Z error: aborting due to previous error
2019-12-23T14:12:48.1173809Z 
---
2019-12-23T14:12:48.1203574Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-23T14:12:48.1203763Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-23T14:12:48.1233155Z 
2019-12-23T14:12:48.1233308Z 
2019-12-23T14:12:48.1236534Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-23T14:12:48.1237583Z 
2019-12-23T14:12:48.1237651Z 
2019-12-23T14:12:48.1238303Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-12-23T14:12:48.1238475Z Build completed unsuccessfully in 1:17:52
2019-12-23T14:12:48.1238475Z Build completed unsuccessfully in 1:17:52
2019-12-23T14:12:48.1303658Z == clock drift check ==
2019-12-23T14:12:48.1322997Z   local time: Mon Dec 23 14:12:48 UTC 2019
2019-12-23T14:12:48.1677465Z   network time: Mon, 23 Dec 2019 14:12:48 GMT
2019-12-23T14:12:48.1677958Z == end clock drift check ==
2019-12-23T14:12:49.5198342Z 
2019-12-23T14:12:49.5312156Z ##[error]Bash exited with code '1'.
2019-12-23T14:12:49.5371473Z ##[section]Starting: Checkout
2019-12-23T14:12:49.5373684Z ==============================================================================
2019-12-23T14:12:49.5373807Z Task         : Get sources
2019-12-23T14:12:49.5373919Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
