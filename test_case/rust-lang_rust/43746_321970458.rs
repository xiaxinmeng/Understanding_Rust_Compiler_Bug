rust
[00:59:58] ---- [run-pass] run-pass/thread-local-extern-static.rs stdout ----
[00:59:58] 	
[00:59:58] error: auxiliary build of "/checkout/src/test/run-pass/auxiliary/thread-local-extern-static.rs" failed to compile: 
[00:59:58] status: exit code: 101
[00:59:58] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/run-pass/auxiliary/thread-local-extern-static.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass --target=arm-linux-androideabi --crate-type=dylib -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thread-local-extern-static.stage2-arm-linux-androideabi.run-pass.libaux -C prefer-dynamic --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thread-local-extern-static.stage2-arm-linux-androideabi.run-pass.libaux -Clinker=/android/ndk/arm-9/bin/arm-linux-androideabi-gcc -Crpath -O -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers
[00:59:58] stdout:
[00:59:58] ------------------------------------------
[00:59:58] 
[00:59:58] ------------------------------------------
[00:59:58] stderr:
[00:59:58] ------------------------------------------
[00:59:58] error[E0277]: the trait bound `std::cell::Cell<u32>: std::marker::Sync` is not satisfied
[00:59:58]   --> /checkout/src/test/run-pass/auxiliary/thread-local-extern-static.rs:18:1
[00:59:58]    |
[00:59:58] 18 | pub static FOO: Cell<u32> = Cell::new(3);
[00:59:58]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::Cell<u32>` cannot be shared between threads safely
[00:59:58]    |
[00:59:58]    = help: the trait `std::marker::Sync` is not implemented for `std::cell::Cell<u32>`
[00:59:58]    = note: shared static variables must have a type that implements `Sync`
[00:59:58] 
[00:59:58] error: aborting due to previous error
[00:59:58] 
[00:59:58] 
[00:59:58] ------------------------------------------
[00:59:58] 
[00:59:58] thread '[run-pass] run-pass/thread-local-extern-static.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2511:8
[00:59:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:58] 
[00:59:58] 
[00:59:58] failures:
[00:59:58]     [run-pass] run-pass/thread-local-extern-static.rs
[00:59:58] 
[00:59:58] test result: FAILED. 2707 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out
