plain
2020-01-03T23:23:05.9162179Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-03T23:23:05.9429178Z ##[command]git config gc.auto 0
2020-01-03T23:23:05.9513009Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-03T23:23:05.9567462Z ##[command]git config --get-all http.proxy
2020-01-03T23:23:05.9745762Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67846/merge:refs/remotes/pull/67846/merge
---
2020-01-04T00:18:13.6340123Z .................................................................................................... 1500/9472
2020-01-04T00:18:19.4993552Z .................................................................................................... 1600/9472
2020-01-04T00:18:24.8802411Z .................................................................................................... 1700/9472
2020-01-04T00:18:34.6781000Z .................................................................................................... 1800/9472
2020-01-04T00:18:42.9850903Z i................................................................................................... 1900/9472
2020-01-04T00:18:50.0714868Z ......................................................................................iiiii......... 2000/9472
2020-01-04T00:19:12.5095851Z .................................................................................................... 2200/9472
2020-01-04T00:19:14.9865186Z .................................................................................................... 2300/9472
2020-01-04T00:19:17.5526056Z .................................................................................................... 2400/9472
2020-01-04T00:19:23.7995594Z .................................................................................................... 2500/9472
---
2020-01-04T00:22:29.8456337Z ..................i...............i................................................................. 4900/9472
2020-01-04T00:22:39.9366744Z .................................................................................................... 5000/9472
2020-01-04T00:22:46.0748512Z ...............................................................i.................................... 5100/9472
2020-01-04T00:22:54.4644375Z .................................................................................................... 5200/9472
2020-01-04T00:23:02.0177844Z ..............................ii.ii...........i..................................................... 5300/9472
2020-01-04T00:23:11.5665837Z .................................................................................................... 5500/9472
2020-01-04T00:23:21.8807666Z .................................................................................................... 5600/9472
2020-01-04T00:23:28.8520226Z ..............i..................................................................................... 5700/9472
2020-01-04T00:23:34.9904742Z .................................................................................................... 5800/9472
2020-01-04T00:23:34.9904742Z .................................................................................................... 5800/9472
2020-01-04T00:23:46.5547090Z .................................................................................................... 5900/9472
2020-01-04T00:23:58.4750648Z ...ii...i..ii...........i........................................................................... 6000/9472
2020-01-04T00:24:15.8491380Z .................................................................................................... 6200/9472
2020-01-04T00:24:23.7361023Z .................................................................................................... 6300/9472
2020-01-04T00:24:23.7361023Z .................................................................................................... 6300/9472
2020-01-04T00:24:42.5789712Z ..............................i..ii................................................................. 6400/9472
2020-01-04T00:25:03.3318696Z .................................................................................................... 6600/9472
2020-01-04T00:25:05.5908471Z ......i............................................................................................. 6700/9472
2020-01-04T00:25:07.9833402Z .................................................................................................... 6800/9472
2020-01-04T00:25:11.2592910Z .....i.............................................................................................. 6900/9472
---
2020-01-04T00:26:50.2110327Z .................................................................................................... 7500/9472
2020-01-04T00:26:54.9167825Z .................................................................................................... 7600/9472
2020-01-04T00:26:59.9522723Z .................................................................................................... 7700/9472
2020-01-04T00:27:11.1189257Z .................................................................................................... 7800/9472
2020-01-04T00:27:18.9908129Z ........................................iiii........................................................ 7900/9472
2020-01-04T00:27:34.2056769Z .................................................................................................... 8100/9472
2020-01-04T00:27:42.8369506Z .................................................................................................... 8200/9472
2020-01-04T00:27:56.8085605Z .................................................................................................... 8300/9472
2020-01-04T00:28:04.6833510Z .................................................................................................... 8400/9472
---
2020-01-04T00:30:08.0300111Z ---- [ui] ui/pattern/issue-67776-match-same-name-enum-variant-refs.rs stdout ----
2020-01-04T00:30:08.0300384Z diff of stderr:
2020-01-04T00:30:08.0300483Z 
2020-01-04T00:30:08.0300551Z 3    |
2020-01-04T00:30:08.0300641Z 4 LL |         Bar => {},
2020-01-04T00:30:08.0300697Z 5    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-04T00:30:08.0301240Z +    = note: `#[warn(bindings_with_variant_name)]` on by default
2020-01-04T00:30:08.0301290Z 6 
2020-01-04T00:30:08.0301290Z 6 
2020-01-04T00:30:08.0301518Z 7 warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-04T00:30:08.0301994Z 
2020-01-04T00:30:08.0302023Z 
2020-01-04T00:30:08.0302089Z The actual stderr differed from the expected stderr.
2020-01-04T00:30:08.0302489Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-67776-match-same-name-enum-variant-refs/issue-67776-match-same-name-enum-variant-refs.stderr
2020-01-04T00:30:08.0302489Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-67776-match-same-name-enum-variant-refs/issue-67776-match-same-name-enum-variant-refs.stderr
2020-01-04T00:30:08.0302763Z To update references, rerun the tests and pass the `--bless` flag
2020-01-04T00:30:08.0303106Z To only update this specific test, also pass `--test-args pattern/issue-67776-match-same-name-enum-variant-refs.rs`
2020-01-04T00:30:08.0303205Z error: 1 errors occurred comparing output.
2020-01-04T00:30:08.0303483Z status: exit code: 0
2020-01-04T00:30:08.0303483Z status: exit code: 0
2020-01-04T00:30:08.0304536Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/issue-67776-match-same-name-enum-variant-refs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-67776-match-same-name-enum-variant-refs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-67776-match-same-name-enum-variant-refs/auxiliary" "-A" "unused"
2020-01-04T00:30:08.0304950Z ------------------------------------------
2020-01-04T00:30:08.0304991Z 
2020-01-04T00:30:08.0305489Z ------------------------------------------
2020-01-04T00:30:08.0305546Z stderr:
2020-01-04T00:30:08.0305546Z stderr:
2020-01-04T00:30:08.0305956Z ------------------------------------------
2020-01-04T00:30:08.0306189Z warning[E0170]: pattern binding `Bar` is named the same as one of the variants of the type `Foo`
2020-01-04T00:30:08.0306658Z    |
2020-01-04T00:30:08.0306703Z LL |         Bar => {},
2020-01-04T00:30:08.0306703Z LL |         Bar => {},
2020-01-04T00:30:08.0306907Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-04T00:30:08.0307033Z    = note: `#[warn(bindings_with_variant_name)]` on by default
2020-01-04T00:30:08.0307219Z 
2020-01-04T00:30:08.0307219Z 
2020-01-04T00:30:08.0307342Z warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-04T00:30:08.0307942Z    |
2020-01-04T00:30:08.0308038Z LL |         Baz => {},
2020-01-04T00:30:08.0308038Z LL |         Baz => {},
2020-01-04T00:30:08.0308092Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Baz`
2020-01-04T00:30:08.0308127Z 
2020-01-04T00:30:08.0308188Z warning[E0170]: pattern binding `Bar` is named the same as one of the variants of the type `Foo`
2020-01-04T00:30:08.0308852Z    |
2020-01-04T00:30:08.0308948Z LL |         Bar => {},
2020-01-04T00:30:08.0308948Z LL |         Bar => {},
2020-01-04T00:30:08.0309024Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-04T00:30:08.0309059Z 
2020-01-04T00:30:08.0309136Z warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-04T00:30:08.0309570Z    |
2020-01-04T00:30:08.0309769Z LL |         Baz => {},
2020-01-04T00:30:08.0309769Z LL |         Baz => {},
2020-01-04T00:30:08.0310093Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Baz`
2020-01-04T00:30:08.0310153Z 
2020-01-04T00:30:08.0310227Z warning[E0170]: pattern binding `Bar` is named the same as one of the variants of the type `Foo`
2020-01-04T00:30:08.0310966Z    |
2020-01-04T00:30:08.0311060Z LL |         Bar => {},
2020-01-04T00:30:08.0311060Z LL |         Bar => {},
2020-01-04T00:30:08.0311113Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-04T00:30:08.0311174Z 
2020-01-04T00:30:08.0311264Z warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-04T00:30:08.0311842Z    |
2020-01-04T00:30:08.0311953Z LL |         Baz => {},
2020-01-04T00:30:08.0311953Z LL |         Baz => {},
2020-01-04T00:30:08.0312006Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Baz`
2020-01-04T00:30:08.0312079Z 
2020-01-04T00:30:08.0312380Z ------------------------------------------
2020-01-04T00:30:08.0312416Z 
2020-01-04T00:30:08.0312443Z 
---
2020-01-04T00:30:08.0326897Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-04T00:30:08.0327390Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-04T00:30:08.0337769Z 
2020-01-04T00:30:08.0338031Z 
2020-01-04T00:30:08.0340051Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-04T00:30:08.0340634Z 
2020-01-04T00:30:08.0340801Z 
2020-01-04T00:30:08.0346903Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-04T00:30:08.0347309Z Build completed unsuccessfully in 1:00:23
2020-01-04T00:30:08.0347309Z Build completed unsuccessfully in 1:00:23
2020-01-04T00:30:08.0401029Z == clock drift check ==
2020-01-04T00:30:08.0421366Z   local time: Sat Jan  4 00:30:08 UTC 2020
2020-01-04T00:30:08.5715145Z   network time: Sat, 04 Jan 2020 00:30:08 GMT
2020-01-04T00:30:08.5716743Z == end clock drift check ==
2020-01-04T00:30:09.6231772Z 
2020-01-04T00:30:09.6335033Z ##[error]Bash exited with code '1'.
2020-01-04T00:30:09.6410384Z ##[section]Starting: Checkout
2020-01-04T00:30:09.6412313Z ==============================================================================
2020-01-04T00:30:09.6412533Z Task         : Get sources
2020-01-04T00:30:09.6412586Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
