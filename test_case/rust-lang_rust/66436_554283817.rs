plain
2019-11-15T09:28:29.4875120Z ---- [ui] ui/consts/issue-66397.rs stdout ----
2019-11-15T09:28:29.4875540Z 
2019-11-15T09:28:29.4876350Z error: test compilation failed although it shouldn't!
2019-11-15T09:28:29.4876810Z status: exit code: 1
2019-11-15T09:28:29.4878480Z command: "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.0/work/1/s/src/test/ui/consts/issue-66397.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/test/ui/consts/issue-66397" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/test/ui/consts/issue-66397/auxiliary" "-A" "unused"
2019-11-15T09:28:29.4879940Z ------------------------------------------
2019-11-15T09:28:29.4880340Z 
2019-11-15T09:28:29.4881120Z ------------------------------------------
2019-11-15T09:28:29.4881560Z stderr:
2019-11-15T09:28:29.4881560Z stderr:
2019-11-15T09:28:29.4882330Z ------------------------------------------
2019-11-15T09:28:29.4882770Z error: the type `[i32; 2147483648]` is too big for the current architecture
2019-11-15T09:28:29.4883630Z   --> /Users/runner/runners/2.160.0/work/1/s/src/test/ui/consts/issue-66397.rs:11:5
2019-11-15T09:28:29.4884190Z    |
2019-11-15T09:28:29.4884590Z LL |     [0; SIZE];
2019-11-15T09:28:29.4885190Z 
2019-11-15T09:28:29.4885330Z error: aborting due to previous error
2019-11-15T09:28:29.4885410Z 
2019-11-15T09:28:29.4885490Z 
---
2019-11-15T09:28:29.4966780Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-15T09:28:29.4967310Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-15T09:28:29.6026950Z 
2019-11-15T09:28:29.6027200Z 
2019-11-15T09:28:29.6030820Z command did not execute successfully: "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.160.0/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-15T09:28:29.6032590Z 
2019-11-15T09:28:29.6032650Z 
2019-11-15T09:28:29.6032810Z failed to run: /Users/runner/runners/2.160.0/work/1/s/build/bootstrap/debug/bootstrap test
2019-11-15T09:28:29.6032960Z Build completed unsuccessfully in 0:59:46
2019-11-15T09:28:29.6032960Z Build completed unsuccessfully in 0:59:46
2019-11-15T09:28:29.6033110Z == clock drift check ==
2019-11-15T09:28:29.6033240Z   local time: Fri Nov 15 09:28:29 UTC 2019
2019-11-15T09:28:29.6033390Z   network time: Fri, 15 Nov 2019 09:28:29 GMT
2019-11-15T09:28:29.6033530Z == end clock drift check ==
2019-11-15T09:28:29.6033620Z 
2019-11-15T09:28:29.6120050Z ##[error]Bash exited with code '1'.
2019-11-15T09:28:29.6179220Z ##[section]Starting: Checkout
2019-11-15T09:28:29.6182630Z ==============================================================================
2019-11-15T09:28:29.6182760Z Task         : Get sources
2019-11-15T09:28:29.6182860Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
