plain
2019-11-26T02:32:36.8968887Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T02:32:36.9151280Z ##[command]git config gc.auto 0
2019-11-26T02:32:36.9234800Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T02:32:36.9295056Z ##[command]git config --get-all http.proxy
2019-11-26T02:32:37.8624416Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66764/merge:refs/remotes/pull/66764/merge
---
2019-11-26T03:33:28.0976147Z .................................................................................................... 1600/9294
2019-11-26T03:33:33.0034501Z .................................................................................................... 1700/9294
2019-11-26T03:33:46.4769924Z ................................i................................................................... 1800/9294
2019-11-26T03:33:54.5644528Z .................................................................................................... 1900/9294
2019-11-26T03:34:08.2584376Z .................iiiii.............................................................................. 2000/9294
2019-11-26T03:34:18.6025674Z .................................................................................................... 2200/9294
2019-11-26T03:34:21.3390758Z .................................................................................................... 2300/9294
2019-11-26T03:34:26.7905762Z .................................................................................................... 2400/9294
2019-11-26T03:34:48.8775833Z .................................................................................................... 2500/9294
---
2019-11-26T03:37:35.9828969Z ..................i...............i................................................................. 4800/9294
2019-11-26T03:37:46.5114296Z .................................................................................................... 4900/9294
2019-11-26T03:37:52.6518462Z .................................................................................................... 5000/9294
2019-11-26T03:38:02.4601426Z .................................................................................................... 5100/9294
2019-11-26T03:38:08.9199338Z .......................ii.ii...........i............................................................ 5200/9294
2019-11-26T03:38:18.2454383Z .................................................................................................... 5400/9294
2019-11-26T03:38:29.4297971Z .................................................................................................... 5500/9294
2019-11-26T03:38:36.7609082Z .....i.............................................................................................. 5600/9294
2019-11-26T03:38:42.9206113Z .................................................................................................... 5700/9294
2019-11-26T03:38:42.9206113Z .................................................................................................... 5700/9294
2019-11-26T03:38:53.5159265Z ...........................................................................................ii...i..i 5800/9294
2019-11-26T03:39:06.3048191Z i...........i....................................................................................... 5900/9294
2019-11-26T03:39:24.9828871Z .................................................................................................... 6100/9294
2019-11-26T03:39:31.5762263Z .................................................................................................... 6200/9294
2019-11-26T03:39:31.5762263Z .................................................................................................... 6200/9294
2019-11-26T03:39:45.4467653Z ..............i..ii................................................................................. 6300/9294
2019-11-26T03:40:05.3847243Z ..................................................................................i................. 6500/9294
2019-11-26T03:40:07.7794470Z .................................................................................................... 6600/9294
2019-11-26T03:40:10.1120071Z .........................................................................i.......................... 6700/9294
2019-11-26T03:40:13.0570350Z .................................................................................................... 6800/9294
---
2019-11-26T03:45:12.8213685Z 
2019-11-26T03:45:12.8214705Z ---- [ui] ui/type/type-check-defaults.rs stdout ----
2019-11-26T03:45:12.8214984Z diff of stderr:
2019-11-26T03:45:12.8215157Z 
2019-11-26T03:45:12.8215726Z - error[E0277]: a value of type `i32` cannot be built from an iteratorover elements of type `i32`
2019-11-26T03:45:12.8215990Z + error[E0277]: a value of type `i32` cannot be built from an iterator over elements of type `i32`
2019-11-26T03:45:12.8216695Z 3    |
2019-11-26T03:45:12.8216695Z 3    |
2019-11-26T03:45:12.8216863Z 4 LL | struct Foo<T, U: FromIterator<T>>(T, U);
2019-11-26T03:45:12.8217632Z 5    | ---------------------------------------- required by `Foo`
2019-11-26T03:45:12.8217632Z 5    | ---------------------------------------- required by `Foo`
2019-11-26T03:45:12.8217859Z 6 LL | struct WellFormed<Z = Foo<i32, i32>>(Z);
2019-11-26T03:45:12.8218391Z -    |                   ^ a value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-26T03:45:12.8218620Z +    |                   ^ value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-26T03:45:12.8218975Z 9    = help: the trait `std::iter::FromIterator<i32>` is not implemented for `i32`
2019-11-26T03:45:12.8219373Z 10 
2019-11-26T03:45:12.8219509Z 
2019-11-26T03:45:12.8220047Z 15    | ---------------------------------------- required by `Foo`
2019-11-26T03:45:12.8220047Z 15    | ---------------------------------------- required by `Foo`
2019-11-26T03:45:12.8220262Z 16 ...
2019-11-26T03:45:12.8220427Z 17 LL | struct WellFormedNoBounds<Z:?Sized = Foo<i32, i32>>(Z);
2019-11-26T03:45:12.8221042Z -    |                           ^ a value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-26T03:45:12.8221347Z +    |                           ^ value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-26T03:45:12.8221713Z 20    = help: the trait `std::iter::FromIterator<i32>` is not implemented for `i32`
2019-11-26T03:45:12.8221869Z 21 
2019-11-26T03:45:12.8222020Z 
2019-11-26T03:45:12.8222150Z 
2019-11-26T03:45:12.8222150Z 
2019-11-26T03:45:12.8222306Z The actual stderr differed from the expected stderr.
2019-11-26T03:45:12.8222871Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check-defaults/type-check-defaults.stderr
2019-11-26T03:45:12.8223407Z To update references, rerun the tests and pass the `--bless` flag
2019-11-26T03:45:12.8223985Z To only update this specific test, also pass `--test-args type/type-check-defaults.rs`
2019-11-26T03:45:12.8224372Z error: 1 errors occurred comparing output.
2019-11-26T03:45:12.8224528Z status: exit code: 1
2019-11-26T03:45:12.8224528Z status: exit code: 1
2019-11-26T03:45:12.8225605Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check-defaults.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check-defaults" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check-defaults/auxiliary" "-A" "unused"
2019-11-26T03:45:12.8226352Z ------------------------------------------
2019-11-26T03:45:12.8226560Z 
2019-11-26T03:45:12.8227024Z ------------------------------------------
2019-11-26T03:45:12.8227282Z stderr:
2019-11-26T03:45:12.8227282Z stderr:
2019-11-26T03:45:12.8227780Z ------------------------------------------
2019-11-26T03:45:12.8228016Z error[E0277]: a value of type `i32` cannot be built from an iterator over elements of type `i32`
2019-11-26T03:45:12.8228485Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:6:19
2019-11-26T03:45:12.8228834Z    |
2019-11-26T03:45:12.8229070Z LL | struct Foo<T, U: FromIterator<T>>(T, U);
2019-11-26T03:45:12.8229569Z    | ---------------------------------------- required by `Foo`
2019-11-26T03:45:12.8229807Z LL | struct WellFormed<Z = Foo<i32, i32>>(Z);
2019-11-26T03:45:12.8229979Z    |                   ^ value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-26T03:45:12.8230313Z    = help: the trait `std::iter::FromIterator<i32>` is not implemented for `i32`
2019-11-26T03:45:12.8230467Z 
2019-11-26T03:45:12.8230712Z error[E0277]: a value of type `i32` cannot be built from an iterator over elements of type `i32`
2019-11-26T03:45:12.8231271Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:8:27
2019-11-26T03:45:12.8231271Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:8:27
2019-11-26T03:45:12.8231488Z    |
2019-11-26T03:45:12.8231673Z LL | struct Foo<T, U: FromIterator<T>>(T, U);
2019-11-26T03:45:12.8232174Z    | ---------------------------------------- required by `Foo`
2019-11-26T03:45:12.8232387Z ...
2019-11-26T03:45:12.8232559Z LL | struct WellFormedNoBounds<Z:?Sized = Foo<i32, i32>>(Z);
2019-11-26T03:45:12.8232761Z    |                           ^ value of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`
2019-11-26T03:45:12.8233086Z    = help: the trait `std::iter::FromIterator<i32>` is not implemented for `i32`
2019-11-26T03:45:12.8233243Z 
2019-11-26T03:45:12.8233427Z error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
2019-11-26T03:45:12.8234027Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:11:17
2019-11-26T03:45:12.8234027Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:11:17
2019-11-26T03:45:12.8234409Z    |
2019-11-26T03:45:12.8234572Z LL | struct Bounds<T:Copy=String>(T);
2019-11-26T03:45:12.8235280Z    | |               |
2019-11-26T03:45:12.8235469Z    | |               the trait `std::marker::Copy` is not implemented for `std::string::String`
2019-11-26T03:45:12.8235469Z    | |               the trait `std::marker::Copy` is not implemented for `std::string::String`
2019-11-26T03:45:12.8235670Z    | required by `Bounds`
2019-11-26T03:45:12.8235988Z error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
2019-11-26T03:45:12.8236493Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:14:42
2019-11-26T03:45:12.8236713Z    |
2019-11-26T03:45:12.8236713Z    |
2019-11-26T03:45:12.8236932Z LL | struct WhereClause<T=String>(T) where T: Copy;
2019-11-26T03:45:12.8237640Z    | |                                        |
2019-11-26T03:45:12.8237943Z    | |                                        the trait `std::marker::Copy` is not implemented for `std::string::String`
2019-11-26T03:45:12.8237943Z    | |                                        the trait `std::marker::Copy` is not implemented for `std::string::String`
2019-11-26T03:45:12.8238139Z    | required by `WhereClause`
2019-11-26T03:45:12.8238465Z error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
2019-11-26T03:45:12.8239021Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:17:20
2019-11-26T03:45:12.8239238Z    |
2019-11-26T03:45:12.8239238Z    |
2019-11-26T03:45:12.8239435Z LL | trait TraitBound<T:Copy=String> {}
2019-11-26T03:45:12.8240089Z    | |                  |
2019-11-26T03:45:12.8240280Z    | |                  the trait `std::marker::Copy` is not implemented for `std::string::String`
2019-11-26T03:45:12.8240448Z    | required by `TraitBound`
2019-11-26T03:45:12.8240602Z 
2019-11-26T03:45:12.8240602Z 
2019-11-26T03:45:12.8240828Z error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2019-11-26T03:45:12.8241357Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:21:1
2019-11-26T03:45:12.8241588Z    |
2019-11-26T03:45:12.8241763Z LL | trait Super<T: Copy> { }
2019-11-26T03:45:12.8242205Z    | -------------------- required by `Super`
2019-11-26T03:45:12.8242442Z LL | trait Base<T = String>: Super<T> { }
2019-11-26T03:45:12.8242885Z    | ^^^^^^^^^^^-^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-26T03:45:12.8243285Z    | |          help: consider restricting this bound: `T: std::marker::Copy`
2019-11-26T03:45:12.8243575Z    | the trait `std::marker::Copy` is not implemented for `T`
2019-11-26T03:45:12.8243807Z 
2019-11-26T03:45:12.8243984Z error[E0277]: cannot add `u8` to `i32`
2019-11-26T03:45:12.8243984Z error[E0277]: cannot add `u8` to `i32`
2019-11-26T03:45:12.8244510Z   --> /checkout/src/test/ui/type/type-check-defaults.rs:24:66
2019-11-26T03:45:12.8244928Z    |
2019-11-26T03:45:12.8245111Z LL | trait ProjectionPred<T:Iterator = IntoIter<i32>> where T::Item : Add<u8> {}
2019-11-26T03:45:12.8245881Z    | |                                                                |
2019-11-26T03:45:12.8245881Z    | |                                                                |
2019-11-26T03:45:12.8246436Z    | |                                                                no implementation for `i32 + u8`
2019-11-26T03:45:12.8246616Z    | required by `ProjectionPred`
2019-11-26T03:45:12.8246775Z    |
2019-11-26T03:45:12.8246961Z    = help: the trait `std::ops::Add<u8>` is not implemented for `i32`
2019-11-26T03:45:12.8247345Z error: aborting due to 7 previous errors
2019-11-26T03:45:12.8247481Z 
2019-11-26T03:45:12.8248003Z For more information about this error, try `rustc --explain E0277`.
2019-11-26T03:45:12.8248200Z 
---
2019-11-26T03:45:12.8249690Z diff of stderr:
2019-11-26T03:45:12.8249823Z 
2019-11-26T03:45:12.8250294Z 14   --> $DIR/type-dependent-def-issue-49241.rs:4:37
2019-11-26T03:45:12.8250764Z 15    |
2019-11-26T03:45:12.8250968Z 16 LL |     let s: [u32; l] = v.into_iter().collect();
2019-11-26T03:45:12.8251609Z -    |                                     ^^^^^^^ a value of type `[u32; _]` cannot be built from `std::iter::Iterator<Item={integer}>`
2019-11-26T03:45:12.8251927Z +    |                                     ^^^^^^^ value of type `[u32; _]` cannot be built from `std::iter::Iterator<Item={integer}>`
2019-11-26T03:45:12.8252470Z 18    |
2019-11-26T03:45:12.8252797Z 19    = help: the trait `std::iter::FromIterator<{integer}>` is not implemented for `[u32; _]`
2019-11-26T03:45:12.8253230Z 
2019-11-26T03:45:12.8253372Z 
2019-11-26T03:45:12.8253576Z The actual stderr differed from the expected stderr.
2019-11-26T03:45:12.8254314Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/type-dependent-def-issue-49241.stderr
2019-11-26T03:45:12.8254314Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/type-dependent-def-issue-49241.stderr
2019-11-26T03:45:12.8254920Z To update references, rerun the tests and pass the `--bless` flag
2019-11-26T03:45:12.8256365Z To only update this specific test, also pass `--test-args type/type-dependent-def-issue-49241.rs`
2019-11-26T03:45:12.8256489Z error: 1 errors occurred comparing output.
2019-11-26T03:45:12.8256539Z status: exit code: 1
2019-11-26T03:45:12.8256539Z status: exit code: 1
2019-11-26T03:45:12.8257516Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-dependent-def-issue-49241.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/auxiliary" "-A" "unused"
2019-11-26T03:45:12.8257944Z ------------------------------------------
2019-11-26T03:45:12.8258001Z 
2019-11-26T03:45:12.8258285Z ------------------------------------------
2019-11-26T03:45:12.8258337Z stderr:
2019-11-26T03:45:12.8258337Z stderr:
2019-11-26T03:45:12.8258627Z ------------------------------------------
2019-11-26T03:45:12.8258936Z error[E0435]: attempt to use a non-constant value in a constant
2019-11-26T03:45:12.8259261Z   --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:3:22
2019-11-26T03:45:12.8259336Z    |
2019-11-26T03:45:12.8259809Z LL |     const l: usize = v.count(); //~ ERROR attempt to use a non-constant value in a constant
2019-11-26T03:45:12.8260893Z    |                      ^ non-constant value
2019-11-26T03:45:12.8261072Z error[E0080]: evaluation of constant value failed
2019-11-26T03:45:12.8261477Z   --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:4:18
2019-11-26T03:45:12.8261534Z    |
2019-11-26T03:45:12.8261534Z    |
2019-11-26T03:45:12.8261608Z LL |     let s: [u32; l] = v.into_iter().collect();
2019-11-26T03:45:12.8261684Z    |                  ^ referenced constant has errors
2019-11-26T03:45:12.8261719Z 
2019-11-26T03:45:12.8261791Z error[E0277]: a value of type `[u32; _]` cannot be built from an iterator over elements of type `{integer}`
2019-11-26T03:45:12.8262132Z   --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:4:37
2019-11-26T03:45:12.8262187Z    |
2019-11-26T03:45:12.8262255Z LL |     let s: [u32; l] = v.into_iter().collect();
2019-11-26T03:45:12.8262330Z    |                                     ^^^^^^^ value of type `[u32; _]` cannot be built from `std::iter::Iterator<Item={integer}>`
2019-11-26T03:45:12.8262390Z    |
2019-11-26T03:45:12.8262467Z    = help: the trait `std::iter::FromIterator<{integer}>` is not implemented for `[u32; _]`
2019-11-26T03:45:12.8262553Z error: aborting due to 3 previous errors
2019-11-26T03:45:12.8262585Z 
2019-11-26T03:45:12.8262652Z Some errors have detailed explanations: E0080, E0277, E0435.
2019-11-26T03:45:12.8262977Z For more information about an error, try `rustc --explain E0080`.
---
2019-11-26T03:45:12.8269299Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-26T03:45:12.8270219Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-26T03:45:12.8286879Z 
2019-11-26T03:45:12.8286992Z 
2019-11-26T03:45:12.8289359Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-26T03:45:12.8289664Z 
2019-11-26T03:45:12.8289719Z 
2019-11-26T03:45:12.8295642Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-26T03:45:12.8295879Z Build completed unsuccessfully in 1:06:26
2019-11-26T03:45:12.8295879Z Build completed unsuccessfully in 1:06:26
2019-11-26T03:45:12.8351382Z == clock drift check ==
2019-11-26T03:45:12.8365669Z   local time: Tue Nov 26 03:45:12 UTC 2019
2019-11-26T03:45:12.9892366Z   network time: Tue, 26 Nov 2019 03:45:12 GMT
2019-11-26T03:45:12.9898101Z == end clock drift check ==
2019-11-26T03:45:13.6848938Z 
2019-11-26T03:45:13.7007108Z ##[error]Bash exited with code '1'.
2019-11-26T03:45:13.7043026Z ##[section]Starting: Checkout
2019-11-26T03:45:13.7045471Z ==============================================================================
2019-11-26T03:45:13.7045551Z Task         : Get sources
2019-11-26T03:45:13.7045604Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
