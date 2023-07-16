plain
2019-12-29T16:04:54.6280930Z test [ui] ui/lifetimes/lifetime-errors/ex3-both-anon-regions.rs ... ok
2019-12-29T16:04:54.6528180Z test [ui] ui/lifetimes/lifetime-errors/liveness-assign-imm-local-notes.rs ... ok
2019-12-29T16:04:54.7244660Z test [ui] ui/lifetimes/lifetime-no-keyword.rs ... ok
2019-12-29T16:04:54.7297940Z test [ui] ui/lifetimes/lifetime-mismatch-between-trait-and-impl.rs ... ok
2019-12-29T16:04:54.7732270Z test [ui] ui/lifetimes/unnamed-closure-doesnt-life-long-enough-issue-67634.rs ... ok
2019-12-29T16:04:55.2620360Z test [ui] ui/linkage-attr/linkage-detect-local-generated-name-collision.rs ... ok
2019-12-29T16:04:55.2720860Z test [ui] ui/link-cfg-works.rs ... ok
2019-12-29T16:04:55.3585670Z test [ui] ui/linkage-attr/linkage-detect-extern-generated-name-collision.rs ... ok
2019-12-29T16:04:55.5132460Z test [ui] ui/linkage-attr/linkage2.rs ... ok
---
2019-12-29T16:10:21.2621270Z ---- [ui] ui/consts/huge-values.rs stdout ----
2019-12-29T16:10:21.2621340Z 
2019-12-29T16:10:21.2621860Z error: test compilation failed although it shouldn't!
2019-12-29T16:10:21.2621950Z status: exit code: 1
2019-12-29T16:10:21.2623460Z command: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/huge-values.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/consts/huge-values" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/consts/huge-values/auxiliary" "-A" "unused"
2019-12-29T16:10:21.2624400Z ------------------------------------------
2019-12-29T16:10:21.2624470Z 
2019-12-29T16:10:21.2624970Z ------------------------------------------
2019-12-29T16:10:21.2625070Z stderr:
2019-12-29T16:10:21.2625070Z stderr:
2019-12-29T16:10:21.2625590Z ------------------------------------------
2019-12-29T16:10:21.2625700Z error: the type `[u8; 4000000000]` is too big for the current architecture
2019-12-29T16:10:21.2626290Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/huge-values.rs:8:13
2019-12-29T16:10:21.2626390Z    |
2019-12-29T16:10:21.2626460Z LL |     let _ = [0u8; 4_000_000_000];
2019-12-29T16:10:21.2626590Z 
2019-12-29T16:10:21.2626650Z error: aborting due to previous error
2019-12-29T16:10:21.2626690Z 
2019-12-29T16:10:21.2626740Z 
---
2019-12-29T16:10:21.2690910Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-29T16:10:21.2691100Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-29T16:10:21.2707110Z 
2019-12-29T16:10:21.2707250Z 
2019-12-29T16:10:21.2710850Z command did not execute successfully: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-29T16:10:21.2712130Z 
2019-12-29T16:10:21.2712190Z 
2019-12-29T16:10:21.2721320Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-29T16:10:21.2721550Z Build completed unsuccessfully in 0:48:37
2019-12-29T16:10:21.2721550Z Build completed unsuccessfully in 0:48:37
2019-12-29T16:10:21.2787200Z == clock drift check ==
2019-12-29T16:10:21.2830690Z   local time: Sun Dec 29 16:10:21 UTC 2019
2019-12-29T16:10:21.3556670Z   network time: Sun, 29 Dec 2019 16:10:21 GMT
2019-12-29T16:10:21.3558520Z == end clock drift check ==
2019-12-29T16:10:21.3598850Z 
2019-12-29T16:10:21.3709750Z ##[error]Bash exited with code '1'.
2019-12-29T16:10:21.3766840Z ##[section]Starting: Checkout
2019-12-29T16:10:21.3769660Z ==============================================================================
2019-12-29T16:10:21.3769750Z Task         : Get sources
2019-12-29T16:10:21.3769840Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
