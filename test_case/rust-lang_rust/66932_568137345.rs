plain
2019-12-21T01:01:23.5567820Z ---- [ui] ui/huge-array-simple-32.rs stdout ----
2019-12-21T01:01:23.5568050Z 
2019-12-21T01:01:23.5568350Z error: ui test compiled successfully!
2019-12-21T01:01:23.5568480Z status: exit code: 0
2019-12-21T01:01:23.5569810Z command: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/huge-array-simple-32.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/huge-array-simple-32" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/huge-array-simple-32/auxiliary" "-A" "unused"
2019-12-21T01:01:23.5570700Z ------------------------------------------
2019-12-21T01:01:23.5570780Z 
2019-12-21T01:01:23.5571390Z ------------------------------------------
2019-12-21T01:01:23.5571590Z stderr:
---
2019-12-21T01:01:23.5573310Z ---- [ui] ui/issues/issue-15919-32.rs stdout ----
2019-12-21T01:01:23.5573370Z 
2019-12-21T01:01:23.5573440Z error: ui test compiled successfully!
2019-12-21T01:01:23.5573720Z status: exit code: 0
2019-12-21T01:01:23.5575030Z command: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/issues/issue-15919-32.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/issues/issue-15919-32" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/issues/issue-15919-32/auxiliary" "-A" "unused"
2019-12-21T01:01:23.5575840Z ------------------------------------------
2019-12-21T01:01:23.5575890Z 
2019-12-21T01:01:23.5576370Z ------------------------------------------
2019-12-21T01:01:23.5576460Z stderr:
---
2019-12-21T01:01:23.5651820Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-21T01:01:23.5651980Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-21T01:01:23.5668820Z 
2019-12-21T01:01:23.5669160Z 
2019-12-21T01:01:23.5672580Z command did not execute successfully: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-21T01:01:23.5673470Z 
2019-12-21T01:01:23.5673510Z 
2019-12-21T01:01:23.5681620Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-21T01:01:23.5681780Z Build completed unsuccessfully in 0:48:40
2019-12-21T01:01:23.5681780Z Build completed unsuccessfully in 0:48:40
2019-12-21T01:01:23.5742740Z == clock drift check ==
2019-12-21T01:01:23.5787380Z   local time: Sat Dec 21 01:01:23 UTC 2019
2019-12-21T01:01:23.6439020Z   network time: Sat, 21 Dec 2019 01:01:23 GMT
2019-12-21T01:01:23.6441110Z == end clock drift check ==
2019-12-21T01:01:23.6482650Z 
2019-12-21T01:01:23.6588910Z ##[error]Bash exited with code '1'.
2019-12-21T01:01:23.6628980Z ##[section]Starting: Checkout
2019-12-21T01:01:23.6631340Z ==============================================================================
2019-12-21T01:01:23.6631570Z Task         : Get sources
2019-12-21T01:01:23.6631650Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
