plain
[01:22:05] ---- [ui] ui/feature-gates/feature-gate-abi.rs stdout ----
[01:22:05] diff of stderr:
[01:22:05] 
[01:22:05] 502    |
[01:22:05] 503    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[01:22:05] - error: aborting due to 63 previous errors
[01:22:05] - error: aborting due to 63 previous errors
[01:22:05] + error[E0570]: The ABI `"vectorcall"` is not supported for the current target
[01:22:05] +   --> $DIR/feature-gate-abi.rs:86:1
[01:22:05] +    |
[01:22:05] + LL | extern "vectorcall" {} //~ ERROR vectorcall is experimental and subject to change
[01:22:05] 506 
[01:22:05] - For more information about this error, try `rustc --explain E0658`.
[01:22:05] - For more information about this error, try `rustc --explain E0658`.
[01:22:05] + error[E0570]: The ABI `"thiscall"` is not supported for the current target
[01:22:05] +   --> $DIR/feature-gate-abi.rs:91:1
[01:22:05] +    |
[01:22:05] + LL | extern "thiscall" {} //~ ERROR thiscall is experimental and subject to change
[01:22:05] + 
[01:22:05] + error: aborting due to 65 previous errors
[01:22:05] + 
[01:22:05] + Some errors occurred: E0570, E0658.
[01:22:05] + Some errors occurred: E0570, E0658.
[01:22:05] + For more information about an error, try `rustc --explain E0570`.
[01:22:05] 508 
[01:22:05] 
[01:22:05] 
[01:22:05] The actual stderr differed from the expected stderr.
[01:22:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/feature-gate-abi.stderr
[01:22:05] To update references, rerun the tests and pass the `--bless` flag
[01:22:05] To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi.rs`
[01:22:05] error: 1 errors occurred comparing output.
[01:22:05] status: exit code: 1
[01:22:05] status: exit code: 1
[01:22:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi.rs" "--target=arm-linux-androideabi" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/auxiliary" "-A" "unused"
[01:22:05] ------------------------------------------
[01:22:05] 
[01:22:05] ------------------------------------------
[01:22:05] stderr:
[01:22:05] stderr:
[01:22:05] ------------------------------------------
[01:22:05] {"message":"intrinsics are subject to change","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n