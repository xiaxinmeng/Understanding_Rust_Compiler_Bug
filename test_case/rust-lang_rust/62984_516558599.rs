plain
2019-07-30T18:15:59.7729088Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-30T18:15:59.7896750Z ##[command]git config gc.auto 0
2019-07-30T18:15:59.7959516Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-30T18:16:00.3472139Z ##[command]git config --get-all http.proxy
2019-07-30T18:16:00.3477606Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62984/merge:refs/remotes/pull/62984/merge
---
2019-07-30T18:16:34.3339972Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-30T18:16:34.3340016Z 
2019-07-30T18:16:34.3340196Z   git checkout -b <new-branch-name>
2019-07-30T18:16:34.3340395Z 
2019-07-30T18:16:34.3340438Z HEAD is now at 2bab48858 Merge 5423e88024eac9a7f5bda3d6a8cc8910c30225c7 into f690098e6d65ad7b33dc7fdefccc387806782027
2019-07-30T18:16:34.3489991Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-30T18:16:34.3492190Z ==============================================================================
2019-07-30T18:16:34.3492233Z Task         : Bash
2019-07-30T18:16:34.3492267Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-30T19:14:05.1984639Z .................................................................................................... 1400/8819
2019-07-30T19:14:10.7244880Z .................................................................................................... 1500/8819
2019-07-30T19:14:22.6302113Z ................................................................i...............i................... 1600/8819
2019-07-30T19:14:29.8463495Z .................................................................................................... 1700/8819
2019-07-30T19:14:43.8220217Z ..................................................iiiii............................................. 1800/8819
2019-07-30T19:14:54.3750728Z .................................................................................................... 2000/8819
2019-07-30T19:14:56.7086444Z .................................................................................................... 2100/8819
2019-07-30T19:15:00.0514002Z .................................................................................................... 2200/8819
2019-07-30T19:15:06.2923722Z .................................................................................................... 2300/8819
---
2019-07-30T19:18:46.7791112Z .................................................................................................... 5300/8819
2019-07-30T19:18:53.1333856Z ..............i..................................................................................... 5400/8819
2019-07-30T19:18:58.2300254Z .................................................................................................... 5500/8819
2019-07-30T19:19:09.5553572Z .................................................................................................... 5600/8819
2019-07-30T19:19:22.2732061Z ........ii...i..ii...........i...................................................................... 5700/8819
2019-07-30T19:19:36.9684058Z .................................................................................................... 5900/8819
2019-07-30T19:19:41.5182229Z .................................................................................................... 6000/8819
2019-07-30T19:19:41.5182229Z .................................................................................................... 6000/8819
2019-07-30T19:19:54.5023352Z ........i..ii....................................................................................... 6100/8819
2019-07-30T19:20:12.3465109Z ...................................................i................................................ 6300/8819
2019-07-30T19:20:14.3087790Z .................................................................................................... 6400/8819
2019-07-30T19:20:16.4613309Z .....................i.............................................................................. 6500/8819
2019-07-30T19:20:20.5803474Z .................................................................................................... 6600/8819
---
2019-07-30T19:24:01.4927480Z normalized stderr:
2019-07-30T19:24:01.4927528Z warning: unnecessary trailing semicolons
2019-07-30T19:24:01.4927818Z   --> $DIR/block-expr-precedence.rs:60:21
2019-07-30T19:24:01.4927865Z    |
2019-07-30T19:24:01.4928076Z LL |   if (true) { 12; };;; -num;
2019-07-30T19:24:01.4928145Z    |                     ^^ help: remove these semicolons
2019-07-30T19:24:01.4928192Z    |
2019-07-30T19:24:01.4928239Z    = note: `#[warn(redundant_semicolon)]` on by default
2019-07-30T19:24:01.4928315Z 
2019-07-30T19:24:01.4928341Z 
2019-07-30T19:24:01.4928367Z 
2019-07-30T19:24:01.4928424Z The actual stderr differed from the expected stderr.
2019-07-30T19:24:01.4928424Z The actual stderr differed from the expected stderr.
2019-07-30T19:24:01.4928736Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-expr-precedence/block-expr-precedence.stderr
2019-07-30T19:24:01.4928999Z To update references, rerun the tests and pass the `--bless` flag
2019-07-30T19:24:01.4929252Z To only update this specific test, also pass `--test-args block-expr-precedence.rs`
2019-07-30T19:24:01.4929349Z error: 1 errors occurred comparing output.
2019-07-30T19:24:01.4929404Z status: exit code: 0
2019-07-30T19:24:01.4929404Z status: exit code: 0
2019-07-30T19:24:01.4930278Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-expr-precedence.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-expr-precedence/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-expr-precedence/auxiliary" "-A" "unused"
2019-07-30T19:24:01.4930566Z ------------------------------------------
2019-07-30T19:24:01.4930610Z 
2019-07-30T19:24:01.4930799Z ------------------------------------------
2019-07-30T19:24:01.4930839Z stderr:
2019-07-30T19:24:01.4930839Z stderr:
2019-07-30T19:24:01.4931020Z ------------------------------------------
2019-07-30T19:24:01.4931087Z warning: unnecessary trailing semicolons
2019-07-30T19:24:01.4931291Z   --> /checkout/src/test/ui/block-expr-precedence.rs:60:21
2019-07-30T19:24:01.4931334Z    |
2019-07-30T19:24:01.4931530Z LL |   if (true) { 12; };;; -num;
2019-07-30T19:24:01.4931574Z    |                     ^^ help: remove these semicolons
2019-07-30T19:24:01.4931611Z    |
2019-07-30T19:24:01.4931668Z    = note: `#[warn(redundant_semicolon)]` on by default
2019-07-30T19:24:01.4931717Z 
2019-07-30T19:24:01.4932057Z ------------------------------------------
2019-07-30T19:24:01.4932116Z 
2019-07-30T19:24:01.4932139Z 
---
2019-07-30T19:24:01.4959379Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-30T19:24:01.4959623Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-30T19:24:01.4976410Z 
2019-07-30T19:24:01.4976507Z 
2019-07-30T19:24:01.4978652Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-30T19:24:01.4978949Z 
2019-07-30T19:24:01.4978999Z 
2019-07-30T19:24:01.4984706Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-30T19:24:01.4984771Z Build completed unsuccessfully in 1:01:31
2019-07-30T19:24:01.4984771Z Build completed unsuccessfully in 1:01:31
2019-07-30T19:24:02.1935335Z ##[error]Bash exited with code '1'.
2019-07-30T19:24:02.1973963Z ##[section]Starting: Checkout
2019-07-30T19:24:02.1975520Z ==============================================================================
2019-07-30T19:24:02.1975583Z Task         : Get sources
2019-07-30T19:24:02.1975623Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
