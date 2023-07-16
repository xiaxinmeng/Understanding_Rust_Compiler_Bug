plain
2020-01-10T18:03:55.5703010Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T18:03:55.5713529Z ##[command]git config gc.auto 0
2020-01-10T18:03:55.5715762Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T18:03:55.5717510Z ##[command]git config --get-all http.proxy
2020-01-10T18:03:55.5719793Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68080/merge:refs/remotes/pull/68080/merge
---
2020-01-10T18:58:29.9861380Z .............................i...............i...................................................... 4900/9503
2020-01-10T18:58:38.9596427Z .................................................................................................... 5000/9503
2020-01-10T18:58:44.9709344Z ..........................................................................i......................... 5100/9503
2020-01-10T18:58:50.6974711Z .................................................................................................... 5200/9503
2020-01-10T18:58:59.3362639Z .........................................ii.ii...........i.......................................... 5300/9503
2020-01-10T18:59:08.4437273Z .................................................................................................... 5500/9503
2020-01-10T18:59:17.5272734Z .................................................................................................... 5600/9503
2020-01-10T18:59:24.0342794Z .........................i.......................................................................... 5700/9503
2020-01-10T18:59:29.7312945Z .................................................................................................... 5800/9503
2020-01-10T18:59:29.7312945Z .................................................................................................... 5800/9503
2020-01-10T18:59:40.1332102Z .................................................................................................... 5900/9503
2020-01-10T18:59:50.3793137Z ................ii...i..ii...........i.............................................................. 6000/9503
2020-01-10T19:00:07.3714548Z .................................................................................................... 6200/9503
2020-01-10T19:00:15.0121873Z .................................................................................................... 6300/9503
2020-01-10T19:00:15.0121873Z .................................................................................................... 6300/9503
2020-01-10T19:00:30.1523488Z ...........................................i..ii.................................................... 6400/9503
2020-01-10T19:00:49.9582037Z .................................................................................................... 6600/9503
2020-01-10T19:00:51.9146943Z ..................i................................................................................. 6700/9503
2020-01-10T19:00:54.1429478Z .................................................................................................... 6800/9503
2020-01-10T19:00:56.5183853Z ..................i................................................................................. 6900/9503
---
2020-01-10T19:02:29.6800273Z .................................................................................................... 7500/9503
2020-01-10T19:02:33.3026020Z .................................................................................................... 7600/9503
2020-01-10T19:02:38.7742342Z .................................................................................................... 7700/9503
2020-01-10T19:02:46.6561824Z .................................................................................................... 7800/9503
2020-01-10T19:02:56.5294795Z .....................................................................iiii........................... 7900/9503
2020-01-10T19:03:11.4870359Z ...i......i......................................................................................... 8100/9503
2020-01-10T19:03:16.9159080Z .................................................................................................... 8200/9503
2020-01-10T19:03:32.1354131Z .................................................................................................... 8300/9503
2020-01-10T19:03:40.6138628Z .................................................................................................... 8400/9503
---
2020-01-10T19:05:55.7420273Z  finished in 6.558
2020-01-10T19:05:55.7609228Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:05:55.9474647Z 
2020-01-10T19:05:55.9475371Z running 166 tests
2020-01-10T19:05:58.8440464Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-10T19:06:00.9884113Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-10T19:06:00.9888761Z 
2020-01-10T19:06:00.9894610Z  finished in 5.228
2020-01-10T19:06:01.0076786Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:06:01.1663224Z 
---
2020-01-10T19:06:03.1932205Z  finished in 2.185
2020-01-10T19:06:03.2109105Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:06:03.3601296Z 
2020-01-10T19:06:03.3602853Z running 9 tests
2020-01-10T19:06:03.3603603Z iiiiiiiii
2020-01-10T19:06:03.3604862Z 
2020-01-10T19:06:03.3604914Z  finished in 0.149
2020-01-10T19:06:03.3814242Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:06:03.5730922Z 
---
2020-01-10T19:06:23.6969388Z  finished in 20.315
2020-01-10T19:06:23.7214890Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:06:23.9064365Z 
2020-01-10T19:06:23.9065405Z running 124 tests
2020-01-10T19:06:47.4607900Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-10T19:06:51.6886017Z .i.iii.....iiiiii.....ii
2020-01-10T19:06:51.6886921Z 
2020-01-10T19:06:51.6891280Z  finished in 27.967
2020-01-10T19:06:51.6897583Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:06:51.6898024Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-10T19:07:30.9426912Z 
2020-01-10T19:07:30.9427285Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2020-01-10T19:07:30.9427348Z diff of stderr:
2020-01-10T19:07:30.9427387Z 
2020-01-10T19:07:30.9427436Z 83 LL |     fn lintmetoo() { }
2020-01-10T19:07:30.9427557Z 85    |
2020-01-10T19:07:30.9427829Z - note: lint level defined here
2020-01-10T19:07:30.9427906Z + note: the lint level is defined here
2020-01-10T19:07:30.9428174Z 87   --> $DIR/lint-tool-test.rs:13:9
2020-01-10T19:07:30.9428174Z 87   --> $DIR/lint-tool-test.rs:13:9
2020-01-10T19:07:30.9428231Z 88    |
2020-01-10T19:07:30.9428297Z 89 LL | #![deny(clippy_group)]
2020-01-10T19:07:30.9428345Z 
2020-01-10T19:07:30.9428376Z 
2020-01-10T19:07:30.9428429Z The actual stderr differed from the expected stderr.
2020-01-10T19:07:30.9428825Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/lint-tool-test.stderr
2020-01-10T19:07:30.9429136Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T19:07:30.9429447Z To only update this specific test, also pass `--test-args lint-tool-test.rs`
2020-01-10T19:07:30.9429560Z error: 1 errors occurred comparing output.
2020-01-10T19:07:30.9429611Z status: exit code: 1
2020-01-10T19:07:30.9429611Z status: exit code: 1
2020-01-10T19:07:30.9430857Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-A" "unused"
2020-01-10T19:07:30.9431341Z ------------------------------------------
2020-01-10T19:07:30.9431384Z 
2020-01-10T19:07:30.9431649Z ------------------------------------------
2020-01-10T19:07:30.9431704Z stderr:
2020-01-10T19:07:30.9431704Z stderr:
2020-01-10T19:07:30.9432063Z ------------------------------------------
2020-01-10T19:07:30.9432451Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T19:07:30.9432827Z    |
2020-01-10T19:07:30.9432827Z    |
2020-01-10T19:07:30.9432879Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-10T19:07:30.9433068Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-10T19:07:30.9433195Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2020-01-10T19:07:30.9433242Z 
2020-01-10T19:07:30.9433242Z 
2020-01-10T19:07:30.9433713Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T19:07:30.9434082Z    |
2020-01-10T19:07:30.9434148Z LL | #![deny(clippy_group)]
2020-01-10T19:07:30.9434205Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-10T19:07:30.9434241Z 
2020-01-10T19:07:30.9434241Z 
2020-01-10T19:07:30.9434640Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T19:07:30.9435001Z    |
2020-01-10T19:07:30.9435001Z    |
2020-01-10T19:07:30.9435063Z LL | #[allow(test_group)]
2020-01-10T19:07:30.9435136Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-10T19:07:30.9435174Z 
2020-01-10T19:07:30.9435233Z warning: unknown lint: `this_lint_does_not_exist`
2020-01-10T19:07:30.9435602Z    |
2020-01-10T19:07:30.9435602Z    |
2020-01-10T19:07:30.9435660Z LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
2020-01-10T19:07:30.9435784Z    |
2020-01-10T19:07:30.9435836Z    = note: `#[warn(unknown_lints)]` on by default
2020-01-10T19:07:30.9435870Z 
2020-01-10T19:07:30.9435870Z 
2020-01-10T19:07:30.9436265Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T19:07:30.9436618Z    |
2020-01-10T19:07:30.9436618Z    |
2020-01-10T19:07:30.9436696Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-10T19:07:30.9436755Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-10T19:07:30.9436792Z 
2020-01-10T19:07:30.9437179Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T19:07:30.9437555Z    |
2020-01-10T19:07:30.9437605Z LL | #![deny(clippy_group)]
2020-01-10T19:07:30.9437678Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-10T19:07:30.9437714Z 
2020-01-10T19:07:30.9437714Z 
2020-01-10T19:07:30.9438084Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T19:07:30.9438450Z    |
2020-01-10T19:07:30.9438450Z    |
2020-01-10T19:07:30.9438499Z LL | #[allow(test_group)]
2020-01-10T19:07:30.9438581Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-10T19:07:30.9438618Z 
2020-01-10T19:07:30.9439220Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-10T19:07:30.9439653Z    |
2020-01-10T19:07:30.9439653Z    |
2020-01-10T19:07:30.9439703Z LL | #![plugin(lint_tool_test)]
2020-01-10T19:07:30.9439833Z    |
2020-01-10T19:07:30.9439886Z    = note: `#[warn(deprecated)]` on by default
2020-01-10T19:07:30.9439921Z 
2020-01-10T19:07:30.9439921Z 
2020-01-10T19:07:30.9440316Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T19:07:30.9440673Z    |
2020-01-10T19:07:30.9440673Z    |
2020-01-10T19:07:30.9440740Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-10T19:07:30.9440897Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-10T19:07:30.9440934Z 
2020-01-10T19:07:30.9441351Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T19:07:30.9441736Z    |
2020-01-10T19:07:30.9441786Z LL | #![deny(clippy_group)]
2020-01-10T19:07:30.9441859Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-10T19:07:30.9441895Z 
2020-01-10T19:07:30.9441895Z 
2020-01-10T19:07:30.9442144Z error: item is named 'lintme'
2020-01-10T19:07:30.9442496Z    |
2020-01-10T19:07:30.9442496Z    |
2020-01-10T19:07:30.9442774Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-10T19:07:30.9442901Z    |
2020-01-10T19:07:30.9442951Z note: the lint level is defined here
2020-01-10T19:07:30.9443252Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
2020-01-10T19:07:30.9443326Z    |
2020-01-10T19:07:30.9443326Z    |
2020-01-10T19:07:30.9443377Z LL | #![deny(clippy_group)]
2020-01-10T19:07:30.9443439Z    |         ^^^^^^^^^^^^
2020-01-10T19:07:30.9443514Z    = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`
2020-01-10T19:07:30.9443552Z 
2020-01-10T19:07:30.9443804Z error: item is named 'lintmetoo'
2020-01-10T19:07:30.9444160Z    |
2020-01-10T19:07:30.9444160Z    |
2020-01-10T19:07:30.9444446Z LL |     fn lintmetoo() { } //~ ERROR item is named 'lintmetoo'
2020-01-10T19:07:30.9444571Z    |
2020-01-10T19:07:30.9444622Z note: the lint level is defined here
2020-01-10T19:07:30.9444914Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
2020-01-10T19:07:30.9444968Z    |
2020-01-10T19:07:30.9444968Z    |
2020-01-10T19:07:30.9445035Z LL | #![deny(clippy_group)]
2020-01-10T19:07:30.9445097Z    |         ^^^^^^^^^^^^
2020-01-10T19:07:30.9445156Z    = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`
2020-01-10T19:07:30.9445210Z 
2020-01-10T19:07:30.9445598Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T19:07:30.9445967Z    |
2020-01-10T19:07:30.9445967Z    |
2020-01-10T19:07:30.9446016Z LL | #[allow(test_group)]
2020-01-10T19:07:30.9446072Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-10T19:07:30.9446175Z error: aborting due to 2 previous errors
2020-01-10T19:07:30.9446209Z 
2020-01-10T19:07:30.9446239Z 
2020-01-10T19:07:30.9446502Z ------------------------------------------
---
2020-01-10T19:07:30.9447747Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-10T19:07:30.9447842Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-10T19:07:30.9455134Z 
2020-01-10T19:07:30.9455214Z 
2020-01-10T19:07:30.9460689Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-10T19:07:30.9461150Z 
2020-01-10T19:07:30.9461200Z 
2020-01-10T19:07:30.9522012Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-10T19:07:30.9522100Z Build completed unsuccessfully in 0:57:45
2020-01-10T19:07:30.9522100Z Build completed unsuccessfully in 0:57:45
2020-01-10T19:07:30.9525954Z == clock drift check ==
2020-01-10T19:07:30.9550799Z   local time: Fri Jan 10 19:07:30 UTC 2020
2020-01-10T19:07:31.0350796Z   network time: Fri, 10 Jan 2020 19:07:31 GMT
2020-01-10T19:07:31.0354488Z == end clock drift check ==
2020-01-10T19:07:31.8972474Z 
2020-01-10T19:07:31.9074486Z ##[error]Bash exited with code '1'.
2020-01-10T19:07:31.9120446Z ##[section]Starting: Checkout
2020-01-10T19:07:31.9122417Z ==============================================================================
2020-01-10T19:07:31.9122479Z Task         : Get sources
2020-01-10T19:07:31.9122534Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
