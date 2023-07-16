plain
2019-11-06T08:11:00.7138765Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-06T08:11:00.7346524Z ##[command]git config gc.auto 0
2019-11-06T08:11:01.2450285Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-06T08:11:01.2453886Z ##[command]git config --get-all http.proxy
2019-11-06T08:11:01.2457416Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66017/merge:refs/remotes/pull/66017/merge
---
2019-11-06T09:21:34.7577381Z .................................................................................................... 1600/9280
2019-11-06T09:21:41.6911758Z .................................................................................................... 1700/9280
2019-11-06T09:21:56.8369074Z ...............................................................i...............i.................... 1800/9280
2019-11-06T09:22:05.8151231Z .................................................................................................... 1900/9280
2019-11-06T09:22:24.4367566Z .....................................................iiiii.......................................... 2000/9280
2019-11-06T09:22:37.6088153Z .................................................................................................... 2200/9280
2019-11-06T09:22:40.7013945Z .................................................................................................... 2300/9280
2019-11-06T09:22:44.7865677Z .................................................................................................... 2400/9280
2019-11-06T09:23:13.1870175Z .................................................................................................... 2500/9280
2019-11-06T09:23:13.1870175Z .................................................................................................... 2500/9280
2019-11-06T09:23:16.6368042Z .................................................................................................... 2600/9280
2019-11-06T09:23:25.8888763Z .................................................................................................... 2700/9280
2019-11-06T09:23:36.4438452Z .....................i.............................................................................. 2800/9280
2019-11-06T09:23:47.2867620Z .................................................................................................... 2900/9280
2019-11-06T09:23:52.6769660Z ....................i............................................................................... 3000/9280
2019-11-06T09:24:02.8973824Z .................................................................................................... 3100/9280
2019-11-06T09:24:09.3980577Z .................................................................................................... 3200/9280
2019-11-06T09:24:20.0036336Z ..ii................................................................................................ 3300/9280
2019-11-06T09:24:39.9989075Z ...............................................................................................i.... 3500/9280
2019-11-06T09:24:48.7149479Z ..........................................i......................................................... 3600/9280
2019-11-06T09:24:56.7367470Z .................................................................................................... 3700/9280
2019-11-06T09:25:04.5715173Z .................................................................................................... 3800/9280
---
2019-11-06T09:26:38.3078419Z .....................................................i...............i.............................. 4800/9280
2019-11-06T09:26:49.4190993Z .................................................................................................... 4900/9280
2019-11-06T09:26:59.9148066Z .................................................................................................... 5000/9280
2019-11-06T09:27:07.4096873Z .................................................................................................... 5100/9280
2019-11-06T09:27:20.4084997Z .....................................................F.ii.ii...........i............................ 5200/9280
2019-11-06T09:27:32.2037635Z .................................................................................................... 5400/9280
2019-11-06T09:27:44.3544121Z .................................................................................................... 5500/9280
2019-11-06T09:27:53.4163072Z ............................i....................................................................... 5600/9280
2019-11-06T09:28:01.5972019Z .................................................................................................... 5700/9280
2019-11-06T09:28:01.5972019Z .................................................................................................... 5700/9280
2019-11-06T09:28:15.9478008Z .................................................................................................... 5800/9280
2019-11-06T09:28:29.9694167Z .............ii...i..ii...........i................................................................. 5900/9280
2019-11-06T09:28:54.3188620Z .................................................................................................... 6100/9280
2019-11-06T09:29:04.6247432Z .................................................................................................... 6200/9280
2019-11-06T09:29:04.6247432Z .................................................................................................... 6200/9280
2019-11-06T09:29:25.3912390Z ................................i..ii............................................................... 6300/9280
2019-11-06T09:29:49.6097520Z ...................................................................................................i 6500/9280
2019-11-06T09:29:52.2400441Z .................................................................................................... 6600/9280
2019-11-06T09:29:54.7770578Z ..............................................................................i..................... 6700/9280
2019-11-06T09:29:57.9520516Z .................................................................................................... 6800/9280
---
2019-11-06T09:32:06.7822133Z .................................................................................................... 7500/9280
2019-11-06T09:32:16.9209600Z .................................................................................................... 7600/9280
2019-11-06T09:32:30.2288711Z .................................................................................................... 7700/9280
2019-11-06T09:32:41.2598567Z .................................................................................................... 7800/9280
2019-11-06T09:32:49.2667034Z .ii......i.......................................................................................... 7900/9280
2019-11-06T09:33:15.5568952Z .................................................................................................... 8100/9280
2019-11-06T09:33:26.2933228Z .................................................................................................... 8200/9280
2019-11-06T09:33:36.3207911Z .................................................................................................... 8300/9280
2019-11-06T09:34:24.4685112Z .................................................................................................... 8400/9280
---
2019-11-06T09:35:51.1985781Z ---- [ui] ui/iterators/into-iter-on-arrays-lint.rs stdout ----
2019-11-06T09:35:51.1986560Z diff of stderr:
2019-11-06T09:35:51.1987973Z 
2019-11-06T09:35:51.1988095Z 34    |
2019-11-06T09:35:51.1988172Z 35    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-11-06T09:35:51.1988818Z 36    = note: for more information, see issue #66145 <***/issues/66145>
2019-11-06T09:35:51.1989135Z 37 
2019-11-06T09:35:51.1989182Z 
2019-11-06T09:35:51.1989230Z 
2019-11-06T09:35:51.1989316Z The actual stderr differed from the expected stderr.
2019-11-06T09:35:51.1989316Z The actual stderr differed from the expected stderr.
2019-11-06T09:35:51.1989902Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/into-iter-on-arrays-lint.stderr
2019-11-06T09:35:51.1990139Z To update references, rerun the tests and pass the `--bless` flag
2019-11-06T09:35:51.1990597Z To only update this specific test, also pass `--test-args iterators/into-iter-on-arrays-lint.rs`
2019-11-06T09:35:51.1993430Z error: 1 errors occurred comparing output.
2019-11-06T09:35:51.2005219Z status: exit code: 0
2019-11-06T09:35:51.2005219Z status: exit code: 0
2019-11-06T09:35:51.2006516Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/auxiliary"
2019-11-06T09:35:51.2008084Z ------------------------------------------
2019-11-06T09:35:51.2008125Z 
2019-11-06T09:35:51.2008356Z ------------------------------------------
2019-11-06T09:35:51.2008402Z stderr:
2019-11-06T09:35:51.2008402Z stderr:
2019-11-06T09:35:51.2008640Z ------------------------------------------
2019-11-06T09:35:51.2008778Z warning: this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added.
2019-11-06T09:35:51.2009061Z   --> /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:9:11
2019-11-06T09:35:51.2009188Z LL |     small.into_iter();
2019-11-06T09:35:51.2009188Z LL |     small.into_iter();
2019-11-06T09:35:51.2009243Z    |           ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
2019-11-06T09:35:51.2009365Z    = note: `#[warn(array_into_iter)]` on by default
2019-11-06T09:35:51.2009365Z    = note: `#[warn(array_into_iter)]` on by default
2019-11-06T09:35:51.2009621Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-11-06T09:35:51.2010349Z    = note: for more information, see issue #66145 <***/issues/66145>
2019-11-06T09:35:51.2010386Z 
2019-11-06T09:35:51.2010445Z warning: this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added.
2019-11-06T09:35:51.2010730Z   --> /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:12:12
2019-11-06T09:35:51.2010815Z LL |     [1, 2].into_iter();
2019-11-06T09:35:51.2010815Z LL |     [1, 2].into_iter();
2019-11-06T09:35:51.2010883Z    |            ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
2019-11-06T09:35:51.2010927Z    |
2019-11-06T09:35:51.2010978Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-11-06T09:35:51.2011426Z    = note: for more information, see issue #66145 <***/issues/66145>
2019-11-06T09:35:51.2011462Z 
2019-11-06T09:35:51.2011521Z warning: this method call currently resolves to `<&[T] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added.
2019-11-06T09:35:51.2011800Z   --> /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:15:9
2019-11-06T09:35:51.2011883Z LL |     big.into_iter();
2019-11-06T09:35:51.2011883Z LL |     big.into_iter();
2019-11-06T09:35:51.2011950Z    |         ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
2019-11-06T09:35:51.2011996Z    |
2019-11-06T09:35:51.2012047Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-11-06T09:35:51.2012351Z    = note: for more information, see issue #66145 <***/issues/66145>
2019-11-06T09:35:51.2012393Z 
2019-11-06T09:35:51.2012458Z warning: this method call currently resolves to `<&[T] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added.
2019-11-06T09:35:51.2012735Z   --> /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:18:15
2019-11-06T09:35:51.2012819Z LL |     [0u8; 33].into_iter();
2019-11-06T09:35:51.2012819Z LL |     [0u8; 33].into_iter();
2019-11-06T09:35:51.2012887Z    |               ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
2019-11-06T09:35:51.2012931Z    |
2019-11-06T09:35:51.2012983Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-11-06T09:35:51.2013283Z    = note: for more information, see issue #66145 <***/issues/66145>
2019-11-06T09:35:51.2013340Z 
2019-11-06T09:35:51.2013565Z ------------------------------------------
2019-11-06T09:35:51.2013603Z 
2019-11-06T09:35:51.2013626Z 
---
2019-11-06T09:35:51.2023217Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-06T09:35:51.2023747Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-06T09:35:51.2040061Z 
2019-11-06T09:35:51.2040183Z 
2019-11-06T09:35:51.2041967Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-06T09:35:51.2042323Z 
2019-11-06T09:35:51.2042353Z 
2019-11-06T09:35:51.2073303Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-06T09:35:51.2073866Z Build completed unsuccessfully in 1:17:37
2019-11-06T09:35:51.2073866Z Build completed unsuccessfully in 1:17:37
2019-11-06T09:35:51.2133662Z == clock drift check ==
2019-11-06T09:35:51.2150256Z   local time: Wed Nov  6 09:35:51 UTC 2019
2019-11-06T09:35:51.3671221Z   network time: Wed, 06 Nov 2019 09:35:51 GMT
2019-11-06T09:35:51.3673192Z == end clock drift check ==
2019-11-06T09:35:52.6907663Z 
2019-11-06T09:35:52.7095105Z ##[error]Bash exited with code '1'.
2019-11-06T09:35:52.7133836Z ##[section]Starting: Checkout
2019-11-06T09:35:52.7137138Z ==============================================================================
2019-11-06T09:35:52.7137223Z Task         : Get sources
2019-11-06T09:35:52.7137271Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
