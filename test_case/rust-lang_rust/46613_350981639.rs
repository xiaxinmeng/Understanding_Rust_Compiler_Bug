
`i686-unknown-linux-musl`

[00:39:46] ---- [ui] ui/feature-gate-extern_absolute_paths.rs stdout ----
[00:39:46] 	
[00:39:46] error: ui test compiled successfully!
[00:39:46] status: exit code: 0
[00:39:46] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:475:21
[00:39:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate-extern_absolute_paths.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=i686-unknown-linux-musl" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-extern_absolute_paths.stage2-i686-unknown-linux-musl" "-Crpath" "-O" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-extern_absolute_paths.stage2-i686-unknown-linux-musl.aux" "-A" "unused"
[00:39:46] stdout:
[00:39:46] ------------------------------------------
[00:39:46] 
[00:39:46] ------------------------------------------
[00:39:46] stderr:
[00:39:46] ------------------------------------------
[00:39:46] 
[00:39:46] ------------------------------------------
[00:39:46] 
[00:39:46] thread '[ui] ui/feature-gate-extern_absolute_paths.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2774:8
[00:39:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:39:46] 
[00:39:46] 
[00:39:46] failures:
[00:39:46]     [ui] ui/feature-gate-extern_absolute_paths.rs
