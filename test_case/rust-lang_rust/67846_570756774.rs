plain
2020-01-04T02:09:32.7494790Z ---- [ui] ui/pattern/issue-67776-match-same-name-enum-variant-refs.rs stdout ----
2020-01-04T02:09:32.7495160Z diff of stderr:
2020-01-04T02:09:32.7495220Z 
2020-01-04T02:09:32.7495280Z 3    |
2020-01-04T02:09:32.7495340Z 4 LL |         Bar => {},
2020-01-04T02:09:32.7495560Z 5    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-04T02:09:32.7495720Z +    = note: `#[warn(bindings_with_variant_name)]` on by default
2020-01-04T02:09:32.7495800Z 6 
2020-01-04T02:09:32.7495800Z 6 
2020-01-04T02:09:32.7495890Z 7 warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-04T02:09:32.7496610Z 
2020-01-04T02:09:32.7496660Z 
2020-01-04T02:09:32.7496730Z The actual stderr differed from the expected stderr.
2020-01-04T02:09:32.7497450Z Actual stderr saved to /Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/pattern/issue-67776-match-same-name-enum-variant-refs/issue-67776-match-same-name-enum-variant-refs.stderr
2020-01-04T02:09:32.7497450Z Actual stderr saved to /Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/pattern/issue-67776-match-same-name-enum-variant-refs/issue-67776-match-same-name-enum-variant-refs.stderr
2020-01-04T02:09:32.7498080Z To update references, rerun the tests and pass the `--bless` flag
2020-01-04T02:09:32.7498840Z To only update this specific test, also pass `--test-args pattern/issue-67776-match-same-name-enum-variant-refs.rs`
2020-01-04T02:09:32.7499000Z error: 1 errors occurred comparing output.
2020-01-04T02:09:32.7499070Z status: exit code: 0
2020-01-04T02:09:32.7499070Z status: exit code: 0
2020-01-04T02:09:32.7501110Z command: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/pattern/issue-67776-match-same-name-enum-variant-refs.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/pattern/issue-67776-match-same-name-enum-variant-refs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/pattern/issue-67776-match-same-name-enum-variant-refs/auxiliary" "-A" "unused"
2020-01-04T02:09:32.7502430Z ------------------------------------------
2020-01-04T02:09:32.7502720Z 
2020-01-04T02:09:32.7503360Z ------------------------------------------
2020-01-04T02:09:32.7503700Z stderr:
2020-01-04T02:09:32.7503700Z stderr:
2020-01-04T02:09:32.7504300Z ------------------------------------------
2020-01-04T02:09:32.7504670Z warning[E0170]: pattern binding `Bar` is named the same as one of the variants of the type `Foo`
2020-01-04T02:09:32.7505740Z    |
2020-01-04T02:09:32.7505870Z LL |         Bar => {},
2020-01-04T02:09:32.7505870Z LL |         Bar => {},
2020-01-04T02:09:32.7505980Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-04T02:09:32.7506220Z    = note: `#[warn(bindings_with_variant_name)]` on by default
2020-01-04T02:09:32.7506310Z 
2020-01-04T02:09:32.7506310Z 
2020-01-04T02:09:32.7506420Z warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-04T02:09:32.7507480Z    |
2020-01-04T02:09:32.7507600Z LL |         Baz => {},
2020-01-04T02:09:32.7507600Z LL |         Baz => {},
2020-01-04T02:09:32.7507730Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Baz`
2020-01-04T02:09:32.7507810Z 
2020-01-04T02:09:32.7507940Z warning[E0170]: pattern binding `Bar` is named the same as one of the variants of the type `Foo`
2020-01-04T02:09:32.7508970Z    |
2020-01-04T02:09:32.7509320Z LL |         Bar => {},
2020-01-04T02:09:32.7509320Z LL |         Bar => {},
2020-01-04T02:09:32.7509430Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-04T02:09:32.7509500Z 
2020-01-04T02:09:32.7509610Z warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-04T02:09:32.7510670Z    |
2020-01-04T02:09:32.7510810Z LL |         Baz => {},
2020-01-04T02:09:32.7510810Z LL |         Baz => {},
2020-01-04T02:09:32.7510940Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Baz`
2020-01-04T02:09:32.7511020Z 
2020-01-04T02:09:32.7511140Z warning[E0170]: pattern binding `Bar` is named the same as one of the variants of the type `Foo`
2020-01-04T02:09:32.7512320Z    |
2020-01-04T02:09:32.7512450Z LL |         Bar => {},
2020-01-04T02:09:32.7512450Z LL |         Bar => {},
2020-01-04T02:09:32.7512580Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-04T02:09:32.7512660Z 
2020-01-04T02:09:32.7512790Z warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-04T02:09:32.7513850Z    |
2020-01-04T02:09:32.7513980Z LL |         Baz => {},
2020-01-04T02:09:32.7513980Z LL |         Baz => {},
2020-01-04T02:09:32.7514260Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Baz`
2020-01-04T02:09:32.7514810Z 
2020-01-04T02:09:32.7515530Z ------------------------------------------
2020-01-04T02:09:32.7515880Z 
2020-01-04T02:09:32.7515990Z 
---
2020-01-04T02:09:32.7585770Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-04T02:09:32.7585910Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-04T02:09:32.7603070Z 
2020-01-04T02:09:32.7603160Z 
2020-01-04T02:09:32.7606650Z command did not execute successfully: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-04T02:09:32.7607820Z 
2020-01-04T02:09:32.7607870Z 
2020-01-04T02:09:32.7617650Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap test
2020-01-04T02:09:32.7617780Z Build completed unsuccessfully in 0:48:43
2020-01-04T02:09:32.7617780Z Build completed unsuccessfully in 0:48:43
2020-01-04T02:09:32.7678420Z == clock drift check ==
2020-01-04T02:09:32.7735000Z   local time: Sat Jan  4 02:09:32 UTC 2020
2020-01-04T02:09:32.8274350Z   network time: Sat, 04 Jan 2020 02:09:32 GMT
2020-01-04T02:09:32.8276030Z == end clock drift check ==
2020-01-04T02:09:32.8318930Z 
2020-01-04T02:09:32.8424620Z ##[error]Bash exited with code '1'.
2020-01-04T02:09:32.8467720Z ##[section]Starting: Checkout
2020-01-04T02:09:32.8470410Z ==============================================================================
2020-01-04T02:09:32.8470530Z Task         : Get sources
2020-01-04T02:09:32.8470610Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
