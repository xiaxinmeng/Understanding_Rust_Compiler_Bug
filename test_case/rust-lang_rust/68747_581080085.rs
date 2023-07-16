plain
2020-02-01T23:21:52.6133070Z ---- [js-doc-test] rustdoc-js/struct-like-variant.rs stdout ----
2020-02-01T23:21:52.6133140Z 
2020-02-01T23:21:52.6133620Z error: rustdoc-js test failed!
2020-02-01T23:21:52.6133710Z status: exit code: 1
2020-02-01T23:21:52.6134590Z command: "/usr/local/bin/node" "/Users/runner/runners/2.164.6/work/1/s/src/tools/rustdoc-js/tester.js" "/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/test/rustdoc-js" "struct-like-variant"
2020-02-01T23:21:52.6135310Z ------------------------------------------
2020-02-01T23:21:52.6135820Z Checking "struct-like-variant" ... 
2020-02-01T23:21:52.6135870Z 
2020-02-01T23:21:52.6136350Z ------------------------------------------
2020-02-01T23:21:52.6136350Z ------------------------------------------
2020-02-01T23:21:52.6136450Z stderr:
2020-02-01T23:21:52.6136940Z ------------------------------------------
2020-02-01T23:21:52.6137020Z FAILED
2020-02-01T23:21:52.6137570Z ==> Result not found in 'others': '{"path":"struct_like_variant::Enum::Bar::name","name":"l"}'
2020-02-01T23:21:52.6138140Z ------------------------------------------
2020-02-01T23:21:52.6138220Z 
2020-02-01T23:21:52.6138250Z 
2020-02-01T23:21:52.6138290Z 
---
2020-02-01T23:21:52.6140090Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-01T23:21:52.6140220Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-01T23:21:52.6140310Z 
2020-02-01T23:21:52.6140350Z 
2020-02-01T23:21:52.6143030Z command did not execute successfully: "/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/runner/runners/2.164.6/work/1/s/src/test/rustdoc-js" "--build-base" "/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/test/rustdoc-js" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "js-doc-test" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.164.6/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.1-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-01T23:21:52.6144130Z 
2020-02-01T23:21:52.6144170Z 
2020-02-01T23:21:52.6150650Z failed to run: /Users/runner/runners/2.164.6/work/1/s/build/bootstrap/debug/bootstrap test
2020-02-01T23:21:52.6150810Z Build completed unsuccessfully in 1:27:21
2020-02-01T23:21:52.6150810Z Build completed unsuccessfully in 1:27:21
2020-02-01T23:21:52.6212600Z == clock drift check ==
2020-02-01T23:21:52.6273570Z   local time: Sat Feb  1 23:21:52 UTC 2020
2020-02-01T23:21:52.7010630Z   network time: Sat, 01 Feb 2020 23:21:52 GMT
2020-02-01T23:21:52.7012280Z == end clock drift check ==
2020-02-01T23:21:52.7052700Z 
2020-02-01T23:21:52.7181500Z ##[error]Bash exited with code '1'.
2020-02-01T23:21:52.7222740Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-01T23:21:52.7225620Z ==============================================================================
2020-02-01T23:21:52.7225700Z Task         : Get sources
2020-02-01T23:21:52.7225790Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
