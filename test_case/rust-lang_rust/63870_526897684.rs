plain
2019-09-01T08:13:48.7066280Z ---- [ui] ui/suggestions/fn-ctor-passed-as-arg-where-it-should-have-been-called.rs stdout ----
2019-09-01T08:13:48.7066400Z diff of stderr:
2019-09-01T08:13:48.7066450Z 
2019-09-01T08:13:48.7066540Z 7 LL |     bar(foo);
2019-09-01T08:13:48.7067140Z 8    |     ^^^ the trait `T` is not implemented for `fn() -> impl T {foo}`
2019-09-01T08:13:48.7067840Z -    = help: it looks like you forgot to use parentheses to call the function: `foo()`
2019-09-01T08:13:48.7067960Z +    = help: use parentheses to call the function: `foo()`
2019-09-01T08:13:48.7068040Z 11 
2019-09-01T08:13:48.7068120Z 12 error: aborting due to previous error
2019-09-01T08:13:48.7068120Z 12 error: aborting due to previous error
2019-09-01T08:13:48.7068190Z 13 
2019-09-01T08:13:48.7068250Z 
2019-09-01T08:13:48.7068290Z 
2019-09-01T08:13:48.7068370Z The actual stderr differed from the expected stderr.
2019-09-01T08:13:48.7069110Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/suggestions/fn-ctor-passed-as-arg-where-it-should-have-been-called/fn-ctor-passed-as-arg-where-it-should-have-been-called.stderr
2019-09-01T08:13:48.7069750Z To update references, rerun the tests and pass the `--bless` flag
2019-09-01T08:13:48.7070670Z To only update this specific test, also pass `--test-args suggestions/fn-ctor-passed-as-arg-where-it-should-have-been-called.rs`
2019-09-01T08:13:48.7070860Z error: 1 errors occurred comparing output.
2019-09-01T08:13:48.7070940Z status: exit code: 1
2019-09-01T08:13:48.7070940Z status: exit code: 1
2019-09-01T08:13:48.7072480Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/suggestions/fn-ctor-passed-as-arg-where-it-should-have-been-called.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/suggestions/fn-ctor-passed-as-arg-where-it-should-have-been-called" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--edition=2018" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/suggestions/fn-ctor-passed-as-arg-where-it-should-have-been-called/auxiliary" "-A" "unused"
2019-09-01T08:13:48.7073440Z ------------------------------------------
2019-09-01T08:13:48.7073510Z 
2019-09-01T08:13:48.7074050Z ------------------------------------------
2019-09-01T08:13:48.7074140Z stderr:
2019-09-01T08:13:48.7074140Z stderr:
2019-09-01T08:13:48.7074670Z ------------------------------------------
2019-09-01T08:13:48.7075250Z error[E0277]: the trait bound `fn() -> impl T {foo}: T` is not satisfied
2019-09-01T08:13:48.7076020Z    |
2019-09-01T08:13:48.7076020Z    |
2019-09-01T08:13:48.7076100Z LL | fn bar(f: impl T<O=()>) {}
2019-09-01T08:13:48.7076750Z ...
2019-09-01T08:13:48.7076830Z LL |     bar(foo); //~ERROR E0277
2019-09-01T08:13:48.7076830Z LL |     bar(foo); //~ERROR E0277
2019-09-01T08:13:48.7077420Z    |     ^^^ the trait `T` is not implemented for `fn() -> impl T {foo}`
2019-09-01T08:13:48.7077620Z    = help: use parentheses to call the function: `foo()`
2019-09-01T08:13:48.7077670Z 
2019-09-01T08:13:48.7077750Z error: aborting due to previous error
2019-09-01T08:13:48.7077800Z 
---
2019-09-01T08:13:48.7142710Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-01T08:13:48.7142890Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-01T08:13:48.7157870Z 
2019-09-01T08:13:48.7158350Z 
2019-09-01T08:13:48.7161190Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-01T08:13:48.7162320Z 
2019-09-01T08:13:48.7162410Z 
2019-09-01T08:13:48.7171650Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-09-01T08:13:48.7171870Z Build completed unsuccessfully in 0:55:09
2019-09-01T08:13:48.7171870Z Build completed unsuccessfully in 0:55:09
2019-09-01T08:13:48.7272510Z == clock drift check ==
2019-09-01T08:13:48.7315190Z   local time: Sun Sep  1 08:13:48 UTC 2019
2019-09-01T08:13:48.8150040Z   network time: Sun, 01 Sep 2019 08:13:49 GMT
2019-09-01T08:13:48.8152720Z == end clock drift check ==
2019-09-01T08:13:48.8304940Z ##[error]Bash exited with code '1'.
2019-09-01T08:13:48.8345310Z ##[section]Starting: Upload CPU usage statistics
2019-09-01T08:13:48.8349520Z ==============================================================================
2019-09-01T08:13:48.8349620Z Task         : Bash
2019-09-01T08:13:48.8349760Z Description  : Run a Bash script on macOS, Linux, or Windows
