plain
2019-12-16T20:23:04.4682248Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T20:23:04.4887104Z ##[command]git config gc.auto 0
2019-12-16T20:23:04.4958803Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T20:23:04.5010623Z ##[command]git config --get-all http.proxy
2019-12-16T20:23:04.5155459Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67359/merge:refs/remotes/pull/67359/merge
---
2019-12-16T21:23:17.4413274Z .................................................................................................... 1600/9380
2019-12-16T21:23:21.8887621Z .................................................................................................... 1700/9380
2019-12-16T21:23:34.0565228Z ...................................................................i................................ 1800/9380
2019-12-16T21:23:41.2110030Z .................................................................................................... 1900/9380
2019-12-16T21:23:56.5238369Z ....................................................iiiii........................................... 2000/9380
2019-12-16T21:24:06.7026285Z .................................................................................................... 2200/9380
2019-12-16T21:24:09.0647032Z .................................................................................................... 2300/9380
2019-12-16T21:24:12.2754692Z .................................................................................................... 2400/9380
2019-12-16T21:24:34.5459751Z .................................................................................................... 2500/9380
---
2019-12-16T21:27:09.0810981Z .............................................................i...............i...................... 4800/9380
2019-12-16T21:27:16.5716218Z .................................................................................................... 4900/9380
2019-12-16T21:27:25.2352855Z .................................................................................................... 5000/9380
2019-12-16T21:27:30.2783423Z .....i.............................................................................................. 5100/9380
2019-12-16T21:27:40.6658915Z .......................................................................ii.ii...........i............ 5200/9380
2019-12-16T21:27:49.1519842Z .......i............................................................................................ 5400/9380
2019-12-16T21:27:59.1373453Z .................................................................................................... 5500/9380
2019-12-16T21:28:05.5209475Z .....................................................i.............................................. 5600/9380
2019-12-16T21:28:12.3200594Z .................................................................................................... 5700/9380
2019-12-16T21:28:12.3200594Z .................................................................................................... 5700/9380
2019-12-16T21:28:22.2258852Z .................................................................................................... 5800/9380
2019-12-16T21:28:29.2510555Z .........................................ii...i..ii...........i..................................... 5900/9380
2019-12-16T21:28:50.7959791Z .................................................................................................... 6100/9380
2019-12-16T21:28:58.7669609Z .................................................................................................... 6200/9380
2019-12-16T21:28:58.7669609Z .................................................................................................... 6200/9380
2019-12-16T21:29:04.2250279Z ..................................................................i..ii............................. 6300/9380
2019-12-16T21:29:32.2190380Z .................................................................................................... 6500/9380
2019-12-16T21:29:34.2398064Z ......................................i............................................................. 6600/9380
2019-12-16T21:29:36.4074358Z .................................................................................................... 6700/9380
2019-12-16T21:29:38.7230615Z ..............................i..................................................................... 6800/9380
---
2019-12-16T21:31:14.7622087Z .................................................................................................... 7400/9380
2019-12-16T21:31:19.4331469Z .................................................................................................... 7500/9380
2019-12-16T21:31:24.7781658Z .................................................................................................... 7600/9380
2019-12-16T21:31:33.8688378Z .................................................................................................... 7700/9380
2019-12-16T21:31:42.6191122Z ....................................................iiii............................................ 7800/9380
2019-12-16T21:31:57.0068215Z .................................................................................................... 8000/9380
2019-12-16T21:32:03.1736771Z .................................................................................................... 8100/9380
2019-12-16T21:32:18.5331886Z .................................................................................................... 8200/9380
2019-12-16T21:32:26.1861428Z .................................................................................................... 8300/9380
---
2019-12-16T21:34:46.5646098Z  finished in 6.189
2019-12-16T21:34:46.5838489Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-16T21:34:46.7825902Z 
2019-12-16T21:34:46.7826493Z running 166 tests
2019-12-16T21:34:49.7763857Z iiii......i........ii..iiii...i.............................i..i..................i....i............ 100/166
2019-12-16T21:34:52.2799840Z i.i.i...iii..iiiiiii.......................iii............ii......
2019-12-16T21:34:52.2801196Z 
2019-12-16T21:34:52.2801301Z  finished in 5.233
2019-12-16T21:34:52.2801663Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-16T21:34:52.2801728Z 
---
2019-12-16T21:34:53.9349960Z  finished in 2.099
2019-12-16T21:34:53.9545996Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-16T21:34:54.1214714Z 
2019-12-16T21:34:54.1215545Z running 9 tests
2019-12-16T21:34:54.1216579Z iiiiiiiii
2019-12-16T21:34:54.1217268Z 
2019-12-16T21:34:54.1220289Z  finished in 0.167
2019-12-16T21:34:54.1417430Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-16T21:34:54.7810657Z 
---
2019-12-16T21:35:13.3526609Z  finished in 19.212
2019-12-16T21:35:13.3782432Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-16T21:35:13.5650604Z 
2019-12-16T21:35:13.5716718Z running 124 tests
2019-12-16T21:35:37.3203192Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2019-12-16T21:35:41.3737770Z .i.iii.....iiiiii.....ii
2019-12-16T21:35:41.3741592Z 
2019-12-16T21:35:41.3741642Z  finished in 27.996
2019-12-16T21:35:41.3751380Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-16T21:35:41.3751794Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-12-16T21:36:25.4802593Z ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
2019-12-16T21:36:25.4803843Z diff of stderr:
2019-12-16T21:36:25.4804184Z 
2019-12-16T21:36:25.4804324Z 42    |
2019-12-16T21:36:25.4804908Z 43    = note: for more information, see ***/issues/27812
2019-12-16T21:36:25.4805488Z 44    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-12-16T21:36:25.4806080Z +    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2019-12-16T21:36:25.4806656Z 46 error: aborting due to 5 previous errors
2019-12-16T21:36:25.4806948Z 47 
2019-12-16T21:36:25.4807089Z 
2019-12-16T21:36:25.4807208Z 
2019-12-16T21:36:25.4807208Z 
2019-12-16T21:36:25.4807344Z The actual stderr differed from the expected stderr.
2019-12-16T21:36:25.4807907Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2019-12-16T21:36:25.4808780Z To update references, rerun the tests and pass the `--bless` flag
2019-12-16T21:36:25.4809406Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2019-12-16T21:36:25.4809687Z error: 1 errors occurred comparing output.
2019-12-16T21:36:25.4809815Z status: exit code: 1
2019-12-16T21:36:25.4809815Z status: exit code: 1
2019-12-16T21:36:25.4810898Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2019-12-16T21:36:25.4811485Z ------------------------------------------
2019-12-16T21:36:25.4811633Z 
2019-12-16T21:36:25.4811992Z ------------------------------------------
2019-12-16T21:36:25.4812312Z stderr:
2019-12-16T21:36:25.4812312Z stderr:
2019-12-16T21:36:25.4814195Z ------------------------------------------
2019-12-16T21:36:25.4814850Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-12-16T21:36:25.4815949Z    |
2019-12-16T21:36:25.4816104Z LL | extern crate rustc_data_structures;
2019-12-16T21:36:25.4816254Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T21:36:25.4816411Z    |
2019-12-16T21:36:25.4816411Z    |
2019-12-16T21:36:25.4816891Z    = note: for more information, see ***/issues/27812
2019-12-16T21:36:25.4817108Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-12-16T21:36:25.4817237Z 
2019-12-16T21:36:25.4817766Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-12-16T21:36:25.4818400Z    |
2019-12-16T21:36:25.4818562Z LL | extern crate rustc;
2019-12-16T21:36:25.4818706Z    | ^^^^^^^^^^^^^^^^^^^
2019-12-16T21:36:25.4818997Z    |
2019-12-16T21:36:25.4818997Z    |
2019-12-16T21:36:25.4819394Z    = note: for more information, see ***/issues/27812
2019-12-16T21:36:25.4819727Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-12-16T21:36:25.4819884Z 
2019-12-16T21:36:25.4820525Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-12-16T21:36:25.4821742Z    |
2019-12-16T21:36:25.4822066Z LL | extern crate rustc_macros;
2019-12-16T21:36:25.4822213Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T21:36:25.4822338Z    |
2019-12-16T21:36:25.4822338Z    |
2019-12-16T21:36:25.4822742Z    = note: for more information, see ***/issues/27812
2019-12-16T21:36:25.4822918Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-12-16T21:36:25.4823036Z 
2019-12-16T21:36:25.4823690Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-12-16T21:36:25.4824710Z    |
2019-12-16T21:36:25.4824852Z LL | use rustc_macros::HashStable;
2019-12-16T21:36:25.4824978Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T21:36:25.4825707Z    |
2019-12-16T21:36:25.4825707Z    |
2019-12-16T21:36:25.4826216Z    = note: for more information, see ***/issues/27812
2019-12-16T21:36:25.4826406Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-12-16T21:36:25.4826531Z 
2019-12-16T21:36:25.4827069Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-12-16T21:36:25.4827702Z    |
2019-12-16T21:36:25.4827845Z LL | #[derive(HashStable)]
2019-12-16T21:36:25.4827994Z    |          ^^^^^^^^^^
2019-12-16T21:36:25.4828150Z    |
2019-12-16T21:36:25.4828150Z    |
2019-12-16T21:36:25.4828596Z    = note: for more information, see ***/issues/27812
2019-12-16T21:36:25.4829183Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-12-16T21:36:25.4829568Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2019-12-16T21:36:25.4829869Z error: aborting due to 5 previous errors
2019-12-16T21:36:25.4829978Z 
2019-12-16T21:36:25.4830317Z For more information about this error, try `rustc --explain E0658`.
2019-12-16T21:36:25.4830454Z 
---
2019-12-16T21:36:25.4831532Z diff of stderr:
2019-12-16T21:36:25.4831638Z 
2019-12-16T21:36:25.4831979Z 21    | -------------------------- in this macro invocation
2019-12-16T21:36:25.4832161Z 22    |
2019-12-16T21:36:25.4832316Z 23    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-12-16T21:36:25.4832706Z +    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2019-12-16T21:36:25.4832985Z 25 error: aborting due to 2 previous errors
2019-12-16T21:36:25.4833101Z 26 
2019-12-16T21:36:25.4833226Z 
2019-12-16T21:36:25.4833328Z 
2019-12-16T21:36:25.4833328Z 
2019-12-16T21:36:25.4833450Z The actual stderr differed from the expected stderr.
2019-12-16T21:36:25.4833889Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
2019-12-16T21:36:25.4834262Z To update references, rerun the tests and pass the `--bless` flag
2019-12-16T21:36:25.4834673Z To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`
2019-12-16T21:36:25.4834944Z error: 1 errors occurred comparing output.
2019-12-16T21:36:25.4835073Z status: exit code: 1
2019-12-16T21:36:25.4835073Z status: exit code: 1
2019-12-16T21:36:25.4836781Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary" "-A" "unused"
2019-12-16T21:36:25.4837456Z ------------------------------------------
2019-12-16T21:36:25.4837609Z 
2019-12-16T21:36:25.4838078Z ------------------------------------------
2019-12-16T21:36:25.4838308Z stderr:
2019-12-16T21:36:25.4838308Z stderr:
2019-12-16T21:36:25.4838802Z ------------------------------------------
2019-12-16T21:36:25.4839153Z error: implementing `LintPass` by hand
2019-12-16T21:36:25.4839516Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:21:6
2019-12-16T21:36:25.4839678Z    |
2019-12-16T21:36:25.4839824Z LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
2019-12-16T21:36:25.4840054Z    |
2019-12-16T21:36:25.4840185Z note: lint level defined here
2019-12-16T21:36:25.4840555Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
2019-12-16T21:36:25.4840705Z    |
2019-12-16T21:36:25.4840705Z    |
2019-12-16T21:36:25.4840823Z LL | #![deny(rustc::lint_pass_impl_without_macro)]
2019-12-16T21:36:25.4840959Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T21:36:25.4841085Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-12-16T21:36:25.4841316Z error: implementing `LintPass` by hand
2019-12-16T21:36:25.4841687Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:31:14
2019-12-16T21:36:25.4841838Z    |
2019-12-16T21:36:25.4841838Z    |
2019-12-16T21:36:25.4841981Z LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
2019-12-16T21:36:25.4842232Z ...
2019-12-16T21:36:25.4842232Z ...
2019-12-16T21:36:25.4842365Z LL | custom_lint_pass_macro!();
2019-12-16T21:36:25.4842820Z    |
2019-12-16T21:36:25.4842820Z    |
2019-12-16T21:36:25.4842940Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-12-16T21:36:25.4843361Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2019-12-16T21:36:25.4843652Z error: aborting due to 2 previous errors
2019-12-16T21:36:25.4843752Z 
2019-12-16T21:36:25.4843848Z 
2019-12-16T21:36:25.4844183Z ------------------------------------------
---
2019-12-16T21:36:25.4846359Z test result: FAILED. 63 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-16T21:36:25.4846540Z 
2019-12-16T21:36:25.4846679Z 
2019-12-16T21:36:25.4846793Z 
2019-12-16T21:36:25.4848646Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-16T21:36:25.4849258Z 
2019-12-16T21:36:25.4849466Z 
2019-12-16T21:36:25.4849642Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-16T21:36:25.4849851Z Build completed unsuccessfully in 1:07:01
2019-12-16T21:36:25.4849851Z Build completed unsuccessfully in 1:07:01
2019-12-16T21:36:25.4870901Z == clock drift check ==
2019-12-16T21:36:25.4887232Z   local time: Mon Dec 16 21:36:25 UTC 2019
2019-12-16T21:36:26.0396515Z   network time: Mon, 16 Dec 2019 21:36:26 GMT
2019-12-16T21:36:26.0402543Z == end clock drift check ==
2019-12-16T21:36:27.4578536Z 
2019-12-16T21:36:27.4672447Z ##[error]Bash exited with code '1'.
2019-12-16T21:36:27.4731515Z ##[section]Starting: Checkout
2019-12-16T21:36:27.4733025Z ==============================================================================
2019-12-16T21:36:27.4733074Z Task         : Get sources
2019-12-16T21:36:27.4733114Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
