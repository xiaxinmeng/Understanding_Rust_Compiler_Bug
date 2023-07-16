plain
2019-12-06T15:00:23.1716760Z 16 
2019-12-06T15:00:23.1716800Z 
2019-12-06T15:00:23.1716960Z 
2019-12-06T15:00:23.1717040Z The actual stderr differed from the expected stderr.
2019-12-06T15:00:23.1717960Z Actual stderr saved to /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/parser/issue-66357-unexpected-unreachable/issue-66357-unexpected-unreachable.stderr
2019-12-06T15:00:23.1719530Z To update references, rerun the tests and pass the `--bless` flag
2019-12-06T15:00:23.1720280Z To only update this specific test, also pass `--test-args parser/issue-66357-unexpected-unreachable.rs`
2019-12-06T15:00:23.1720560Z error: 1 errors occurred comparing output.
2019-12-06T15:00:23.1720680Z status: exit code: 1
2019-12-06T15:00:23.1720680Z status: exit code: 1
2019-12-06T15:00:23.1722280Z command: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui/parser/issue-66357-unexpected-unreachable.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/parser/issue-66357-unexpected-unreachable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/parser/issue-66357-unexpected-unreachable/auxiliary" "-A" "unused"
2019-12-06T15:00:23.1723880Z ------------------------------------------
2019-12-06T15:00:23.1723970Z 
2019-12-06T15:00:23.1724590Z ------------------------------------------
2019-12-06T15:00:23.1724760Z stderr:
2019-12-06T15:00:23.1724760Z stderr:
2019-12-06T15:00:23.1725340Z ------------------------------------------
2019-12-06T15:00:23.1725880Z error: expected one of `,` or `:`, found `(`
2019-12-06T15:00:23.1726830Z    |
2019-12-06T15:00:23.1726830Z    |
2019-12-06T15:00:23.1726920Z LL | fn f() { |[](* }
2019-12-06T15:00:23.1727210Z    |             ^ expected one of `,` or `:` here
2019-12-06T15:00:23.1727310Z 
2019-12-06T15:00:23.1727940Z error: expected one of `)`, `-`, `_`, `box`, `mut`, `ref`, `|`, identifier, or path, found `*`
2019-12-06T15:00:23.1728790Z    |
2019-12-06T15:00:23.1728790Z    |
2019-12-06T15:00:23.1728860Z LL | fn f() { |[](* }
2019-12-06T15:00:23.1730030Z    |             -^ help: `)` may belong here
2019-12-06T15:00:23.1730220Z    |             unclosed delimiter
2019-12-06T15:00:23.1730270Z 
2019-12-06T15:00:23.1730340Z error: aborting due to 2 previous errors
2019-12-06T15:00:23.1730390Z 
---
2019-12-06T15:00:23.1903830Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-06T15:00:23.1904050Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-06T15:00:23.2821600Z 
2019-12-06T15:00:23.2821970Z 
2019-12-06T15:00:23.2826480Z command did not execute successfully: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-06T15:00:23.2827970Z 
2019-12-06T15:00:23.2828040Z 
2019-12-06T15:00:23.2828170Z failed to run: /Users/runner/runners/2.160.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-06T15:00:23.2828300Z Build completed unsuccessfully in 0:49:10
2019-12-06T15:00:23.2828300Z Build completed unsuccessfully in 0:49:10
2019-12-06T15:00:23.2828420Z == clock drift check ==
2019-12-06T15:00:23.2828530Z   local time: Fri Dec  6 15:00:23 UTC 2019
2019-12-06T15:00:23.2828640Z   network time: Fri, 06 Dec 2019 15:00:23 GMT
2019-12-06T15:00:23.2828760Z == end clock drift check ==
2019-12-06T15:00:23.2828830Z 
2019-12-06T15:00:23.2900090Z ##[error]Bash exited with code '1'.
2019-12-06T15:00:23.2941780Z ##[section]Starting: Checkout
2019-12-06T15:00:23.2944730Z ==============================================================================
2019-12-06T15:00:23.2944840Z Task         : Get sources
2019-12-06T15:00:23.2944930Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
