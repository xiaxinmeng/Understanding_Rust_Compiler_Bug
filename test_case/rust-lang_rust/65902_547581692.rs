plain
2019-10-29T18:01:05.3174244Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-29T18:01:05.3414239Z ##[command]git config gc.auto 0
2019-10-29T18:01:05.3490028Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-29T18:01:05.3563328Z ##[command]git config --get-all http.proxy
2019-10-29T18:01:05.3701728Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65902/merge:refs/remotes/pull/65902/merge
---
2019-10-29T18:54:28.1806380Z .................................................................................................... 1600/9263
2019-10-29T18:54:33.4055118Z .................................................................................................... 1700/9263
2019-10-29T18:54:44.6238921Z ...........................................................i...............i........................ 1800/9263
2019-10-29T18:54:51.4354376Z .................................................................................................... 1900/9263
2019-10-29T18:55:04.5556999Z .................................................iiiii.............................................. 2000/9263
2019-10-29T18:55:14.4438106Z .................................................................................................... 2200/9263
2019-10-29T18:55:16.8210548Z .................................................................................................... 2300/9263
2019-10-29T18:55:20.2437894Z .................................................................................................... 2400/9263
2019-10-29T18:55:40.9658819Z .................................................................................................... 2500/9263
---
2019-10-29T18:58:22.3711967Z ..................................................i...............i................................. 4800/9263
2019-10-29T18:58:31.1898032Z ..F................................................................................................. 4900/9263
2019-10-29T18:58:39.1908865Z .................................................................................................... 5000/9263
2019-10-29T18:58:45.1480618Z .................................................................................................... 5100/9263
2019-10-29T18:58:54.9643578Z ...................................................ii.ii...........i................................ 5200/9263
2019-10-29T18:59:04.1352027Z .................................................................................................... 5400/9263
2019-10-29T18:59:12.8426523Z .................................................................................................... 5500/9263
2019-10-29T18:59:19.8179318Z .......................i............................................................................ 5600/9263
2019-10-29T18:59:25.5602260Z .................................................................................................... 5700/9263
2019-10-29T18:59:25.5602260Z .................................................................................................... 5700/9263
2019-10-29T18:59:36.3202569Z .................................................................................................... 5800/9263
2019-10-29T18:59:47.3525108Z ........ii...i..ii...........i...................................................................... 5900/9263
2019-10-29T19:00:07.5027129Z .................................................................................................... 6100/9263
2019-10-29T19:00:14.5856175Z .................................................................................................... 6200/9263
2019-10-29T19:00:14.5856175Z .................................................................................................... 6200/9263
2019-10-29T19:00:27.2080656Z ...........................i..ii.................................................................... 6300/9263
2019-10-29T19:00:45.6090314Z .............................................................................................i...... 6500/9263
2019-10-29T19:00:47.6225575Z .................................................................................................... 6600/9263
2019-10-29T19:00:49.8243328Z ....................................................................i............................... 6700/9263
2019-10-29T19:00:52.4819262Z .................................................................................................... 6800/9263
---
2019-10-29T19:04:36.0216897Z failures:
2019-10-29T19:04:36.0250009Z 
2019-10-29T19:04:36.0250792Z ---- [ui] ui/issues/issue-50480.rs stdout ----
2019-10-29T19:04:36.0250967Z 
2019-10-29T19:04:36.0251169Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-10-29T19:04:36.0251299Z status: exit code: 101
2019-10-29T19:04:36.0252036Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50480.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50480" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50480/auxiliary" "-A" "unused"
2019-10-29T19:04:36.0252590Z ------------------------------------------
2019-10-29T19:04:36.0252729Z 
2019-10-29T19:04:36.0253031Z ------------------------------------------
2019-10-29T19:04:36.0253202Z stderr:
2019-10-29T19:04:36.0253202Z stderr:
2019-10-29T19:04:36.0253492Z ------------------------------------------
2019-10-29T19:04:36.0253647Z error[E0412]: cannot find type `NotDefined` in this scope
2019-10-29T19:04:36.0254300Z   --> /checkout/src/test/ui/issues/issue-50480.rs:3:12
2019-10-29T19:04:36.0254448Z    |
2019-10-29T19:04:36.0254564Z LL | struct Foo(NotDefined, <i32 as Iterator>::Item, Vec<i32>, String);
2019-10-29T19:04:36.0254799Z 
2019-10-29T19:04:36.0254799Z 
2019-10-29T19:04:36.0255161Z error: internal compiler error: src/librustc/hir/map/mod.rs:594: couldn't find hir id HirId { owner: DefIndex(0), local_id: 0 } in the HIR map
2019-10-29T19:04:36.0255846Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:925:9
2019-10-29T19:04:36.0256006Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-29T19:04:36.0256126Z 
2019-10-29T19:04:36.0256241Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-29T19:04:36.0256241Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-29T19:04:36.0256337Z 
2019-10-29T19:04:36.0256801Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-29T19:04:36.0257248Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-29T19:04:36.0257397Z 
2019-10-29T19:04:36.0257397Z 
2019-10-29T19:04:36.0257725Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-29T19:04:36.0257997Z error: aborting due to 2 previous errors
2019-10-29T19:04:36.0258092Z 
2019-10-29T19:04:36.0258392Z For more information about this error, try `rustc --explain E0412`.
2019-10-29T19:04:36.0258547Z 
---
2019-10-29T19:04:36.0282853Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-29T19:04:36.0282960Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-29T19:04:36.0302311Z 
2019-10-29T19:04:36.0302384Z 
2019-10-29T19:04:36.0304284Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-29T19:04:36.0304546Z 
2019-10-29T19:04:36.0304571Z 
2019-10-29T19:04:36.0356660Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-29T19:04:36.0357022Z Build completed unsuccessfully in 0:57:10
2019-10-29T19:04:36.0357022Z Build completed unsuccessfully in 0:57:10
2019-10-29T19:04:36.0362699Z == clock drift check ==
2019-10-29T19:04:36.0379053Z   local time: Tue Oct 29 19:04:36 UTC 2019
2019-10-29T19:04:36.3443887Z   network time: Tue, 29 Oct 2019 19:04:36 GMT
2019-10-29T19:04:36.3444146Z == end clock drift check ==
2019-10-29T19:04:37.7231495Z 
2019-10-29T19:04:37.7334456Z ##[error]Bash exited with code '1'.
2019-10-29T19:04:37.7368992Z ##[section]Starting: Checkout
2019-10-29T19:04:37.7370714Z ==============================================================================
2019-10-29T19:04:37.7370760Z Task         : Get sources
2019-10-29T19:04:37.7370947Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
