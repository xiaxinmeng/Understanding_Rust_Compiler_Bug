
0: backtrace::backtrace::libunwind::trace::hc410fcb66fe85b11
         at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.9/src/backtrace/libunwind.rs:53
0: backtrace::backtrace::trace::h2106294a22648407
         at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.9/src/backtrace/mod.rs:42
1: backtrace::capture::Backtrace::new_unresolved::h5d8d98b993d092ba
         at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.9/src/capture.rs:88
2: <rustc::mir::interpret::error::EvalError<'tcx> as core::convert::From<rustc::mir::interpret::error::EvalErrorKind<'tcx, u64>>>::from::h6355269b2a661412
         at librustc/mir/interpret/error.rs:236
3: <T as core::convert::Into<U>>::into::h70fcb917509539bd
         at /home/r/src/rust/rustc.2/src/libcore/convert.rs:455
4: <rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, miri::Evaluator<'tcx>> as miri::fn_call::EvalContextExt<'tcx, 'mir>>::emulate_foreign_item::h9cde0e3ce7455a4a
         at src/fn_call.rs:292
5: <rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, miri::Evaluator<'tcx>> as miri::fn_call::EvalContextExt<'tcx, 'mir>>::find_fn::h83f89524b9d1a49a
         at src/fn_call.rs:74
6: <miri::Evaluator<'tcx> as rustc_mir::interpret::machine::Machine<'a, 'mir, 'tcx>>::find_fn::hf9980473c4775f0c
         at src/lib.rs:345
6: rustc_mir::interpret::terminator::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::eval_fn_call::h401dec4a687f96e9
         at /home/r/src/rust/rustc.2/src/librustc_mir/interpret/terminator.rs:285
