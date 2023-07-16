plain
2019-09-20T20:25:33.9861540Z test [ui] ui/coherence/coherence_local_err_tuple.rs#re ... ok
2019-09-20T20:25:34.0902280Z test [ui] ui/coherence/conflicting-impl-with-err.rs ... ok
2019-09-20T20:25:34.2386810Z test [ui] ui/coherence/coherence_local_ref.rs#old ... ok
2019-09-20T20:25:34.3178160Z test [ui] ui/coherence/coherence_local_ref.rs#re ... ok
2019-09-20T20:25:34.3681250Z test [ui] ui/coherence/impl-foreign[foreign]-for-foreign.rs ... ok
2019-09-20T20:25:34.4651930Z test [ui] ui/coherence/impl-foreign[foreign]-for-local.rs ... ok
2019-09-20T20:25:34.6446320Z test [ui] ui/coherence/impl[t]-foreign[foreign[t],local]-for-foreign.rs ... ok
2019-09-20T20:25:34.7014990Z test [ui] ui/coherence/impl[t]-foreign[foreign]-for-fundamental[t].rs ... ok
2019-09-20T20:25:34.7267510Z test [ui] ui/coherence/impl[t]-foreign[foreign]-for-t.rs ... ok
2019-09-20T20:25:34.8374510Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t],local]-for-foreign.rs ... ok
2019-09-20T20:25:35.0085690Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-foreign.rs ... ok
2019-09-20T20:25:35.1029060Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs ... ok
2019-09-20T20:25:35.1129050Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-local.rs ... ok
2019-09-20T20:25:35.2034790Z test [ui] ui/coherence/impl[t]-foreign[fundamental[t]]-for-t.rs ... ok
2019-09-20T20:25:35.4010880Z test [ui] ui/coherence/impl[t]-foreign[local, fundamental[t]]-for-foreign.rs ... ok
2019-09-20T20:25:35.4900840Z test [ui] ui/coherence/impl[t]-foreign[local]-for-fundamental[t].rs ... ok
2019-09-20T20:25:35.5002880Z test [ui] ui/coherence/impl[t]-foreign[local]-for-foreign.rs ... ok
2019-09-20T20:25:35.5963020Z test [ui] ui/coherence/impl[t]-foreign[local]-for-local.rs ... ok
2019-09-20T20:25:35.7849240Z test [ui] ui/coherence/impl[t]-foreign[local]-for-t.rs ... ok
2019-09-20T20:25:35.8636590Z test [ui] ui/coherence/impl[t]-foreign[t]-for-foreign.rs ... ok
2019-09-20T20:25:35.8843920Z test [ui] ui/coherence/impl[t]-foreign[t]-for-fundamental.rs ... ok
2019-09-20T20:25:35.9953330Z test [ui] ui/coherence/impl[t]-foreign[t]-for-local.rs ... ok
2019-09-20T20:25:36.1180710Z test [ui] ui/coherence/impl[t]-foreign[t]-for-t.rs ... ok
2019-09-20T20:25:36.2654830Z test [ui] ui/command-line-diagnostics.rs ... ok
2019-09-20T20:25:36.5063840Z test [ui] ui/coherence/re-rebalance-coherence.rs ... ok
2019-09-20T20:25:36.5163850Z test [ui] ui/coherence/re-rebalance-coherence-default-generic-associated-type.rs ... ok
2019-09-20T20:25:36.5722790Z test [ui] ui/commandline-argfile-badutf8.rs ... ok
---
2019-09-20T20:33:39.1414900Z test [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs ... ok
2019-09-20T20:33:39.2218750Z test [ui] ui/rfc-2565-param-attrs/param-attrs-cfg.rs ... ok
2019-09-20T20:33:39.2418840Z test [ui] ui/rfc-2565-param-attrs/param-attrs-feature-gate.rs ... ok
2019-09-20T20:33:39.3122710Z test [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs ... ok
2019-09-20T20:33:39.3524490Z test [ui] ui/rfc-2627-raw-dylib/feature-gate-raw-dylib-2.rs ... ok
2019-09-20T20:33:39.4704120Z test [ui] ui/rfc-2627-raw-dylib/link-ordinal-and-name.rs ... ok
2019-09-20T20:33:39.4806390Z test [ui] ui/rfc-2627-raw-dylib/feature-gate-raw-dylib.rs ... ok
2019-09-20T20:33:39.6014830Z test [ui] ui/rfc-2627-raw-dylib/link-ordinal-too-large.rs ... ok
2019-09-20T20:33:39.6114680Z test [ui] ui/rfc-2627-raw-dylib/link-ordinal-invalid-format.rs ... ok
2019-09-20T20:33:40.1010090Z test [ui] ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-param.rs ... ok
2019-09-20T20:33:40.1416240Z test [ui] ui/rfc1445/allow-hide-behind-direct-unsafe-ptr-embedded.rs ... ok
2019-09-20T20:33:40.1817970Z test [ui] ui/rfc1445/allow-hide-behind-indirect-unsafe-ptr-embedded.rs ... ok
2019-09-20T20:33:40.2722660Z test [ui] ui/rfc-2565-param-attrs/param-attrs-pretty.rs ... ok
---
2019-09-20T20:36:34.8217240Z diff of stderr:
2019-09-20T20:36:34.8217350Z 
2019-09-20T20:36:34.8217500Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-20T20:36:34.8217620Z 9    |
2019-09-20T20:36:34.8218380Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-20T20:36:34.8219140Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-20T20:36:34.8219890Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-20T20:36:34.8220030Z 12 
2019-09-20T20:36:34.8220810Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-20T20:36:34.8221690Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-20T20:36:34.8222390Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-09-20T20:36:34.8222600Z 15    |
2019-09-20T20:36:34.8223330Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-20T20:36:34.8223490Z 
2019-09-20T20:36:34.8223590Z The actual stderr differed from the expected stderr.
2019-09-20T20:36:34.8224350Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-20T20:36:34.8224350Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-20T20:36:34.8225060Z To update references, rerun the tests and pass the `--bless` flag
2019-09-20T20:36:34.8225730Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-09-20T20:36:34.8225920Z error: 1 errors occurred comparing output.
2019-09-20T20:36:34.8226020Z status: exit code: 1
2019-09-20T20:36:34.8226020Z status: exit code: 1
2019-09-20T20:36:34.8227450Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-09-20T20:36:34.8228430Z ------------------------------------------
2019-09-20T20:36:34.8228510Z 
2019-09-20T20:36:34.8229130Z ------------------------------------------
2019-09-20T20:36:34.8229250Z stderr:
2019-09-20T20:36:34.8229250Z stderr:
2019-09-20T20:36:34.8229850Z ------------------------------------------
2019-09-20T20:36:34.8229990Z error[E0308]: cannot coerce intrinsics to function pointers
2019-09-20T20:36:34.8230680Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/reify-intrinsic.rs:6:64
2019-09-20T20:36:34.8230820Z    |
2019-09-20T20:36:34.8231510Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-09-20T20:36:34.8231800Z    |                                                                |
2019-09-20T20:36:34.8232220Z    |                                                                cannot coerce intrinsics to function pointers
2019-09-20T20:36:34.8232400Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-20T20:36:34.8232530Z    |
2019-09-20T20:36:34.8232530Z    |
2019-09-20T20:36:34.8233230Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-20T20:36:34.8233960Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-20T20:36:34.8234050Z 
2019-09-20T20:36:34.8235030Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-20T20:36:34.8236000Z    |
2019-09-20T20:36:34.8236000Z    |
2019-09-20T20:36:34.8236660Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-20T20:36:34.8236910Z 
2019-09-20T20:36:34.8237000Z error: aborting due to 2 previous errors
2019-09-20T20:36:34.8237060Z 
2019-09-20T20:36:34.8237150Z Some errors have detailed explanations: E0308, E0606.
---
2019-09-20T20:36:34.8241040Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-20T20:36:34.8241210Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-20T20:36:34.8241280Z 
2019-09-20T20:36:34.8242040Z 
2019-09-20T20:36:34.8245830Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-20T20:36:34.8247410Z 
2019-09-20T20:36:34.8247580Z 
2019-09-20T20:36:34.8247800Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-09-20T20:36:34.8248840Z Build completed unsuccessfully in 0:59:11
2019-09-20T20:36:34.8248840Z Build completed unsuccessfully in 0:59:11
2019-09-20T20:36:34.8249110Z == clock drift check ==
2019-09-20T20:36:34.8249320Z   local time: Fri Sep 20 20:36:34 UTC 2019
2019-09-20T20:36:34.8249490Z   network time: Fri, 20 Sep 2019 20:36:34 GMT
2019-09-20T20:36:34.8249700Z == end clock drift check ==
2019-09-20T20:36:34.8334270Z ##[error]Bash exited with code '1'.
2019-09-20T20:36:34.8390980Z ##[section]Starting: Upload CPU usage statistics
2019-09-20T20:36:34.8421690Z ==============================================================================
2019-09-20T20:36:34.8421820Z Task         : Bash
2019-09-20T20:36:34.8421900Z Description  : Run a Bash script on macOS, Linux, or Windows
