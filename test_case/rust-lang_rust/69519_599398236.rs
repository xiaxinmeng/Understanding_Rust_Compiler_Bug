plain
2020-03-16T08:06:07.4509282Z ---- [ui] ui/proc-macro/crt-static.rs stdout ----
2020-03-16T08:06:07.4509484Z 
2020-03-16T08:06:07.4509865Z error: test compilation failed although it shouldn't!
2020-03-16T08:06:07.4510111Z status: exit code: 1
2020-03-16T08:06:07.4511658Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/crt-static.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/crt-static" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-A" "unused" "-Ctarget-feature=" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/crt-static/auxiliary"
2020-03-16T08:06:07.4512985Z ------------------------------------------
2020-03-16T08:06:07.4513137Z 
2020-03-16T08:06:07.4513451Z ------------------------------------------
2020-03-16T08:06:07.4513631Z stderr:
2020-03-16T08:06:07.4513631Z stderr:
2020-03-16T08:06:07.4513948Z ------------------------------------------
2020-03-16T08:06:07.4514658Z warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`
2020-03-16T08:06:07.4514907Z 
2020-03-16T08:06:07.4515346Z error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
2020-03-16T08:06:07.4515862Z   --> /checkout/src/test/ui/proc-macro/crt-static.rs:12:1
2020-03-16T08:06:07.4516452Z LL | #[proc_macro_derive(Foo)]
2020-03-16T08:06:07.4516676Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-16T08:06:07.4516811Z 
2020-03-16T08:06:07.4516991Z error: aborting due to previous error
---
2020-03-16T08:06:07.4548564Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-16T08:06:07.4549541Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-16T08:06:07.4575929Z 
2020-03-16T08:06:07.4576108Z 
2020-03-16T08:06:07.4582263Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-16T08:06:07.4584652Z 
2020-03-16T08:06:07.4584759Z 
2020-03-16T08:06:07.4585597Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2020-03-16T08:06:07.4586109Z Build completed unsuccessfully in 1:34:26
2020-03-16T08:06:07.4586109Z Build completed unsuccessfully in 1:34:26
2020-03-16T08:06:07.4646439Z == clock drift check ==
2020-03-16T08:06:07.4685540Z   local time: Mon Mar 16 08:06:07 UTC 2020
2020-03-16T08:06:07.9983974Z   network time: Mon, 16 Mar 2020 08:06:07 GMT
2020-03-16T08:06:07.9985602Z == end clock drift check ==
2020-03-16T08:06:08.9464011Z 
2020-03-16T08:06:08.9553735Z ##[error]Bash exited with code '1'.
2020-03-16T08:06:08.9613861Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-16T08:06:08.9621459Z ==============================================================================
2020-03-16T08:06:08.9621879Z Task         : Get sources
2020-03-16T08:06:08.9622294Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
