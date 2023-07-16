
---- [run-pass] run-pass/alignment-gep-tup-like-1.rs stdout ----
	
error: test run failed!
status: exit code: 101
command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/qemu-test-client run /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/alignment-gep-tup-like-1.stage2-arm-unknown-linux-gnueabihf
stdout:
------------------------------------------
uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/alignment-gep-tup-like-1.stage2-arm-unknown-linux-gnueabihf", waiting for result
a=22 b=44

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'client.read_exact(&mut header) failed with failed to fill whole buffer', /checkout/src/tools/qemu-test-client/src/main.rs:174
note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------

thread '[run-pass] run-pass/alignment-gep-tup-like-1.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2637
note: Run with `RUST_BACKTRACE=1` for a backtrace.
