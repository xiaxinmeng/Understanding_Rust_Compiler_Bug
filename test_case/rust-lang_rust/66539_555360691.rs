plain
2019-11-19T05:03:22.5927058Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-19T05:03:22.6107906Z ##[command]git config gc.auto 0
2019-11-19T05:03:22.6183238Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-19T05:03:22.6231682Z ##[command]git config --get-all http.proxy
2019-11-19T05:03:22.6375273Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66539/merge:refs/remotes/pull/66539/merge
---
2019-11-19T06:01:10.6590741Z .................................................................................................... 1500/9253
2019-11-19T06:01:16.6026780Z .................................................................................................... 1600/9253
2019-11-19T06:01:24.6274016Z .................................................................................................... 1700/9253
2019-11-19T06:01:32.7653838Z ........i........................................................................................... 1800/9253
2019-11-19T06:01:38.7365762Z .............................................................................................iiiii.. 1900/9253
2019-11-19T06:01:58.4225830Z .................................................................................................... 2100/9253
2019-11-19T06:02:00.4829286Z .................................................................................................... 2200/9253
2019-11-19T06:02:02.6243741Z .................................................................................................... 2300/9253
2019-11-19T06:02:08.2978215Z .................................................................................................... 2400/9253
---
2019-11-19T06:05:31.3365471Z .................................................................................................... 5400/9253
2019-11-19T06:05:40.8198903Z ...............................................................................i.................... 5500/9253
2019-11-19T06:05:47.9497757Z .................................................................................................... 5600/9253
2019-11-19T06:05:53.9484845Z .................................................................................................... 5700/9253
2019-11-19T06:06:03.2705611Z .................................................................ii...i..ii...........i............. 5800/9253
2019-11-19T06:06:23.4428618Z .................................................................................................... 6000/9253
2019-11-19T06:06:30.9121286Z .................................................................................................... 6100/9253
2019-11-19T06:06:30.9121286Z .................................................................................................... 6100/9253
2019-11-19T06:06:35.9335755Z ......................................................................................i..ii......... 6200/9253
2019-11-19T06:07:00.8445867Z .................................................................................................... 6400/9253
2019-11-19T06:07:05.3881814Z ......................................................i............................................. 6500/9253
2019-11-19T06:07:07.2990673Z .................................................................................................... 6600/9253
2019-11-19T06:07:09.3958315Z ...........................................i........................................................ 6700/9253
---
2019-11-19T06:11:57.5333172Z  finished in 5.218
2019-11-19T06:11:57.5493878Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-19T06:11:57.7186446Z 
2019-11-19T06:11:57.7187546Z running 156 tests
2019-11-19T06:12:00.3513944Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-19T06:12:02.0532396Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-19T06:12:02.0534350Z 
2019-11-19T06:12:02.0537467Z  finished in 4.504
2019-11-19T06:12:02.0720488Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-19T06:12:02.2291437Z 
---
2019-11-19T06:12:04.0169602Z  finished in 1.945
2019-11-19T06:12:04.0342559Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-19T06:12:04.1815843Z 
2019-11-19T06:12:04.1815999Z running 9 tests
2019-11-19T06:12:04.1816566Z iiiiiiiii
2019-11-19T06:12:04.1816874Z 
2019-11-19T06:12:04.1816911Z  finished in 0.146
2019-11-19T06:12:04.1956294Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-19T06:12:04.3514418Z 
---
2019-11-19T06:12:21.9416084Z  finished in 17.745
2019-11-19T06:12:21.9603515Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-19T06:12:22.1267350Z 
2019-11-19T06:12:22.1267566Z running 123 tests
2019-11-19T06:12:44.8565578Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-19T06:12:49.1697680Z i.i.i......iii.i.....ii
2019-11-19T06:12:49.1698638Z 
2019-11-19T06:12:49.1701393Z  finished in 27.209
2019-11-19T06:12:49.1710214Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-19T06:12:49.1710901Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-19T06:24:25.4987787Z 
2019-11-19T06:24:25.4992983Z    Doc-tests core
2019-11-19T06:24:30.3216926Z 
2019-11-19T06:24:30.3217924Z running 2421 tests
2019-11-19T06:24:40.3840911Z ......iiiii......................................................................................... 100/2421
2019-11-19T06:24:50.0730852Z .................................................................................ii................. 200/2421
2019-11-19T06:25:17.3908868Z ...i................................................................................................ 400/2421
2019-11-19T06:25:17.3908868Z ...i................................................................................................ 400/2421
2019-11-19T06:25:30.2480266Z ...................................................i..i..................iiii....................... 500/2421
2019-11-19T06:25:54.7683998Z .................................................................................................... 700/2421
2019-11-19T06:26:05.0633701Z .................................................................................................... 800/2421
2019-11-19T06:26:15.2490552Z .................................................................................................... 900/2421
2019-11-19T06:26:25.2914052Z .................................................................................................... 1000/2421
---
2019-11-19T06:30:16.7685587Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
2019-11-19T06:30:16.7698426Z ..thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-11-19T06:30:16.7698515Z   left: `1`,
2019-11-19T06:30:16.7698839Z  right: `2`', src/libstd/sync/mutex.rs:653:13
2019-11-19T06:30:16.7752100Z ..........thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:791:13
2019-11-19T06:30:16.7762054Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock.', src/libstd/sync/rwlock.rs:768:13
2019-11-19T06:30:16.7769436Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:705:13
2019-11-19T06:30:16.7772528Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:635:13
2019-11-19T06:30:16.7776459Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:646:13
2019-11-19T06:30:16.7781146Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:611:13
2019-11-19T06:30:16.7785629Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:623:13
2019-11-19T06:30:18.8110938Z ..........................thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', src/libstd/sys_common/remutex.rs:233:13
2019-11-19T06:30:18.8231497Z ...................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1544:13
2019-11-19T06:30:19.4315834Z ............thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1676:13
2019-11-19T06:30:19.4316189Z .thread '<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1662:13
2019-11-19T06:30:19.4317122Z thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1648:13
---
2019-11-19T06:30:25.9747805Z 
2019-11-19T06:30:25.9752305Z running 999 tests
2019-11-19T06:30:45.8446760Z i................................................................................................... 100/999
2019-11-19T06:30:56.4950009Z .................................................................................................... 200/999
2019-11-19T06:31:03.9307596Z ..................i.ii.....i......i...i......i...................................................... 300/999
2019-11-19T06:31:08.9353956Z .................................................................................................... 400/999
2019-11-19T06:31:15.9438083Z ..........................................i..i.................................ii................... 500/999
2019-11-19T06:31:29.1953617Z .................................................................................................... 700/999
2019-11-19T06:31:29.1953617Z .................................................................................................... 700/999
2019-11-19T06:31:36.1508794Z .........................iiii....................................................................... 800/999
2019-11-19T06:31:50.8375534Z .................................................................................................... 900/999
2019-11-19T06:31:57.9506238Z ...............................................iiii................................................
2019-11-19T06:31:57.9510809Z 
2019-11-19T06:31:57.9625011Z  finished in 184.672
2019-11-19T06:31:57.9641470Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-19T06:31:58.1730363Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-11-19T06:48:57.7611117Z  finished in 37.770
2019-11-19T06:48:57.7935906Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-19T06:48:57.9827281Z 
2019-11-19T06:48:57.9827714Z running 204 tests
2019-11-19T06:49:26.5862511Z ....................i...ii...................................................................i...... 100/204
2019-11-19T06:50:03.2397370Z .................................iiii.......i............iiii.iii................................... 200/204
2019-11-19T06:50:05.9841842Z .i..
2019-11-19T06:50:05.9842267Z 
2019-11-19T06:50:05.9848058Z  finished in 68.190
2019-11-19T06:50:05.9854713Z doc tests for: /checkout/src/doc/rustdoc/src/command-line-arguments.md
2019-11-19T06:50:05.9943080Z doc tests for: /checkout/src/doc/rustdoc/src/documentation-tests.md
---
2019-11-19T06:50:31.3959732Z 15               found type `i32`
2019-11-19T06:50:31.3959769Z 
2019-11-19T06:50:31.3959788Z 
2019-11-19T06:50:31.3959822Z The actual stdout differed from the expected stdout.
2019-11-19T06:50:31.3960298Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-missing-codes/failed-doctest-missing-codes.stdout
2019-11-19T06:50:31.3960499Z To update references, rerun the tests and pass the `--bless` flag
2019-11-19T06:50:31.3960702Z To only update this specific test, also pass `--test-args failed-doctest-missing-codes.rs`
2019-11-19T06:50:31.3960776Z error: 1 errors occurred comparing output.
2019-11-19T06:50:31.3960808Z status: exit code: 101
2019-11-19T06:50:31.3960808Z status: exit code: 101
2019-11-19T06:50:31.3961395Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-missing-codes" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-missing-codes/auxiliary"
2019-11-19T06:50:31.3961648Z ------------------------------------------
2019-11-19T06:50:31.3961673Z 
2019-11-19T06:50:31.3961704Z running 1 test
2019-11-19T06:50:31.3961905Z test /checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs - Foo (line 8) ... FAILED
---
2019-11-19T06:50:31.3962907Z 
2019-11-19T06:50:31.3962940Z error: aborting due to previous error
2019-11-19T06:50:31.3962961Z 
2019-11-19T06:50:31.3963144Z For more information about this error, try `rustc --explain E0308`.
2019-11-19T06:50:31.3963199Z Some expected error codes were not found: ["E0004"]
2019-11-19T06:50:31.3963250Z failures:
2019-11-19T06:50:31.3963461Z     /checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs - Foo (line 8)
2019-11-19T06:50:31.3963488Z 
2019-11-19T06:50:31.3963523Z test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
---
2019-11-19T06:50:31.3965301Z test result: FAILED. 31 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-19T06:50:31.3965352Z 
2019-11-19T06:50:31.3965378Z 
2019-11-19T06:50:31.3965403Z 
2019-11-19T06:50:31.3967176Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-19T06:50:31.3967494Z 
2019-11-19T06:50:31.3967525Z 
2019-11-19T06:50:31.3978638Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-19T06:50:31.4028344Z Build completed unsuccessfully in 1:40:32
2019-11-19T06:50:31.4028344Z Build completed unsuccessfully in 1:40:32
2019-11-19T06:50:31.4033943Z == clock drift check ==
2019-11-19T06:50:31.4052562Z   local time: Tue Nov 19 06:50:31 UTC 2019
2019-11-19T06:50:31.6826940Z   network time: Tue, 19 Nov 2019 06:50:31 GMT
2019-11-19T06:50:31.6829872Z == end clock drift check ==
2019-11-19T06:50:32.7213665Z 
2019-11-19T06:50:32.7325868Z ##[error]Bash exited with code '1'.
2019-11-19T06:50:32.7377452Z ##[section]Starting: Checkout
2019-11-19T06:50:32.7379984Z ==============================================================================
2019-11-19T06:50:32.7380031Z Task         : Get sources
2019-11-19T06:50:32.7380085Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
