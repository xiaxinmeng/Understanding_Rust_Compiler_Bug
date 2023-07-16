plain
2020-02-24T00:29:11.6419910Z failures:
2020-02-24T00:29:11.6426270Z 
2020-02-24T00:29:11.6427630Z ---- [ui] ui/consts/const-eval/validate_uninhabited_zsts.rs stdout ----
2020-02-24T00:29:11.6428320Z 
2020-02-24T00:29:11.6429890Z error: /Users/runner/runners/2.165.0/work/1/s/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5: unexpected error: '5:14: 5:37: evaluation of constant value failed [E0080]'
2020-02-24T00:29:11.6430820Z 
2020-02-24T00:29:11.6432370Z error: /Users/runner/runners/2.165.0/work/1/s/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:14: unexpected warning: '14:26: 14:31: any use of this value will cause an error [const_err]'
2020-02-24T00:29:11.6434760Z error: /Users/runner/runners/2.165.0/work/1/s/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5: expected warning not found: any use of this value will cause an error [const_err]
2020-02-24T00:29:11.6435650Z 
2020-02-24T00:29:11.6436260Z error: 2 unexpected errors found, 1 expected errors not found
2020-02-24T00:29:11.6436920Z status: exit code: 1
2020-02-24T00:29:11.6436920Z status: exit code: 1
2020-02-24T00:29:11.6440030Z command: "/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.165.0/work/1/s/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/test/ui/consts/const-eval/validate_uninhabited_zsts" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary" "-A" "unused"
2020-02-24T00:29:11.6443170Z     Error {
2020-02-24T00:29:11.6443710Z         line_num: 5,
2020-02-24T00:29:11.6444360Z         kind: Some(
2020-02-24T00:29:11.6444950Z             Error,
2020-02-24T00:29:11.6444950Z             Error,
2020-02-24T00:29:11.6445670Z         ),
2020-02-24T00:29:11.6446410Z         msg: "5:14: 5:37: evaluation of constant value failed [E0080]",
2020-02-24T00:29:11.6447580Z     Error {
2020-02-24T00:29:11.6448120Z         line_num: 14,
2020-02-24T00:29:11.6448680Z         kind: Some(
2020-02-24T00:29:11.6449240Z             Warning,
2020-02-24T00:29:11.6449240Z             Warning,
2020-02-24T00:29:11.6449760Z         ),
2020-02-24T00:29:11.6450440Z         msg: "14:26: 14:31: any use of this value will cause an error [const_err]",
2020-02-24T00:29:11.6451580Z ]
2020-02-24T00:29:11.6452020Z 
2020-02-24T00:29:11.6452560Z not found errors (from test file): [
2020-02-24T00:29:11.6453150Z     Error {
---
2020-02-24T00:29:11.6465090Z 
2020-02-24T00:29:11.6516040Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2020-02-24T00:29:11.6536320Z 
2020-02-24T00:29:11.6536710Z 
2020-02-24T00:29:11.6542760Z command did not execute successfully: "/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.165.0/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.165.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.41.1-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-24T00:29:11.6547110Z 
2020-02-24T00:29:11.6547230Z 
2020-02-24T00:29:11.6548990Z failed to run: /Users/runner/runners/2.165.0/work/1/s/build/bootstrap/debug/bootstrap test
2020-02-24T00:29:11.6549470Z Build completed unsuccessfully in 0:48:46
2020-02-24T00:29:11.6549470Z Build completed unsuccessfully in 0:48:46
2020-02-24T00:29:11.6615430Z == clock drift check ==
2020-02-24T00:29:11.6679900Z   local time: Mon Feb 24 00:29:11 UTC 2020
2020-02-24T00:29:11.7254360Z   network time: Mon, 24 Feb 2020 00:29:11 GMT
2020-02-24T00:29:11.7256160Z == end clock drift check ==
2020-02-24T00:29:11.7296820Z 
2020-02-24T00:29:11.7378990Z ##[error]Bash exited with code '1'.
2020-02-24T00:29:11.7459270Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-24T00:29:11.7466450Z ==============================================================================
2020-02-24T00:29:11.7466910Z Task         : Get sources
2020-02-24T00:29:11.7467340Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
