
thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 2', src/tools/cargo/src/cargo/util/cpu.rs:152:29

stack backtrace:

   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt

   1: core::fmt::write

   2: std::io::Write::write_fmt

   3: std::panicking::default_hook::{{closure}}

   4: std::panicking::default_hook

   5: std::panicking::rust_panic_with_hook

   6: std::panicking::continue_panic_fmt

   7: rust_begin_unwind

   8: core::panicking::panic_fmt

   9: core::panicking::panic_bounds_check

  10: cargo::util::cpu::State::current

  11: cargo::core::compiler::job_queue::JobQueue::new

  12: cargo::core::compiler::context::Context::compile

  13: cargo::ops::cargo_compile::compile_ws

  14: cargo::ops::cargo_compile::compile

  15: cargo::commands::build::exec

  16: cargo::cli::main

  17: cargo::main

  18: std::rt::lang_start::{{closure}}

  19: std::panicking::try::do_call

  20: __rust_maybe_catch_panic

  21: std::rt::lang_start_internal

  22: main

note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
