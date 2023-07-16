plain
2019-12-28T16:58:24.1864799Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-28T16:58:24.1963584Z ##[command]git config gc.auto 0
2019-12-28T16:58:24.2060726Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-28T16:58:24.2143143Z ##[command]git config --get-all http.proxy
2019-12-28T16:58:24.2296050Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67681/merge:refs/remotes/pull/67681/merge
---
2019-12-28T18:00:03.2037878Z .................................................................................................... 1600/9464
2019-12-28T18:00:08.1920110Z .................................................................................................... 1700/9464
2019-12-28T18:00:17.7692491Z ..................................................................................................i. 1800/9464
2019-12-28T18:00:26.2947456Z .................................................................................................... 1900/9464
2019-12-28T18:00:33.1610752Z ....................................................................................iiiii........... 2000/9464
2019-12-28T18:00:55.4292801Z .................................................................................................... 2200/9464
2019-12-28T18:00:57.9006627Z .................................................................................................... 2300/9464
2019-12-28T18:01:00.4275142Z .................................................................................................... 2400/9464
2019-12-28T18:01:06.6913708Z .................................................................................................... 2500/9464
---
2019-12-28T18:04:10.8786864Z ...............i...............i.................................................................... 4900/9464
2019-12-28T18:04:21.2257233Z .................................................................................................... 5000/9464
2019-12-28T18:04:26.9877763Z ............................................................i....................................... 5100/9464
2019-12-28T18:04:35.3753888Z .................................................................................................... 5200/9464
2019-12-28T18:04:43.1161226Z ...........................ii.ii...........i........................................................ 5300/9464
2019-12-28T18:04:52.4968921Z .................................................................................................... 5500/9464
2019-12-28T18:05:03.2787754Z .................................................................................................... 5600/9464
2019-12-28T18:05:10.3263419Z .........i.......................................................................................... 5700/9464
2019-12-28T18:05:16.7661395Z .................................................................................................... 5800/9464
2019-12-28T18:05:16.7661395Z .................................................................................................... 5800/9464
2019-12-28T18:05:27.6545966Z .................................................................................................ii. 5900/9464
2019-12-28T18:05:40.0543214Z ..i..ii...........i................................................................................. 6000/9464
2019-12-28T18:05:58.4214043Z .................................................................................................... 6200/9464
2019-12-28T18:06:06.3946464Z .................................................................................................... 6300/9464
2019-12-28T18:06:06.3946464Z .................................................................................................... 6300/9464
2019-12-28T18:06:23.6713406Z ........................i..ii....................................................................... 6400/9464
2019-12-28T18:06:44.0917546Z .................................................................................................... 6600/9464
2019-12-28T18:06:46.2779679Z .i.................................................................................................. 6700/9464
2019-12-28T18:06:48.6918609Z .................................................................................................... 6800/9464
2019-12-28T18:06:51.3636543Z .i.................................................................................................. 6900/9464
---
2019-12-28T18:08:32.4178676Z .................................................................................................... 7500/9464
2019-12-28T18:08:37.5925183Z .................................................................................................... 7600/9464
2019-12-28T18:08:43.2428142Z .................................................................................................... 7700/9464
2019-12-28T18:08:53.7299817Z .................................................................................................... 7800/9464
2019-12-28T18:09:01.4864182Z ................................iiii................................................................ 7900/9464
2019-12-28T18:09:16.5564557Z .................................................................................................... 8100/9464
2019-12-28T18:09:26.1981068Z .................................................................................................... 8200/9464
2019-12-28T18:09:40.7994590Z .................................................................................................... 8300/9464
2019-12-28T18:09:48.6051438Z .................................................................................................... 8400/9464
---
2019-12-28T18:12:13.6133932Z  finished in 6.670
2019-12-28T18:12:13.6349572Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-28T18:12:13.8459642Z 
2019-12-28T18:12:13.8461801Z running 166 tests
2019-12-28T18:12:16.9993645Z iiii......i........ii..iiii...i.............................i..i..................i....i............ 100/166
2019-12-28T18:12:19.1808301Z i..ii...iii..iiiiiii.......................iii............ii......
2019-12-28T18:12:19.1809243Z 
2019-12-28T18:12:19.1812437Z  finished in 5.546
2019-12-28T18:12:19.2019193Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-28T18:12:19.3823921Z 
---
2019-12-28T18:12:21.4261581Z  finished in 2.223
2019-12-28T18:12:21.4454414Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-28T18:12:21.6164467Z 
2019-12-28T18:12:21.6164709Z running 9 tests
2019-12-28T18:12:21.6165730Z iiiiiiiii
2019-12-28T18:12:21.6166148Z 
2019-12-28T18:12:21.6166195Z  finished in 0.170
2019-12-28T18:12:21.6369437Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-28T18:12:21.8498469Z 
2019-12-28T18:12:21.8498469Z 
2019-12-28T18:12:21.8498971Z running 113 tests
2019-12-28T18:12:39.9459615Z ............................F...................................FF.................................. 100/113
2019-12-28T18:12:41.9471775Z .............
2019-12-28T18:12:41.9472918Z failures:
2019-12-28T18:12:41.9473012Z 
2019-12-28T18:12:41.9473527Z ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
2019-12-28T18:12:41.9473590Z 
2019-12-28T18:12:41.9473875Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-12-28T18:12:41.9473928Z status: exit code: 101
2019-12-28T18:12:41.9475676Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
2019-12-28T18:12:41.9477436Z ------------------------------------------
2019-12-28T18:12:41.9477476Z 
2019-12-28T18:12:41.9477699Z ------------------------------------------
2019-12-28T18:12:41.9477764Z stderr:
2019-12-28T18:12:41.9477764Z stderr:
2019-12-28T18:12:41.9478574Z ------------------------------------------
2019-12-28T18:12:41.9479013Z thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&rustc::mir::BorrowCheckResult>>::specialized_decode` not overridden', src/libserialize/serialize.rs:893:9
2019-12-28T18:12:41.9479169Z 
2019-12-28T18:12:41.9479572Z error: internal compiler error: unexpected panic
2019-12-28T18:12:41.9479847Z 
2019-12-28T18:12:41.9479912Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T18:12:41.9479912Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T18:12:41.9479951Z 
2019-12-28T18:12:41.9480451Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-28T18:12:41.9480631Z 
2019-12-28T18:12:41.9481322Z note: rustc 1.42.0-nightly (2ee72b593 2019-12-28) running on x86_64-unknown-linux-gnu
2019-12-28T18:12:41.9481364Z 
2019-12-28T18:12:41.9482042Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-28T18:12:41.9482519Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-12-28T18:12:41.9482519Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-12-28T18:12:41.9482586Z   left: `LLVMing`,
2019-12-28T18:12:41.9482973Z  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1443:21
2019-12-28T18:12:41.9483447Z error: internal compiler error: unexpected panic
2019-12-28T18:12:41.9483493Z 
2019-12-28T18:12:41.9483667Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T18:12:41.9483720Z 
2019-12-28T18:12:41.9483720Z 
2019-12-28T18:12:41.9484220Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-28T18:12:41.9486023Z 
2019-12-28T18:12:41.9487095Z note: rustc 1.42.0-nightly (2ee72b593 2019-12-28) running on x86_64-unknown-linux-gnu
2019-12-28T18:12:41.9487377Z 
2019-12-28T18:12:41.9488134Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-28T18:12:41.9488447Z 
2019-12-28T18:12:41.9488746Z ------------------------------------------
2019-12-28T18:12:41.9488780Z 
2019-12-28T18:12:41.9488839Z 
2019-12-28T18:12:41.9488839Z 
2019-12-28T18:12:41.9489269Z ---- [incremental] incremental/issue-42602.rs stdout ----
2019-12-28T18:12:41.9489431Z 
2019-12-28T18:12:41.9489808Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-12-28T18:12:41.9489875Z status: exit code: 101
2019-12-28T18:12:41.9491187Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-42602.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/issue-42602.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/auxiliary"
2019-12-28T18:12:41.9491794Z ------------------------------------------
2019-12-28T18:12:41.9491830Z 
2019-12-28T18:12:41.9492217Z ------------------------------------------
2019-12-28T18:12:41.9492413Z stderr:
2019-12-28T18:12:41.9492413Z stderr:
2019-12-28T18:12:41.9492681Z ------------------------------------------
2019-12-28T18:12:41.9493221Z thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&rustc::mir::BorrowCheckResult>>::specialized_decode` not overridden', src/libserialize/serialize.rs:893:9
2019-12-28T18:12:41.9493492Z 
2019-12-28T18:12:41.9493575Z error: internal compiler error: unexpected panic
2019-12-28T18:12:41.9493604Z 
2019-12-28T18:12:41.9493673Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T18:12:41.9493673Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T18:12:41.9493732Z 
2019-12-28T18:12:41.9494130Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-28T18:12:41.9494255Z 
2019-12-28T18:12:41.9497587Z note: rustc 1.42.0-nightly (2ee72b593 2019-12-28) running on x86_64-unknown-linux-gnu
2019-12-28T18:12:41.9497808Z 
2019-12-28T18:12:41.9516834Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-28T18:12:41.9520851Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-12-28T18:12:41.9520851Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-12-28T18:12:41.9520928Z   left: `LLVMing`,
2019-12-28T18:12:41.9521367Z  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1443:21
2019-12-28T18:12:41.9521649Z error: internal compiler error: unexpected panic
2019-12-28T18:12:41.9521716Z 
2019-12-28T18:12:41.9521762Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T18:12:41.9521964Z 
2019-12-28T18:12:41.9521964Z 
2019-12-28T18:12:41.9522387Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-28T18:12:41.9522425Z 
2019-12-28T18:12:41.9522790Z note: rustc 1.42.0-nightly (2ee72b593 2019-12-28) running on x86_64-unknown-linux-gnu
2019-12-28T18:12:41.9522847Z 
2019-12-28T18:12:41.9523435Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-28T18:12:41.9523693Z 
2019-12-28T18:12:41.9523949Z ------------------------------------------
2019-12-28T18:12:41.9523980Z 
2019-12-28T18:12:41.9524003Z 
2019-12-28T18:12:41.9524003Z 
2019-12-28T18:12:41.9524430Z ---- [incremental] incremental/issue-49595/issue-49595.rs stdout ----
2019-12-28T18:12:41.9524582Z 
2019-12-28T18:12:41.9524917Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-12-28T18:12:41.9524989Z status: exit code: 101
2019-12-28T18:12:41.9526611Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-49595/issue-49595.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue-49595/issue-49595.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue-49595" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue-49595/auxiliary"
2019-12-28T18:12:41.9527831Z ------------------------------------------
2019-12-28T18:12:41.9528038Z 
2019-12-28T18:12:41.9528378Z ------------------------------------------
2019-12-28T18:12:41.9528428Z stderr:
2019-12-28T18:12:41.9528428Z stderr:
2019-12-28T18:12:41.9528834Z ------------------------------------------
2019-12-28T18:12:41.9529427Z thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&rustc::mir::BorrowCheckResult>>::specialized_decode` not overridden', src/libserialize/serialize.rs:893:9
2019-12-28T18:12:41.9529710Z 
2019-12-28T18:12:41.9529780Z error: internal compiler error: unexpected panic
2019-12-28T18:12:41.9529820Z 
2019-12-28T18:12:41.9529896Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T18:12:41.9529896Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T18:12:41.9529948Z 
2019-12-28T18:12:41.9530539Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-28T18:12:41.9530580Z 
2019-12-28T18:12:41.9531025Z note: rustc 1.42.0-nightly (2ee72b593 2019-12-28) running on x86_64-unknown-linux-gnu
2019-12-28T18:12:41.9531209Z 
2019-12-28T18:12:41.9531688Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-28T18:12:41.9531974Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-12-28T18:12:41.9531974Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-12-28T18:12:41.9532037Z   left: `LLVMing`,
2019-12-28T18:12:41.9532458Z  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1443:21
2019-12-28T18:12:41.9532714Z error: internal compiler error: unexpected panic
2019-12-28T18:12:41.9532744Z 
2019-12-28T18:12:41.9532789Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T18:12:41.9532819Z 
2019-12-28T18:12:41.9532819Z 
2019-12-28T18:12:41.9533298Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-28T18:12:41.9533456Z 
2019-12-28T18:12:41.9533838Z note: rustc 1.42.0-nightly (2ee72b593 2019-12-28) running on x86_64-unknown-linux-gnu
2019-12-28T18:12:41.9534011Z 
2019-12-28T18:12:41.9534682Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-28T18:12:41.9534990Z 
2019-12-28T18:12:41.9535294Z ------------------------------------------
2019-12-28T18:12:41.9535452Z 
2019-12-28T18:12:41.9535538Z 
---
2019-12-28T18:12:41.9537628Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-28T18:12:41.9537848Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-28T18:12:41.9537908Z 
2019-12-28T18:12:41.9537968Z 
2019-12-28T18:12:41.9539717Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-28T18:12:41.9539965Z 
2019-12-28T18:12:41.9540008Z 
2019-12-28T18:12:41.9540350Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-28T18:12:41.9540457Z Build completed unsuccessfully in 1:07:25
2019-12-28T18:12:41.9540457Z Build completed unsuccessfully in 1:07:25
2019-12-28T18:12:41.9568136Z == clock drift check ==
2019-12-28T18:12:41.9594905Z   local time: Sat Dec 28 18:12:41 UTC 2019
2019-12-28T18:12:42.0001178Z   network time: Sat, 28 Dec 2019 18:12:42 GMT
2019-12-28T18:12:42.0001263Z == end clock drift check ==
2019-12-28T18:12:48.1533916Z 
2019-12-28T18:12:48.1677099Z ##[error]Bash exited with code '1'.
2019-12-28T18:12:48.1730193Z ##[section]Starting: Checkout
2019-12-28T18:12:48.1732073Z ==============================================================================
2019-12-28T18:12:48.1732129Z Task         : Get sources
2019-12-28T18:12:48.1732195Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
