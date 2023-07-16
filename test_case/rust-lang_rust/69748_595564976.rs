plain
2020-03-06T02:20:59.8053820Z 
2020-03-06T02:20:59.8053930Z 
2020-03-06T02:20:59.8054180Z The actual stderr differed from the expected stderr.
2020-03-06T02:20:59.8055280Z Actual stderr saved to /Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/ui/issues/issue-69396-const-no-type-in-macro/issue-69396-const-no-type-in-macro.stderr
2020-03-06T02:20:59.8056290Z To update references, rerun the tests and pass the `--bless` flag
2020-03-06T02:20:59.8057160Z To only update this specific test, also pass `--test-args issues/issue-69396-const-no-type-in-macro.rs`
2020-03-06T02:20:59.8057960Z error: 1 errors occurred comparing output.
2020-03-06T02:20:59.8058260Z status: exit code: 1
2020-03-06T02:20:59.8058260Z status: exit code: 1
2020-03-06T02:20:59.8061270Z command: "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.165.0/work/1/s/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/ui/issues/issue-69396-const-no-type-in-macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/ui/issues/issue-69396-const-no-type-in-macro/auxiliary" "-A" "unused"
2020-03-06T02:20:59.8063720Z ------------------------------------------
2020-03-06T02:20:59.8063930Z 
2020-03-06T02:20:59.8064470Z ------------------------------------------
2020-03-06T02:20:59.8064730Z stderr:
2020-03-06T02:20:59.8064730Z stderr:
2020-03-06T02:20:59.8065300Z ------------------------------------------
2020-03-06T02:20:59.8065650Z error[E0428]: the name `A` is defined multiple times
2020-03-06T02:20:59.8066520Z   --> /Users/runner/runners/2.165.0/work/1/s/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs:4:13
2020-03-06T02:20:59.8066920Z    |
2020-03-06T02:20:59.8067150Z LL |               const A = "A".$fn();
2020-03-06T02:20:59.8067710Z    |               |
2020-03-06T02:20:59.8067970Z    |               `A` redefined here
2020-03-06T02:20:59.8068310Z    |               previous definition of the value `A` here
2020-03-06T02:20:59.8068580Z ...
2020-03-06T02:20:59.8068580Z ...
2020-03-06T02:20:59.8068750Z LL | / suite! {
2020-03-06T02:20:59.8068950Z LL | |     len;
2020-03-06T02:20:59.8069160Z LL | |     is_empty;
2020-03-06T02:20:59.8069910Z    | |_- in this macro invocation
2020-03-06T02:20:59.8070160Z    |
2020-03-06T02:20:59.8070460Z    = note: `A` must be defined only once in the value namespace of this module
2020-03-06T02:20:59.8070760Z 
2020-03-06T02:20:59.8070760Z 
2020-03-06T02:20:59.8070970Z error: missing type for `const` item
2020-03-06T02:20:59.8071810Z   --> /Users/runner/runners/2.165.0/work/1/s/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs:4:19
2020-03-06T02:20:59.8072200Z    |
2020-03-06T02:20:59.8072440Z LL |               const A = "A".$fn();
2020-03-06T02:20:59.8072840Z    |                     ^ help: provide a type for the item: `A: usize`
2020-03-06T02:20:59.8073160Z ...
2020-03-06T02:20:59.8073340Z LL | / suite! {
2020-03-06T02:20:59.8073540Z LL | |     len;
2020-03-06T02:20:59.8073750Z LL | |     is_empty;
2020-03-06T02:20:59.8074490Z    | |_- in this macro invocation
2020-03-06T02:20:59.8074680Z 
2020-03-06T02:20:59.8075020Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2020-03-06T02:20:59.8075940Z   --> /Users/runner/runners/2.165.0/work/1/s/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs:4:19
2020-03-06T02:20:59.8075940Z   --> /Users/runner/runners/2.165.0/work/1/s/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs:4:19
2020-03-06T02:20:59.8076340Z    |
2020-03-06T02:20:59.8076570Z LL |               const A = "A".$fn();
2020-03-06T02:20:59.8077320Z    |                     |
2020-03-06T02:20:59.8077820Z    |                     not allowed in type signatures
2020-03-06T02:20:59.8078450Z    |                     help: replace `_` with the correct type: `bool`
2020-03-06T02:20:59.8078800Z ...
2020-03-06T02:20:59.8078800Z ...
2020-03-06T02:20:59.8079000Z LL | / suite! {
2020-03-06T02:20:59.8079210Z LL | |     len;
2020-03-06T02:20:59.8079440Z LL | |     is_empty;
2020-03-06T02:20:59.8080330Z    | |_- in this macro invocation
2020-03-06T02:20:59.8080540Z 
2020-03-06T02:20:59.8080800Z error: aborting due to 3 previous errors
2020-03-06T02:20:59.8081010Z 
---
2020-03-06T02:20:59.8108940Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-03-06T02:20:59.8109550Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-06T02:20:59.8125200Z 
2020-03-06T02:20:59.8125440Z 
2020-03-06T02:20:59.8131350Z command did not execute successfully: "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.165.0/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1100.0.30.12\nApple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15)\n" "--lldb-python-dir" "/Applications/Xcode_11.3.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python3" "--llvm-version" "9.0.1-rust-1.42.0-beta\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-06T02:20:59.8135470Z 
2020-03-06T02:20:59.8135590Z 
2020-03-06T02:20:59.8137680Z failed to run: /Users/runner/runners/2.165.0/work/1/s/build/bootstrap/debug/bootstrap test
2020-03-06T02:20:59.8138240Z Build completed unsuccessfully in 0:48:41
2020-03-06T02:20:59.8138240Z Build completed unsuccessfully in 0:48:41
2020-03-06T02:20:59.8188770Z == clock drift check ==
2020-03-06T02:20:59.8243410Z   local time: Fri Mar  6 02:20:59 UTC 2020
2020-03-06T02:20:59.9200930Z   network time: Fri, 06 Mar 2020 02:20:59 GMT
2020-03-06T02:20:59.9201810Z == end clock drift check ==
2020-03-06T02:20:59.9243410Z 
2020-03-06T02:20:59.9316540Z ##[error]Bash exited with code '1'.
2020-03-06T02:20:59.9395550Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-06T02:20:59.9402080Z ==============================================================================
2020-03-06T02:20:59.9402490Z Task         : Get sources
2020-03-06T02:20:59.9402910Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
