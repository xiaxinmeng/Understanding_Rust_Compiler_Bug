plain
2019-09-09T11:46:30.1203897Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T11:46:30.9841098Z ##[command]git config gc.auto 0
2019-09-09T11:46:30.9847056Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T11:46:30.9853273Z ##[command]git config --get-all http.proxy
2019-09-09T11:46:30.9859031Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64290/merge:refs/remotes/pull/64290/merge
---
2019-09-09T12:51:48.6317238Z .................................................................................................... 1500/9009
2019-09-09T12:51:54.9245324Z .................................................................................................... 1600/9009
2019-09-09T12:52:08.5738516Z ......................................................i...............i............................. 1700/9009
2019-09-09T12:52:17.0783802Z .................................................................................................... 1800/9009
2019-09-09T12:52:32.6747612Z .............................................iiiii.................................................. 1900/9009
2019-09-09T12:52:44.5161118Z .................................................................................................... 2100/9009
2019-09-09T12:52:47.2919221Z .................................................................................................... 2200/9009
2019-09-09T12:52:51.2169627Z .................................................................................................... 2300/9009
2019-09-09T12:52:59.7878867Z .................................................................................................... 2400/9009
---
2019-09-09T12:56:11.8451232Z ...................................i...............i................................................ 4700/9009
2019-09-09T12:56:24.4027043Z .................................................................................................... 4800/9009
2019-09-09T12:56:31.5599622Z .................................................................................................... 4900/9009
2019-09-09T12:56:43.0891544Z .................................................................................................... 5000/9009
2019-09-09T12:56:49.7222525Z .................ii.ii.............................................................................. 5100/9009
2019-09-09T12:57:01.2126186Z .................................................................................................... 5300/9009
2019-09-09T12:57:12.1805010Z ................................................................................i................... 5400/9009
2019-09-09T12:57:20.5817919Z .................................................................................................... 5500/9009
2019-09-09T12:57:27.0002803Z .................................................................................................... 5600/9009
2019-09-09T12:57:27.0002803Z .................................................................................................... 5600/9009
2019-09-09T12:57:38.5749004Z ..........................................................................ii...i..ii...........i.... 5700/9009
2019-09-09T12:58:05.2452386Z .................................................................................................... 5900/9009
2019-09-09T12:58:15.8069225Z .................................................................................................... 6000/9009
2019-09-09T12:58:15.8069225Z .................................................................................................... 6000/9009
2019-09-09T12:58:22.1846049Z ............................................................................i..ii................... 6100/9009
2019-09-09T12:58:53.5898233Z .................................................................................................... 6300/9009
2019-09-09T12:58:55.9171687Z ...................................i................................................................ 6400/9009
2019-09-09T12:58:58.1707017Z .................................................................................................... 6500/9009
2019-09-09T12:59:00.9460009Z .......i............................................................................................ 6600/9009
---
2019-09-09T13:04:03.3671900Z  finished in 20.432
2019-09-09T13:04:03.3877058Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T13:04:03.5889083Z 
2019-09-09T13:04:03.5890289Z running 150 tests
2019-09-09T13:04:07.0857725Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-09T13:04:09.2722602Z ..iiii..............i.........iii.i.......ii......
2019-09-09T13:04:09.9065045Z 
2019-09-09T13:04:09.9101278Z  finished in 5.885
2019-09-09T13:04:09.9102189Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T13:04:09.9144620Z 
---
2019-09-09T13:04:11.6719550Z  finished in 2.376
2019-09-09T13:04:11.6920724Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T13:04:11.8565536Z 
2019-09-09T13:04:11.8565830Z running 9 tests
2019-09-09T13:04:11.8567376Z iiiiiiiii
2019-09-09T13:04:11.8567835Z 
2019-09-09T13:04:11.8567878Z  finished in 0.164
2019-09-09T13:04:11.8767463Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T13:04:12.0684762Z 
---
2019-09-09T13:04:31.6065678Z  finished in 19.730
2019-09-09T13:04:31.6312758Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T13:04:31.8255734Z 
2019-09-09T13:04:31.8256068Z running 123 tests
2019-09-09T13:04:56.7728794Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-09T13:05:01.6390800Z i.i.i......iii.i.....ii
2019-09-09T13:05:01.6392848Z 
2019-09-09T13:05:01.6393616Z  finished in 30.008
2019-09-09T13:05:01.6405205Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T13:05:01.6405596Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-09T13:06:08.7594132Z 
2019-09-09T13:06:08.7600106Z ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
2019-09-09T13:06:08.7600624Z diff of stderr:
2019-09-09T13:06:08.7603096Z 
2019-09-09T13:06:08.7603493Z 1 error[E0601]: `main` function not found in crate `hash_stable_is_unstable`
2019-09-09T13:06:08.7604165Z +   --> $DIR/hash-stable-is-unstable.rs:3:1
2019-09-09T13:06:08.7604960Z 3 LL | / extern crate rustc_data_structures;
2019-09-09T13:06:08.7606093Z 4 LL | |
2019-09-09T13:06:08.7606392Z 
2019-09-09T13:06:08.7606482Z 
2019-09-09T13:06:08.7606482Z 
2019-09-09T13:06:08.7606537Z The actual stderr differed from the expected stderr.
2019-09-09T13:06:08.7607625Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2019-09-09T13:06:08.7612934Z To update references, rerun the tests and pass the `--bless` flag
2019-09-09T13:06:08.7613328Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2019-09-09T13:06:08.7613425Z error: 1 errors occurred comparing output.
2019-09-09T13:06:08.7613473Z status: exit code: 1
2019-09-09T13:06:08.7613473Z status: exit code: 1
2019-09-09T13:06:08.7614883Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2019-09-09T13:06:08.7615241Z ------------------------------------------
2019-09-09T13:06:08.7615275Z 
2019-09-09T13:06:08.7615506Z ------------------------------------------
2019-09-09T13:06:08.7615553Z stderr:
2019-09-09T13:06:08.7615553Z stderr:
2019-09-09T13:06:08.7615761Z ------------------------------------------
2019-09-09T13:06:08.7615830Z error[E0601]: `main` function not found in crate `hash_stable_is_unstable`
2019-09-09T13:06:08.7616120Z    |
2019-09-09T13:06:08.7616180Z LL | / extern crate rustc_data_structures;
2019-09-09T13:06:08.7616180Z LL | / extern crate rustc_data_structures;
2019-09-09T13:06:08.7616418Z LL | | //~^ use of unstable library feature 'rustc_private'
2019-09-09T13:06:08.7616470Z LL | | extern crate rustc;
2019-09-09T13:06:08.7616712Z LL | | //~^ use of unstable library feature 'rustc_private'
2019-09-09T13:06:08.7616760Z ...  |
2019-09-09T13:06:08.7616983Z LL | | //~^ use of unstable library feature 'rustc_private'
2019-09-09T13:06:08.7617032Z LL | | struct Test;
2019-09-09T13:06:08.7617332Z    | |____________^ consider adding a `main` function to `/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs`
2019-09-09T13:06:08.7617382Z 
2019-09-09T13:06:08.7617729Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T13:06:08.7618054Z    |
2019-09-09T13:06:08.7618113Z LL | extern crate rustc_data_structures;
2019-09-09T13:06:08.7618166Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T13:06:08.7618207Z    |
2019-09-09T13:06:08.7618207Z    |
2019-09-09T13:06:08.7618637Z    = note: for more information, see ***/issues/27812
2019-09-09T13:06:08.7618696Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T13:06:08.7618726Z 
2019-09-09T13:06:08.7619624Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T13:06:08.7620565Z    |
2019-09-09T13:06:08.7620610Z LL | extern crate rustc;
2019-09-09T13:06:08.7620684Z    | ^^^^^^^^^^^^^^^^^^^
2019-09-09T13:06:08.7620726Z    |
2019-09-09T13:06:08.7620726Z    |
2019-09-09T13:06:08.7621123Z    = note: for more information, see ***/issues/27812
2019-09-09T13:06:08.7621193Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T13:06:08.7621230Z 
2019-09-09T13:06:08.7621867Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T13:06:08.7622329Z    |
2019-09-09T13:06:08.7622375Z LL | extern crate rustc_macros;
2019-09-09T13:06:08.7622601Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T13:06:08.7622646Z    |
2019-09-09T13:06:08.7622646Z    |
2019-09-09T13:06:08.7623007Z    = note: for more information, see ***/issues/27812
2019-09-09T13:06:08.7623095Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T13:06:08.7623130Z 
2019-09-09T13:06:08.7623539Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T13:06:08.7624227Z    |
2019-09-09T13:06:08.7624267Z LL | use rustc_macros::HashStable;
2019-09-09T13:06:08.7624331Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-09T13:06:08.7624370Z    |
2019-09-09T13:06:08.7624370Z    |
2019-09-09T13:06:08.7624657Z    = note: for more information, see ***/issues/27812
2019-09-09T13:06:08.7624735Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T13:06:08.7624778Z 
2019-09-09T13:06:08.7625145Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-09-09T13:06:08.7625471Z    |
2019-09-09T13:06:08.7625512Z LL | #[derive(HashStable)]
2019-09-09T13:06:08.7625553Z    |          ^^^^^^^^^^
2019-09-09T13:06:08.7625612Z    |
2019-09-09T13:06:08.7625612Z    |
2019-09-09T13:06:08.7625901Z    = note: for more information, see ***/issues/27812
2019-09-09T13:06:08.7625977Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-09-09T13:06:08.7626051Z error: aborting due to 6 previous errors
2019-09-09T13:06:08.7626077Z 
2019-09-09T13:06:08.7626136Z Some errors have detailed explanations: E0601, E0658.
2019-09-09T13:06:08.7626390Z For more information about an error, try `rustc --explain E0601`.
---
2019-09-09T13:06:08.7627629Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-09T13:06:08.7627706Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-09T13:06:08.7627738Z 
2019-09-09T13:06:08.7627761Z 
2019-09-09T13:06:08.7629263Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T13:06:08.7629585Z 
2019-09-09T13:06:08.7629614Z 
2019-09-09T13:06:08.7629676Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T13:06:08.7629724Z Build completed unsuccessfully in 1:12:30
2019-09-09T13:06:08.7629724Z Build completed unsuccessfully in 1:12:30
2019-09-09T13:06:08.7669230Z == clock drift check ==
2019-09-09T13:06:08.7684757Z   local time: Mon Sep  9 13:06:08 UTC 2019
2019-09-09T13:06:08.8543722Z   network time: Mon, 09 Sep 2019 13:06:08 GMT
2019-09-09T13:06:08.8544191Z == end clock drift check ==
2019-09-09T13:06:09.4265903Z ##[error]Bash exited with code '1'.
2019-09-09T13:06:09.4309882Z ##[section]Starting: Checkout
2019-09-09T13:06:09.4312706Z ==============================================================================
2019-09-09T13:06:09.4312768Z Task         : Get sources
2019-09-09T13:06:09.4312838Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
