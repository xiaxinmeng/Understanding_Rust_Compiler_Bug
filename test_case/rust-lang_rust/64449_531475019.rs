plain
2019-09-14T12:15:34.7488179Z ---- [ui] ui/impl-trait/bound-normalization-pass.rs stdout ----
2019-09-14T12:15:34.7488425Z 
2019-09-14T12:15:34.7488861Z error: test compilation failed although it shouldn't!
2019-09-14T12:15:34.7489136Z status: exit code: 101
2019-09-14T12:15:34.7490493Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bound-normalization-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-pass" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-pass/auxiliary" "-A" "unused"
2019-09-14T12:15:34.7491195Z ------------------------------------------
2019-09-14T12:15:34.7491370Z 
2019-09-14T12:15:34.7491696Z ------------------------------------------
2019-09-14T12:15:34.7491890Z stderr:
---
2019-09-14T12:15:34.7494201Z    |            ^^^^^^^^^^^^^^^^^^^^^^
2019-09-14T12:15:34.7494365Z    |
2019-09-14T12:15:34.7494509Z    = note: `#[warn(incomplete_features)]` on by default
2019-09-14T12:15:34.7494655Z 
2019-09-14T12:15:34.7494900Z error: internal compiler error: src/librustc/ty/context.rs:212: node type <T>::Assoc (hir_id=HirId { owner: DefIndex(52), local_id: 1 }) with HirId::owner DefId(0:52 ~ bound_normalization_pass[317d]::impl_trait_in_bindings[0]::foo[0]::{{opaque}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:50 ~ bound_normalization_pass[317d]::impl_trait_in_bindings[0]::foo[0])
2019-09-14T12:15:34.7495830Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-09-14T12:15:34.7496559Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-14T12:15:34.7496796Z error: aborting due to previous error
2019-09-14T12:15:34.7496940Z 
2019-09-14T12:15:34.7496940Z 
2019-09-14T12:15:34.7497074Z 
2019-09-14T12:15:34.7497264Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-14T12:15:34.7497407Z 
2019-09-14T12:15:34.7497953Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-09-14T12:15:34.7498199Z 
2019-09-14T12:15:34.7498667Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-09-14T12:15:34.7498862Z 
2019-09-14T12:15:34.7499377Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-09-14T12:15:34.7499895Z 
2019-09-14T12:15:34.7500600Z ------------------------------------------
2019-09-14T12:15:34.7500960Z 
2019-09-14T12:15:34.7501094Z 
---
2019-09-14T12:15:34.7536812Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-14T12:15:34.7537689Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-14T12:15:34.7554092Z 
2019-09-14T12:15:34.7554353Z 
2019-09-14T12:15:34.7558316Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-14T12:15:34.7559035Z 
2019-09-14T12:15:34.7559097Z 
2019-09-14T12:15:34.7566468Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-14T12:15:34.7567130Z Build completed unsuccessfully in 1:45:10
2019-09-14T12:15:34.7567130Z Build completed unsuccessfully in 1:45:10
2019-09-14T12:15:34.7620453Z == clock drift check ==
2019-09-14T12:15:34.7645272Z   local time: Sat Sep 14 12:15:34 UTC 2019
2019-09-14T12:15:34.9133412Z   network time: Sat, 14 Sep 2019 12:15:34 GMT
2019-09-14T12:15:34.9134263Z == end clock drift check ==
2019-09-14T12:15:35.7949268Z ##[error]Bash exited with code '1'.
2019-09-14T12:15:35.7987426Z ##[section]Starting: Upload CPU usage statistics
2019-09-14T12:15:35.8001604Z ==============================================================================
2019-09-14T12:15:35.8001703Z Task         : Bash
2019-09-14T12:15:35.8001762Z Description  : Run a Bash script on macOS, Linux, or Windows
