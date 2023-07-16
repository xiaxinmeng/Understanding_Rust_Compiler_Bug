
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/driver/jit.rs:239:59
stack backtrace:
   0: rust_begin_unwind
             at /rustc/2019147c5642c08cdb9ad4cacd97dd1fa4ffa701/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/2019147c5642c08cdb9ad4cacd97dd1fa4ffa701/library/core/src/panicking.rs:142:14
   2: core::panicking::panic
             at /rustc/2019147c5642c08cdb9ad4cacd97dd1fa4ffa701/library/core/src/panicking.rs:48:5
   3: rustc_codegen_cranelift::driver::jit::jit_fn
   4: rustc_codegen_cranelift::driver::jit::run_jit
