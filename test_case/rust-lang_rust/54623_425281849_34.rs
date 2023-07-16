compile_fail,E0562\nfn main() {\n    let count_to_ten: impl Iterator<Item=usize> = 0..10;\n    // error: `impl Trait` not allowed outside of function and inherent method\n    //        return types\n    for i in counrors occurred: E0562, E0666.\n"}
[00:53:19] {"message":"For more information about an error, try `rustc --explain E0562`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0562`.\n"}
[00:53:19] ------------------------------------------
[00:53:19] 
[00:53:19] thread '[ui] ui/impl-trait/where-allowed.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:53:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:53:19] test result: FAILED. 6817 passed; 1 failed; 28 ignored; 0 measured; 0 filtered out
[00:53:19] 
[00:53:19] 
[00:53:19] 
[00:53:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-hstage1-rustc/x86_64-unknown-linux-gnu
129772 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
129768 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
129520 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
122624 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
