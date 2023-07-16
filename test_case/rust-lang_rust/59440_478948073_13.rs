\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs","byte_start":354,"byte_end":364,"line_start":13,"line_end":13,"column_start":10,"column_end":20,"is_primary":true,"text":[{"text":"#[derive(HashStable)]","highlight_start":10,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs","byte_start":354,"byte_end":364,"line_start":13,"line_end":13,"column_start":10,"column_end":20,"is_primary":false,"text":[{"text":"#[derive(HashStable)]","highlight_start":10,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(HashStable)]","def_site_span":null}}],"children":[{"message":"add #![feature(rustc_private)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'rustc_private': crate \"rustc\" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)\n  --> /checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs:13:10\n   |\nLL | #[derive(HashStable)]\n   |          ^^^^^^^^^^\n   |\n   = help: add #![feature(rustc_private)] to the crate attributes to enable\n\n"}
[01:25:54] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[01:25:54] {"message":"Some errors occurred: E0601, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0601, E0658.\n"}
[01:25:54] 
[01:25:54] ------------------------------------------
[01:25:54] 
[01:25:54] thread '[ui] ui-fulldeps/hash-stable-is-unstable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
---
[01:25:54] test result: FAILED. 19 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:25:54] 
[01:25:54] 
[01:25:54] 
[01:25:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:25:54] 
[01:25:54] 
[01:25:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:54] Build completed unsuccessfully in 0:13:58
[01:25:54] Build completed unsuccessfully in 0:13:58
[01:25:54] Makefile:48: recipe for target 'check' failed
[01:25:54] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f7fcd25
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr  2 11:04:02 UTC 2019
