plain
2019-08-18T11:54:56.4853043Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-18T11:54:56.4853143Z 
2019-08-18T11:54:56.4853397Z   git checkout -b <new-branch-name>
2019-08-18T11:54:56.4853447Z 
2019-08-18T11:54:56.4853776Z HEAD is now at d3fd7af24 Auto merge of #63683 - Centril:rollup-pnig1vc, r=Centril
2019-08-18T11:54:56.5038707Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-18T11:54:56.5042481Z ==============================================================================
2019-08-18T11:54:56.5042590Z Task         : Bash
2019-08-18T11:54:56.5042661Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-18T12:12:51.8398278Z [RUSTC-TIMING] rustc_typeck test:false 187.120
2019-08-18T12:12:51.8464128Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-08-18T12:13:35.6279772Z [RUSTC-TIMING] rustc_lint test:false 43.770
2019-08-18T12:13:35.6321096Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-08-18T12:13:35.7507000Z warning: diagnostic code E0267 already used
2019-08-18T12:13:35.7507415Z    --> <::syntax::diagnostics::macros::struct_span_err macros>:3:6
2019-08-18T12:13:35.7508017Z     |
2019-08-18T12:13:35.7508389Z 1   | / ($ session : expr , $ span : expr , $ code : ident , $ ($ message : tt) *) =>
2019-08-18T12:13:35.7509414Z 2   | | ({
2019-08-18T12:13:35.7510641Z 3   | |      __diagnostic_used ! ($ code) ; $ session . struct_span_err_with_code
2019-08-18T12:13:35.7511107Z     | |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-18T12:13:35.7511870Z 4   | |      ($ span , & format ! ($ ($ message) *) , $ crate :: errors ::
2019-08-18T12:13:35.7512339Z 5   | |       DiagnosticId :: Error (stringify ! ($ code) . to_owned ()) ,)
2019-08-18T12:13:35.7513706Z 6   | |  })
2019-08-18T12:13:35.7514630Z     | |___- in this expansion of `struct_span_err!`
2019-08-18T12:13:35.7515297Z    ::: src/librustc_passes/loops.rs:181:17
2019-08-18T12:13:35.7515594Z     |
2019-08-18T12:13:35.7515594Z     |
2019-08-18T12:13:35.7516036Z 181 |                   struct_span_err!(self.sess, span, E0267, "`{}` inside of an async block", name)
2019-08-18T12:13:35.7516930Z     |
2019-08-18T12:13:35.7517227Z note: previous invocation
2019-08-18T12:13:35.7517227Z note: previous invocation
2019-08-18T12:13:35.7517604Z    --> <::syntax::diagnostics::macros::struct_span_err macros>:3:6
2019-08-18T12:13:35.7517888Z     |
2019-08-18T12:13:35.7518361Z 1   | / ($ session : expr , $ span : expr , $ code : ident , $ ($ message : tt) *) =>
2019-08-18T12:13:35.7518774Z 2   | | ({
2019-08-18T12:13:35.7519227Z 3   | |      __diagnostic_used ! ($ code) ; $ session . struct_span_err_with_code
2019-08-18T12:13:35.7519704Z     | |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-18T12:13:35.7520167Z 4   | |      ($ span , & format ! ($ ($ message) *) , $ crate :: errors ::
2019-08-18T12:13:35.7520628Z 5   | |       DiagnosticId :: Error (stringify ! ($ code) . to_owned ()) ,)
2019-08-18T12:13:35.7521256Z 6   | |  })
2019-08-18T12:13:35.7521784Z     | |___- in this expansion of `struct_span_err!`
2019-08-18T12:13:35.7522419Z    ::: src/librustc_passes/loops.rs:176:17
2019-08-18T12:13:35.7522711Z     |
2019-08-18T12:13:35.7522711Z     |
2019-08-18T12:13:35.7523131Z 176 |                   struct_span_err!(self.sess, span, E0267, "`{}` inside of a closure", name)
2019-08-18T12:13:35.7548882Z 
2019-08-18T12:14:15.4225254Z [RUSTC-TIMING] rustc_passes test:false 39.782
2019-08-18T12:14:15.4269490Z    Compiling rustc_ast_borrowck v0.0.0 (/checkout/src/librustc_ast_borrowck)
2019-08-18T12:14:45.2825197Z [RUSTC-TIMING] rustc_ast_borrowck test:false 29.850
---
2019-08-18T12:48:49.8520558Z [RUSTC-TIMING] rustc_traits test:false 93.365
2019-08-18T12:48:49.8556366Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-08-18T12:50:06.9108421Z [RUSTC-TIMING] rustc test:false 831.862
2019-08-18T12:50:06.9202218Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-08-18T12:50:07.0330219Z warning: diagnostic code E0267 already used
2019-08-18T12:50:07.0330687Z    --> <::syntax::diagnostics::macros::struct_span_err macros>:3:6
2019-08-18T12:50:07.0330991Z     |
2019-08-18T12:50:07.0331513Z 1   | / ($ session : expr , $ span : expr , $ code : ident , $ ($ message : tt) *) =>
2019-08-18T12:50:07.0331956Z 2   | | ({
2019-08-18T12:50:07.0332309Z 3   | |      __diagnostic_used ! ($ code) ; $ session . struct_span_err_with_code
2019-08-18T12:50:07.0332645Z     | |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-18T12:50:07.0333181Z 4   | |      ($ span , & format ! ($ ($ message) *) , $ crate :: errors ::
2019-08-18T12:50:07.0333615Z 5   | |       DiagnosticId :: Error (stringify ! ($ code) . to_owned ()) ,)
2019-08-18T12:50:07.0333943Z 6   | |  })
2019-08-18T12:50:07.0334257Z     | |___- in this expansion of `struct_span_err!`
2019-08-18T12:50:07.0334749Z    ::: src/librustc_passes/loops.rs:181:17
2019-08-18T12:50:07.0334977Z     |
2019-08-18T12:50:07.0334977Z     |
2019-08-18T12:50:07.0335323Z 181 |                   struct_span_err!(self.sess, span, E0267, "`{}` inside of an async block", name)
2019-08-18T12:50:07.0336168Z     |
2019-08-18T12:50:07.0336400Z note: previous invocation
2019-08-18T12:50:07.0336400Z note: previous invocation
2019-08-18T12:50:07.0336948Z    --> <::syntax::diagnostics::macros::struct_span_err macros>:3:6
2019-08-18T12:50:07.0337204Z     |
2019-08-18T12:50:07.0337551Z 1   | / ($ session : expr , $ span : expr , $ code : ident , $ ($ message : tt) *) =>
2019-08-18T12:50:07.0337994Z 2   | | ({
2019-08-18T12:50:07.0338853Z 3   | |      __diagnostic_used ! ($ code) ; $ session . struct_span_err_with_code
2019-08-18T12:50:07.0339259Z     | |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-18T12:50:07.0339688Z 4   | |      ($ span , & format ! ($ ($ message) *) , $ crate :: errors ::
2019-08-18T12:50:07.0340101Z 5   | |       DiagnosticId :: Error (stringify ! ($ code) . to_owned ()) ,)
2019-08-18T12:50:07.0340459Z 6   | |  })
2019-08-18T12:50:07.0340816Z     | |___- in this expansion of `struct_span_err!`
2019-08-18T12:50:07.0341565Z    ::: src/librustc_passes/loops.rs:176:17
2019-08-18T12:50:07.0341905Z     |
2019-08-18T12:50:07.0341905Z     |
2019-08-18T12:50:07.0342256Z 176 |                   struct_span_err!(self.sess, span, E0267, "`{}` inside of a closure", name)
2019-08-18T12:50:07.0346296Z 
2019-08-18T12:50:56.2984105Z [RUSTC-TIMING] rustc_passes test:false 49.369
2019-08-18T12:50:56.3028716Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-08-18T12:51:51.2136307Z [RUSTC-TIMING] rustc_lint test:false 54.902
---
2019-08-18T13:06:46.5134759Z test [ui] ui/consts/uninhabited-const-issue-61744.rs ... ok
2019-08-18T13:06:46.5655574Z test [ui] ui/consts/validate_never_arrays.rs ... ok
2019-08-18T13:06:47.5338616Z test [ui] ui/consts/union_constant.rs ... ok
2019-08-18T13:06:47.5340037Z test [ui] ui/continue-after-missing-main.rs ... ok
2019-08-18T13:06:47.5340743Z test [ui] ui/consts/zst_no_llvm_alloc.rs ... ok
2019-08-18T13:06:47.5342037Z test [ui] ui/copy-a-resource.rs ... ok
2019-08-18T13:06:47.5342587Z test [ui] ui/crate-in-paths.rs ... ok
2019-08-18T13:06:47.7171253Z test [ui] ui/crate-leading-sep.rs ... ok
2019-08-18T13:06:48.4820677Z test [ui] ui/core-run-destroy.rs ... ok
---
2019-08-18T13:17:51.7940795Z 
2019-08-18T13:17:51.7941584Z ---- [ui] ui/issues/issue-44415.rs stdout ----
2019-08-18T13:17:51.7941874Z diff of stderr:
2019-08-18T13:17:51.7941981Z 
2019-08-18T13:17:51.7942063Z 10 LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
2019-08-18T13:17:51.7942782Z 11    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-18T13:17:51.7943301Z 12 note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-08-18T13:17:51.7943885Z -   --> $SRC_DIR/libcore/intrinsics.rs:LL:COL
2019-08-18T13:17:51.7944359Z -    |
2019-08-18T13:17:51.7944844Z - LL |     pub fn size_of<T>() -> usize;
2019-08-18T13:17:51.7945344Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-18T13:17:51.7945885Z 17 note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-08-18T13:17:51.7946416Z -   --> $SRC_DIR/libcore/intrinsics.rs:LL:COL
2019-08-18T13:17:51.7946860Z -    |
2019-08-18T13:17:51.7947349Z - LL |     pub fn size_of<T>() -> usize;
2019-08-18T13:17:51.7947844Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-18T13:17:51.7948094Z 22    = note: ...which requires computing layout of `Foo`...
2019-08-18T13:17:51.7948271Z 23    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
2019-08-18T13:17:51.7948947Z 24 note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`...
2019-08-18T13:17:51.7949358Z 
2019-08-18T13:17:51.7949484Z The actual stderr differed from the expected stderr.
2019-08-18T13:17:51.7950012Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/issue-44415.stderr
2019-08-18T13:17:51.7950012Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/issue-44415.stderr
2019-08-18T13:17:51.7950605Z To update references, rerun the tests and pass the `--bless` flag
2019-08-18T13:17:51.7951200Z To only update this specific test, also pass `--test-args issues/issue-44415.rs`
2019-08-18T13:17:51.7951541Z error: 1 errors occurred comparing output.
2019-08-18T13:17:51.7951674Z status: exit code: 1
2019-08-18T13:17:51.7951674Z status: exit code: 1
2019-08-18T13:17:51.7953215Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-44415.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/auxiliary" "-A" "unused"
2019-08-18T13:17:51.7954322Z ------------------------------------------
2019-08-18T13:17:51.7954535Z 
2019-08-18T13:17:51.7954921Z ------------------------------------------
2019-08-18T13:17:51.7955169Z stderr:
2019-08-18T13:17:51.7955169Z stderr:
2019-08-18T13:17:51.7955513Z ------------------------------------------
2019-08-18T13:17:51.7956076Z error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}#0`
2019-08-18T13:17:51.7956639Z   --> /checkout/src/test/ui/issues/issue-44415.rs:6:17
2019-08-18T13:17:51.7956917Z    |
2019-08-18T13:17:51.7957049Z LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
2019-08-18T13:17:51.7957266Z    |
2019-08-18T13:17:51.7957266Z    |
2019-08-18T13:17:51.7957784Z note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`...
2019-08-18T13:17:51.7958418Z   --> /checkout/src/test/ui/issues/issue-44415.rs:6:26
2019-08-18T13:17:51.7958676Z    |
2019-08-18T13:17:51.7958822Z LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
2019-08-18T13:17:51.7958917Z    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-18T13:17:51.7959336Z note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-08-18T13:17:51.7959686Z note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-08-18T13:17:51.7959988Z    = note: ...which requires computing layout of `Foo`...
2019-08-18T13:17:51.7960166Z    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
2019-08-18T13:17:51.7960730Z note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`...
2019-08-18T13:17:51.7961328Z   --> /checkout/src/test/ui/issues/issue-44415.rs:6:17
2019-08-18T13:17:51.7961603Z    |
2019-08-18T13:17:51.7961714Z LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
2019-08-18T13:17:51.7961843Z    |                 ^^^^^^
2019-08-18T13:17:51.7962644Z    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`, completing the cycle
2019-08-18T13:17:51.7962778Z note: cycle used when processing `Foo`
2019-08-18T13:17:51.7963077Z   --> /checkout/src/test/ui/issues/issue-44415.rs:5:1
2019-08-18T13:17:51.7963238Z LL | struct Foo {
2019-08-18T13:17:51.7963326Z    | ^^^^^^^^^^
2019-08-18T13:17:51.7963372Z 
2019-08-18T13:17:51.7963455Z error: aborting due to previous error
---
2019-08-18T13:17:51.8013107Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-18T13:17:51.8013823Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-18T13:17:51.8033457Z 
2019-08-18T13:17:51.8033685Z 
2019-08-18T13:17:51.8037792Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-18T13:17:51.8039162Z 
2019-08-18T13:17:51.8039425Z 
2019-08-18T13:17:51.8045949Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-08-18T13:17:51.8046466Z Build completed unsuccessfully in 1:17:57
2019-08-18T13:17:51.8046466Z Build completed unsuccessfully in 1:17:57
2019-08-18T13:17:51.8100947Z == clock drift check ==
2019-08-18T13:17:51.8120039Z   local time: Sun Aug 18 13:17:51 UTC 2019
2019-08-18T13:17:51.9081174Z   network time: Sun, 18 Aug 2019 13:17:51 GMT
2019-08-18T13:17:51.9082657Z == end clock drift check ==
2019-08-18T13:17:52.6513913Z ##[error]Bash exited with code '1'.
2019-08-18T13:17:52.6555167Z ##[section]Starting: Upload CPU usage statistics
2019-08-18T13:17:52.6563173Z ==============================================================================
2019-08-18T13:17:52.6563268Z Task         : Bash
2019-08-18T13:17:52.6563351Z Description  : Run a Bash script on macOS, Linux, or Windows
