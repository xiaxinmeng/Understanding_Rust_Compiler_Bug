
backtrace frames: 72
0: backtrace::backtrace::libunwind::trace::h4333246b1309ea02
	at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.9/src/backtrace/libunwind.rs:53
0: backtrace::backtrace::trace::h04e30d93063b4479
	at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.9/src/backtrace/mod.rs:42
1: backtrace::capture::Backtrace::new_unresolved::haf9a7c94d7c84f11
	at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.9/src/capture.rs:88
2: <rustc::mir::interpret::error::EvalError<'tcx> as core::convert::From<rustc::mir::interpret::error::EvalErrorKind<'tcx, u64>>>::from::hcea04b849bfd8ad1
	at librustc/mir/interpret/error.rs:206
3: <T as core::convert::Into<U>>::into::h6971e8e95ad92abe
	at /home/r/src/rust/rustc.3/src/libcore/convert.rs:456
3: <rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M> as rustc_target::abi::LayoutOf>::layout_of::{{closure}}::h79fb29344e377e52
	at librustc_mir/interpret/eval_context.rs:169
3: <core::result::Result<T, E>>::map_err::hd0b511dcc0d127aa
	at /home/r/src/rust/rustc.3/src/libcore/result.rs:530
3: <rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M> as rustc_target::abi::LayoutOf>::layout_of::h977bf27fbf74f806
	at librustc_mir/interpret/eval_context.rs:168
3: rustc_mir::interpret::intrinsics::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::emulate_intrinsic::hfb24b0825733ab16
	at librustc_mir/interpret/intrinsics.rs:77
4: <rustc_mir::const_eval::CompileTimeInterpreter<'a, 'mir, 'tcx> as rustc_mir::interpret::machine::Machine<'a, 'mir, 'tcx>>::call_intrinsic::h2e2017486d607b75
	at librustc_mir/const_eval.rs:406
5: rustc_mir::interpret::terminator::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::eval_fn_call::h4cc4f67756e1846c
	at librustc_mir/interpret/terminator.rs:252
6: rustc_mir::interpret::terminator::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::eval_terminator::hde1131b053413afa
	at librustc_mir/interpret/terminator.rs:105
7: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::terminator::h313c5d2bd41bf6cb
	at librustc_mir/interpret/step.rs:307
7: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::step::hed0450c3d705762a
	at librustc_mir/interpret/step.rs:79
7: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::run::hb00fb5f6b0079b0e
	at librustc_mir/interpret/step.rs:50
8: rustc_mir::const_eval::eval_body_using_ecx::he486bdd5ad00e3d1
	at librustc_mir/const_eval.rs:204
9: rustc_mir::const_eval::eval_body_and_ecx::h295b12b32be5cca7
	at librustc_mir/const_eval.rs:167
9: rustc_mir::const_eval::const_eval_raw_provider::h3e776eaa15edbc8f
	at librustc_mir/const_eval.rs:658
10: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval_raw<'tcx>>::compute::{{closure}}::ha5592ff73bea82f6
	at librustc/ty/query/plumbing.rs:834
10: rustc::ty::query::__query_compute::const_eval_raw::h2542e1d85c704c97
	at librustc/ty/query/plumbing.rs:796
11: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval_raw<'tcx>>::compute::h3b525a50b8883ade
	at librustc/ty/query/plumbing.rs:826
