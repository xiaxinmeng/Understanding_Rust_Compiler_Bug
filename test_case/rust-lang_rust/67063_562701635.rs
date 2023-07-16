plain
2019-12-06T19:12:53.7009370Z + 
2019-12-06T19:12:53.7009740Z 
2019-12-06T19:12:53.7010110Z 
2019-12-06T19:12:53.7010560Z The actual stderr differed from the expected stderr.
2019-12-06T19:12:53.7011660Z Actual stderr saved to /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/parser/issue-66357-unexpected-unreachable/issue-66357-unexpected-unreachable.stderr
2019-12-06T19:12:53.7012710Z To update references, rerun the tests and pass the `--bless` flag
2019-12-06T19:12:53.7013770Z To only update this specific test, also pass `--test-args parser/issue-66357-unexpected-unreachable.rs`
2019-12-06T19:12:53.7014690Z error: 1 errors occurred comparing output.
2019-12-06T19:12:53.7015130Z status: exit code: 1
2019-12-06T19:12:53.7015130Z status: exit code: 1
2019-12-06T19:12:53.7016820Z command: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui/parser/issue-66357-unexpected-unreachable.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/parser/issue-66357-unexpected-unreachable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/parser/issue-66357-unexpected-unreachable/auxiliary" "-A" "unused"
2019-12-06T19:12:53.7018460Z ------------------------------------------
2019-12-06T19:12:53.7018890Z 
2019-12-06T19:12:53.7020130Z ------------------------------------------
2019-12-06T19:12:53.7020820Z stderr:
2019-12-06T19:12:53.7020820Z stderr:
2019-12-06T19:12:53.7021720Z ------------------------------------------
2019-12-06T19:12:53.7022220Z error: expected one of `,` or `:`, found `(`
2019-12-06T19:12:53.7023710Z    |
2019-12-06T19:12:53.7023710Z    |
2019-12-06T19:12:53.7024140Z LL | fn f() { |[](* }
2019-12-06T19:12:53.7024560Z    |             ^ expected one of `,` or `:` here
2019-12-06T19:12:53.7024950Z 
2019-12-06T19:12:53.7025890Z error: expected one of `)`, `-`, `_`, `box`, `mut`, `ref`, `|`, identifier, or path, found `*`
2019-12-06T19:12:53.7027460Z    |
2019-12-06T19:12:53.7027460Z    |
2019-12-06T19:12:53.7027870Z LL | fn f() { |[](* }
2019-12-06T19:12:53.7028780Z    |             -^ help: `)` may belong here
2019-12-06T19:12:53.7029700Z    |             unclosed delimiter
2019-12-06T19:12:53.7030080Z 
2019-12-06T19:12:53.7030520Z error: aborting due to 2 previous errors
2019-12-06T19:12:53.7030880Z 
---
2019-12-06T19:12:53.7148210Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-06T19:12:53.7916980Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-06T19:12:53.7917340Z 
2019-12-06T19:12:53.7917560Z 
2019-12-06T19:12:53.7922300Z command did not execute successfully: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-06T19:12:53.7923920Z 
2019-12-06T19:12:53.7924320Z 
2019-12-06T19:12:53.7924760Z failed to run: /Users/runner/runners/2.160.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-06T19:12:53.7925210Z Build completed unsuccessfully in 0:49:08
2019-12-06T19:12:53.7925210Z Build completed unsuccessfully in 0:49:08
2019-12-06T19:12:53.7925650Z == clock drift check ==
2019-12-06T19:12:53.7926340Z   local time: Fri Dec  6 19:12:53 UTC 2019
2019-12-06T19:12:53.7950720Z   network time: Fri, 06 Dec 2019 19:12:53 GMT
2019-12-06T19:12:53.7951360Z == end clock drift check ==
2019-12-06T19:12:53.7951800Z 
2019-12-06T19:12:53.8025290Z ##[error]Bash exited with code '1'.
2019-12-06T19:12:53.8070710Z ##[section]Starting: Checkout
2019-12-06T19:12:53.8073350Z ==============================================================================
2019-12-06T19:12:53.8073470Z Task         : Get sources
2019-12-06T19:12:53.8073550Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
