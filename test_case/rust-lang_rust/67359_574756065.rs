plain
2020-01-15T15:51:54.7974971Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-15T15:51:54.8061601Z ##[command]git config gc.auto 0
2020-01-15T15:51:54.8139124Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-15T15:51:54.8195681Z ##[command]git config --get-all http.proxy
2020-01-15T15:51:54.8334538Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67359/merge:refs/remotes/pull/67359/merge
---
2020-01-15T16:48:27.7172379Z .................................................................................................... 1600/9519
2020-01-15T16:48:33.0078649Z .................................................................................................... 1700/9519
2020-01-15T16:48:41.8206727Z .................................................................................................... 1800/9519
2020-01-15T16:48:51.0755582Z ........i........................................................................................... 1900/9519
2020-01-15T16:48:58.0623585Z ..................................................................................................ii 2000/9519
2020-01-15T16:49:14.5789217Z iii................................................................................................. 2100/9519
2020-01-15T16:49:23.0124744Z .................................................................................................... 2300/9519
2020-01-15T16:49:26.4040408Z .................................................................................................... 2400/9519
2020-01-15T16:49:31.4563252Z .................................................................................................... 2500/9519
2020-01-15T16:49:52.0165179Z .................................................................................................... 2600/9519
---
2020-01-15T16:52:34.6851154Z .........................................i...............i.......................................... 4900/9519
2020-01-15T16:52:44.0905004Z .................................................................................................... 5000/9519
2020-01-15T16:52:50.8850679Z ....................................................................................i............... 5100/9519
2020-01-15T16:52:56.4952651Z .................................................................................................... 5200/9519
2020-01-15T16:53:07.0364827Z .......................................................ii.ii...........i............................ 5300/9519
2020-01-15T16:53:16.4300394Z .................................................................................................... 5500/9519
2020-01-15T16:53:26.9467125Z .................................................................................................... 5600/9519
2020-01-15T16:53:33.6518003Z ........................................i........................................................... 5700/9519
2020-01-15T16:53:40.5608601Z .................................................................................................... 5800/9519
2020-01-15T16:53:40.5608601Z .................................................................................................... 5800/9519
2020-01-15T16:53:51.6510618Z .................................................................................................... 5900/9519
2020-01-15T16:54:01.6475975Z ...............................ii...i..ii...........i............................................... 6000/9519
2020-01-15T16:54:21.3588633Z .................................................................................................... 6200/9519
2020-01-15T16:54:29.7457296Z .................................................................................................... 6300/9519
2020-01-15T16:54:29.7457296Z .................................................................................................... 6300/9519
2020-01-15T16:54:37.3842932Z ...........................................................i..ii.................................... 6400/9519
2020-01-15T16:55:05.7591612Z .................................................................................................... 6600/9519
2020-01-15T16:55:07.9928459Z ...................................i................................................................ 6700/9519
2020-01-15T16:55:10.2973677Z .................................................................................................... 6800/9519
2020-01-15T16:55:12.8795197Z ...................................i................................................................ 6900/9519
---
2020-01-15T16:56:52.0895110Z .................................................................................................... 7500/9519
2020-01-15T16:56:56.5480807Z .................................................................................................... 7600/9519
2020-01-15T16:57:02.7683197Z .................................................................................................... 7700/9519
2020-01-15T16:57:10.1129003Z .................................................................................................... 7800/9519
2020-01-15T16:57:20.2198950Z ....................................................................................iiii............ 7900/9519
2020-01-15T16:57:37.3038961Z ..................i......i.......................................................................... 8100/9519
2020-01-15T16:57:42.5758371Z .................................................................................................... 8200/9519
2020-01-15T16:57:56.4053723Z .................................................................................................... 8300/9519
2020-01-15T16:58:06.8125205Z .................................................................................................... 8400/9519
---
2020-01-15T17:00:37.5462337Z  finished in 7.516
2020-01-15T17:00:37.5666299Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T17:00:37.7539178Z 
2020-01-15T17:00:37.7539461Z running 166 tests
2020-01-15T17:00:40.8435825Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-15T17:00:43.2046457Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-15T17:00:43.2047115Z 
2020-01-15T17:00:43.2052061Z  finished in 5.638
2020-01-15T17:00:43.2254510Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T17:00:43.3946966Z 
---
2020-01-15T17:00:45.4051166Z  finished in 2.179
2020-01-15T17:00:45.4251835Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T17:00:45.5876150Z 
2020-01-15T17:00:45.5877692Z running 9 tests
2020-01-15T17:00:45.5878464Z iiiiiiiii
2020-01-15T17:00:45.5878853Z 
2020-01-15T17:00:45.5878901Z  finished in 0.162
2020-01-15T17:00:45.6114889Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T17:00:45.7885649Z 
---
2020-01-15T17:01:06.2028657Z  finished in 20.591
2020-01-15T17:01:06.2222364Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T17:01:06.3792697Z 
2020-01-15T17:01:06.3793151Z running 116 tests
2020-01-15T17:01:33.0819638Z .iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii 100/116
2020-01-15T17:01:36.5585621Z .....iiii.....ii
2020-01-15T17:01:36.5587613Z 
2020-01-15T17:01:36.5593455Z  finished in 30.337
2020-01-15T17:01:36.5602169Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T17:01:36.5602710Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-15T17:02:18.2020200Z ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
2020-01-15T17:02:18.2020592Z diff of stderr:
2020-01-15T17:02:18.2021125Z 
2020-01-15T17:02:18.2021375Z 42    |
2020-01-15T17:02:18.2022159Z 43    = note: for more information, see ***/issues/27812
2020-01-15T17:02:18.2022536Z 44    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T17:02:18.2023362Z -    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace
2020-01-15T17:02:18.2024152Z +    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-01-15T17:02:18.2024781Z 47 error: aborting due to 5 previous errors
2020-01-15T17:02:18.2025029Z 48 
2020-01-15T17:02:18.2025587Z 
2020-01-15T17:02:18.2025884Z 
2020-01-15T17:02:18.2025884Z 
2020-01-15T17:02:18.2026133Z The actual stderr differed from the expected stderr.
2020-01-15T17:02:18.2026817Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2020-01-15T17:02:18.2027466Z To update references, rerun the tests and pass the `--bless` flag
2020-01-15T17:02:18.2028112Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2020-01-15T17:02:18.2028667Z error: 1 errors occurred comparing output.
2020-01-15T17:02:18.2028916Z status: exit code: 1
2020-01-15T17:02:18.2028916Z status: exit code: 1
2020-01-15T17:02:18.2030128Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2020-01-15T17:02:18.2031058Z ------------------------------------------
2020-01-15T17:02:18.2031338Z 
2020-01-15T17:02:18.2031839Z ------------------------------------------
2020-01-15T17:02:18.2032163Z stderr:
2020-01-15T17:02:18.2032163Z stderr:
2020-01-15T17:02:18.2032655Z ------------------------------------------
2020-01-15T17:02:18.2033560Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-01-15T17:02:18.2034541Z    |
2020-01-15T17:02:18.2034803Z LL | extern crate rustc_data_structures;
2020-01-15T17:02:18.2035042Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-15T17:02:18.2035269Z    |
2020-01-15T17:02:18.2035269Z    |
2020-01-15T17:02:18.2036396Z    = note: for more information, see ***/issues/27812
2020-01-15T17:02:18.2036757Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T17:02:18.2037008Z 
2020-01-15T17:02:18.2037753Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-01-15T17:02:18.2038711Z    |
2020-01-15T17:02:18.2038947Z LL | extern crate rustc;
2020-01-15T17:02:18.2039194Z    | ^^^^^^^^^^^^^^^^^^^
2020-01-15T17:02:18.2039425Z    |
2020-01-15T17:02:18.2039425Z    |
2020-01-15T17:02:18.2040011Z    = note: for more information, see ***/issues/27812
2020-01-15T17:02:18.2041005Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T17:02:18.2041207Z 
2020-01-15T17:02:18.2041926Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-01-15T17:02:18.2042870Z    |
2020-01-15T17:02:18.2043019Z LL | extern crate rustc_macros;
2020-01-15T17:02:18.2043190Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-15T17:02:18.2043337Z    |
2020-01-15T17:02:18.2043337Z    |
2020-01-15T17:02:18.2043982Z    = note: for more information, see ***/issues/27812
2020-01-15T17:02:18.2044263Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T17:02:18.2044401Z 
2020-01-15T17:02:18.2045018Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-01-15T17:02:18.2046301Z    |
2020-01-15T17:02:18.2046480Z LL | use rustc_macros::HashStable;
2020-01-15T17:02:18.2046640Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-15T17:02:18.2046787Z    |
2020-01-15T17:02:18.2046787Z    |
2020-01-15T17:02:18.2047300Z    = note: for more information, see ***/issues/27812
2020-01-15T17:02:18.2047555Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T17:02:18.2047704Z 
2020-01-15T17:02:18.2048371Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-01-15T17:02:18.2049116Z    |
2020-01-15T17:02:18.2049433Z LL | #[derive(HashStable)]
2020-01-15T17:02:18.2049624Z    |          ^^^^^^^^^^
2020-01-15T17:02:18.2049799Z    |
2020-01-15T17:02:18.2049799Z    |
2020-01-15T17:02:18.2050467Z    = note: for more information, see ***/issues/27812
2020-01-15T17:02:18.2050732Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T17:02:18.2051256Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-01-15T17:02:18.2051675Z error: aborting due to 5 previous errors
2020-01-15T17:02:18.2051814Z 
2020-01-15T17:02:18.2052273Z For more information about this error, try `rustc --explain E0658`.
2020-01-15T17:02:18.2052455Z 
---
2020-01-15T17:02:18.2059490Z test result: FAILED. 62 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-01-15T17:02:18.2060170Z 
2020-01-15T17:02:18.2060355Z 
2020-01-15T17:02:18.2060484Z 
2020-01-15T17:02:18.2062687Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-15T17:02:18.2063554Z 
2020-01-15T17:02:18.2063691Z 
2020-01-15T17:02:18.2064383Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-15T17:02:18.2064668Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-15T17:02:18.2064668Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-15T17:02:18.2064847Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-15T17:02:18.2065001Z Build completed unsuccessfully in 1:04:39
2020-01-15T17:02:18.2081582Z == clock drift check ==
2020-01-15T17:02:18.2103112Z   local time: Wed Jan 15 17:02:18 UTC 2020
2020-01-15T17:02:18.5074575Z   network time: Wed, 15 Jan 2020 17:02:18 GMT
2020-01-15T17:02:18.5076650Z == end clock drift check ==
2020-01-15T17:02:19.3847705Z 
2020-01-15T17:02:19.3961704Z ##[error]Bash exited with code '1'.
2020-01-15T17:02:19.4007425Z ##[section]Starting: Checkout
2020-01-15T17:02:19.4015442Z ==============================================================================
2020-01-15T17:02:19.4015515Z Task         : Get sources
2020-01-15T17:02:19.4015573Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
