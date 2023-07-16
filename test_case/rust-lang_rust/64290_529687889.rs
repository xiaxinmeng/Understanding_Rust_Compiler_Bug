plain
2019-09-09T20:55:45.3408640Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T20:55:45.3695604Z ##[command]git config gc.auto 0
2019-09-09T20:55:45.3748396Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T20:55:45.3821487Z ##[command]git config --get-all http.proxy
2019-09-09T20:55:45.3978993Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64290/merge:refs/remotes/pull/64290/merge
---
2019-09-09T21:59:45.0551774Z .................................................................................................... 1500/9006
2019-09-09T21:59:50.9571710Z .................................................................................................... 1600/9006
2019-09-09T22:00:03.8283954Z .......................................................i...............i............................ 1700/9006
2019-09-09T22:00:11.7211316Z .................................................................................................... 1800/9006
2019-09-09T22:00:26.6873948Z ..............................................iiiii................................................. 1900/9006
2019-09-09T22:00:37.8435434Z .................................................................................................... 2100/9006
2019-09-09T22:00:40.4623055Z .................................................................................................... 2200/9006
2019-09-09T22:00:44.3235673Z .................................................................................................... 2300/9006
2019-09-09T22:00:52.3063591Z .................................................................................................... 2400/9006
---
2019-09-09T22:03:52.9219706Z .................................i...............i.................................................. 4700/9006
2019-09-09T22:04:04.8214764Z .................................................................................................... 4800/9006
2019-09-09T22:04:11.5260667Z .................................................................................................... 4900/9006
2019-09-09T22:04:22.5461534Z .................................................................................................... 5000/9006
2019-09-09T22:04:28.7313541Z ................ii.ii............................................................................... 5100/9006
2019-09-09T22:04:39.5724341Z .................................................................................................... 5300/9006
2019-09-09T22:04:49.9981088Z ...............................................................................i.................... 5400/9006
2019-09-09T22:04:57.9927714Z .................................................................................................... 5500/9006
2019-09-09T22:05:04.1342159Z .................................................................................................... 5600/9006
2019-09-09T22:05:04.1342159Z .................................................................................................... 5600/9006
2019-09-09T22:05:14.9559560Z .........................................................................ii...i..ii...........i..... 5700/9006
2019-09-09T22:05:40.4180132Z .................................................................................................... 5900/9006
2019-09-09T22:05:50.1562866Z .................................................................................................... 6000/9006
2019-09-09T22:05:50.1562866Z .................................................................................................... 6000/9006
2019-09-09T22:05:59.4981187Z ...........................................................................i..ii.................... 6100/9006
2019-09-09T22:06:29.4374772Z .................................................................................................... 6300/9006
2019-09-09T22:06:31.5471245Z ..................................i................................................................. 6400/9006
2019-09-09T22:06:33.8293994Z .................................................................................................... 6500/9006
2019-09-09T22:06:36.4817055Z ......i............................................................................................. 6600/9006
---
2019-09-09T22:11:29.0801002Z  finished in 21.409
2019-09-09T22:11:29.1010700Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T22:11:29.3138211Z 
2019-09-09T22:11:29.3139094Z running 150 tests
2019-09-09T22:11:32.6362497Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-09T22:11:34.6485471Z ..iiii..............i.........iii.i.......ii......
2019-09-09T22:11:34.6490644Z 
2019-09-09T22:11:34.6495907Z  finished in 5.550
2019-09-09T22:11:34.6685877Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T22:11:34.8338850Z 
---
2019-09-09T22:11:36.9180631Z  finished in 2.249
2019-09-09T22:11:36.9364881Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T22:11:37.0996352Z 
2019-09-09T22:11:37.0997221Z running 9 tests
2019-09-09T22:11:37.0998188Z iiiiiiiii
2019-09-09T22:11:37.0998828Z 
2019-09-09T22:11:37.1003917Z  finished in 0.164
2019-09-09T22:11:37.1196611Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T22:11:37.2999829Z 
---
2019-09-09T22:11:55.6471042Z  finished in 18.527
2019-09-09T22:11:55.6699125Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T22:11:55.8525275Z 
2019-09-09T22:11:55.8525625Z running 123 tests
2019-09-09T22:12:20.4070404Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-09T22:12:25.2594242Z i.i.i......iii.i.....ii
2019-09-09T22:12:25.2595802Z 
2019-09-09T22:12:25.2599703Z  finished in 29.589
2019-09-09T22:12:25.2609165Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T22:12:25.2612900Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-09T22:13:28.5744210Z 50 
2019-09-09T22:13:28.5744256Z 
2019-09-09T22:13:28.5744309Z 
2019-09-09T22:13:28.5744355Z The actual stderr differed from the expected stderr.
2019-09-09T22:13:28.5744857Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2019-09-09T22:13:28.5745490Z To update references, rerun the tests and pass the `--bless` flag
2019-09-09T22:13:28.5746155Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2019-09-09T22:13:28.5746247Z error: 1 errors occurred comparing output.
2019-09-09T22:13:28.5746310Z status: exit code: 1
2019-09-09T22:13:28.5746310Z status: exit code: 1
2019-09-09T22:13:28.5747344Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2019-09-09T22:13:28.5747768Z ------------------------------------------
2019-09-09T22:13:28.5747804Z 
2019-09-09T22:13:28.5748074Z ------------------------------------------
2019-09-09T22:13:28.5748118Z stderr:
2019-09-09T22:13:28.5748118Z stderr:
2019-09-09T22:13:28.5748331Z ------------------------------------------
2019-09-09T22:13:28.5748718Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T22:13:28.5749043Z    |
2019-09-09T22:13:28.5749101Z LL | extern crate rustc_data_structures;
2019-09-09T22:13:28.5749154Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T22:13:28.5749197Z    |
2019-09-09T22:13:28.5749197Z    |
2019-09-09T22:13:28.5751189Z    = note: for more information, see ***/issues/27812
2019-09-09T22:13:28.5752601Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T22:13:28.5752682Z 
2019-09-09T22:13:28.5753242Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T22:13:28.5753559Z    |
2019-09-09T22:13:28.5753617Z LL | extern crate rustc;
2019-09-09T22:13:28.5753660Z    | ^^^^^^^^^^^^^^^^^^^
2019-09-09T22:13:28.5753699Z    |
2019-09-09T22:13:28.5753699Z    |
2019-09-09T22:13:28.5754050Z    = note: for more information, see ***/issues/27812
2019-09-09T22:13:28.5754108Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T22:13:28.5754142Z 
2019-09-09T22:13:28.5754553Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T22:13:28.5754866Z    |
2019-09-09T22:13:28.5754924Z LL | extern crate rustc_macros;
2019-09-09T22:13:28.5754968Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T22:13:28.5755008Z    |
2019-09-09T22:13:28.5755008Z    |
2019-09-09T22:13:28.5755297Z    = note: for more information, see ***/issues/27812
2019-09-09T22:13:28.5755354Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T22:13:28.5755386Z 
2019-09-09T22:13:28.5755773Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T22:13:28.5756714Z    |
2019-09-09T22:13:28.5756775Z LL | use rustc_macros::HashStable;
2019-09-09T22:13:28.5756821Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T22:13:28.5756861Z    |
2019-09-09T22:13:28.5756861Z    |
2019-09-09T22:13:28.5757191Z    = note: for more information, see ***/issues/27812
2019-09-09T22:13:28.5757253Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T22:13:28.5757288Z 
2019-09-09T22:13:28.5757700Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T22:13:28.5758062Z    |
2019-09-09T22:13:28.5758106Z LL | #[derive(HashStable)]
2019-09-09T22:13:28.5758167Z    |          ^^^^^^^^^^
2019-09-09T22:13:28.5758210Z    |
2019-09-09T22:13:28.5758210Z    |
2019-09-09T22:13:28.5758653Z    = note: for more information, see ***/issues/27812
2019-09-09T22:13:28.5758745Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T22:13:28.5758825Z error: aborting due to 5 previous errors
2019-09-09T22:13:28.5758855Z 
2019-09-09T22:13:28.5759439Z For more information about this error, try `rustc --explain E0658`.
2019-09-09T22:13:28.5759481Z 
---
2019-09-09T22:13:28.5760775Z ' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-09T22:13:28.5760857Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-09T22:13:28.5760966Z 
2019-09-09T22:13:28.5760996Z 
2019-09-09T22:13:28.5762506Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T22:13:28.5762751Z 
2019-09-09T22:13:28.5762781Z 
2019-09-09T22:13:28.5762842Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T22:13:28.5762892Z Build completed unsuccessfully in 1:10:30
2019-09-09T22:13:28.5762892Z Build completed unsuccessfully in 1:10:30
2019-09-09T22:13:28.5811744Z == clock drift check ==
2019-09-09T22:13:28.5824127Z   local time: Mon Sep  9 22:13:28 UTC 2019
2019-09-09T22:13:28.6744122Z   network time: Mon, 09 Sep 2019 22:13:28 GMT
2019-09-09T22:13:28.6748318Z == end clock drift check ==
2019-09-09T22:13:29.5068243Z ##[error]Bash exited with code '1'.
2019-09-09T22:13:29.5109594Z ##[section]Starting: Checkout
2019-09-09T22:13:29.5111619Z ==============================================================================
2019-09-09T22:13:29.5111685Z Task         : Get sources
2019-09-09T22:13:29.5111729Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
