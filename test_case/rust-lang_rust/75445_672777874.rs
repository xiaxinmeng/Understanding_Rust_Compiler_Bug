
   0: std::panicking::begin_panic
             at ./library/std/src/panicking.rs:497
   1: rustc_errors::HandlerInner::bug
             at ./src/librustc_errors/lib.rs:915
   2: rustc_errors::Handler::bug
             at ./src/librustc_errors/lib.rs:665
   3: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
             at ./src/librustc_middle/util/bug.rs:33
   4: rustc_middle::ty::context::tls::with_opt::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1792
   5: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1744
   6: rustc_middle::ty::context::tls::with_opt
             at ./src/librustc_middle/ty/context.rs:1792
   7: rustc_middle::util::bug::opt_span_bug_fmt
             at ./src/librustc_middle/util/bug.rs:29
   8: rustc_middle::util::bug::bug_fmt
             at ./src/librustc_middle/util/bug.rs:14
   9: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand::{{closure}}
             at ./src/librustc_mir/interpret/operand.rs:526
  10: core::result::Result<T,E>::map_err
             at ./library/core/src/result.rs:612
  11: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand
             at ./src/librustc_mir/interpret/operand.rs:521
  12: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
             at ./src/librustc_mir/interpret/step.rs:153
  13: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::statement
             at ./src/librustc_mir/interpret/step.rs:89
  14: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step
             at ./src/librustc_mir/interpret/step.rs:65
  15: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run
             at ./src/librustc_mir/interpret/step.rs:34
  16: rustc_mir::const_eval::eval_queries::eval_body_using_ecx
             at ./src/librustc_mir/const_eval/eval_queries.rs:57
  17: rustc_mir::const_eval::eval_queries::const_eval_raw_provider::{{closure}}
             at ./src/librustc_mir/const_eval/eval_queries.rs:313
  18: core::result::Result<T,E>::and_then
             at ./library/core/src/result.rs:729
  19: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
             at ./src/librustc_mir/const_eval/eval_queries.rs:313
