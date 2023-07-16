plain
2019-09-19T15:51:02.1102155Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-19T15:51:02.1274602Z ##[command]git config gc.auto 0
2019-09-19T15:51:02.1350641Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-19T15:51:02.1434415Z ##[command]git config --get-all http.proxy
2019-09-19T15:51:02.1576115Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64584/merge:refs/remotes/pull/64584/merge
---
2019-09-19T16:54:59.4704862Z .................................................................................................... 1500/9027
2019-09-19T16:55:05.5508289Z .................................................................................................... 1600/9027
2019-09-19T16:55:18.3972955Z ....................................................................i...............i............... 1700/9027
2019-09-19T16:55:25.2806609Z .................................................................................................... 1800/9027
2019-09-19T16:55:40.9932925Z ...........................................................iiiii.................................... 1900/9027
2019-09-19T16:55:53.3347309Z .................................................................................................... 2100/9027
2019-09-19T16:55:55.9047497Z .................................................................................................... 2200/9027
2019-09-19T16:55:59.2275189Z .................................................................................................... 2300/9027
2019-09-19T16:56:08.0164822Z .................................................................................................... 2400/9027
---
2019-09-19T16:59:11.0128981Z ...............................................i...............i.................................... 4700/9027
2019-09-19T16:59:21.1800014Z .................................................................................................... 4800/9027
2019-09-19T16:59:28.9325284Z .................................................................................................... 4900/9027
2019-09-19T16:59:38.9645765Z .................................................................................................... 5000/9027
2019-09-19T16:59:47.1360543Z ...............................ii.ii................................................................ 5100/9027
2019-09-19T16:59:56.7849335Z .................................................................................................... 5300/9027
2019-09-19T17:00:07.7807797Z ...............................................................................................i.... 5400/9027
2019-09-19T17:00:16.3804684Z .................................................................................................... 5500/9027
2019-09-19T17:00:21.3723424Z .................................................................................................... 5600/9027
2019-09-19T17:00:21.3723424Z .................................................................................................... 5600/9027
2019-09-19T17:00:31.9717571Z ..........................................................................................ii...i..ii 5700/9027
2019-09-19T17:00:58.1488661Z .................................................................................................... 5900/9027
2019-09-19T17:01:07.2020885Z .................................................................................................... 6000/9027
2019-09-19T17:01:07.2020885Z .................................................................................................... 6000/9027
2019-09-19T17:01:12.0252256Z ............................................................................................i..ii... 6100/9027
2019-09-19T17:01:41.0759995Z .................................................................................................... 6300/9027
2019-09-19T17:01:45.7259042Z ...................................................i................................................ 6400/9027
2019-09-19T17:01:48.3925475Z .................................................................................................... 6500/9027
2019-09-19T17:01:50.5844719Z .......................i............................................................................ 6600/9027
---
2019-09-19T17:06:03.9067843Z failures:
2019-09-19T17:06:03.9107964Z 
2019-09-19T17:06:03.9109053Z ---- [ui] ui/async-await/issues/issue-64391-2.rs stdout ----
2019-09-19T17:06:03.9111035Z normalized stderr:
2019-09-19T17:06:03.9111110Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-09-19T17:06:03.9111536Z   --> $DIR/issue-64391-2.rs:8:1
2019-09-19T17:06:03.9111669Z LL | async fn connect() {
2019-09-19T17:06:03.9111713Z    | ^^^^^
2019-09-19T17:06:03.9111744Z 
2019-09-19T17:06:03.9111744Z 
2019-09-19T17:06:03.9111791Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-09-19T17:06:03.9112045Z   --> $DIR/issue-64391-2.rs:13:1
2019-09-19T17:06:03.9112092Z    |
2019-09-19T17:06:03.9112139Z LL | async fn connect2(_config: &u32, _tls: String) {
2019-09-19T17:06:03.9112230Z 
2019-09-19T17:06:03.9112277Z error[E0609]: no field `await` on type `impl std::future::Future`
2019-09-19T17:06:03.9112519Z   --> $DIR/issue-64391-2.rs:10:39
2019-09-19T17:06:03.9112566Z    |
2019-09-19T17:06:03.9112566Z    |
2019-09-19T17:06:03.9112613Z LL |     connect2(&config, "".to_string()).await
2019-09-19T17:06:03.9143401Z    |
2019-09-19T17:06:03.9143401Z    |
2019-09-19T17:06:03.9143453Z    = note: to `.await` a `Future`, switch to Rust 2018
2019-09-19T17:06:03.9143503Z    = help: set `edition = "2018"` in `Cargo.toml`
2019-09-19T17:06:03.9144023Z    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
2019-09-19T17:06:03.9144271Z error: aborting due to 3 previous errors
2019-09-19T17:06:03.9144298Z 
2019-09-19T17:06:03.9144356Z Some errors have detailed explanations: E0609, E0670.
2019-09-19T17:06:03.9145402Z For more information about an error, try `rustc --explain E0609`.
2019-09-19T17:06:03.9145402Z For more information about an error, try `rustc --explain E0609`.
2019-09-19T17:06:03.9145440Z 
2019-09-19T17:06:03.9145466Z 
2019-09-19T17:06:03.9145513Z 
2019-09-19T17:06:03.9145560Z The actual stderr differed from the expected stderr.
2019-09-19T17:06:03.9145893Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64391-2/issue-64391-2.stderr
2019-09-19T17:06:03.9146172Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T17:06:03.9146489Z To only update this specific test, also pass `--test-args async-await/issues/issue-64391-2.rs`
2019-09-19T17:06:03.9146776Z error: 1 errors occurred comparing output.
2019-09-19T17:06:03.9146845Z status: exit code: 1
2019-09-19T17:06:03.9146845Z status: exit code: 1
2019-09-19T17:06:03.9147673Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-64391-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64391-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64391-2/auxiliary" "-A" "unused"
2019-09-19T17:06:03.9148201Z ------------------------------------------
2019-09-19T17:06:03.9148235Z 
2019-09-19T17:06:03.9148634Z ------------------------------------------
2019-09-19T17:06:03.9148677Z stderr:
2019-09-19T17:06:03.9148677Z stderr:
2019-09-19T17:06:03.9148869Z ------------------------------------------
2019-09-19T17:06:03.9148941Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-09-19T17:06:03.9149172Z   --> /checkout/src/test/ui/async-await/issues/issue-64391-2.rs:8:1
2019-09-19T17:06:03.9149260Z LL | async fn connect() {
2019-09-19T17:06:03.9149314Z    | ^^^^^
2019-09-19T17:06:03.9149339Z 
2019-09-19T17:06:03.9149339Z 
2019-09-19T17:06:03.9149380Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-09-19T17:06:03.9149627Z   --> /checkout/src/test/ui/async-await/issues/issue-64391-2.rs:13:1
2019-09-19T17:06:03.9149671Z    |
2019-09-19T17:06:03.9149712Z LL | async fn connect2(_config: &u32, _tls: String) {
2019-09-19T17:06:03.9149793Z 
2019-09-19T17:06:03.9149924Z error[E0609]: no field `await` on type `impl std::future::Future`
2019-09-19T17:06:03.9150184Z   --> /checkout/src/test/ui/async-await/issues/issue-64391-2.rs:10:39
2019-09-19T17:06:03.9150245Z    |
2019-09-19T17:06:03.9150245Z    |
2019-09-19T17:06:03.9150286Z LL |     connect2(&config, "".to_string()).await
2019-09-19T17:06:03.9150395Z    |
2019-09-19T17:06:03.9150395Z    |
2019-09-19T17:06:03.9150436Z    = note: to `.await` a `Future`, switch to Rust 2018
2019-09-19T17:06:03.9150480Z    = help: set `edition = "2018"` in `Cargo.toml`
2019-09-19T17:06:03.9151235Z    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
2019-09-19T17:06:03.9151339Z error: aborting due to 3 previous errors
2019-09-19T17:06:03.9151369Z 
2019-09-19T17:06:03.9151413Z Some errors have detailed explanations: E0609, E0670.
2019-09-19T17:06:03.9151678Z For more information about an error, try `rustc --explain E0609`.
---
2019-09-19T17:06:03.9152259Z ---- [ui] ui/async-await/issues/issue-64433.rs stdout ----
2019-09-19T17:06:03.9152292Z 
2019-09-19T17:06:03.9152541Z error: test compilation failed although it shouldn't!
2019-09-19T17:06:03.9152610Z status: exit code: 1
2019-09-19T17:06:03.9153692Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-64433.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64433" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64433/auxiliary" "-A" "unused"
2019-09-19T17:06:03.9154049Z ------------------------------------------
2019-09-19T17:06:03.9154085Z 
2019-09-19T17:06:03.9154474Z ------------------------------------------
2019-09-19T17:06:03.9154516Z stderr:
2019-09-19T17:06:03.9154516Z stderr:
2019-09-19T17:06:03.9154706Z ------------------------------------------
2019-09-19T17:06:03.9154894Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-09-19T17:06:03.9155154Z   --> /checkout/src/test/ui/async-await/issues/issue-64433.rs:16:5
2019-09-19T17:06:03.9155204Z    |
2019-09-19T17:06:03.9155454Z LL |     async fn something_with_a(&mut self, a: A<'_>) -> Result<(), String> {
2019-09-19T17:06:03.9155526Z 
2019-09-19T17:06:03.9155526Z 
2019-09-19T17:06:03.9155567Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-09-19T17:06:03.9156176Z   --> /checkout/src/test/ui/async-await/issues/issue-64433.rs:22:1
2019-09-19T17:06:03.9156219Z    |
2019-09-19T17:06:03.9156430Z LL | async fn can_error(some_string: &str) -> Result<(), String> {
2019-09-19T17:06:03.9156524Z 
2019-09-19T17:06:03.9156565Z error[E0609]: no field `await` on type `impl std::future::Future`
2019-09-19T17:06:03.9156789Z   --> /checkout/src/test/ui/async-await/issues/issue-64433.rs:25:30
2019-09-19T17:06:03.9156857Z    |
2019-09-19T17:06:03.9156857Z    |
2019-09-19T17:06:03.9156898Z LL |     Ok(b.something_with_a(a).await.map(|_| ())?)
2019-09-19T17:06:03.9157003Z    |
2019-09-19T17:06:03.9157003Z    |
2019-09-19T17:06:03.9157047Z    = note: to `.await` a `Future`, switch to Rust 2018
2019-09-19T17:06:03.9157093Z    = help: set `edition = "2018"` in `Cargo.toml`
2019-09-19T17:06:03.9157367Z    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
2019-09-19T17:06:03.9157442Z error: aborting due to 3 previous errors
2019-09-19T17:06:03.9157470Z 
2019-09-19T17:06:03.9157529Z Some errors have detailed explanations: E0609, E0670.
2019-09-19T17:06:03.9157870Z For more information about an error, try `rustc --explain E0609`.
---
2019-09-19T17:06:03.9159158Z 
2019-09-19T17:06:03.9159364Z ------------------------------------------
2019-09-19T17:06:03.9159425Z stderr:
2019-09-19T17:06:03.9159622Z ------------------------------------------
2019-09-19T17:06:03.9159937Z thread 'main' panicked at 'used 12496 bytes of stack, but `struct Big` is only 384 bytes', /checkout/src/test/ui/issues/issue-40883.rs:83:9
2019-09-19T17:06:03.9160056Z 
2019-09-19T17:06:03.9160267Z ------------------------------------------
2019-09-19T17:06:03.9160299Z 
2019-09-19T17:06:03.9160341Z 
---
2019-09-19T17:06:03.9166133Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-19T17:06:03.9166205Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-19T17:06:03.9166261Z 
2019-09-19T17:06:03.9166284Z 
2019-09-19T17:06:03.9167918Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-19T17:06:03.9169774Z 
2019-09-19T17:06:03.9169804Z 
2019-09-19T17:06:03.9169880Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-19T17:06:03.9169945Z Build completed unsuccessfully in 1:07:55
2019-09-19T17:06:03.9169945Z Build completed unsuccessfully in 1:07:55
2019-09-19T17:06:03.9223297Z == clock drift check ==
2019-09-19T17:06:03.9239760Z   local time: Thu Sep 19 17:06:03 UTC 2019
2019-09-19T17:06:04.0231757Z   network time: Thu, 19 Sep 2019 17:06:04 GMT
2019-09-19T17:06:04.0235601Z == end clock drift check ==
2019-09-19T17:06:04.9975116Z ##[error]Bash exited with code '1'.
2019-09-19T17:06:05.0045650Z ##[section]Starting: Checkout
2019-09-19T17:06:05.0047370Z ==============================================================================
2019-09-19T17:06:05.0047422Z Task         : Get sources
2019-09-19T17:06:05.0047479Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
