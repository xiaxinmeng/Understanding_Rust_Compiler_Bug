plain
2019-12-06T04:28:39.1625850Z diff of stderr:
2019-12-06T04:28:39.1625890Z 
2019-12-06T04:28:39.1626440Z 2   --> $DIR/issue-66357-unexpected-unreachable.rs:12:13
2019-12-06T04:28:39.1626530Z 3    |
2019-12-06T04:28:39.1626610Z 4 LL | fn f() { |[](* }
2019-12-06T04:28:39.1627150Z -    |             ^ expected one of `,` or `:`
2019-12-06T04:28:39.1627260Z +    |             ^ expected one of `,` or `:` here
2019-12-06T04:28:39.1627340Z 6 
2019-12-06T04:28:39.1627910Z 7 error: expected one of `)`, `-`, `_`, `box`, `mut`, `ref`, `|`, identifier, or path, found `*`
2019-12-06T04:28:39.1628570Z 
2019-12-06T04:28:39.1628620Z 
2019-12-06T04:28:39.1628700Z The actual stderr differed from the expected stderr.
2019-12-06T04:28:39.1629420Z Actual stderr saved to /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/parser/issue-66357-unexpected-unreachable/issue-66357-unexpected-unreachable.stderr
2019-12-06T04:28:39.1629420Z Actual stderr saved to /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/parser/issue-66357-unexpected-unreachable/issue-66357-unexpected-unreachable.stderr
2019-12-06T04:28:39.1630030Z To update references, rerun the tests and pass the `--bless` flag
2019-12-06T04:28:39.1630650Z To only update this specific test, also pass `--test-args parser/issue-66357-unexpected-unreachable.rs`
2019-12-06T04:28:39.1630820Z error: 1 errors occurred comparing output.
2019-12-06T04:28:39.1630890Z status: exit code: 1
2019-12-06T04:28:39.1630890Z status: exit code: 1
2019-12-06T04:28:39.1632380Z command: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui/parser/issue-66357-unexpected-unreachable.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/parser/issue-66357-unexpected-unreachable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/parser/issue-66357-unexpected-unreachable/auxiliary" "-A" "unused"
2019-12-06T04:28:39.1633330Z ------------------------------------------
2019-12-06T04:28:39.1633400Z 
2019-12-06T04:28:39.1633910Z ------------------------------------------
2019-12-06T04:28:39.1634020Z stderr:
2019-12-06T04:28:39.1634020Z stderr:
2019-12-06T04:28:39.1634540Z ------------------------------------------
2019-12-06T04:28:39.1634630Z error: expected one of `,` or `:`, found `(`
2019-12-06T04:28:39.1635340Z    |
2019-12-06T04:28:39.1635340Z    |
2019-12-06T04:28:39.1635430Z LL | fn f() { |[](* }
2019-12-06T04:28:39.1635500Z    |             ^ expected one of `,` or `:` here
2019-12-06T04:28:39.1635570Z 
2019-12-06T04:28:39.1636150Z error: expected one of `)`, `-`, `_`, `box`, `mut`, `ref`, `|`, identifier, or path, found `*`
2019-12-06T04:28:39.1636910Z    |
2019-12-06T04:28:39.1636910Z    |
2019-12-06T04:28:39.1636980Z LL | fn f() { |[](* }
2019-12-06T04:28:39.1637510Z    |             -^ help: `)` may belong here
2019-12-06T04:28:39.1637680Z    |             unclosed delimiter
2019-12-06T04:28:39.1637740Z 
2019-12-06T04:28:39.1637800Z error: aborting due to 2 previous errors
2019-12-06T04:28:39.1637850Z 
---
2019-12-06T04:28:39.1761340Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-06T04:28:39.1761510Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-06T04:28:39.1779820Z 
2019-12-06T04:28:39.1780000Z 
2019-12-06T04:28:39.1782870Z command did not execute successfully: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-06T04:28:39.1783720Z 
2019-12-06T04:28:39.1783800Z 
2019-12-06T04:28:39.1792970Z failed to run: /Users/runner/runners/2.160.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-06T04:28:39.1793110Z Build completed unsuccessfully in 0:53:30
2019-12-06T04:28:39.1793110Z Build completed unsuccessfully in 0:53:30
2019-12-06T04:28:39.1851610Z == clock drift check ==
2019-12-06T04:28:39.1897710Z   local time: Fri Dec  6 04:28:39 UTC 2019
2019-12-06T04:28:39.2422290Z   network time: Fri, 06 Dec 2019 04:28:39 GMT
2019-12-06T04:28:39.2424310Z == end clock drift check ==
2019-12-06T04:28:39.2459870Z 
2019-12-06T04:28:39.2578150Z ##[error]Bash exited with code '1'.
2019-12-06T04:28:39.2618540Z ##[section]Starting: Checkout
2019-12-06T04:28:39.2621120Z ==============================================================================
2019-12-06T04:28:39.2621240Z Task         : Get sources
2019-12-06T04:28:39.2621320Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
