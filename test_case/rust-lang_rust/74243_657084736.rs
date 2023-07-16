
2020-07-11T15:40:58.7249200Z test [codegen] codegen/lto-removes-invokes.rs ... [32mok(B[m
2020-07-11T15:40:58.7249787Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:344:22
2020-07-11T15:40:58.7249896Z 
2020-07-11T15:40:58.7250031Z failures:
2020-07-11T15:40:58.7250117Z 
2020-07-11T15:40:58.7250446Z ---- [codegen] codegen/codemodels.rs#MODEL-KERNEL stdout ----
2020-07-11T15:40:58.7250533Z 
2020-07-11T15:40:58.7250832Z error in revision `MODEL-KERNEL`: compilation failed!
2020-07-11T15:40:58.7250984Z status: exit code: 101
2020-07-11T15:40:58.7251937Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/codemodels.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "model_kernel" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/codemodels.MODEL-KERNEL" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-C" "code-model=kernel" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/codemodels.MODEL-KERNEL/auxiliary" "--emit=llvm-ir"
2020-07-11T15:40:58.7252879Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-07-11T15:40:58.7255604Z stdout:
2020-07-11T15:40:58.7256264Z ------------------------------------------
2020-07-11T15:40:58.7256379Z 
2020-07-11T15:40:58.7256718Z ------------------------------------------
2020-07-11T15:40:58.7256998Z stderr:
2020-07-11T15:40:58.7257310Z ------------------------------------------
2020-07-11T15:40:58.7257477Z LLVM ERROR: Target does not support the kernel CodeModel
2020-07-11T15:40:58.7257585Z 
2020-07-11T15:40:58.7257875Z ------------------------------------------
2020-07-11T15:40:58.7257970Z 
2020-07-11T15:40:58.7258033Z 
2020-07-11T15:40:58.7258112Z 
2020-07-11T15:40:58.7258242Z failures:
2020-07-11T15:40:58.7258545Z     [codegen] codegen/codemodels.rs#MODEL-KERNEL
2020-07-11T15:40:58.7258626Z 
2020-07-11T15:40:58.7259007Z test result: [31mFAILED(B[m. 150 passed; 1 failed; 59 ignored; 0 measured; 0 filtered out
