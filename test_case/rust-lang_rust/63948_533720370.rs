plain
2019-09-20T21:43:56.5596110Z test [ui] ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs ... ok
2019-09-20T21:43:56.5796920Z test [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs ... ok
2019-09-20T21:43:56.5898780Z test [ui] ui/rfc-2565-param-attrs/param-attrs-cfg.rs ... ok
2019-09-20T21:43:56.6809880Z test [ui] ui/rfc-2565-param-attrs/param-attrs-feature-gate.rs ... ok
2019-09-20T21:43:56.7853810Z test [ui] ui/rfc-2627-raw-dylib/feature-gate-raw-dylib-2.rs ... ok
2019-09-20T21:43:56.7954300Z test [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs ... ok
2019-09-20T21:43:56.9216680Z test [ui] ui/rfc-2627-raw-dylib/link-ordinal-and-name.rs ... ok
2019-09-20T21:43:56.9718210Z test [ui] ui/rfc-2627-raw-dylib/feature-gate-raw-dylib.rs ... ok
2019-09-20T21:43:57.0723570Z test [ui] ui/rfc-2627-raw-dylib/link-ordinal-invalid-format.rs ... ok
2019-09-20T21:43:57.0955500Z test [ui] ui/rfc-2565-param-attrs/proc-macro-cannot-be-used.rs ... ok
2019-09-20T21:43:57.1000150Z test [ui] ui/rfc-2627-raw-dylib/link-ordinal-too-large.rs ... ok
2019-09-20T21:43:57.6108170Z test [ui] ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-embedded.rs ... ok
2019-09-20T21:43:57.6210040Z test [ui] ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-param.rs ... ok
2019-09-20T21:43:57.7415830Z test [ui] ui/rfc-2565-param-attrs/param-attrs-pretty.rs ... ok
2019-09-20T21:43:57.7718970Z test [ui] ui/rfc1445/cant-hide-behind-direct-struct-embedded.rs ... ok
---
2019-09-20T21:46:57.8985030Z diff of stderr:
2019-09-20T21:46:57.8985100Z 
2019-09-20T21:46:57.8985210Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-20T21:46:57.8985350Z 9    |
2019-09-20T21:46:57.8986070Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-20T21:46:57.8986810Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-20T21:46:57.8987540Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-20T21:46:57.8987670Z 12 
2019-09-20T21:46:57.8988450Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-20T21:46:57.8989310Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-20T21:46:57.8990130Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-09-20T21:46:57.8990320Z 15    |
2019-09-20T21:46:57.8991280Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-20T21:46:57.8991880Z 
2019-09-20T21:46:57.8992020Z The actual stderr differed from the expected stderr.
2019-09-20T21:46:57.8993110Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-20T21:46:57.8993110Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-20T21:46:57.8994070Z To update references, rerun the tests and pass the `--bless` flag
2019-09-20T21:46:57.8995010Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-09-20T21:46:57.8995300Z error: 1 errors occurred comparing output.
2019-09-20T21:46:57.8995470Z status: exit code: 1
2019-09-20T21:46:57.8995470Z status: exit code: 1
2019-09-20T21:46:57.8997750Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-09-20T21:46:57.8999130Z ------------------------------------------
2019-09-20T21:46:57.8999240Z 
2019-09-20T21:46:57.9000080Z ------------------------------------------
2019-09-20T21:46:57.9000230Z stderr:
2019-09-20T21:46:57.9000230Z stderr:
2019-09-20T21:46:57.9001070Z ------------------------------------------
2019-09-20T21:46:57.9001480Z error[E0308]: cannot coerce intrinsics to function pointers
2019-09-20T21:46:57.9002560Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/reify-intrinsic.rs:6:64
2019-09-20T21:46:57.9002780Z    |
2019-09-20T21:46:57.9003690Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-09-20T21:46:57.9004140Z    |                                                                |
2019-09-20T21:46:57.9004360Z    |                                                                cannot coerce intrinsics to function pointers
2019-09-20T21:46:57.9004640Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-20T21:46:57.9004830Z    |
2019-09-20T21:46:57.9004830Z    |
2019-09-20T21:46:57.9006130Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-20T21:46:57.9006890Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-20T21:46:57.9007010Z 
2019-09-20T21:46:57.9007820Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-20T21:46:57.9008770Z    |
2019-09-20T21:46:57.9008770Z    |
2019-09-20T21:46:57.9009470Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-20T21:46:57.9009700Z 
2019-09-20T21:46:57.9009790Z error: aborting due to 2 previous errors
2019-09-20T21:46:57.9009840Z 
2019-09-20T21:46:57.9009930Z Some errors have detailed explanations: E0308, E0606.
---
2019-09-20T21:46:57.9067280Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-20T21:46:57.9067460Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-20T21:46:57.9083050Z 
2019-09-20T21:46:57.9083640Z 
2019-09-20T21:46:57.9088050Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-20T21:46:57.9089160Z 
2019-09-20T21:46:57.9089210Z 
2019-09-20T21:46:57.9109070Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-09-20T21:46:57.9109380Z Build completed unsuccessfully in 1:01:01
2019-09-20T21:46:57.9109380Z Build completed unsuccessfully in 1:01:01
2019-09-20T21:46:57.9176340Z == clock drift check ==
2019-09-20T21:46:57.9210320Z   local time: Fri Sep 20 21:46:57 UTC 2019
2019-09-20T21:46:57.9663380Z   network time: Fri, 20 Sep 2019 21:46:57 GMT
2019-09-20T21:46:57.9665680Z == end clock drift check ==
2019-09-20T21:46:57.9835780Z ##[error]Bash exited with code '1'.
2019-09-20T21:46:57.9930780Z ##[section]Starting: Upload CPU usage statistics
2019-09-20T21:46:57.9954420Z ==============================================================================
2019-09-20T21:46:57.9954540Z Task         : Bash
2019-09-20T21:46:57.9954650Z Description  : Run a Bash script on macOS, Linux, or Windows
