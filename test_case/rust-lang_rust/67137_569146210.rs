plain
2019-12-26T21:40:54.3678544Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-26T21:40:54.3983494Z ##[command]git config gc.auto 0
2019-12-26T21:40:54.4031977Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-26T21:40:54.4091064Z ##[command]git config --get-all http.proxy
2019-12-26T21:40:54.4248705Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67137/merge:refs/remotes/pull/67137/merge
---
2019-12-26T22:43:06.3149827Z .................................................................................................... 1600/9456
2019-12-26T22:43:11.2712825Z .................................................................................................... 1700/9456
2019-12-26T22:43:20.9835961Z .............................................................................................i...... 1800/9456
2019-12-26T22:43:29.3482501Z .................................................................................................... 1900/9456
2019-12-26T22:43:36.2764343Z ...............................................................................iiiii................ 2000/9456
2019-12-26T22:43:59.2428314Z .................................................................................................... 2200/9456
2019-12-26T22:44:01.6811144Z .................................................................................................... 2300/9456
2019-12-26T22:44:04.3223155Z .................................................................................................... 2400/9456
2019-12-26T22:44:10.6844695Z .................................................................................................... 2500/9456
---
2019-12-26T22:47:13.5607928Z ..........i...............i......................................................................... 4900/9456
2019-12-26T22:47:23.7358243Z .................................................................................................... 5000/9456
2019-12-26T22:47:28.8598051Z ......................................................i............................................. 5100/9456
2019-12-26T22:47:38.5495563Z .................................................................................................... 5200/9456
2019-12-26T22:47:45.0867463Z .....................ii.ii...........i.............................................................. 5300/9456
2019-12-26T22:47:54.4709142Z .................................................................................................... 5500/9456
2019-12-26T22:48:05.7404262Z .................................................................................................... 5600/9456
2019-12-26T22:48:13.2082880Z ...i................................................................................................ 5700/9456
2019-12-26T22:48:19.0569858Z .................................................................................................... 5800/9456
2019-12-26T22:48:19.0569858Z .................................................................................................... 5800/9456
2019-12-26T22:48:29.2767471Z ...........................................................................................ii...i..i 5900/9456
2019-12-26T22:48:41.9694438Z i...........i....................................................................................... 6000/9456
2019-12-26T22:48:59.9985281Z .................................................................................................... 6200/9456
2019-12-26T22:49:07.2744468Z .................................................................................................... 6300/9456
2019-12-26T22:49:07.2744468Z .................................................................................................... 6300/9456
2019-12-26T22:49:20.7533092Z ..................i..ii............................................................................. 6400/9456
2019-12-26T22:49:40.3359224Z ...............................................................................................i.... 6600/9456
2019-12-26T22:49:42.4380916Z .................................................................................................... 6700/9456
2019-12-26T22:49:44.6822909Z ...............................................................................................i.... 6800/9456
2019-12-26T22:49:47.3241219Z .................................................................................................... 6900/9456
---
2019-12-26T22:51:26.8458421Z .................................................................................................... 7500/9456
2019-12-26T22:51:31.7231240Z .................................................................................................... 7600/9456
2019-12-26T22:51:38.5159954Z .................................................................................................... 7700/9456
2019-12-26T22:51:49.0063491Z .................................................................................................... 7800/9456
2019-12-26T22:51:55.7723079Z ...........................iiii..................................................................... 7900/9456
2019-12-26T22:52:10.2963530Z .................................................................................................... 8100/9456
2019-12-26T22:52:19.7905614Z .................................................................................................... 8200/9456
2019-12-26T22:52:33.7278152Z .................................................................................................... 8300/9456
2019-12-26T22:52:41.1158499Z .................................................................................................... 8400/9456
---
2019-12-26T22:55:02.6834117Z ---- [mir-opt] mir-opt/retain-never-const.rs stdout ----
2019-12-26T22:55:02.6834154Z 
2019-12-26T22:55:02.6834221Z error: compilation failed!
2019-12-26T22:55:02.6834266Z status: exit code: 1
2019-12-26T22:55:02.6835491Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/retain-never-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retain-never-const" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retain-never-const/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--emit" "mir,link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retain-never-const/auxiliary"
2019-12-26T22:55:02.6835933Z ------------------------------------------
2019-12-26T22:55:02.6835971Z 
2019-12-26T22:55:02.6836203Z ------------------------------------------
2019-12-26T22:55:02.6836251Z stderr:
2019-12-26T22:55:02.6836251Z stderr:
2019-12-26T22:55:02.6836498Z ------------------------------------------
2019-12-26T22:55:02.6836571Z warning: due to multiple output types requested, the explicitly specified output file name will be adapted for each output type
2019-12-26T22:55:02.6836608Z 
2019-12-26T22:55:02.6836674Z warning: struct is never constructed: `PrintName`
2019-12-26T22:55:02.6836934Z   --> /checkout/src/test/mir-opt/retain-never-const.rs:10:8
2019-12-26T22:55:02.6836986Z    |
2019-12-26T22:55:02.6837028Z 10 | struct PrintName<T>(T);
2019-12-26T22:55:02.6837132Z    |
2019-12-26T22:55:02.6837176Z    = note: `#[warn(dead_code)]` on by default
2019-12-26T22:55:02.6837224Z 
2019-12-26T22:55:02.6837224Z 
2019-12-26T22:55:02.6837268Z warning: associated const is never used: `VOID`
2019-12-26T22:55:02.6837836Z    |
2019-12-26T22:55:02.6837836Z    |
2019-12-26T22:55:02.6837901Z 13 |     const VOID: ! = panic!();
2019-12-26T22:55:02.6837978Z 
2019-12-26T22:55:02.6837978Z 
2019-12-26T22:55:02.6838040Z warning: function is never used: `no_codegen`
2019-12-26T22:55:02.6838460Z    |
2019-12-26T22:55:02.6838514Z 16 | fn no_codegen<T>() {
2019-12-26T22:55:02.6838580Z    |    ^^^^^^^^^^
2019-12-26T22:55:02.6838608Z 
2019-12-26T22:55:02.6838608Z 
2019-12-26T22:55:02.6838650Z error: any use of this value will cause an error
2019-12-26T22:55:02.6838960Z   --> /checkout/src/test/mir-opt/retain-never-const.rs:13:21
2019-12-26T22:55:02.6839011Z    |
2019-12-26T22:55:02.6839053Z 13 |     const VOID: ! = panic!();
2019-12-26T22:55:02.6839922Z    |                     |
2019-12-26T22:55:02.6840342Z    |                     the evaluated program panicked at 'explicit panic', /checkout/src/test/mir-opt/retain-never-const.rs:13:21
2019-12-26T22:55:02.6840402Z    |
2019-12-26T22:55:02.6840471Z    = note: `#[deny(const_err)]` on by default
2019-12-26T22:55:02.6840471Z    = note: `#[deny(const_err)]` on by default
2019-12-26T22:55:02.6840829Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-26T22:55:02.6840880Z 
2019-12-26T22:55:02.6840965Z error[E0080]: erroneous constant used
2019-12-26T22:55:02.6841245Z   --> /checkout/src/test/mir-opt/retain-never-const.rs:17:13
2019-12-26T22:55:02.6841297Z    |
2019-12-26T22:55:02.6841363Z 17 |     let _ = PrintName::<T>::VOID;
2019-12-26T22:55:02.6841417Z    |             ^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-26T22:55:02.6841495Z error: aborting due to 2 previous errors
2019-12-26T22:55:02.6841543Z 
2019-12-26T22:55:02.6841819Z For more information about this error, try `rustc --explain E0080`.
2019-12-26T22:55:02.6841869Z 
---
2019-12-26T22:55:02.6853157Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-26T22:55:02.6853221Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-26T22:55:02.6853254Z 
2019-12-26T22:55:02.6853280Z 
2019-12-26T22:55:02.6854979Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-26T22:55:02.6855377Z 
2019-12-26T22:55:02.6855426Z 
2019-12-26T22:55:02.6860388Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-26T22:55:02.6860463Z Build completed unsuccessfully in 1:07:21
2019-12-26T22:55:02.6860463Z Build completed unsuccessfully in 1:07:21
2019-12-26T22:55:02.6916439Z == clock drift check ==
2019-12-26T22:55:02.6930151Z   local time: Thu Dec 26 22:55:02 UTC 2019
2019-12-26T22:55:02.9788050Z   network time: Thu, 26 Dec 2019 22:55:02 GMT
2019-12-26T22:55:02.9793474Z == end clock drift check ==
2019-12-26T22:55:08.7287285Z 
2019-12-26T22:55:08.7392571Z ##[error]Bash exited with code '1'.
2019-12-26T22:55:08.7482209Z ##[section]Starting: Checkout
2019-12-26T22:55:08.7484938Z ==============================================================================
2019-12-26T22:55:08.7485019Z Task         : Get sources
2019-12-26T22:55:08.7485060Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
