
[00:54:59] failures:
[00:54:59] 
[00:54:59] ---- [mir-opt] mir-opt/nll/region-liveness-drop-no-may-dangle.rs stdout ----
[00:54:59] 	
[00:54:59] error: compilation failed!
[00:54:59] status: exit code: 101
[00:54:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/nll/region-liveness-drop-no-may-dangle.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--target=i686-unknown-linux-musl" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/region-liveness-drop-no-may-dangle" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/region-liveness-drop-no-may-dangle.stage2-i686-unknown-linux-musl" "-Crpath" "-O" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-Znll" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/region-liveness-drop-no-may-dangle.stage2-i686-unknown-linux-musl.aux"
[00:54:59] stdout:
[00:54:59] ------------------------------------------
[00:54:59] 
[00:54:59] ------------------------------------------
[00:54:59] stderr:
[00:54:59] ------------------------------------------
[00:54:59] error[E0597]: borrowed value does not live long enough (Mir)
[00:54:59]   --> /checkout/src/test/mir-opt/nll/region-liveness-drop-no-may-dangle.rs:25:51
[00:54:59]    |
[00:54:59] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:331:21
[00:54:59] 24 |     let mut v = [1, 2, 3];
[00:54:59]    |         ----- temporary value created here
[00:54:59] 25 |     let p: Wrap<& /* R1 */ usize> = Wrap { value: &v[0] };
[00:54:59]    |                                                   ^^^^^ temporary value dropped here while still borrowed
[00:54:59]    |
[00:54:59]    = note: consider using a `let` binding to increase its lifetime
[00:54:59] 
[00:54:59] error: aborting due to previous error
[00:54:59] 
[00:54:59] 
[00:54:59] ------------------------------------------
[00:54:59] 
[00:54:59] thread '[mir-opt] mir-opt/nll/region-liveness-drop-no-may-dangle.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2570:8
[00:54:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
