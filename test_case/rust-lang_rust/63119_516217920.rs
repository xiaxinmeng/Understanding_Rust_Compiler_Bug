plain
2019-07-30T01:00:22.1947660Z ---- [ui] ui/hygiene/generic_params.rs stdout ----
2019-07-30T01:00:22.1947990Z diff of stderr:
2019-07-30T01:00:22.1948130Z 
2019-07-30T01:00:22.1948310Z 3    |
2019-07-30T01:00:22.1948600Z 4 LL | #![feature(decl_macro, rustc_attrs, const_generics)]
2019-07-30T01:00:22.1949090Z +    |
2019-07-30T01:00:22.1949290Z +    = note: `#[warn(incomplete_features)]` on by default
2019-07-30T01:00:22.1949530Z 6 
2019-07-30T01:00:22.1949700Z 7 
2019-07-30T01:00:22.1949700Z 7 
2019-07-30T01:00:22.1949840Z 
2019-07-30T01:00:22.1950220Z 
2019-07-30T01:00:22.1950440Z The actual stderr differed from the expected stderr.
2019-07-30T01:00:22.1951580Z Actual stderr saved to /Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/test/ui/hygiene/generic_params/generic_params.stderr
2019-07-30T01:00:22.1952330Z To update references, rerun the tests and pass the `--bless` flag
2019-07-30T01:00:22.1953100Z To only update this specific test, also pass `--test-args hygiene/generic_params.rs`
2019-07-30T01:00:22.1953440Z error: 1 errors occurred comparing output.
2019-07-30T01:00:22.1953520Z status: exit code: 0
2019-07-30T01:00:22.1953520Z status: exit code: 0
2019-07-30T01:00:22.1955290Z command: "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.154.3/work/1/s/src/test/ui/hygiene/generic_params.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/test/ui/hygiene/generic_params" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/test/ui/hygiene/generic_params/auxiliary" "-A" "unused"
2019-07-30T01:00:22.1956660Z ------------------------------------------
2019-07-30T01:00:22.1956740Z 
2019-07-30T01:00:22.1957390Z ------------------------------------------
2019-07-30T01:00:22.1957520Z stderr:
2019-07-30T01:00:22.1957520Z stderr:
2019-07-30T01:00:22.1958160Z ------------------------------------------
2019-07-30T01:00:22.1958320Z warning: the feature `const_generics` is incomplete and may cause the compiler to crash
2019-07-30T01:00:22.1959220Z   --> /Users/vsts/agent/2.154.3/work/1/s/src/test/ui/hygiene/generic_params.rs:6:37
2019-07-30T01:00:22.1959350Z    |
2019-07-30T01:00:22.1959440Z LL | #![feature(decl_macro, rustc_attrs, const_generics)]
2019-07-30T01:00:22.1959620Z    |
2019-07-30T01:00:22.1959860Z    = note: `#[warn(incomplete_features)]` on by default
2019-07-30T01:00:22.1959910Z 
2019-07-30T01:00:22.1959960Z 
---
2019-07-30T01:00:22.1962160Z 
2019-07-30T01:00:22.1962200Z 
2019-07-30T01:00:22.1962290Z The actual stderr differed from the expected stderr.
2019-07-30T01:00:22.1963080Z Actual stderr saved to /Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/test/ui/hygiene/issue-61574-const-parameters/issue-61574-const-parameters.stderr
2019-07-30T01:00:22.1963800Z To update references, rerun the tests and pass the `--bless` flag
2019-07-30T01:00:22.1964510Z To only update this specific test, also pass `--test-args hygiene/issue-61574-const-parameters.rs`
2019-07-30T01:00:22.1964930Z error: 1 errors occurred comparing output.
2019-07-30T01:00:22.1965010Z status: exit code: 0
2019-07-30T01:00:22.1965010Z status: exit code: 0
2019-07-30T01:00:22.1966500Z command: "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.154.3/work/1/s/src/test/ui/hygiene/issue-61574-const-parameters.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/test/ui/hygiene/issue-61574-const-parameters" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/test/ui/hygiene/issue-61574-const-parameters/auxiliary" "-A" "unused"
2019-07-30T01:00:22.1967480Z ------------------------------------------
2019-07-30T01:00:22.1967560Z 
2019-07-30T01:00:22.1968190Z ------------------------------------------
2019-07-30T01:00:22.1968290Z stderr:
---
2019-07-30T01:00:22.2076730Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-30T01:00:22.2076930Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-30T01:00:22.2095590Z 
2019-07-30T01:00:22.2095820Z 
2019-07-30T01:00:22.2098980Z command did not execute successfully: "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.154.3/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-30T01:00:22.2100400Z 
2019-07-30T01:00:22.2100460Z 
2019-07-30T01:00:22.2112430Z failed to run: /Users/vsts/agent/2.154.3/work/1/s/build/bootstrap/debug/bootstrap test
2019-07-30T01:00:22.2112610Z Build completed unsuccessfully in 1:06:31
2019-07-30T01:00:22.2112610Z Build completed unsuccessfully in 1:06:31
2019-07-30T01:00:22.2386090Z ##[error]Bash exited with code '1'.
2019-07-30T01:00:22.2425590Z ##[section]Starting: Upload CPU usage statistics
2019-07-30T01:00:22.2430590Z ==============================================================================
2019-07-30T01:00:22.2430710Z Task         : Bash
2019-07-30T01:00:22.2430790Z Description  : Run a Bash script on macOS, Linux, or Windows
