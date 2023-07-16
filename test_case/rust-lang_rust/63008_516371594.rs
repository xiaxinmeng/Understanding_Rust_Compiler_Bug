plain
2019-07-30T09:50:35.3380662Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-30T09:50:35.3553822Z ##[command]git config gc.auto 0
2019-07-30T09:50:35.3626461Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-30T09:50:35.3678134Z ##[command]git config --get-all http.proxy
2019-07-30T09:50:35.3811543Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63008/merge:refs/remotes/pull/63008/merge
---
2019-07-30T09:51:09.2529702Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-30T09:51:09.2529727Z 
2019-07-30T09:51:09.2529891Z   git checkout -b <new-branch-name>
2019-07-30T09:51:09.2529914Z 
2019-07-30T09:51:09.2529952Z HEAD is now at 350964d60 Merge f1d1ba5646c9a774e049d20dad3b18c66e1ae3bc into 4eeaaa722d6ac6d24de6e4d3faefb7c44e674b37
2019-07-30T09:51:09.2667118Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-30T09:51:09.2669623Z ==============================================================================
2019-07-30T09:51:09.2669669Z Task         : Bash
2019-07-30T09:51:09.2669707Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-30T10:49:05.8072150Z .................................................................................................... 1400/8813
2019-07-30T10:49:11.1687084Z .................................................................................................... 1500/8813
2019-07-30T10:49:22.7442881Z ................................................................i...............i................... 1600/8813
2019-07-30T10:49:29.7601262Z .................................................................................................... 1700/8813
2019-07-30T10:49:43.1703845Z ..................................................iiiii............................................. 1800/8813
2019-07-30T10:49:53.4827053Z .................................................................................................... 2000/8813
2019-07-30T10:49:55.8357892Z .................................................................................................... 2100/8813
2019-07-30T10:49:59.0141445Z .................................................................................................... 2200/8813
2019-07-30T10:50:05.1032492Z .................................................................................................... 2300/8813
---
2019-07-30T10:53:41.8199509Z .................................................................................................... 5300/8813
2019-07-30T10:53:48.3887208Z ........i........................................................................................... 5400/8813
2019-07-30T10:53:53.4615810Z .................................................................................................... 5500/8813
2019-07-30T10:54:05.0971074Z .................................................................................................... 5600/8813
2019-07-30T10:54:18.0103539Z ..ii...i..ii...........i............................................................................ 5700/8813
2019-07-30T10:54:31.1785456Z .................................................................................................... 5900/8813
2019-07-30T10:54:35.4802979Z .................................................................................................... 6000/8813
2019-07-30T10:54:35.4802979Z .................................................................................................... 6000/8813
2019-07-30T10:54:48.7432821Z ..i..ii............................................................................................. 6100/8813
2019-07-30T10:55:06.0155549Z .............................................i...................................................... 6300/8813
2019-07-30T10:55:07.9962249Z .................................................................................................... 6400/8813
2019-07-30T10:55:10.3392386Z ...............i.................................................................................... 6500/8813
2019-07-30T10:55:14.4307528Z .................................................................................................... 6600/8813
---
2019-07-30T10:59:30.3293345Z  finished in 19.104
2019-07-30T10:59:30.3443806Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-30T10:59:30.4866468Z 
2019-07-30T10:59:30.4866900Z running 146 tests
2019-07-30T10:59:33.4667607Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-07-30T10:59:35.2207571Z iii..............i.........iii.i......ii......
2019-07-30T10:59:35.2208212Z 
2019-07-30T10:59:35.2209979Z  finished in 4.876
2019-07-30T10:59:35.2386688Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-30T10:59:35.3829060Z 
---
2019-07-30T10:59:37.2788987Z  finished in 2.040
2019-07-30T10:59:37.2948745Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-30T10:59:37.4298130Z 
2019-07-30T10:59:37.4298272Z running 9 tests
2019-07-30T10:59:37.4298885Z iiiiiiiii
2019-07-30T10:59:37.4299266Z 
2019-07-30T10:59:37.4303201Z  finished in 0.135
2019-07-30T10:59:37.4462661Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-30T10:59:37.5962034Z 
---
2019-07-30T10:59:54.7576474Z  finished in 16.865
2019-07-30T10:59:54.7576704Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-30T10:59:54.7576764Z 
2019-07-30T10:59:54.7576803Z running 122 tests
2019-07-30T11:00:16.0595880Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-07-30T11:00:20.1947657Z .i.i......iii.i.....ii
2019-07-30T11:00:20.1949154Z 
2019-07-30T11:00:20.1950518Z  finished in 25.866
2019-07-30T11:00:20.1953152Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-30T11:00:20.1953637Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-07-30T11:01:14.7159404Z 
2019-07-30T11:01:14.7159442Z 8 LL | extern crate rustc_data_structures;
2019-07-30T11:01:14.7159479Z 9    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-30T11:01:14.7159513Z 10    |
2019-07-30T11:01:14.7160237Z -    = note: for more information, see issue #27812 <***/issues/27812>
2019-07-30T11:01:14.7160540Z +    = note: see issue #27812 <***/issues/27812> for more information
2019-07-30T11:01:14.7160612Z 12    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-30T11:01:14.7160650Z 13 
2019-07-30T11:01:14.7160960Z 14 error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-30T11:01:14.7161054Z 17 LL | extern crate rustc;
2019-07-30T11:01:14.7161097Z 18    | ^^^^^^^^^^^^^^^^^^^
2019-07-30T11:01:14.7161147Z 19    |
2019-07-30T11:01:14.7161147Z 19    |
2019-07-30T11:01:14.7161385Z -    = note: for more information, see issue #27812 <***/issues/27812>
2019-07-30T11:01:14.7161642Z +    = note: see issue #27812 <***/issues/27812> for more information
2019-07-30T11:01:14.7161690Z 21    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-30T11:01:14.7161725Z 22 
2019-07-30T11:01:14.7162046Z 23 error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-30T11:01:14.7162118Z 26 LL | extern crate rustc_macros;
2019-07-30T11:01:14.7162173Z 27    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-30T11:01:14.7162206Z 28    |
2019-07-30T11:01:14.7162206Z 28    |
2019-07-30T11:01:14.7162435Z -    = note: for more information, see issue #27812 <***/issues/27812>
2019-07-30T11:01:14.7162690Z +    = note: see issue #27812 <***/issues/27812> for more information
2019-07-30T11:01:14.7162743Z 30    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-30T11:01:14.7162780Z 31 
2019-07-30T11:01:14.7163096Z 32 error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-30T11:01:14.7163166Z 35 LL | use rustc_macros::HashStable;
2019-07-30T11:01:14.7163228Z 36    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-30T11:01:14.7163262Z 37    |
2019-07-30T11:01:14.7163262Z 37    |
2019-07-30T11:01:14.7163495Z -    = note: for more information, see issue #27812 <***/issues/27812>
2019-07-30T11:01:14.7163751Z +    = note: see issue #27812 <***/issues/27812> for more information
2019-07-30T11:01:14.7163797Z 39    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-30T11:01:14.7163852Z 40 
2019-07-30T11:01:14.7164593Z 41 error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-30T11:01:14.7164724Z 44 LL | #[derive(HashStable)]
2019-07-30T11:01:14.7164770Z 45    |          ^^^^^^^^^^
2019-07-30T11:01:14.7164813Z 46    |
2019-07-30T11:01:14.7164813Z 46    |
2019-07-30T11:01:14.7165142Z -    = note: for more information, see issue #27812 <***/issues/27812>
2019-07-30T11:01:14.7165615Z +    = note: see issue #27812 <***/issues/27812> for more information
2019-07-30T11:01:14.7165687Z 48    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-30T11:01:14.7165802Z 50 error: aborting due to 6 previous errors
2019-07-30T11:01:14.7165833Z 
2019-07-30T11:01:14.7165859Z 
2019-07-30T11:01:14.7165922Z The actual stderr differed from the expected stderr.
2019-07-30T11:01:14.7165922Z The actual stderr differed from the expected stderr.
2019-07-30T11:01:14.7166297Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2019-07-30T11:01:14.7166665Z To update references, rerun the tests and pass the `--bless` flag
2019-07-30T11:01:14.7166948Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2019-07-30T11:01:14.7167031Z error: 1 errors occurred comparing output.
2019-07-30T11:01:14.7167095Z status: exit code: 1
2019-07-30T11:01:14.7167095Z status: exit code: 1
2019-07-30T11:01:14.7167953Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2019-07-30T11:01:14.7168409Z ------------------------------------------
2019-07-30T11:01:14.7168437Z 
2019-07-30T11:01:14.7168605Z ------------------------------------------
2019-07-30T11:01:14.7168660Z stderr:
2019-07-30T11:01:14.7168660Z stderr:
2019-07-30T11:01:14.7168825Z ------------------------------------------
2019-07-30T11:01:14.7168867Z error[E0601]: `main` function not found in crate `hash_stable_is_unstable`
2019-07-30T11:01:14.7168922Z    |
2019-07-30T11:01:14.7169151Z    = note: consider adding a `main` function to `/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs`
2019-07-30T11:01:14.7169183Z 
2019-07-30T11:01:14.7169492Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-30T11:01:14.7169753Z    |
2019-07-30T11:01:14.7169795Z LL | extern crate rustc_data_structures;
2019-07-30T11:01:14.7169832Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-30T11:01:14.7169865Z    |
2019-07-30T11:01:14.7169865Z    |
2019-07-30T11:01:14.7170120Z    = note: see issue #27812 <***/issues/27812> for more information
2019-07-30T11:01:14.7170166Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-30T11:01:14.7170193Z 
2019-07-30T11:01:14.7170514Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-30T11:01:14.7170753Z    |
2019-07-30T11:01:14.7170800Z LL | extern crate rustc;
2019-07-30T11:01:14.7170835Z    | ^^^^^^^^^^^^^^^^^^^
2019-07-30T11:01:14.7209630Z    |
2019-07-30T11:01:14.7209630Z    |
2019-07-30T11:01:14.7210094Z    = note: see issue #27812 <***/issues/27812> for more information
2019-07-30T11:01:14.7210160Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-30T11:01:14.7210203Z 
2019-07-30T11:01:14.7210524Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-30T11:01:14.7210783Z    |
2019-07-30T11:01:14.7210954Z LL | extern crate rustc_macros;
2019-07-30T11:01:14.7211001Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-30T11:01:14.7211051Z    |
2019-07-30T11:01:14.7211051Z    |
2019-07-30T11:01:14.7211330Z    = note: see issue #27812 <***/issues/27812> for more information
2019-07-30T11:01:14.7211378Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-30T11:01:14.7211420Z 
2019-07-30T11:01:14.7211719Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-30T11:01:14.7212084Z    |
2019-07-30T11:01:14.7212118Z LL | use rustc_macros::HashStable;
2019-07-30T11:01:14.7212154Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-30T11:01:14.7212202Z    |
2019-07-30T11:01:14.7212202Z    |
2019-07-30T11:01:14.7212437Z    = note: see issue #27812 <***/issues/27812> for more information
2019-07-30T11:01:14.7212493Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-30T11:01:14.7212536Z 
2019-07-30T11:01:14.7212836Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-30T11:01:14.7213092Z    |
2019-07-30T11:01:14.7213125Z LL | #[derive(HashStable)]
2019-07-30T11:01:14.7213167Z    |          ^^^^^^^^^^
2019-07-30T11:01:14.7213200Z    |
2019-07-30T11:01:14.7213200Z    |
2019-07-30T11:01:14.7213444Z    = note: see issue #27812 <***/issues/27812> for more information
2019-07-30T11:01:14.7213491Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-30T11:01:14.7213566Z error: aborting due to 6 previous errors
2019-07-30T11:01:14.7213589Z 
2019-07-30T11:01:14.7213623Z Some errors have detailed explanations: E0601, E0658.
2019-07-30T11:01:14.7213846Z For more information about an error, try `rustc --explain E0601`.
2019-07-30T11:01:14.7213846Z For more information about an error, try `rustc --explain E0601`.
2019-07-30T11:01:14.7213874Z 
2019-07-30T11:01:14.7214426Z ------------------------------------------
2019-07-30T11:01:14.7214466Z 
2019-07-30T11:01:14.7214492Z 
2019-07-30T11:01:14.7214769Z ---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
2019-07-30T11:01:14.7214818Z diff of stderr:
2019-07-30T11:01:14.7214846Z 
2019-07-30T11:01:14.7214890Z 4 LL | #![plugin(attr_plugin_test)]
2019-07-30T11:01:14.7215008Z 6    |
2019-07-30T11:01:14.7215008Z 6    |
2019-07-30T11:01:14.7215343Z -    = note: for more information, see issue #29597 <***/issues/29597>
2019-07-30T11:01:14.7215654Z +    = note: see issue #29597 <***/issues/29597> for more information
2019-07-30T11:01:14.7215713Z 8    = help: add `#![feature(plugin)]` to the crate attributes to enable
2019-07-30T11:01:14.7215822Z 10 error: aborting due to previous error
2019-07-30T11:01:14.7215852Z 
2019-07-30T11:01:14.7215878Z 
2019-07-30T11:01:14.7215950Z The actual stderr differed from the expected stderr.
2019-07-30T11:01:14.7215950Z The actual stderr differed from the expected stderr.
2019-07-30T11:01:14.7216257Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/gated-plugin.stderr
2019-07-30T11:01:14.7216504Z To update references, rerun the tests and pass the `--bless` flag
2019-07-30T11:01:14.7216779Z To only update this specific test, also pass `--test-args gated-plugin.rs`
2019-07-30T11:01:14.7216859Z error: 1 errors occurred comparing output.
2019-07-30T11:01:14.7216914Z status: exit code: 1
2019-07-30T11:01:14.7216914Z status: exit code: 1
2019-07-30T11:01:14.7217874Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/gated-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-A" "unused"
2019-07-30T11:01:14.7218183Z ------------------------------------------
2019-07-30T11:01:14.7218212Z 
2019-07-30T11:01:14.7218388Z ------------------------------------------
2019-07-30T11:01:14.7218441Z stderr:
2019-07-30T11:01:14.7218441Z stderr:
2019-07-30T11:01:14.7218615Z ------------------------------------------
2019-07-30T11:01:14.7218657Z error[E0658]: compiler plugins are experimental and possibly buggy
2019-07-30T11:01:14.7218953Z   --> /checkout/src/test/ui-fulldeps/gated-plugin.rs:3:1
2019-07-30T11:01:14.7218995Z    |
2019-07-30T11:01:14.7219030Z LL | #![plugin(attr_plugin_test)]
2019-07-30T11:01:14.7219118Z    |
2019-07-30T11:01:14.7219118Z    |
2019-07-30T11:01:14.7219362Z    = note: see issue #29597 <***/issues/29597> for more information
2019-07-30T11:01:14.7219432Z    = help: add `#![feature(plugin)]` to the crate attributes to enable
2019-07-30T11:01:14.7219493Z error: aborting due to previous error
2019-07-30T11:01:14.7219517Z 
2019-07-30T11:01:14.7219737Z For more information about this error, try `rustc --explain E0658`.
2019-07-30T11:01:14.7219766Z 
---
2019-07-30T11:01:14.7262365Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-30T11:01:14.7262435Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-30T11:01:14.7262478Z 
2019-07-30T11:01:14.7262500Z 
2019-07-30T11:01:14.7264461Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-30T11:01:14.7264785Z 
2019-07-30T11:01:14.7264817Z 
2019-07-30T11:01:14.7264873Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-30T11:01:14.7264927Z Build completed unsuccessfully in 1:04:02
2019-07-30T11:01:14.7264927Z Build completed unsuccessfully in 1:04:02
2019-07-30T11:01:15.6431063Z ##[error]Bash exited with code '1'.
2019-07-30T11:01:15.6469614Z ##[section]Starting: Checkout
2019-07-30T11:01:15.6473047Z ==============================================================================
2019-07-30T11:01:15.6473105Z Task         : Get sources
2019-07-30T11:01:15.6473154Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
