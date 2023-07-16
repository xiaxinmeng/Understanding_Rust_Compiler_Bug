plain
2019-10-29T01:52:35.9900532Z ---- [ui] ui/in-band-lifetimes/mismatched_trait_impl-2.rs stdout ----
2019-10-29T01:52:35.9900701Z diff of stderr:
2019-10-29T01:52:35.9900751Z 
2019-10-29T01:52:35.9901137Z 3    |
2019-10-29T01:52:35.9901536Z 4 LL |     fn deref(&self) -> &dyn Trait {
2019-10-29T01:52:35.9902274Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found fn(&Struct) -> &dyn Trait
2019-10-29T01:52:35.9903111Z -    | 
2019-10-29T01:52:35.9903523Z -   ::: $SRC_DIR/libcore/ops/deref.rs:LL:COL
2019-10-29T01:52:35.9903904Z -    |
2019-10-29T01:52:35.9904288Z - LL |     fn deref(&self) -> &Self::Target;
2019-10-29T01:52:35.9904759Z -    |     --------------------------------- expected fn(&Struct) -> &(dyn Trait + 'static)
2019-10-29T01:52:35.9904963Z 11    |
2019-10-29T01:52:35.9905465Z 12    = note: expected `fn(&Struct) -> &(dyn Trait + 'static)`
2019-10-29T01:52:35.9905708Z 13               found `fn(&Struct) -> &dyn Trait`
2019-10-29T01:52:35.9905924Z 
2019-10-29T01:52:35.9906010Z The actual stderr differed from the expected stderr.
2019-10-29T01:52:35.9906010Z The actual stderr differed from the expected stderr.
2019-10-29T01:52:35.9906395Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl-2/mismatched_trait_impl-2.stderr
2019-10-29T01:52:35.9906847Z To update references, rerun the tests and pass the `--bless` flag
2019-10-29T01:52:35.9907336Z To only update this specific test, also pass `--test-args in-band-lifetimes/mismatched_trait_impl-2.rs`
2019-10-29T01:52:35.9907622Z error: 1 errors occurred comparing output.
2019-10-29T01:52:35.9907681Z status: exit code: 1
2019-10-29T01:52:35.9907681Z status: exit code: 1
2019-10-29T01:52:35.9908612Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/in-band-lifetimes/mismatched_trait_impl-2.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl-2/auxiliary" "-A" "unused"
2019-10-29T01:52:35.9910726Z ------------------------------------------
2019-10-29T01:52:35.9912075Z 
2019-10-29T01:52:35.9912980Z ------------------------------------------
2019-10-29T01:52:35.9913298Z stderr:
2019-10-29T01:52:35.9913298Z stderr:
2019-10-29T01:52:35.9913719Z ------------------------------------------
2019-10-29T01:52:35.9914205Z error: `impl` item signature doesn't match `trait` item signature
2019-10-29T01:52:35.9915766Z   --> /checkout/src/test/ui/in-band-lifetimes/mismatched_trait_impl-2.rs:8:5
2019-10-29T01:52:35.9915888Z    |
2019-10-29T01:52:35.9916160Z LL |     fn deref(&self) -> &dyn Trait {
2019-10-29T01:52:35.9916409Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found fn(&Struct) -> &dyn Trait
2019-10-29T01:52:35.9916641Z    |
2019-10-29T01:52:35.9916919Z    = note: expected `fn(&Struct) -> &(dyn Trait + 'static)`
2019-10-29T01:52:35.9917129Z               found `fn(&Struct) -> &dyn Trait`
2019-10-29T01:52:35.9917241Z error: aborting due to previous error
2019-10-29T01:52:35.9917296Z 
2019-10-29T01:52:35.9917323Z 
2019-10-29T01:52:35.9917526Z ------------------------------------------
---
2019-10-29T01:52:35.9945299Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-29T01:52:35.9945462Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-29T01:52:35.9966217Z 
2019-10-29T01:52:35.9966498Z 
2019-10-29T01:52:35.9968601Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-29T01:52:35.9969947Z 
2019-10-29T01:52:35.9970008Z 
2019-10-29T01:52:35.9973420Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-10-29T01:52:35.9973549Z Build completed unsuccessfully in 1:11:21
2019-10-29T01:52:35.9973549Z Build completed unsuccessfully in 1:11:21
2019-10-29T01:52:36.0030163Z == clock drift check ==
2019-10-29T01:52:36.0045542Z   local time: Tue Oct 29 01:52:36 UTC 2019
2019-10-29T01:52:36.2944608Z   network time: Tue, 29 Oct 2019 01:52:36 GMT
2019-10-29T01:52:36.2945273Z == end clock drift check ==
2019-10-29T01:52:37.5514166Z 
2019-10-29T01:52:37.5610785Z ##[error]Bash exited with code '1'.
2019-10-29T01:52:37.5681214Z ##[section]Starting: Upload CPU usage statistics
2019-10-29T01:52:37.5708195Z ==============================================================================
2019-10-29T01:52:37.5708285Z Task         : Bash
2019-10-29T01:52:37.5708354Z Description  : Run a Bash script on macOS, Linux, or Windows
