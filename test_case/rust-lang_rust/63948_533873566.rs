plain
2019-09-22T11:23:11.7008800Z test [ui] ui/rfc-2565-param-attrs/param-attrs-allowed.rs ... ok
2019-09-22T11:23:11.7614570Z test [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs ... ok
2019-09-22T11:23:11.7716920Z test [ui] ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs ... ok
2019-09-22T11:23:11.8621120Z test [ui] ui/rfc-2565-param-attrs/param-attrs-cfg.rs ... ok
2019-09-22T11:23:11.9627050Z test [ui] ui/rfc-2627-raw-dylib/feature-gate-raw-dylib-2.rs ... ok
2019-09-22T11:23:11.9831080Z test [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs ... ok
2019-09-22T11:23:12.1037230Z test [ui] ui/rfc-2627-raw-dylib/link-ordinal-and-name.rs ... ok
2019-09-22T11:23:12.1440540Z test [ui] ui/rfc-2627-raw-dylib/feature-gate-raw-dylib.rs ... ok
2019-09-22T11:23:12.2665790Z test [ui] ui/rfc-2627-raw-dylib/link-ordinal-invalid-format.rs ... ok
2019-09-22T11:23:12.2768020Z test [ui] ui/rfc-2565-param-attrs/proc-macro-cannot-be-used.rs ... ok
2019-09-22T11:23:12.3070860Z test [ui] ui/rfc-2627-raw-dylib/link-ordinal-too-large.rs ... ok
2019-09-22T11:23:12.7059590Z test [ui] ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-param.rs ... ok
2019-09-22T11:23:12.7464030Z test [ui] ui/rfc-2565-param-attrs/param-attrs-pretty.rs ... ok
2019-09-22T11:23:12.7566220Z test [ui] ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-embedded.rs ... ok
2019-09-22T11:23:12.9191040Z test [ui] ui/rfc1445/cant-hide-behind-direct-struct-embedded.rs ... ok
---
2019-09-22T11:26:07.3414390Z diff of stderr:
2019-09-22T11:26:07.3414450Z 
2019-09-22T11:26:07.3414600Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-22T11:26:07.3414740Z 9    |
2019-09-22T11:26:07.3415460Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-22T11:26:07.3416200Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-22T11:26:07.3416920Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-22T11:26:07.3417070Z 12 
2019-09-22T11:26:07.3417910Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-22T11:26:07.3418780Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-22T11:26:07.3419870Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-09-22T11:26:07.3420010Z 15    |
2019-09-22T11:26:07.3428340Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-22T11:26:07.3428580Z 
2019-09-22T11:26:07.3428980Z The actual stderr differed from the expected stderr.
2019-09-22T11:26:07.3429930Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-22T11:26:07.3429930Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-22T11:26:07.3432280Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T11:26:07.3433260Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-09-22T11:26:07.3433460Z error: 1 errors occurred comparing output.
2019-09-22T11:26:07.3433550Z status: exit code: 1
2019-09-22T11:26:07.3433550Z status: exit code: 1
2019-09-22T11:26:07.3436630Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-09-22T11:26:07.3437700Z ------------------------------------------
2019-09-22T11:26:07.3437790Z 
2019-09-22T11:26:07.3438420Z ------------------------------------------
2019-09-22T11:26:07.3438550Z stderr:
2019-09-22T11:26:07.3438550Z stderr:
2019-09-22T11:26:07.3439170Z ------------------------------------------
2019-09-22T11:26:07.3439310Z error[E0308]: cannot coerce intrinsics to function pointers
2019-09-22T11:26:07.3440040Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/reify-intrinsic.rs:6:64
2019-09-22T11:26:07.3440160Z    |
2019-09-22T11:26:07.3441170Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-09-22T11:26:07.3441510Z    |                                                                |
2019-09-22T11:26:07.3441650Z    |                                                                cannot coerce intrinsics to function pointers
2019-09-22T11:26:07.3441980Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-22T11:26:07.3442620Z    |
2019-09-22T11:26:07.3442620Z    |
2019-09-22T11:26:07.3444380Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-22T11:26:07.3445260Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-22T11:26:07.3445410Z 
2019-09-22T11:26:07.3447450Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-22T11:26:07.3448420Z    |
2019-09-22T11:26:07.3448420Z    |
2019-09-22T11:26:07.3449130Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-22T11:26:07.3449370Z 
2019-09-22T11:26:07.3449450Z error: aborting due to 2 previous errors
2019-09-22T11:26:07.3449520Z 
2019-09-22T11:26:07.3449600Z Some errors have detailed explanations: E0308, E0606.
---
2019-09-22T11:26:07.3500080Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-22T11:26:07.3500280Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-22T11:26:07.3515670Z 
2019-09-22T11:26:07.3516150Z 
2019-09-22T11:26:07.3520230Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-22T11:26:07.3521480Z 
2019-09-22T11:26:07.3521530Z 
2019-09-22T11:26:07.3530510Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-09-22T11:26:07.3530690Z Build completed unsuccessfully in 0:59:03
2019-09-22T11:26:07.3530690Z Build completed unsuccessfully in 0:59:03
2019-09-22T11:26:07.3612350Z == clock drift check ==
2019-09-22T11:26:07.3670820Z   local time: Sun Sep 22 11:26:07 UTC 2019
2019-09-22T11:26:07.4411720Z   network time: Sun, 22 Sep 2019 11:26:07 GMT
2019-09-22T11:26:07.4414310Z == end clock drift check ==
2019-09-22T11:26:07.4606200Z ##[error]Bash exited with code '1'.
2019-09-22T11:26:07.4658780Z ##[section]Starting: Upload CPU usage statistics
2019-09-22T11:26:07.4682460Z ==============================================================================
2019-09-22T11:26:07.4682580Z Task         : Bash
2019-09-22T11:26:07.4682670Z Description  : Run a Bash script on macOS, Linux, or Windows
