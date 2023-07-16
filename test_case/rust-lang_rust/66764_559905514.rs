plain
2019-11-30T01:13:33.7414408Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T01:13:33.7624073Z ##[command]git config gc.auto 0
2019-11-30T01:13:33.7689389Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T01:13:33.7741999Z ##[command]git config --get-all http.proxy
2019-11-30T01:13:33.7884087Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66764/merge:refs/remotes/pull/66764/merge
---
2019-11-30T02:10:52.9862563Z .................................................................................................... 1600/9312
2019-11-30T02:10:57.7023345Z .................................................................................................... 1700/9312
2019-11-30T02:11:09.9977459Z ........................................i........................................................... 1800/9312
2019-11-30T02:11:17.6964666Z .................................................................................................... 1900/9312
2019-11-30T02:11:30.8950721Z .........................iiiii...................................................................... 2000/9312
2019-11-30T02:11:40.5193020Z .................................................................................................... 2200/9312
2019-11-30T02:11:42.9162811Z .................................................................................................... 2300/9312
2019-11-30T02:11:47.3704476Z .................................................................................................... 2400/9312
2019-11-30T02:12:09.1946699Z .................................................................................................... 2500/9312
---
2019-11-30T02:14:52.4494979Z ...........................i...............i........................................................ 4800/9312
2019-11-30T02:15:03.1718295Z .................................................................................................... 4900/9312
2019-11-30T02:15:09.2859820Z .................................................................................................... 5000/9312
2019-11-30T02:15:17.4316259Z .................................................................................................... 5100/9312
2019-11-30T02:15:25.2716579Z ................................ii.ii...........i................................................... 5200/9312
2019-11-30T02:15:34.9524747Z .................................................................................................... 5400/9312
2019-11-30T02:15:45.2980165Z .................................................................................................... 5500/9312
2019-11-30T02:15:52.4480336Z ..............i..................................................................................... 5600/9312
2019-11-30T02:15:58.9687978Z .................................................................................................... 5700/9312
2019-11-30T02:15:58.9687978Z .................................................................................................... 5700/9312
2019-11-30T02:16:10.3322391Z .................................................................................................... 5800/9312
2019-11-30T02:16:22.5383439Z ii...i..ii...........i.............................................................................. 5900/9312
2019-11-30T02:16:41.1910871Z .................................................................................................... 6100/9312
2019-11-30T02:16:48.3672194Z .................................................................................................... 6200/9312
2019-11-30T02:16:48.3672194Z .................................................................................................... 6200/9312
2019-11-30T02:17:02.3568346Z .......................i..ii........................................................................ 6300/9312
2019-11-30T02:17:22.3422412Z ...........................................................................................i........ 6500/9312
2019-11-30T02:17:24.7046899Z .................................................................................................... 6600/9312
2019-11-30T02:17:26.9763286Z ..................................................................................i................. 6700/9312
2019-11-30T02:17:29.7012800Z .................................................................................................... 6800/9312
---
2019-11-30T02:22:21.7743281Z 
2019-11-30T02:22:21.7744917Z ---- [ui] ui/type/type-check-defaults.rs stdout ----
2019-11-30T02:22:21.7744996Z diff of stderr:
2019-11-30T02:22:21.7745081Z 
2019-11-30T02:22:21.7747065Z 4 LL | struct Foo<T, U: FromIterator<T>>(T, U);
2019-11-30T02:22:21.7747556Z 5    | ---------------------------------------- required by `Foo`
2019-11-30T02:22:21.7747614Z 6 LL | struct WellFormed<Z = Foo<i32, i32>>(Z);
2019-11-30T02:22:21.7747924Z -    |                   ^ a value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-30T02:22:21.7747987Z +    |                   ^ value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-30T02:22:21.7748103Z 9    = help: the trait `std::iter::FromIterator<i32>` is not implemented for `i32`
2019-11-30T02:22:21.7748151Z 10 
2019-11-30T02:22:21.7748179Z 
2019-11-30T02:22:21.7748429Z 15    | ---------------------------------------- required by `Foo`
2019-11-30T02:22:21.7748429Z 15    | ---------------------------------------- required by `Foo`
2019-11-30T02:22:21.7748494Z 16 ...
2019-11-30T02:22:21.7748541Z 17 LL | struct WellFormedNoBounds<Z:?Sized = Foo<i32, i32>>(Z);
2019-11-30T02:22:21.7748847Z -    |                           ^ a value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-30T02:22:21.7748935Z +    |                           ^ value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-30T02:22:21.7749031Z 20    = help: the trait `std::iter::FromIterator<i32>` is not implemented for `i32`
2019-11-30T02:22:21.7749095Z 21 
2019-11-30T02:22:21.7749122Z 
2019-11-30T02:22:21.7749148Z 
2019-11-30T02:22:21.7749148Z 
2019-11-30T02:22:21.7749193Z The actual stderr differed from the expected stderr.
2019-11-30T02:22:21.7749681Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check-defaults/type-check-defaults.stderr
2019-11-30T02:22:21.7749909Z To update references, rerun the tests and pass the `--bless` flag
2019-11-30T02:22:21.7750330Z To only update this specific test, also pass `--test-args type/type-check-defaults.rs`
2019-11-30T02:22:21.7750416Z error: 1 errors occurred comparing output.
2019-11-30T02:22:21.7750468Z status: exit code: 1
2019-11-30T02:22:21.7750468Z status: exit code: 1
2019-11-30T02:22:21.7751453Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check-defaults.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check-defaults" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check-defaults/auxiliary" "-A" "unused"
2019-11-30T02:22:21.7751770Z ------------------------------------------
2019-11-30T02:22:21.7751800Z 
2019-11-30T02:22:21.7752159Z ------------------------------------------
2019-11-30T02:22:21.7752217Z stderr:
2019-11-30T02:22:21.7752217Z stderr:
2019-11-30T02:22:21.7752405Z ------------------------------------------
2019-11-30T02:22:21.7752633Z error[E0277]: a value of type `i32` cannot be built from an iterator over elements of type `i32`
2019-11-30T02:22:21.7752920Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:6:19
2019-11-30T02:22:21.7753061Z    |
2019-11-30T02:22:21.7753099Z LL | struct Foo<T, U: FromIterator<T>>(T, U);
2019-11-30T02:22:21.7753354Z    | ---------------------------------------- required by `Foo`
2019-11-30T02:22:21.7753399Z LL | struct WellFormed<Z = Foo<i32, i32>>(Z);
2019-11-30T02:22:21.7753445Z    |                   ^ value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-30T02:22:21.7753547Z    = help: the trait `std::iter::FromIterator<i32>` is not implemented for `i32`
2019-11-30T02:22:21.7753576Z 
2019-11-30T02:22:21.7753618Z error[E0277]: a value of type `i32` cannot be built from an iterator over elements of type `i32`
2019-11-30T02:22:21.7753854Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:8:27
2019-11-30T02:22:21.7753854Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:8:27
2019-11-30T02:22:21.7753895Z    |
2019-11-30T02:22:21.7753940Z LL | struct Foo<T, U: FromIterator<T>>(T, U);
2019-11-30T02:22:21.7754172Z    | ---------------------------------------- required by `Foo`
2019-11-30T02:22:21.7754220Z ...
2019-11-30T02:22:21.7754259Z LL | struct WellFormedNoBounds<Z:?Sized = Foo<i32, i32>>(Z);
2019-11-30T02:22:21.7754325Z    |                           ^ value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-30T02:22:21.7754407Z    = help: the trait `std::iter::FromIterator<i32>` is not implemented for `i32`
2019-11-30T02:22:21.7754453Z 
2019-11-30T02:22:21.7754495Z error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
2019-11-30T02:22:21.7754715Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:11:17
2019-11-30T02:22:21.7754715Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:11:17
2019-11-30T02:22:21.7754755Z    |
2019-11-30T02:22:21.7754810Z LL | struct Bounds<T:Copy=String>(T);
2019-11-30T02:22:21.7755039Z    | |               |
2019-11-30T02:22:21.7755109Z    | |               the trait `std::marker::Copy` is not implemented for `std::string::String`
2019-11-30T02:22:21.7755109Z    | |               the trait `std::marker::Copy` is not implemented for `std::string::String`
2019-11-30T02:22:21.7755159Z    | required by `Bounds`
2019-11-30T02:22:21.7755226Z error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
2019-11-30T02:22:21.7755818Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:14:42
2019-11-30T02:22:21.7755876Z    |
2019-11-30T02:22:21.7755876Z    |
2019-11-30T02:22:21.7755921Z LL | struct WhereClause<T=String>(T) where T: Copy;
2019-11-30T02:22:21.7756257Z    | |                                        |
2019-11-30T02:22:21.7756314Z    | |                                        the trait `std::marker::Copy` is not implemented for `std::string::String`
2019-11-30T02:22:21.7756314Z    | |                                        the trait `std::marker::Copy` is not implemented for `std::string::String`
2019-11-30T02:22:21.7756384Z    | required by `WhereClause`
2019-11-30T02:22:21.7756462Z error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
2019-11-30T02:22:21.7756739Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:17:20
2019-11-30T02:22:21.7756796Z    |
2019-11-30T02:22:21.7756796Z    |
2019-11-30T02:22:21.7756838Z LL | trait TraitBound<T:Copy=String> {}
2019-11-30T02:22:21.7757123Z    | |                  |
2019-11-30T02:22:21.7757175Z    | |                  the trait `std::marker::Copy` is not implemented for `std::string::String`
2019-11-30T02:22:21.7757224Z    | required by `TraitBound`
2019-11-30T02:22:21.7757271Z 
2019-11-30T02:22:21.7757271Z 
2019-11-30T02:22:21.7757317Z error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2019-11-30T02:22:21.7757564Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:21:1
2019-11-30T02:22:21.7757630Z    |
2019-11-30T02:22:21.7757671Z LL | trait Super<T: Copy> { }
2019-11-30T02:22:21.7757894Z    | -------------------- required by `Super`
2019-11-30T02:22:21.7757944Z LL | trait Base<T = String>: Super<T> { }
2019-11-30T02:22:21.7758181Z    | ^^^^^^^^^^^-^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-30T02:22:21.7758393Z    | |          help: consider restricting this bound: `T: std::marker::Copy`
2019-11-30T02:22:21.7758529Z    | the trait `std::marker::Copy` is not implemented for `T`
2019-11-30T02:22:21.7758560Z 
2019-11-30T02:22:21.7758603Z error[E0277]: cannot add `u8` to `i32`
2019-11-30T02:22:21.7758603Z error[E0277]: cannot add `u8` to `i32`
2019-11-30T02:22:21.7758891Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:24:66
2019-11-30T02:22:21.7758938Z    |
2019-11-30T02:22:21.7759149Z LL | trait ProjectionPred<T:Iterator = IntoIter<i32>> where T::Item : Add<u8> {}
2019-11-30T02:22:21.7759616Z    | |                                                                |
2019-11-30T02:22:21.7759616Z    | |                                                                |
2019-11-30T02:22:21.7759664Z    | |                                                                no implementation for `i32 + u8`
2019-11-30T02:22:21.7759709Z    | required by `ProjectionPred`
2019-11-30T02:22:21.7759762Z    |
2019-11-30T02:22:21.7759810Z    = help: the trait `std::ops::Add<u8>` is not implemented for `i32`
2019-11-30T02:22:21.7759902Z error: aborting due to 7 previous errors
2019-11-30T02:22:21.7759927Z 
2019-11-30T02:22:21.7760147Z For more information about this error, try `rustc --explain E0277`.
2019-11-30T02:22:21.7760176Z 
---
2019-11-30T02:22:21.7761250Z diff of stderr:
2019-11-30T02:22:21.7761276Z 
2019-11-30T02:22:21.7761493Z 14   --> $DIR/type-dependent-def-issue-49241.rs:4:37
2019-11-30T02:22:21.7761534Z 15    |
2019-11-30T02:22:21.7761592Z 16 LL |     let s: [u32; l] = v.into_iter().collect();
2019-11-30T02:22:21.7761871Z -    |                                     ^^^^^^^ a value of type `[u32; _]` cannot be built from `std::iter::Iterator<Item={integer}>`
2019-11-30T02:22:21.7761941Z +    |                                     ^^^^^^^ value of type `[u32; _]` cannot be built from `std::iter::Iterator<Item={integer}>`
2019-11-30T02:22:21.7762014Z 18    |
2019-11-30T02:22:21.7762058Z 19    = help: the trait `std::iter::FromIterator<{integer}>` is not implemented for `[u32; _]`
2019-11-30T02:22:21.7762140Z 
2019-11-30T02:22:21.7762162Z 
2019-11-30T02:22:21.7762200Z The actual stderr differed from the expected stderr.
2019-11-30T02:22:21.7762502Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/type-dependent-def-issue-49241.stderr
2019-11-30T02:22:21.7762502Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/type-dependent-def-issue-49241.stderr
2019-11-30T02:22:21.7762743Z To update references, rerun the tests and pass the `--bless` flag
2019-11-30T02:22:21.7762988Z To only update this specific test, also pass `--test-args type/type-dependent-def-issue-49241.rs`
2019-11-30T02:22:21.7763238Z error: 1 errors occurred comparing output.
2019-11-30T02:22:21.7763282Z status: exit code: 1
2019-11-30T02:22:21.7763282Z status: exit code: 1
2019-11-30T02:22:21.7763964Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-dependent-def-issue-49241.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/auxiliary" "-A" "unused"
2019-11-30T02:22:21.7764258Z ------------------------------------------
2019-11-30T02:22:21.7764304Z 
2019-11-30T02:22:21.7764489Z ------------------------------------------
2019-11-30T02:22:21.7764527Z stderr:
2019-11-30T02:22:21.7764527Z stderr:
2019-11-30T02:22:21.7764707Z ------------------------------------------
2019-11-30T02:22:21.7765033Z error[E0435]: attempt to use a non-constant value in a constant
2019-11-30T02:22:21.7765285Z   --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:3:22
2019-11-30T02:22:21.7765397Z    |
2019-11-30T02:22:21.7766221Z LL |     const l: usize = v.count(); //~ ERROR attempt to use a non-constant value in a constant
2019-11-30T02:22:21.7766480Z    |                      ^ non-constant value
2019-11-30T02:22:21.7766793Z error[E0080]: evaluation of constant value failed
2019-11-30T02:22:21.7767081Z   --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:4:18
2019-11-30T02:22:21.7767127Z    |
2019-11-30T02:22:21.7767127Z    |
2019-11-30T02:22:21.7767195Z LL |     let s: [u32; l] = v.into_iter().collect();
2019-11-30T02:22:21.7767243Z    |                  ^ referenced constant has errors
2019-11-30T02:22:21.7767274Z 
2019-11-30T02:22:21.7767324Z error[E0277]: a value of type `[u32; _]` cannot be built from an iterator over elements of type `{integer}`
2019-11-30T02:22:21.7767619Z   --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:4:37
2019-11-30T02:22:21.7767667Z    |
2019-11-30T02:22:21.7767711Z LL |     let s: [u32; l] = v.into_iter().collect();
2019-11-30T02:22:21.7767798Z    |                                     ^^^^^^^ value of type `[u32; _]` cannot be built from `std::iter::Iterator<Item={integer}>`
2019-11-30T02:22:21.7767850Z    |
2019-11-30T02:22:21.7767901Z    = help: the trait `std::iter::FromIterator<{integer}>` is not implemented for `[u32; _]`
2019-11-30T02:22:21.7767995Z error: aborting due to 3 previous errors
2019-11-30T02:22:21.7768025Z 
2019-11-30T02:22:21.7768069Z Some errors have detailed explanations: E0080, E0277, E0435.
2019-11-30T02:22:21.7768346Z For more information about an error, try `rustc --explain E0080`.
---
2019-11-30T02:22:21.7793759Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-30T02:22:21.7793828Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-30T02:22:21.7805320Z 
2019-11-30T02:22:21.7806050Z 
2019-11-30T02:22:21.7816551Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-30T02:22:21.7816847Z 
2019-11-30T02:22:21.7816897Z 
2019-11-30T02:22:21.7824191Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-30T02:22:21.7824814Z Build completed unsuccessfully in 1:03:02
2019-11-30T02:22:21.7824814Z Build completed unsuccessfully in 1:03:02
2019-11-30T02:22:21.7881333Z == clock drift check ==
2019-11-30T02:22:21.7902099Z   local time: Sat Nov 30 02:22:21 UTC 2019
2019-11-30T02:22:22.0695217Z   network time: Sat, 30 Nov 2019 02:22:22 GMT
2019-11-30T02:22:22.0695538Z == end clock drift check ==
2019-11-30T02:22:22.9126889Z 
2019-11-30T02:22:22.9233700Z ##[error]Bash exited with code '1'.
2019-11-30T02:22:22.9271909Z ##[section]Starting: Checkout
2019-11-30T02:22:22.9274113Z ==============================================================================
2019-11-30T02:22:22.9274159Z Task         : Get sources
2019-11-30T02:22:22.9274220Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
