
thread 'main' panicked at 'no output generated for "libgoto_def-501a4807e737f416" "rmeta"', compile.rs:1448:21
stack backtrace:
   0: rust_begin_unwind
             at /rustc/efd358333ac55ca50f786ca1ab841c003bfdfff0/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/efd358333ac55ca50f786ca1ab841c003bfdfff0/library/core/src/panicking.rs:142:14
   2: bootstrap::compile::run_cargo
             at ./src/bootstrap/compile.rs:1448:21
   3: <bootstrap::check::RustAnalyzer as bootstrap::builder::Step>::run
             at ./src/bootstrap/check.rs:353:9
   4: bootstrap::builder::Builder::ensure
             at ./src/bootstrap/builder.rs:2126:23
   5: <bootstrap::check::RustAnalyzer as bootstrap::builder::Step>::make_run
             at ./src/bootstrap/check.rs:319:9
   6: bootstrap::builder::StepDescription::maybe_run
             at ./src/bootstrap/builder.rs:270:13
   7: bootstrap::builder::StepDescription::run
             at ./src/bootstrap/builder.rs:336:17
   8: bootstrap::builder::Builder::run_step_descriptions
             at ./src/bootstrap/builder.rs:841:9
   9: bootstrap::builder::Builder::execute_cli
             at ./src/bootstrap/builder.rs:821:9
  10: bootstrap::Build::build
             at ./src/bootstrap/lib.rs:702:13
  11: bootstrap::main
             at ./src/bootstrap/bin/main.rs:50:5
  12: core::ops::function::FnOnce::call_once
             at /rustc/efd358333ac55ca50f786ca1ab841c003bfdfff0/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
