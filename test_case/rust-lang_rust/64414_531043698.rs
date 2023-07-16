plain
2019-09-12T22:08:13.1822656Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-12T22:08:13.7856362Z ##[command]git config gc.auto 0
2019-09-12T22:08:13.7859319Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-12T22:08:13.7861291Z ##[command]git config --get-all http.proxy
2019-09-12T22:08:13.7866064Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64414/merge:refs/remotes/pull/64414/merge
---
2019-09-12T23:14:16.2782306Z .................................................................................................... 1500/9013
2019-09-12T23:14:22.6607999Z .................................................................................................... 1600/9013
2019-09-12T23:14:36.1642912Z ...........................................................i...............i........................ 1700/9013
2019-09-12T23:14:44.5217245Z .................................................................................................... 1800/9013
2019-09-12T23:15:00.6266267Z ..................................................iiiii............................................. 1900/9013
2019-09-12T23:15:12.4735746Z .................................................................................................... 2100/9013
2019-09-12T23:15:15.1437359Z .................................................................................................... 2200/9013
2019-09-12T23:15:18.7831071Z .................................................................................................... 2300/9013
2019-09-12T23:15:27.4676350Z .................................................................................................... 2400/9013
---
2019-09-12T23:18:36.9773651Z .....................................i...............i.............................................. 4700/9013
2019-09-12T23:18:49.0798458Z .................................................................................................... 4800/9013
2019-09-12T23:18:56.1035682Z .................................................................................................... 4900/9013
2019-09-12T23:19:07.5934332Z .................................................................................................... 5000/9013
2019-09-12T23:19:14.1921722Z ....................ii.ii........................................................................... 5100/9013
2019-09-12T23:19:25.2917233Z .................................................................................................... 5300/9013
2019-09-12T23:19:36.0250310Z ....................................................................................i............... 5400/9013
2019-09-12T23:19:44.5997093Z .................................................................................................... 5500/9013
2019-09-12T23:19:50.4390523Z .................................................................................................... 5600/9013
2019-09-12T23:19:50.4390523Z .................................................................................................... 5600/9013
2019-09-12T23:20:01.6084658Z ...............................................................................ii...i..i.i.......... 5700/9013
2019-09-12T23:20:17.9456576Z i................................................................................................... 5800/9013
2019-09-12T23:20:38.7109758Z .................................................................................................... 6000/9013
2019-09-12T23:20:38.7109758Z .................................................................................................... 6000/9013
2019-09-12T23:20:46.6306665Z .................................................................................i..ii.............. 6100/9013
2019-09-12T23:21:18.3579917Z .................................................................................................... 6300/9013
2019-09-12T23:21:20.7827542Z ........................................i........................................................... 6400/9013
2019-09-12T23:21:23.1352472Z .................................................................................................... 6500/9013
2019-09-12T23:21:25.7983969Z ............i....................................................................................... 6600/9013
---
2019-09-12T23:25:41.9438418Z .....................i.............................................................................. 9000/9013
2019-09-12T23:25:48.1550256Z .............
2019-09-12T23:25:48.1550998Z failures:
2019-09-12T23:25:48.1609621Z 
2019-09-12T23:25:48.1610324Z ---- [ui] ui/coherence/impl[t]-foreign[local]-for-fundamental[t].rs stdout ----
2019-09-12T23:25:48.1610439Z warning: unused import: `std::rc::Rc`
2019-09-12T23:25:48.1610439Z warning: unused import: `std::rc::Rc`
2019-09-12T23:25:48.1610736Z   --> $DIR/impl[t]-foreign[local]-for-fundamental[t].rs:9:5
2019-09-12T23:25:48.1610830Z LL | use std::rc::Rc;
2019-09-12T23:25:48.1610890Z    |     ^^^^^^^^^^^
2019-09-12T23:25:48.1610933Z    |
2019-09-12T23:25:48.1610979Z    = note: `#[warn(unused_imports)]` on by default
2019-09-12T23:25:48.1610979Z    = note: `#[warn(unused_imports)]` on by default
2019-09-12T23:25:48.1611012Z 
2019-09-12T23:25:48.1611056Z 
2019-09-12T23:25:48.1611081Z 
2019-09-12T23:25:48.1611226Z 
2019-09-12T23:25:48.1611281Z The actual stderr differed from the expected stderr.
2019-09-12T23:25:48.1611677Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[local]-for-fundamental[t]/impl[t]-foreign[local]-for-fundamental[t].stderr
2019-09-12T23:25:48.1611930Z To update references, rerun the tests and pass the `--bless` flag
2019-09-12T23:25:48.1612226Z To only update this specific test, also pass `--test-args coherence/impl[t]-foreign[local]-for-fundamental[t].rs`
2019-09-12T23:25:48.1612339Z error: 1 errors occurred comparing output.
2019-09-12T23:25:48.1612384Z status: exit code: 0
2019-09-12T23:25:48.1612384Z status: exit code: 0
2019-09-12T23:25:48.1613229Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/impl[t]-foreign[local]-for-fundamental[t].rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[local]-for-fundamental[t]/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-name=test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[local]-for-fundamental[t]/auxiliary"
2019-09-12T23:25:48.1613574Z ------------------------------------------
2019-09-12T23:25:48.1613610Z 
2019-09-12T23:25:48.1613832Z ------------------------------------------
2019-09-12T23:25:48.1614127Z stderr:
2019-09-12T23:25:48.1614127Z stderr:
2019-09-12T23:25:48.1614403Z ------------------------------------------
2019-09-12T23:25:48.1614453Z warning: unused import: `std::rc::Rc`
2019-09-12T23:25:48.1614712Z   --> /checkout/src/test/ui/coherence/impl[t]-foreign[local]-for-fundamental[t].rs:9:5
2019-09-12T23:25:48.1614828Z LL | use std::rc::Rc;
2019-09-12T23:25:48.1614873Z    |     ^^^^^^^^^^^
2019-09-12T23:25:48.1614934Z    |
2019-09-12T23:25:48.1614981Z    = note: `#[warn(unused_imports)]` on by default
2019-09-12T23:25:48.1614981Z    = note: `#[warn(unused_imports)]` on by default
2019-09-12T23:25:48.1615013Z 
2019-09-12T23:25:48.1615039Z 
2019-09-12T23:25:48.1615275Z ------------------------------------------
2019-09-12T23:25:48.1615308Z 
2019-09-12T23:25:48.1615335Z 
2019-09-12T23:25:48.1615360Z 
2019-09-12T23:25:48.1615401Z failures:
2019-09-12T23:25:48.1615781Z     [ui] ui/coherence/impl[t]-foreign[local]-for-fundamental[t].rs
2019-09-12T23:25:48.1616118Z test result: FAILED. 8974 passed; 1 failed; 38 ignored; 0 measured; 0 filtered out
2019-09-12T23:25:48.1616166Z 
2019-09-12T23:25:48.1651030Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-12T23:25:48.1651179Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-12T23:25:48.1651179Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-12T23:25:48.1668943Z 
2019-09-12T23:25:48.1669073Z 
2019-09-12T23:25:48.1675162Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-12T23:25:48.1675784Z 
2019-09-12T23:25:48.1675821Z 
2019-09-12T23:25:48.1689913Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-12T23:25:48.1690058Z Build completed unsuccessfully in 1:10:11
2019-09-12T23:25:48.1690058Z Build completed unsuccessfully in 1:10:11
2019-09-12T23:25:48.1745737Z == clock drift check ==
2019-09-12T23:25:48.1759173Z   local time: Thu Sep 12 23:25:48 UTC 2019
2019-09-12T23:25:48.3274180Z   network time: Thu, 12 Sep 2019 23:25:48 GMT
2019-09-12T23:25:48.3281038Z == end clock drift check ==
2019-09-12T23:25:49.2737121Z ##[error]Bash exited with code '1'.
2019-09-12T23:25:49.2782012Z ##[section]Starting: Checkout
2019-09-12T23:25:49.2784001Z ==============================================================================
2019-09-12T23:25:49.2784057Z Task         : Get sources
2019-09-12T23:25:49.2784106Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
