plain
2019-10-29T15:54:28.7254410Z failures:
2019-10-29T15:54:28.7254450Z 
2019-10-29T15:54:28.7255140Z ---- [ui] ui/huge-struct.rs stdout ----
2019-10-29T15:54:28.7255240Z 
2019-10-29T15:54:28.7256130Z error: /Users/runner/runners/2.159.2/work/1/s/src/test/ui/huge-struct.rs:49: unexpected error: '49:9: 49:12: the type `S32<S32<S1M<u32>>>` is too big for the current architecture'
2019-10-29T15:54:28.7256290Z 
2019-10-29T15:54:28.7257310Z error: /Users/runner/runners/2.159.2/work/1/s/src/test/ui/huge-struct.rs:49: expected error not found: the type `S32<S1M<S1M<u32>>>` is too big for the current architecture
2019-10-29T15:54:28.7257540Z error: 1 unexpected errors found, 1 expected errors not found
2019-10-29T15:54:28.7257640Z status: exit code: 1
2019-10-29T15:54:28.7257640Z status: exit code: 1
2019-10-29T15:54:28.7259120Z command: "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.159.2/work/1/s/src/test/ui/huge-struct.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/test/ui/huge-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/test/ui/huge-struct/auxiliary" "-A" "unused"
2019-10-29T15:54:28.7259500Z unexpected errors (from JSON output): [
2019-10-29T15:54:28.7259690Z         line_num: 49,
2019-10-29T15:54:28.7259780Z         kind: Some(
2019-10-29T15:54:28.7259880Z             Error,
2019-10-29T15:54:28.7259970Z         ),
2019-10-29T15:54:28.7259970Z         ),
2019-10-29T15:54:28.7260070Z         msg: "49:9: 49:12: the type `S32<S32<S1M<u32>>>` is too big for the current architecture",
2019-10-29T15:54:28.7260280Z ]
2019-10-29T15:54:28.7260340Z 
2019-10-29T15:54:28.7260410Z not found errors (from test file): [
2019-10-29T15:54:28.7260510Z     Error {
2019-10-29T15:54:28.7260510Z     Error {
2019-10-29T15:54:28.7260590Z         line_num: 49,
2019-10-29T15:54:28.7260680Z         kind: Some(
2019-10-29T15:54:28.7260760Z             Error,
2019-10-29T15:54:28.7260840Z         ),
2019-10-29T15:54:28.7260940Z         msg: "the type `S32<S1M<S1M<u32>>>` is too big for the current architecture",
2019-10-29T15:54:28.7261130Z ]
2019-10-29T15:54:28.7261180Z 
2019-10-29T15:54:28.7261950Z thread '[ui] ui/huge-struct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-10-29T15:54:28.7262140Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
2019-10-29T15:54:28.7264300Z 
2019-10-29T15:54:28.7355400Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-29T15:54:28.7372730Z 
2019-10-29T15:54:28.7373050Z 
2019-10-29T15:54:28.7376950Z command did not execute successfully: "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.159.2/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-29T15:54:28.7378070Z 
2019-10-29T15:54:28.7378120Z 
2019-10-29T15:54:28.7387320Z failed to run: /Users/runner/runners/2.159.2/work/1/s/build/bootstrap/debug/bootstrap test
2019-10-29T15:54:28.7387510Z Build completed unsuccessfully in 0:55:59
2019-10-29T15:54:28.7387510Z Build completed unsuccessfully in 0:55:59
2019-10-29T15:54:28.7459340Z == clock drift check ==
2019-10-29T15:54:28.7509130Z   local time: Tue Oct 29 15:54:28 UTC 2019
2019-10-29T15:54:28.8188140Z   network time: Tue, 29 Oct 2019 15:54:28 GMT
2019-10-29T15:54:28.8190860Z == end clock drift check ==
2019-10-29T15:54:28.8235800Z 
2019-10-29T15:54:28.8364730Z ##[error]Bash exited with code '1'.
2019-10-29T15:54:28.8411320Z ##[section]Starting: Upload CPU usage statistics
2019-10-29T15:54:28.8416760Z ==============================================================================
2019-10-29T15:54:28.8416890Z Task         : Bash
2019-10-29T15:54:28.8416980Z Description  : Run a Bash script on macOS, Linux, or Windows
