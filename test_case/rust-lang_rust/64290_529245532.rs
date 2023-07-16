plain
2019-09-08T20:44:32.3316194Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-08T20:44:32.3500191Z ##[command]git config gc.auto 0
2019-09-08T20:44:32.3578191Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-08T20:44:32.3636640Z ##[command]git config --get-all http.proxy
2019-09-08T20:44:32.3783074Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64290/merge:refs/remotes/pull/64290/merge
---
2019-09-08T21:45:24.5455954Z .................................................................................................... 1500/9008
2019-09-08T21:45:30.3238602Z .................................................................................................... 1600/9008
2019-09-08T21:45:42.8861180Z ......................................................i...............i............................. 1700/9008
2019-09-08T21:45:50.7207725Z .................................................................................................... 1800/9008
2019-09-08T21:46:05.1496627Z .............................................iiiii.................................................. 1900/9008
2019-09-08T21:46:16.0847196Z .................................................................................................... 2100/9008
2019-09-08T21:46:18.6759518Z .................................................................................................... 2200/9008
2019-09-08T21:46:22.3281971Z .................................................................................................... 2300/9008
2019-09-08T21:46:30.4360562Z .................................................................................................... 2400/9008
---
2019-09-08T21:49:29.8872344Z ..................................i...............i................................................. 4700/9008
2019-09-08T21:49:41.4374892Z .................................................................................................... 4800/9008
2019-09-08T21:49:47.5910608Z .................................................................................................... 4900/9008
2019-09-08T21:49:58.2561490Z .................................................................................................... 5000/9008
2019-09-08T21:50:04.2489257Z ................ii.ii............................................................................... 5100/9008
2019-09-08T21:50:14.7038250Z .................................................................................................... 5300/9008
2019-09-08T21:50:24.7114683Z ...............................................................................i.................... 5400/9008
2019-09-08T21:50:32.5019284Z .................................................................................................... 5500/9008
2019-09-08T21:50:38.4096890Z .................................................................................................... 5600/9008
2019-09-08T21:50:38.4096890Z .................................................................................................... 5600/9008
2019-09-08T21:50:49.0274144Z .........................................................................ii...i..ii...........i..... 5700/9008
2019-09-08T21:51:14.1226796Z .................................................................................................... 5900/9008
2019-09-08T21:51:23.5618458Z .................................................................................................... 6000/9008
2019-09-08T21:51:23.5618458Z .................................................................................................... 6000/9008
2019-09-08T21:51:28.8472514Z ...........................................................................i..ii.................... 6100/9008
2019-09-08T21:51:58.0587014Z .................................................................................................... 6300/9008
2019-09-08T21:52:00.0468643Z ..................................i................................................................. 6400/9008
2019-09-08T21:52:02.1466077Z .................................................................................................... 6500/9008
2019-09-08T21:52:04.6582859Z ......i............................................................................................. 6600/9008
---
2019-09-08T21:56:45.8293406Z  finished in 19.754
2019-09-08T21:56:45.8471087Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-08T21:56:46.0395383Z 
2019-09-08T21:56:46.0396379Z running 150 tests
2019-09-08T21:56:49.2275823Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-08T21:56:51.1752520Z ..iiii..............i.........iii.i.......ii......
2019-09-08T21:56:51.1753961Z 
2019-09-08T21:56:51.1758967Z  finished in 5.329
2019-09-08T21:56:51.1955607Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-08T21:56:51.3715790Z 
---
2019-09-08T21:56:53.3992778Z  finished in 2.205
2019-09-08T21:56:53.4168984Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-08T21:56:53.5781473Z 
2019-09-08T21:56:53.5781675Z running 9 tests
2019-09-08T21:56:53.5782960Z iiiiiiiii
2019-09-08T21:56:53.5783384Z 
2019-09-08T21:56:53.5787843Z  finished in 0.162
2019-09-08T21:56:53.5967854Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-08T21:56:53.7955307Z 
---
2019-09-08T21:57:11.4958026Z  finished in 17.899
2019-09-08T21:57:11.5175605Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-08T21:57:11.7136719Z 
2019-09-08T21:57:11.7137218Z running 123 tests
2019-09-08T21:57:35.5639911Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-08T21:57:40.2146998Z i.i.i......iii.i.....ii
2019-09-08T21:57:40.2148448Z 
2019-09-08T21:57:40.2151717Z  finished in 28.697
2019-09-08T21:57:40.2154900Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-08T21:57:40.2158621Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-08T21:58:41.5109120Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-08T21:58:41.5109597Z ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
2019-09-08T21:58:41.5109796Z diff of stderr:
2019-09-08T21:58:41.5109948Z 
2019-09-08T21:58:41.5110142Z 1 error[E0601]: `main` function not found in crate `hash_stable_is_unstable`
2019-09-08T21:58:41.5110567Z +   --> $DIR/hash-stable-is-unstable.rs:3:1
2019-09-08T21:58:41.5110764Z 2    |
2019-09-08T21:58:41.5111215Z -    = note: consider adding a `main` function to `$DIR/hash-stable-is-unstable.rs`
2019-09-08T21:58:41.5111422Z + LL | / extern crate rustc_data_structures;
2019-09-08T21:58:41.5111595Z + LL | |
2019-09-08T21:58:41.5111772Z + LL | | extern crate rustc;
2019-09-08T21:58:41.5111926Z + LL | |
2019-09-08T21:58:41.5112085Z + ...  |
2019-09-08T21:58:41.5112257Z + LL | |
2019-09-08T21:58:41.5112409Z + LL | | struct Test;
2019-09-08T21:58:41.5112841Z +    | |____________^ consider adding a `main` function to `$DIR/hash-stable-is-unstable.rs`
2019-09-08T21:58:41.5113068Z 4 
2019-09-08T21:58:41.5113633Z 5 error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-08T21:58:41.5114282Z 
2019-09-08T21:58:41.5114419Z 
2019-09-08T21:58:41.5114614Z The actual stderr differed from the expected stderr.
2019-09-08T21:58:41.5115101Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2019-09-08T21:58:41.5115101Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2019-09-08T21:58:41.5115556Z To update references, rerun the tests and pass the `--bless` flag
2019-09-08T21:58:41.5116033Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2019-09-08T21:58:41.5116370Z error: 1 errors occurred comparing output.
2019-09-08T21:58:41.5116545Z status: exit code: 1
2019-09-08T21:58:41.5116545Z status: exit code: 1
2019-09-08T21:58:41.5117699Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2019-09-08T21:58:41.5118706Z ------------------------------------------
2019-09-08T21:58:41.5118925Z 
2019-09-08T21:58:41.5119289Z ------------------------------------------
2019-09-08T21:58:41.5119476Z stderr:
2019-09-08T21:58:41.5119476Z stderr:
2019-09-08T21:58:41.5119846Z ------------------------------------------
2019-09-08T21:58:41.5120041Z error[E0601]: `main` function not found in crate `hash_stable_is_unstable`
2019-09-08T21:58:41.5120639Z    |
2019-09-08T21:58:41.5120802Z LL | / extern crate rustc_data_structures;
2019-09-08T21:58:41.5120802Z LL | / extern crate rustc_data_structures;
2019-09-08T21:58:41.5121195Z LL | | //~^ use of unstable library feature 'rustc_private'
2019-09-08T21:58:41.5121392Z LL | | extern crate rustc;
2019-09-08T21:58:41.5121760Z LL | | //~^ use of unstable library feature 'rustc_private'
2019-09-08T21:58:41.5121968Z ...  |
2019-09-08T21:58:41.5122335Z LL | | //~^ use of unstable library feature 'rustc_private'
2019-09-08T21:58:41.5122518Z LL | | struct Test;
2019-09-08T21:58:41.5122965Z    | |____________^ consider adding a `main` function to `/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs`
2019-09-08T21:58:41.5123327Z 
2019-09-08T21:58:41.5123884Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-08T21:58:41.5124541Z    |
2019-09-08T21:58:41.5124709Z LL | extern crate rustc_data_structures;
2019-09-08T21:58:41.5124871Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-08T21:58:41.5125020Z    |
2019-09-08T21:58:41.5125020Z    |
2019-09-08T21:58:41.5125572Z    = note: for more information, see ***/issues/27812
2019-09-08T21:58:41.5125783Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-08T21:58:41.5125938Z 
2019-09-08T21:58:41.5126461Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-08T21:58:41.5127138Z    |
2019-09-08T21:58:41.5127284Z LL | extern crate rustc;
2019-09-08T21:58:41.5127429Z    | ^^^^^^^^^^^^^^^^^^^
2019-09-08T21:58:41.5127594Z    |
2019-09-08T21:58:41.5127594Z    |
2019-09-08T21:58:41.5128015Z    = note: for more information, see ***/issues/27812
2019-09-08T21:58:41.5128738Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-08T21:58:41.5128913Z 
2019-09-08T21:58:41.5129482Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-08T21:58:41.5130136Z    |
2019-09-08T21:58:41.5130283Z LL | extern crate rustc_macros;
2019-09-08T21:58:41.5130463Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-08T21:58:41.5130607Z    |
2019-09-08T21:58:41.5130607Z    |
2019-09-08T21:58:41.5131103Z    = note: for more information, see ***/issues/27812
2019-09-08T21:58:41.5131430Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-08T21:58:41.5131600Z 
2019-09-08T21:58:41.5132139Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-08T21:58:41.5133036Z    |
2019-09-08T21:58:41.5133193Z LL | use rustc_macros::HashStable;
2019-09-08T21:58:41.5133345Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-08T21:58:41.5133499Z    |
2019-09-08T21:58:41.5133499Z    |
2019-09-08T21:58:41.5133988Z    = note: for more information, see ***/issues/27812
2019-09-08T21:58:41.5134220Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-08T21:58:41.5134377Z 
2019-09-08T21:58:41.5134907Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-08T21:58:41.5135602Z    |
2019-09-08T21:58:41.5135753Z LL | #[derive(HashStable)]
2019-09-08T21:58:41.5135903Z    |          ^^^^^^^^^^
2019-09-08T21:58:41.5136050Z    |
2019-09-08T21:58:41.5136050Z    |
2019-09-08T21:58:41.5136493Z    = note: for more information, see ***/issues/27812
2019-09-08T21:58:41.5136748Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-08T21:58:41.5137053Z error: aborting due to 6 previous errors
2019-09-08T21:58:41.5137204Z 
2019-09-08T21:58:41.5137369Z Some errors have detailed explanations: E0601, E0658.
2019-09-08T21:58:41.5137812Z For more information about an error, try `rustc --explain E0601`.
---
2019-09-08T21:58:41.5140470Z test result: FAILED. 68 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-08T21:58:41.5140662Z 
2019-09-08T21:58:41.5140795Z 
2019-09-08T21:58:41.5141560Z 
2019-09-08T21:58:41.5143334Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-08T21:58:41.5143884Z 
2019-09-08T21:58:41.5144010Z 
2019-09-08T21:58:41.5144259Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-08T21:58:41.5144425Z Build completed unsuccessfully in 1:07:23
2019-09-08T21:58:41.5144425Z Build completed unsuccessfully in 1:07:23
2019-09-08T21:58:41.5190040Z == clock drift check ==
2019-09-08T21:58:41.5207850Z   local time: Sun Sep  8 21:58:41 UTC 2019
2019-09-08T21:58:41.6697301Z   network time: Sun, 08 Sep 2019 21:58:41 GMT
2019-09-08T21:58:41.6702524Z == end clock drift check ==
2019-09-08T21:58:42.4904152Z ##[error]Bash exited with code '1'.
2019-09-08T21:58:42.4941910Z ##[section]Starting: Checkout
2019-09-08T21:58:42.4943768Z ==============================================================================
2019-09-08T21:58:42.4943841Z Task         : Get sources
2019-09-08T21:58:42.4943890Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
