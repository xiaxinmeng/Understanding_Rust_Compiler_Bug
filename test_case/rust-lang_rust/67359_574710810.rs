plain
2020-01-15T14:14:51.2315290Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-15T14:14:51.2324975Z ##[command]git config gc.auto 0
2020-01-15T14:14:51.2326991Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-15T14:14:51.2328527Z ##[command]git config --get-all http.proxy
2020-01-15T14:14:51.2331190Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67359/merge:refs/remotes/pull/67359/merge
---
2020-01-15T15:12:10.0964945Z .................................................................................................... 1600/9519
2020-01-15T15:12:15.5171732Z .................................................................................................... 1700/9519
2020-01-15T15:12:24.7261680Z .................................................................................................... 1800/9519
2020-01-15T15:12:34.1022980Z ........i........................................................................................... 1900/9519
2020-01-15T15:12:41.3112513Z ..................................................................................................ii 2000/9519
2020-01-15T15:12:58.1786555Z iii................................................................................................. 2100/9519
2020-01-15T15:13:06.7270488Z .................................................................................................... 2300/9519
2020-01-15T15:13:09.3298566Z .................................................................................................... 2400/9519
2020-01-15T15:13:15.3885235Z .................................................................................................... 2500/9519
2020-01-15T15:13:36.3406486Z .................................................................................................... 2600/9519
---
2020-01-15T15:16:24.4906432Z .........................................i...............i.......................................... 4900/9519
2020-01-15T15:16:34.3816726Z .................................................................................................... 5000/9519
2020-01-15T15:16:41.4782411Z ....................................................................................i............... 5100/9519
2020-01-15T15:16:47.3937215Z .................................................................................................... 5200/9519
2020-01-15T15:16:58.3918084Z .......................................................ii.ii...........i............................ 5300/9519
2020-01-15T15:17:08.0891549Z .................................................................................................... 5500/9519
2020-01-15T15:17:18.8546424Z .................................................................................................... 5600/9519
2020-01-15T15:17:25.6994943Z ........................................i........................................................... 5700/9519
2020-01-15T15:17:32.7989679Z .................................................................................................... 5800/9519
2020-01-15T15:17:32.7989679Z .................................................................................................... 5800/9519
2020-01-15T15:17:44.1725817Z .................................................................................................... 5900/9519
2020-01-15T15:17:54.8856510Z ...............................ii...i..ii...........i............................................... 6000/9519
2020-01-15T15:18:14.6911395Z .................................................................................................... 6200/9519
2020-01-15T15:18:23.3349614Z .................................................................................................... 6300/9519
2020-01-15T15:18:23.3349614Z .................................................................................................... 6300/9519
2020-01-15T15:18:33.2734249Z ...........................................................i..ii.................................... 6400/9519
2020-01-15T15:19:01.9009610Z .................................................................................................... 6600/9519
2020-01-15T15:19:04.1441765Z ...................................i................................................................ 6700/9519
2020-01-15T15:19:06.5317985Z .................................................................................................... 6800/9519
2020-01-15T15:19:09.1192404Z ...................................i................................................................ 6900/9519
---
2020-01-15T15:20:50.6563749Z .................................................................................................... 7500/9519
2020-01-15T15:20:55.1656444Z .................................................................................................... 7600/9519
2020-01-15T15:21:01.4216315Z .................................................................................................... 7700/9519
2020-01-15T15:21:08.8312312Z .................................................................................................... 7800/9519
2020-01-15T15:21:19.1143702Z ....................................................................................iiii............ 7900/9519
2020-01-15T15:21:36.3062718Z ..................i......i.......................................................................... 8100/9519
2020-01-15T15:21:41.7511817Z .................................................................................................... 8200/9519
2020-01-15T15:21:55.6932911Z .................................................................................................... 8300/9519
2020-01-15T15:22:06.1276311Z .................................................................................................... 8400/9519
---
2020-01-15T15:24:38.2132873Z  finished in 7.587
2020-01-15T15:24:38.2355472Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T15:24:38.4089800Z 
2020-01-15T15:24:38.4090090Z running 166 tests
2020-01-15T15:24:41.4938291Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-15T15:24:43.8932033Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-15T15:24:43.8933949Z 
2020-01-15T15:24:43.8936986Z  finished in 5.658
2020-01-15T15:24:43.9125652Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T15:24:44.0801112Z 
---
2020-01-15T15:24:46.1320043Z  finished in 2.218
2020-01-15T15:24:46.1508763Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T15:24:46.3156566Z 
2020-01-15T15:24:46.3157296Z running 9 tests
2020-01-15T15:24:46.3159695Z iiiiiiiii
2020-01-15T15:24:46.3160430Z 
2020-01-15T15:24:46.3165742Z  finished in 0.165
2020-01-15T15:24:46.3366073Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T15:24:46.5077484Z 
---
2020-01-15T15:25:07.3141913Z  finished in 20.977
2020-01-15T15:25:07.3358185Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T15:25:08.3133612Z 
2020-01-15T15:25:08.3133824Z running 116 tests
2020-01-15T15:25:34.3203777Z .iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii 100/116
2020-01-15T15:25:37.0202763Z .....iiii.....ii
2020-01-15T15:25:37.0203294Z 
2020-01-15T15:25:37.0206867Z  finished in 29.685
2020-01-15T15:25:37.0212322Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T15:25:37.0212666Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-15T15:26:18.9335649Z ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
2020-01-15T15:26:18.9335898Z diff of stderr:
2020-01-15T15:26:18.9336050Z 
2020-01-15T15:26:18.9336509Z 42    |
2020-01-15T15:26:18.9337386Z 43    = note: for more information, see ***/issues/27812
2020-01-15T15:26:18.9337607Z 44    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T15:26:18.9338031Z +    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-01-15T15:26:18.9338531Z 46 error: aborting due to 5 previous errors
2020-01-15T15:26:18.9338661Z 47 
2020-01-15T15:26:18.9338790Z 
2020-01-15T15:26:18.9338902Z 
2020-01-15T15:26:18.9338902Z 
2020-01-15T15:26:18.9339034Z The actual stderr differed from the expected stderr.
2020-01-15T15:26:18.9339986Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
2020-01-15T15:26:18.9340585Z To update references, rerun the tests and pass the `--bless` flag
2020-01-15T15:26:18.9341785Z To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
2020-01-15T15:26:18.9342160Z error: 1 errors occurred comparing output.
2020-01-15T15:26:18.9342305Z status: exit code: 1
2020-01-15T15:26:18.9342305Z status: exit code: 1
2020-01-15T15:26:18.9343325Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2020-01-15T15:26:18.9344666Z ------------------------------------------
2020-01-15T15:26:18.9345368Z 
2020-01-15T15:26:18.9345861Z ------------------------------------------
2020-01-15T15:26:18.9346233Z stderr:
2020-01-15T15:26:18.9346233Z stderr:
2020-01-15T15:26:18.9346609Z ------------------------------------------
2020-01-15T15:26:18.9347153Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-01-15T15:26:18.9347935Z    |
2020-01-15T15:26:18.9348076Z LL | extern crate rustc_data_structures;
2020-01-15T15:26:18.9348213Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-15T15:26:18.9348365Z    |
2020-01-15T15:26:18.9348365Z    |
2020-01-15T15:26:18.9348799Z    = note: for more information, see ***/issues/27812
2020-01-15T15:26:18.9349027Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T15:26:18.9349180Z 
2020-01-15T15:26:18.9349724Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-01-15T15:26:18.9351070Z    |
2020-01-15T15:26:18.9351640Z LL | extern crate rustc;
2020-01-15T15:26:18.9351939Z    | ^^^^^^^^^^^^^^^^^^^
2020-01-15T15:26:18.9352190Z    |
2020-01-15T15:26:18.9352190Z    |
2020-01-15T15:26:18.9352791Z    = note: for more information, see ***/issues/27812
2020-01-15T15:26:18.9353970Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T15:26:18.9354014Z 
2020-01-15T15:26:18.9354646Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-01-15T15:26:18.9355098Z    |
2020-01-15T15:26:18.9355159Z LL | extern crate rustc_macros;
2020-01-15T15:26:18.9355267Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-15T15:26:18.9355307Z    |
2020-01-15T15:26:18.9355307Z    |
2020-01-15T15:26:18.9355637Z    = note: for more information, see ***/issues/27812
2020-01-15T15:26:18.9355693Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T15:26:18.9355726Z 
2020-01-15T15:26:18.9356104Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-01-15T15:26:18.9356389Z    |
2020-01-15T15:26:18.9356449Z LL | use rustc_macros::HashStable;
2020-01-15T15:26:18.9356492Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-15T15:26:18.9356531Z    |
2020-01-15T15:26:18.9356531Z    |
2020-01-15T15:26:18.9356819Z    = note: for more information, see ***/issues/27812
2020-01-15T15:26:18.9356874Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T15:26:18.9356912Z 
2020-01-15T15:26:18.9357275Z error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2020-01-15T15:26:18.9357583Z    |
2020-01-15T15:26:18.9357623Z LL | #[derive(HashStable)]
2020-01-15T15:26:18.9357752Z    |          ^^^^^^^^^^
2020-01-15T15:26:18.9357791Z    |
2020-01-15T15:26:18.9357791Z    |
2020-01-15T15:26:18.9358068Z    = note: for more information, see ***/issues/27812
2020-01-15T15:26:18.9358128Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2020-01-15T15:26:18.9358425Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-01-15T15:26:18.9358535Z error: aborting due to 5 previous errors
2020-01-15T15:26:18.9358565Z 
2020-01-15T15:26:18.9358815Z For more information about this error, try `rustc --explain E0658`.
2020-01-15T15:26:18.9358857Z 
---
2020-01-15T15:26:18.9359782Z diff of stderr:
2020-01-15T15:26:18.9359808Z 
2020-01-15T15:26:18.9360029Z 21    | -------------------------- in this macro invocation
2020-01-15T15:26:18.9360074Z 22    |
2020-01-15T15:26:18.9360138Z 23    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-15T15:26:18.9360408Z +    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-01-15T15:26:18.9361128Z 25 error: aborting due to 2 previous errors
2020-01-15T15:26:18.9361186Z 26 
2020-01-15T15:26:18.9361214Z 
2020-01-15T15:26:18.9361240Z 
2020-01-15T15:26:18.9361240Z 
2020-01-15T15:26:18.9361302Z The actual stderr differed from the expected stderr.
2020-01-15T15:26:18.9361712Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
2020-01-15T15:26:18.9361964Z To update references, rerun the tests and pass the `--bless` flag
2020-01-15T15:26:18.9362263Z To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`
2020-01-15T15:26:18.9362346Z error: 1 errors occurred comparing output.
2020-01-15T15:26:18.9362407Z status: exit code: 1
2020-01-15T15:26:18.9362407Z status: exit code: 1
2020-01-15T15:26:18.9363438Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary" "-A" "unused"
2020-01-15T15:26:18.9363871Z ------------------------------------------
2020-01-15T15:26:18.9363905Z 
2020-01-15T15:26:18.9364143Z ------------------------------------------
2020-01-15T15:26:18.9364351Z stderr:
2020-01-15T15:26:18.9364351Z stderr:
2020-01-15T15:26:18.9364549Z ------------------------------------------
2020-01-15T15:26:18.9364611Z error: implementing `LintPass` by hand
2020-01-15T15:26:18.9364860Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:20:6
2020-01-15T15:26:18.9364916Z    |
2020-01-15T15:26:18.9364982Z LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
2020-01-15T15:26:18.9365073Z    |
2020-01-15T15:26:18.9365114Z note: lint level defined here
2020-01-15T15:26:18.9365388Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
2020-01-15T15:26:18.9365436Z    |
2020-01-15T15:26:18.9365436Z    |
2020-01-15T15:26:18.9365478Z LL | #![deny(rustc::lint_pass_impl_without_macro)]
2020-01-15T15:26:18.9365725Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-15T15:26:18.9365776Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-15T15:26:18.9365867Z error: implementing `LintPass` by hand
2020-01-15T15:26:18.9366129Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:30:14
2020-01-15T15:26:18.9366178Z    |
2020-01-15T15:26:18.9366178Z    |
2020-01-15T15:26:18.9366226Z LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
2020-01-15T15:26:18.9366341Z ...
2020-01-15T15:26:18.9366341Z ...
2020-01-15T15:26:18.9366384Z LL | custom_lint_pass_macro!();
2020-01-15T15:26:18.9366684Z    |
2020-01-15T15:26:18.9366684Z    |
2020-01-15T15:26:18.9366730Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-15T15:26:18.9367019Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-01-15T15:26:18.9367099Z error: aborting due to 2 previous errors
2020-01-15T15:26:18.9367127Z 
2020-01-15T15:26:18.9367169Z 
2020-01-15T15:26:18.9367379Z ------------------------------------------
---
2020-01-15T15:26:18.9368279Z test result: FAILED. 61 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2020-01-15T15:26:18.9368323Z 
2020-01-15T15:26:18.9368348Z 
2020-01-15T15:26:18.9368374Z 
2020-01-15T15:26:18.9370153Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-15T15:26:18.9370646Z 
2020-01-15T15:26:18.9370955Z 
2020-01-15T15:26:18.9371012Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-15T15:26:18.9371062Z Build completed unsuccessfully in 1:05:18
2020-01-15T15:26:18.9371062Z Build completed unsuccessfully in 1:05:18
2020-01-15T15:26:18.9407829Z == clock drift check ==
2020-01-15T15:26:18.9439269Z   local time: Wed Jan 15 15:26:18 UTC 2020
2020-01-15T15:26:19.8305545Z   network time: Wed, 15 Jan 2020 15:26:19 GMT
2020-01-15T15:26:19.8346821Z == end clock drift check ==
2020-01-15T15:26:20.3831382Z 
2020-01-15T15:26:20.3932347Z ##[error]Bash exited with code '1'.
2020-01-15T15:26:20.3967344Z ##[section]Starting: Checkout
2020-01-15T15:26:20.3969330Z ==============================================================================
2020-01-15T15:26:20.3969388Z Task         : Get sources
2020-01-15T15:26:20.3969438Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
