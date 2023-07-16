plain
2019-09-09T01:19:22.1233329Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T01:19:22.9520205Z ##[command]git config gc.auto 0
2019-09-09T01:19:22.9524891Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T01:19:22.9531135Z ##[command]git config --get-all http.proxy
2019-09-09T01:19:22.9533691Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64290/merge:refs/remotes/pull/64290/merge
---
2019-09-09T02:19:57.0036552Z .................................................................................................... 1500/9008
2019-09-09T02:20:02.5428747Z .................................................................................................... 1600/9008
2019-09-09T02:20:14.3652373Z ......................................................i...............i............................. 1700/9008
2019-09-09T02:20:21.7165249Z .................................................................................................... 1800/9008
2019-09-09T02:20:35.3111507Z .............................................iiiii.................................................. 1900/9008
2019-09-09T02:20:45.9643311Z .................................................................................................... 2100/9008
2019-09-09T02:20:48.4152689Z .................................................................................................... 2200/9008
2019-09-09T02:20:51.6838257Z .................................................................................................... 2300/9008
2019-09-09T02:20:59.1332471Z .................................................................................................... 2400/9008
---
2019-09-09T02:23:49.3616401Z ..................................i...............i................................................. 4700/9008
2019-09-09T02:24:00.6991724Z .................................................................................................... 4800/9008
2019-09-09T02:24:06.7731799Z .................................................................................................... 4900/9008
2019-09-09T02:24:16.9208886Z .................................................................................................... 5000/9008
2019-09-09T02:24:22.6721922Z ................ii.ii............................................................................... 5100/9008
2019-09-09T02:24:32.7118149Z .................................................................................................... 5300/9008
2019-09-09T02:24:42.3044183Z ...............................................................................i.................... 5400/9008
2019-09-09T02:24:49.7589292Z .................................................................................................... 5500/9008
2019-09-09T02:24:55.4019446Z .................................................................................................... 5600/9008
2019-09-09T02:24:55.4019446Z .................................................................................................... 5600/9008
2019-09-09T02:25:05.6155776Z .........................................................................ii...i..ii...........i..... 5700/9008
2019-09-09T02:25:29.4924366Z .................................................................................................... 5900/9008
2019-09-09T02:25:38.4599654Z .................................................................................................... 6000/9008
2019-09-09T02:25:38.4599654Z .................................................................................................... 6000/9008
2019-09-09T02:25:44.8303628Z ...........................................................................i..ii.................... 6100/9008
2019-09-09T02:26:12.6526550Z .................................................................................................... 6300/9008
2019-09-09T02:26:14.5404120Z ..................................i................................................................. 6400/9008
2019-09-09T02:26:16.5188863Z .................................................................................................... 6500/9008
2019-09-09T02:26:18.8455349Z ......i............................................................................................. 6600/9008
---
2019-09-09T02:30:44.7845517Z  finished in 18.256
2019-09-09T02:30:44.8049205Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T02:30:44.9547478Z 
2019-09-09T02:30:44.9549141Z running 150 tests
2019-09-09T02:30:47.9354798Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-09T02:30:49.7789464Z ..iiii..............i.........iii.i.......ii......
2019-09-09T02:30:49.7791984Z 
2019-09-09T02:30:49.7796626Z  finished in 4.974
2019-09-09T02:30:49.7958845Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T02:30:49.9362977Z 
---
2019-09-09T02:30:51.8225288Z  finished in 2.026
2019-09-09T02:30:51.8411489Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T02:30:51.9992673Z 
2019-09-09T02:30:51.9992931Z running 9 tests
2019-09-09T02:30:51.9993664Z iiiiiiiii
2019-09-09T02:30:51.9993947Z 
2019-09-09T02:30:51.9993982Z  finished in 0.158
2019-09-09T02:30:52.0160031Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T02:30:52.1745101Z 
---
2019-09-09T02:31:09.0704297Z  finished in 17.054
2019-09-09T02:31:09.0882529Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T02:31:10.1903208Z 
2019-09-09T02:31:10.1915483Z running 123 tests
2019-09-09T02:31:30.7051003Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-09T02:31:34.9437933Z i.i.i......iii.i.....ii
2019-09-09T02:31:34.9439699Z 
2019-09-09T02:31:34.9439838Z  finished in 25.855
2019-09-09T02:31:34.9448661Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T02:31:34.9449629Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-09T02:32:32.8952995Z 
2019-09-09T02:32:32.8955580Z ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
2019-09-09T02:32:32.8956013Z diff of stderr:
2019-09-09T02:32:32.8956169Z 
2019-09-09T02:32:32.8956300Z 1 error[E0601]: `main` function not found in crate `hash_stable_is_unstable`
2019-09-09T02:32:32.8956675Z +   --> $DIR/hash-stable-is-unstable.rs:3:1
2019-09-09T02:32:32.8956984Z 3 LL | / extern crate rustc_data_structures;
2019-09-09T02:32:32.8957106Z 4 LL | |
2019-09-09T02:32:32.8957224Z 
2019-09-09T02:32:32.8957339Z 5 LL | | extern crate rustc;
2019-09-09T02:32:32.8957339Z 5 LL | | extern crate rustc;
2019-09-09T02:32:32.8957455Z 6 LL | |
2019-09-09T02:32:32.8957748Z - ... |
2019-09-09T02:32:32.8958067Z + ...  |
2019-09-09T02:32:32.8958351Z 8 LL | |
2019-09-09T02:32:32.8958479Z 9 LL | | struct Test;
2019-09-09T02:32:32.8958806Z - | |____________^ consider adding a `main` function to `$DIR/hash-stable-is-unstable.rs`
2019-09-09T02:32:32.8959175Z +    | |____________^ consider adding a `main` function to `$DIR/hash-stable-is-unstable.rs`
2019-09-09T02:32:32.8959368Z 11 
2019-09-09T02:32:32.8960640Z 12 error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T02:32:32.8961337Z 
2019-09-09T02:32:32.8961675Z 
2019-09-09T02:32:32.8961911Z The actual stderr differed from the expected stderr.
2019-09-09T02:32:32.8962430Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2019-09-09T02:32:32.8962430Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2019-09-09T02:32:32.8962866Z To update references, rerun the tests and pass the `--bless` flag
2019-09-09T02:32:32.8963331Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2019-09-09T02:32:32.8963944Z error: 1 errors occurred comparing output.
2019-09-09T02:32:32.8964081Z status: exit code: 1
2019-09-09T02:32:32.8964081Z status: exit code: 1
2019-09-09T02:32:32.8964789Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2019-09-09T02:32:32.8965306Z ------------------------------------------
2019-09-09T02:32:32.8965460Z 
2019-09-09T02:32:32.8965751Z ------------------------------------------
2019-09-09T02:32:32.8965896Z stderr:
2019-09-09T02:32:32.8965896Z stderr:
2019-09-09T02:32:32.8966194Z ------------------------------------------
2019-09-09T02:32:32.8966347Z error[E0601]: `main` function not found in crate `hash_stable_is_unstable`
2019-09-09T02:32:32.8966834Z    |
2019-09-09T02:32:32.8966949Z LL | / extern crate rustc_data_structures;
2019-09-09T02:32:32.8966949Z LL | / extern crate rustc_data_structures;
2019-09-09T02:32:32.8967426Z LL | | //~^ use of unstable library feature 'rustc_private'
2019-09-09T02:32:32.8967605Z LL | | extern crate rustc;
2019-09-09T02:32:32.8967918Z LL | | //~^ use of unstable library feature 'rustc_private'
2019-09-09T02:32:32.8968089Z ...  |
2019-09-09T02:32:32.8968393Z LL | | //~^ use of unstable library feature 'rustc_private'
2019-09-09T02:32:32.8968540Z LL | | struct Test;
2019-09-09T02:32:32.8968907Z    | |____________^ consider adding a `main` function to `/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs`
2019-09-09T02:32:32.8969046Z 
2019-09-09T02:32:32.8969636Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T02:32:32.8970683Z    |
2019-09-09T02:32:32.8970852Z LL | extern crate rustc_data_structures;
2019-09-09T02:32:32.8971021Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T02:32:32.8971162Z    |
2019-09-09T02:32:32.8971162Z    |
2019-09-09T02:32:32.8971783Z    = note: for more information, see ***/issues/27812
2019-09-09T02:32:32.8972003Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T02:32:32.8972137Z 
2019-09-09T02:32:32.8972695Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T02:32:32.8973498Z    |
2019-09-09T02:32:32.8973796Z LL | extern crate rustc;
2019-09-09T02:32:32.8973911Z    | ^^^^^^^^^^^^^^^^^^^
2019-09-09T02:32:32.8974042Z    |
2019-09-09T02:32:32.8974042Z    |
2019-09-09T02:32:32.8974396Z    = note: for more information, see ***/issues/27812
2019-09-09T02:32:32.8974579Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T02:32:32.8974700Z 
2019-09-09T02:32:32.8975604Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T02:32:32.8976212Z    |
2019-09-09T02:32:32.8976332Z LL | extern crate rustc_macros;
2019-09-09T02:32:32.8976555Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T02:32:32.8976923Z    |
2019-09-09T02:32:32.8976923Z    |
2019-09-09T02:32:32.8977392Z    = note: for more information, see ***/issues/27812
2019-09-09T02:32:32.8978098Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T02:32:32.8978132Z 
2019-09-09T02:32:32.8978524Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T02:32:32.8978787Z    |
2019-09-09T02:32:32.8978836Z LL | use rustc_macros::HashStable;
2019-09-09T02:32:32.8978880Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T02:32:32.8979089Z    |
2019-09-09T02:32:32.8979089Z    |
2019-09-09T02:32:32.8979346Z    = note: for more information, see ***/issues/27812
2019-09-09T02:32:32.8979393Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T02:32:32.8979421Z 
2019-09-09T02:32:32.8979723Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T02:32:32.8980414Z    |
2019-09-09T02:32:32.8980457Z LL | #[derive(HashStable)]
2019-09-09T02:32:32.8980520Z    |          ^^^^^^^^^^
2019-09-09T02:32:32.8980561Z    |
2019-09-09T02:32:32.8980561Z    |
2019-09-09T02:32:32.8980870Z    = note: for more information, see ***/issues/27812
2019-09-09T02:32:32.8981095Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T02:32:32.8981183Z error: aborting due to 6 previous errors
2019-09-09T02:32:32.8981230Z 
2019-09-09T02:32:32.8981277Z Some errors have detailed explanations: E0601, E0658.
2019-09-09T02:32:32.8981568Z For more information about an error, try `rustc --explain E0601`.
---
2019-09-09T02:32:32.8982487Z test result: FAILED. 68 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-09T02:32:32.8982524Z 
2019-09-09T02:32:32.8982569Z 
2019-09-09T02:32:32.8982595Z 
2019-09-09T02:32:32.8984182Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T02:32:32.8984410Z 
2019-09-09T02:32:32.8984433Z 
2019-09-09T02:32:32.8984469Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T02:32:32.8984525Z Build completed unsuccessfully in 1:05:45
2019-09-09T02:32:32.8984525Z Build completed unsuccessfully in 1:05:45
2019-09-09T02:32:32.9006741Z == clock drift check ==
2019-09-09T02:32:32.9026360Z   local time: Mon Sep  9 02:32:32 UTC 2019
2019-09-09T02:32:33.1933167Z   network time: Mon, 09 Sep 2019 02:32:33 GMT
2019-09-09T02:32:33.7181213Z == end clock drift check ==
2019-09-09T02:32:33.7940832Z ##[error]Bash exited with code '1'.
2019-09-09T02:32:33.7978867Z ##[section]Starting: Checkout
2019-09-09T02:32:33.7981449Z ==============================================================================
2019-09-09T02:32:33.7981506Z Task         : Get sources
2019-09-09T02:32:33.7981553Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
