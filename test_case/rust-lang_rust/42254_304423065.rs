
[00:57:20] failures:

[00:57:20] 

[00:57:20] ---- [compile-fail] compile-fail/variadic-ffi.rs stdout ----

[00:57:20] 	

[00:57:20] error: /checkout/src/test/compile-fail/variadic-ffi.rs:11: unexpected "error": '11:1: 13:2: The ABI `"stdcall"` is not supported for the current target [E0570]'

[00:57:20] 

[00:57:20] error: 1 unexpected errors found, 0 expected errors not found

[00:57:20] status: exit code: 101

[00:57:20] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/variadic-ffi.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=arm-unknown-linux-gnueabihf --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/variadic-ffi.stage2-arm-unknown-linux-gnueabihf.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/variadic-ffi.stage2-arm-unknown-linux-gnueabihf -Clinker=arm-linux-gnueabihf-gcc -Crpath -O -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers

[00:57:20] unexpected errors (from JSON output): [

[00:57:20]     Error {

[00:57:20]         line_num: 11,

[00:57:20]         kind: Some(

[00:57:20]             Error

[00:57:20]         ),

[00:57:20]         msg: "11:1: 13:2: The ABI `\"stdcall\"` is not supported for the current target [E0570]"

[00:57:20]     }

[00:57:20] ]

[00:57:20] 

[00:57:20] thread '[compile-fail] compile-fail/variadic-ffi.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1129

[00:57:20] 

[00:57:20] 

[00:57:20] failures:

[00:57:20]     [compile-fail] compile-fail/variadic-ffi.rs

[00:57:20] 

[00:57:20] test result: FAILED. 2643 passed; 1 failed; 16 ignored; 0 measured; 0 filtered out
