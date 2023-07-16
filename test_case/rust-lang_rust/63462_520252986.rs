plain
2019-08-11T17:58:49.7218728Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-11T17:58:49.7401645Z ##[command]git config gc.auto 0
2019-08-11T17:58:49.7462761Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-11T17:58:50.5455871Z ##[command]git config --get-all http.proxy
2019-08-11T17:58:50.5465344Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63462/merge:refs/remotes/pull/63462/merge
---
2019-08-11T17:59:24.8958959Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-11T17:59:24.8959014Z 
2019-08-11T17:59:24.8959245Z   git checkout -b <new-branch-name>
2019-08-11T17:59:24.8959296Z 
2019-08-11T17:59:24.8959347Z HEAD is now at de991faf7 Merge 2e8fae4beae2e44c4bb3fc468e078c263054811d into 8a068699a24de306334a1f66b9a83552766d853c
2019-08-11T17:59:24.9122029Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-11T17:59:24.9124878Z ==============================================================================
2019-08-11T17:59:24.9124934Z Task         : Bash
2019-08-11T17:59:24.9124997Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-11T19:03:22.8418537Z .................................................................................................... 1300/8873
2019-08-11T19:03:29.5754159Z .................................................................................................... 1400/8873
2019-08-11T19:03:36.1175885Z .................................................................................................... 1500/8873
2019-08-11T19:03:47.2828503Z ....................................................................................i............... 1600/8873
2019-08-11T19:03:55.3192896Z i................................................................................................... 1700/8873
2019-08-11T19:04:02.4469926Z ............................................................................iiiii................... 1800/8873
2019-08-11T19:04:25.5271752Z .................................................................................................... 2000/8873
2019-08-11T19:04:28.0922739Z .................................................................................................... 2100/8873
2019-08-11T19:04:30.8680924Z .................................................................................................... 2200/8873
2019-08-11T19:04:38.9188962Z .................................................................................................... 2300/8873
---
2019-08-11T19:08:45.6670330Z .................................................................................................... 5300/8873
2019-08-11T19:08:53.1453378Z ........i........................................................................................... 5400/8873
2019-08-11T19:08:58.6330272Z .................................................................................................... 5500/8873
2019-08-11T19:09:11.9199908Z .................................................................................................... 5600/8873
2019-08-11T19:09:27.2104627Z ...ii...i..ii...........i........................................................................... 5700/8873
2019-08-11T19:09:42.2413696Z .................................................................................................... 5900/8873
2019-08-11T19:09:47.1420172Z .................................................................................................... 6000/8873
2019-08-11T19:09:47.1420172Z .................................................................................................... 6000/8873
2019-08-11T19:10:01.9356025Z ....i..ii........................................................................................... 6100/8873
2019-08-11T19:10:21.6612892Z ...............................................i.................................................... 6300/8873
2019-08-11T19:10:23.9570674Z .................................................................................................... 6400/8873
2019-08-11T19:10:26.5917180Z ...................i................................................................................ 6500/8873
2019-08-11T19:10:31.3732316Z .................................................................................................... 6600/8873
---
2019-08-11T19:15:25.4726655Z  finished in 21.481
2019-08-11T19:15:25.4919853Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T19:15:25.6846397Z 
2019-08-11T19:15:25.6846731Z running 146 tests
2019-08-11T19:15:29.0557708Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-08-11T19:15:30.9938650Z iii..............i.........iii..i.....ii......
2019-08-11T19:15:30.9943711Z 
2019-08-11T19:15:30.9944033Z  finished in 5.501
2019-08-11T19:15:31.0137917Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T19:15:31.1880085Z 
---
2019-08-11T19:15:33.3715562Z  finished in 2.357
2019-08-11T19:15:33.3921487Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T19:15:33.5678640Z 
2019-08-11T19:15:33.5679065Z running 9 tests
2019-08-11T19:15:33.5679854Z iiiiiiiii
2019-08-11T19:15:33.5680274Z 
2019-08-11T19:15:33.5680316Z  finished in 0.175
2019-08-11T19:15:33.5867199Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T19:15:33.7749516Z 
2019-08-11T19:15:33.7749516Z 
2019-08-11T19:15:33.7749749Z running 104 tests
2019-08-11T19:15:51.9905825Z ................................F................................................................... 100/104
2019-08-11T19:15:52.7528167Z ....
2019-08-11T19:15:52.7529280Z failures:
2019-08-11T19:15:52.7529609Z 
2019-08-11T19:15:52.7530381Z ---- [incremental] incremental/hashes/inline_asm.rs stdout ----
2019-08-11T19:15:52.7530502Z 
2019-08-11T19:15:52.7530761Z error in revision `cfail1`: test compilation failed although it shouldn't!
2019-08-11T19:15:52.7531017Z status: exit code: 101
2019-08-11T19:15:52.7531978Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/inline_asm.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/auxiliary"
2019-08-11T19:15:52.7532646Z ------------------------------------------
2019-08-11T19:15:52.7533067Z 
2019-08-11T19:15:52.7533338Z ------------------------------------------
2019-08-11T19:15:52.7533408Z stderr:
2019-08-11T19:15:52.7533408Z stderr:
2019-08-11T19:15:52.7533620Z ------------------------------------------
2019-08-11T19:15:52.7534048Z thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheEncoder<serialize::opaque::Encoder> as SpecializedEncoder<syntax_pos::hygiene::SyntaxContext>>::specialized_encode` not overridden', src/libserialize/serialize.rs:851:9
2019-08-11T19:15:52.7534470Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-08-11T19:15:52.7534470Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-08-11T19:15:52.7534539Z   left: `LLVMing`,
2019-08-11T19:15:52.7534783Z  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1506:21
2019-08-11T19:15:52.7535041Z error: internal compiler error: unexpected panic
2019-08-11T19:15:52.7535072Z 
2019-08-11T19:15:52.7535118Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-11T19:15:52.7535157Z 
2019-08-11T19:15:52.7535157Z 
2019-08-11T19:15:52.7535621Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-11T19:15:52.7535917Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-08-11T19:15:52.7535952Z 
2019-08-11T19:15:52.7535952Z 
2019-08-11T19:15:52.7536351Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-11T19:15:52.7536426Z 
2019-08-11T19:15:52.7536632Z ------------------------------------------
2019-08-11T19:15:52.7536682Z 
2019-08-11T19:15:52.7536707Z 
---
2019-08-11T19:15:52.7537140Z test result: FAILED. 103 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-08-11T19:15:52.7537186Z 
2019-08-11T19:15:52.7537228Z 
2019-08-11T19:15:52.7537253Z 
2019-08-11T19:15:52.7538842Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-11T19:15:52.7539175Z 
2019-08-11T19:15:52.7539203Z 
2019-08-11T19:15:52.7539505Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-11T19:15:52.7539588Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-11T19:15:52.7539588Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-11T19:15:52.7546960Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-11T19:15:52.7547479Z Build completed unsuccessfully in 1:10:07
2019-08-11T19:15:56.3359908Z ##[error]Bash exited with code '1'.
2019-08-11T19:15:56.3446236Z ##[section]Starting: Checkout
2019-08-11T19:15:56.3448078Z ==============================================================================
2019-08-11T19:15:56.3448132Z Task         : Get sources
2019-08-11T19:15:56.3448179Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
