plain
2019-09-09T14:55:02.0239868Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T14:55:02.0448998Z ##[command]git config gc.auto 0
2019-09-09T14:55:02.0562526Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T14:55:02.0632326Z ##[command]git config --get-all http.proxy
2019-09-09T14:55:02.0789905Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64290/merge:refs/remotes/pull/64290/merge
---
2019-09-09T16:01:48.9732430Z .................................................................................................... 1500/9009
2019-09-09T16:01:55.4125984Z .................................................................................................... 1600/9009
2019-09-09T16:02:09.4512373Z ......................................................i...............i............................. 1700/9009
2019-09-09T16:02:18.2997712Z .................................................................................................... 1800/9009
2019-09-09T16:02:34.1394684Z .............................................iiiii.................................................. 1900/9009
2019-09-09T16:02:45.9759050Z .................................................................................................... 2100/9009
2019-09-09T16:02:48.6817607Z .................................................................................................... 2200/9009
2019-09-09T16:02:52.6093230Z .................................................................................................... 2300/9009
2019-09-09T16:03:01.3522884Z .................................................................................................... 2400/9009
---
2019-09-09T16:06:16.3323225Z ....................................i..............i................................................ 4700/9009
2019-09-09T16:06:28.8819593Z .................................................................................................... 4800/9009
2019-09-09T16:06:35.9414857Z .................................................................................................... 4900/9009
2019-09-09T16:06:47.5671707Z .................................................................................................... 5000/9009
2019-09-09T16:06:54.1672572Z .................ii.ii.............................................................................. 5100/9009
2019-09-09T16:07:05.6976051Z .................................................................................................... 5300/9009
2019-09-09T16:07:16.9225573Z ................................................................................i................... 5400/9009
2019-09-09T16:07:25.5806863Z .................................................................................................... 5500/9009
2019-09-09T16:07:32.1085236Z .................................................................................................... 5600/9009
2019-09-09T16:07:32.1085236Z .................................................................................................... 5600/9009
2019-09-09T16:07:43.7538465Z ..........................................................................ii...i..ii...........i.... 5700/9009
2019-09-09T16:08:11.0575361Z .................................................................................................... 5900/9009
2019-09-09T16:08:21.8894153Z .................................................................................................... 6000/9009
2019-09-09T16:08:21.8894153Z .................................................................................................... 6000/9009
2019-09-09T16:08:29.6571704Z ............................................................................i..ii................... 6100/9009
2019-09-09T16:09:02.2369850Z .................................................................................................... 6300/9009
2019-09-09T16:09:04.5827466Z ...................................i................................................................ 6400/9009
2019-09-09T16:09:06.9985380Z .................................................................................................... 6500/9009
2019-09-09T16:09:09.8460677Z .......i............................................................................................ 6600/9009
---
2019-09-09T16:14:19.6861184Z  finished in 20.814
2019-09-09T16:14:19.7068354Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T16:14:19.9266150Z 
2019-09-09T16:14:19.9268048Z running 150 tests
2019-09-09T16:14:23.6207824Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-09T16:14:25.7807726Z ..iiii..............i.........iii.i.......ii......
2019-09-09T16:14:25.7810912Z 
2019-09-09T16:14:25.7815411Z  finished in 6.075
2019-09-09T16:14:25.8015734Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T16:14:25.9896709Z 
---
2019-09-09T16:14:28.2435253Z  finished in 2.442
2019-09-09T16:14:28.2640239Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T16:14:28.4428486Z 
2019-09-09T16:14:28.4430807Z running 9 tests
2019-09-09T16:14:28.4432266Z iiiiiiiii
2019-09-09T16:14:28.4438091Z 
2019-09-09T16:14:28.4438172Z  finished in 0.179
2019-09-09T16:14:28.4637348Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T16:14:28.6684999Z 
---
2019-09-09T16:14:48.2128680Z  finished in 19.749
2019-09-09T16:14:48.2371228Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T16:14:48.4456423Z 
2019-09-09T16:14:48.4456709Z running 123 tests
2019-09-09T16:15:14.1092543Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-09T16:15:19.0917067Z i.i.i......iii.i.....ii
2019-09-09T16:15:19.0919381Z 
2019-09-09T16:15:19.0926231Z  finished in 30.856
2019-09-09T16:15:19.0938941Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T16:15:19.0939714Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-09T16:16:26.4601979Z 
2019-09-09T16:16:26.4602250Z ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
2019-09-09T16:16:26.4602320Z diff of stderr:
2019-09-09T16:16:26.4602352Z 
2019-09-09T16:16:26.4602837Z 43    = note: for more information, see ***/issues/27812
2019-09-09T16:16:26.4602921Z 44    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T16:16:26.4603217Z - error: aborting due to 6 previous errors
2019-09-09T16:16:26.4603283Z + error: aborting due to 5 previous errors
2019-09-09T16:16:26.4603325Z 47 
2019-09-09T16:16:26.4603547Z - Some errors have detailed explanations: E0601, E0658.
2019-09-09T16:16:26.4603547Z - Some errors have detailed explanations: E0601, E0658.
2019-09-09T16:16:26.4603790Z - For more information about an error, try `rustc --explain E0601`.
2019-09-09T16:16:26.4604245Z + For more information about this error, try `rustc --explain E0658`.
2019-09-09T16:16:26.4604310Z 50 
2019-09-09T16:16:26.4604336Z 
2019-09-09T16:16:26.4604360Z 
2019-09-09T16:16:26.4604442Z The actual stderr differed from the expected stderr.
2019-09-09T16:16:26.4604784Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2019-09-09T16:16:26.4605101Z To update references, rerun the tests and pass the `--bless` flag
2019-09-09T16:16:26.4605948Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2019-09-09T16:16:26.4606036Z error: 1 errors occurred comparing output.
2019-09-09T16:16:26.4606102Z status: exit code: 1
2019-09-09T16:16:26.4606102Z status: exit code: 1
2019-09-09T16:16:26.4606874Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2019-09-09T16:16:26.4607228Z ------------------------------------------
2019-09-09T16:16:26.4607263Z 
2019-09-09T16:16:26.4607505Z ------------------------------------------
2019-09-09T16:16:26.4607551Z stderr:
2019-09-09T16:16:26.4607551Z stderr:
2019-09-09T16:16:26.4607763Z ------------------------------------------
2019-09-09T16:16:26.4608157Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T16:16:26.4608859Z    |
2019-09-09T16:16:26.4608924Z LL | extern crate rustc_data_structures;
2019-09-09T16:16:26.4608970Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T16:16:26.4609011Z    |
2019-09-09T16:16:26.4609011Z    |
2019-09-09T16:16:26.4609510Z    = note: for more information, see ***/issues/27812
2019-09-09T16:16:26.4609573Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T16:16:26.4609608Z 
2019-09-09T16:16:26.4610466Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T16:16:26.4610802Z    |
2019-09-09T16:16:26.4610864Z LL | extern crate rustc;
2019-09-09T16:16:26.4610925Z    | ^^^^^^^^^^^^^^^^^^^
2019-09-09T16:16:26.4610966Z    |
2019-09-09T16:16:26.4610966Z    |
2019-09-09T16:16:26.4611306Z    = note: for more information, see ***/issues/27812
2019-09-09T16:16:26.4611365Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T16:16:26.4611400Z 
2019-09-09T16:16:26.4611812Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T16:16:26.4612120Z    |
2019-09-09T16:16:26.4612181Z LL | extern crate rustc_macros;
2019-09-09T16:16:26.4612227Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T16:16:26.4612269Z    |
2019-09-09T16:16:26.4612269Z    |
2019-09-09T16:16:26.4612567Z    = note: for more information, see ***/issues/27812
2019-09-09T16:16:26.4612631Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T16:16:26.4612677Z 
2019-09-09T16:16:26.4613403Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T16:16:26.4613768Z    |
2019-09-09T16:16:26.4613811Z LL | use rustc_macros::HashStable;
2019-09-09T16:16:26.4613875Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T16:16:26.4613916Z    |
2019-09-09T16:16:26.4613916Z    |
2019-09-09T16:16:26.4614196Z    = note: for more information, see ***/issues/27812
2019-09-09T16:16:26.4614275Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T16:16:26.4614310Z 
2019-09-09T16:16:26.4614700Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T16:16:26.4615635Z    |
2019-09-09T16:16:26.4615692Z LL | #[derive(HashStable)]
2019-09-09T16:16:26.4615759Z    |          ^^^^^^^^^^
2019-09-09T16:16:26.4615801Z    |
2019-09-09T16:16:26.4615801Z    |
2019-09-09T16:16:26.4616153Z    = note: for more information, see ***/issues/27812
2019-09-09T16:16:26.4616232Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T16:16:26.4616310Z error: aborting due to 5 previous errors
2019-09-09T16:16:26.4616339Z 
2019-09-09T16:16:26.4616614Z For more information about this error, try `rustc --explain E0658`.
2019-09-09T16:16:26.4616648Z 
---
2019-09-09T16:16:26.4617550Z test result: FAILED. 68 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-09T16:16:26.4617724Z 
2019-09-09T16:16:26.4622350Z 
2019-09-09T16:16:26.4622424Z 
2019-09-09T16:16:26.4624000Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T16:16:26.4624254Z 
2019-09-09T16:16:26.4624301Z 
2019-09-09T16:16:26.4624344Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T16:16:26.4624395Z Build completed unsuccessfully in 1:13:53
2019-09-09T16:16:26.4624395Z Build completed unsuccessfully in 1:13:53
2019-09-09T16:16:26.4628572Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-09T16:16:26.4628656Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-09T16:16:26.4674534Z == clock drift check ==
2019-09-09T16:16:26.4697261Z   local time: Mon Sep  9 16:16:26 UTC 2019
2019-09-09T16:16:26.5072056Z   network time: Mon, 09 Sep 2019 16:16:26 GMT
2019-09-09T16:16:26.5076214Z == end clock drift check ==
2019-09-09T16:16:27.1247800Z ##[error]Bash exited with code '1'.
2019-09-09T16:16:27.1286519Z ##[section]Starting: Checkout
2019-09-09T16:16:27.1288440Z ==============================================================================
2019-09-09T16:16:27.1288515Z Task         : Get sources
2019-09-09T16:16:27.1288564Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
