
020-08-08T00:40:34.2071170Z ## Running run-pass tests in tests/run-pass against miri for target x86_64-unknown-linux-gnu
2020-08-08T00:40:34.2072957Z    Compiler flags: --edition 2018 -Astable-features --sysroot /home/user/.cache/miri/HOST
2020-08-08T00:40:34.2254641Z 
2020-08-08T00:40:34.2255563Z running 194 tests
2020-08-08T00:40:34.3864259Z normalized stderr:
2020-08-08T00:40:34.3864893Z error: unsupported operation: inline assembly is not supported
2020-08-08T00:40:34.3866373Z    --> /checkout/library/core/src/hint.rs:124:9
2020-08-08T00:40:34.3866623Z     |
2020-08-08T00:40:34.3866801Z 124 |         llvm_asm!("" : : "r"(&dummy));
2020-08-08T00:40:34.3866987Z     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ inline assembly is not supported
2020-08-08T00:40:34.3867160Z     |
2020-08-08T00:40:34.3867358Z     = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
2020-08-08T00:40:34.3977300Z             
2020-08-08T00:40:34.3978122Z     = note: inside `std::hint::black_box::<()>` at /checkout/library/core/src/hint.rs:124:9
2020-08-08T00:40:34.3978923Z     = note: inside `std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>` at /checkout/library/std/src/sys_common/backtrace.rs:140:5
2020-08-08T00:40:34.3979344Z     = note: inside closure at /checkout/library/std/src/rt.rs:66:18
2020-08-08T00:40:34.3980919Z     = note: inside `std::ops::function::impls::<impl std::ops::FnOnce<()> for &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>::call_once` at /checkout/library/core/src/ops/function.rs:265:13
2020-08-08T00:40:34.3981920Z     = note: inside `std::panicking::r#try::do_call::<&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe, i32>` at /checkout/library/std/src/panicking.rs:348:40
2020-08-08T00:40:34.3982833Z     = note: inside `std::panicking::r#try::<i32, &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>` at /checkout/library/std/src/panicking.rs:325:15
2020-08-08T00:40:34.3983777Z     = note: inside `std::panic::catch_unwind::<&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe, i32>` at /checkout/library/std/src/panic.rs:394:14
2020-08-08T00:40:34.3984193Z     = note: inside `std::rt::lang_start_internal` at /checkout/library/std/src/rt.rs:51:25
2020-08-08T00:40:34.3984554Z     = note: inside `std::rt::lang_start::<()>` at /checkout/library/std/src/rt.rs:65:5
2020-08-08T00:40:34.3985180Z     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-08-08T00:40:34.3985466Z 
2020-08-08T00:40:34.3985733Z error: aborting due to previous error
