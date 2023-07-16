
$ rustc test.rs
error: internal compiler error: compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:327:26: while adjusting Expr { hir_id: HirId { owner: DefId(0:3 ~ test[da6a]::main), local_id: 42 }, kind: MethodCall(PathSegment { ident: into_iter#0, hir_id: Some(HirId { owner: DefId(0:3 ~ test[da6a]::main), local_id: 39 }), res: Some(Err), args: None, infer_args: true }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ test[da6a]::main), local_id: 41 }, kind: Path(Resolved(None, Path { span: test.rs:10:18: 10:19 (#0), res: Local(HirId { owner: DefId(0:3 ~ test[da6a]::main), local_id: 34 }), segments: [PathSegment { ident: t#0, hir_id: Some(HirId { owner: DefId(0:3 ~ test[da6a]::main), local_id: 40 }), res: Some(Local(HirId { owner: DefId(0:3 ~ test[da6a]::main), local_id: 34 })), args: None, infer_args: true }] })), span: test.rs:10:18: 10:19 (#0) }], test.rs:10:20: 10:31 (#0)), span: test.rs:10:18: 10:31 (#0) }, can't compose [Borrow(Ref('_#6r, Mut { allow_two_phase_borrow: Yes })) -> &mut std::vec::IntoIter<{integer}>] and [Borrow(Ref('_#8r, Mut { allow_two_phase_borrow: Yes })) -> &mut std::vec::IntoIter<{integer}>]

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1158:9
stack backtrace:
   0:        0x103718b84 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1eb510dfd09ed899
   1:        0x10376dee0 - core::fmt::write::h00f1496c6caeaf10
   2:        0x10370aa8c - std::io::Write::write_fmt::h2bb9a347f72939fa
   3:        0x10371c8c8 - std::panicking::default_hook::{{closure}}::h437574f21bffb80b
   4:        0x10371c538 - std::panicking::default_hook::h0d118f23c310e4da
   5:        0x10a74da4c - rustc_driver[511e453c955ff185]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:        0x10371d0c8 - std::panicking::rust_panic_with_hook::hf2ea2a5cb14e624b
   7:        0x10e1a9b7c - std[e23bf27bba9e92a8]::panicking::begin_panic::<rustc_errors[f4b0e5b914d5bbb1]::ExplicitBug>::{closure#0}
   8:        0x10e1a9b34 - std[e23bf27bba9e92a8]::sys_common::backtrace::__rust_end_short_backtrace::<std[e23bf27bba9e92a8]::panicking::begin_panic<rustc_errors[f4b0e5b914d5bbb1]::ExplicitBug>::{closure#0}, !>
   9:        0x10e4778f8 - std[e23bf27bba9e92a8]::panicking::begin_panic::<rustc_errors[f4b0e5b914d5bbb1]::ExplicitBug>
  10:        0x10e1d7ae4 - std[e23bf27bba9e92a8]::panic::panic_any::<rustc_errors[f4b0e5b914d5bbb1]::ExplicitBug>
  11:        0x10e1d54b0 - <rustc_errors[f4b0e5b914d5bbb1]::HandlerInner>::bug
  12:        0x10e1d3c44 - <rustc_errors[f4b0e5b914d5bbb1]::Handler>::bug
  13:        0x10e0824dc - rustc_middle[45aced61d2d8c29b]::ty::context::tls::with_opt::<rustc_middle[45aced61d2d8c29b]::util::bug::opt_span_bug_fmt<rustc_span[4a73d757271419d3]::span_encoding::Span>::{closure#0}, ()>
  14:        0x10e083948 - rustc_middle[45aced61d2d8c29b]::util::bug::opt_span_bug_fmt::<rustc_span[4a73d757271419d3]::span_encoding::Span>
  15:        0x10e46d628 - rustc_middle[45aced61d2d8c29b]::util::bug::bug_fmt
  16:        0x10cb5f78c - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::apply_adjustments
  17:        0x10cc4ac94 - <rustc_typeck[f85cebba5e32df34]::check::method::confirm::ConfirmContext>::confirm
  18:        0x10cb64148 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::lookup_method
  19:        0x10cb5b118 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_kind
  20:        0x10cb24490 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:        0x10cb5b0d4 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_kind
  22:        0x10cb24490 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  23:        0x10cb30aa0 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_argument_types
  24:        0x10cb14354 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  25:        0x10cb121fc - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_call
  26:        0x10cb24490 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  27:        0x10cb33e00 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  28:        0x10cb24490 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:        0x10cb24fa4 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_return_expr
  30:        0x10ccf3104 - rustc_typeck[f85cebba5e32df34]::check::check::check_fn
  31:        0x10cb58878 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_closure
  32:        0x10cb5b048 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_kind
  33:        0x10cb24490 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:        0x10cb30aa0 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_argument_types
  35:        0x10cb2fca8 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_method_argument_types
  36:        0x10cb5d2d8 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_kind
  37:        0x10cb24490 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  38:        0x10cb5b0d4 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_kind
  39:        0x10cb24490 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  40:        0x10cb33318 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_decl_initializer
  41:        0x10cb33408 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_decl
  42:        0x10cb335d0 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_stmt
  43:        0x10cb33dd0 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  44:        0x10cb24490 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  45:        0x10cb24fa4 - <rustc_typeck[f85cebba5e32df34]::check::fn_ctxt::FnCtxt>::check_return_expr
  46:        0x10ccf3104 - rustc_typeck[f85cebba5e32df34]::check::check::check_fn
  47:        0x10cc2c118 - <rustc_infer[6d388a7be5ba51cf]::infer::InferCtxtBuilder>::enter::<&rustc_middle[45aced61d2d8c29b]::ty::context::TypeckResults, <rustc_typeck[f85cebba5e32df34]::check::inherited::InheritedBuilder>::enter<rustc_typeck[f85cebba5e32df34]::check::typeck_with_fallback<rustc_typeck[f85cebba5e32df34]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[45aced61d2d8c29b]::ty::context::TypeckResults>::{closure#0}>
  48:        0x10cbe95d8 - rustc_typeck[f85cebba5e32df34]::check::typeck
  49:        0x10d41b560 - rustc_query_system[6d9035de7c2bf143]::query::plumbing::try_execute_query::<rustc_query_impl[3c53bdc758f33676]::plumbing::QueryCtxt, rustc_query_system[6d9035de7c2bf143]::query::caches::DefaultCache<rustc_span[4a73d757271419d3]::def_id::LocalDefId, &rustc_middle[45aced61d2d8c29b]::ty::context::TypeckResults>>
  50:        0x10d4fc0a4 - rustc_query_system[6d9035de7c2bf143]::query::plumbing::get_query::<rustc_query_impl[3c53bdc758f33676]::queries::typeck, rustc_query_impl[3c53bdc758f33676]::plumbing::QueryCtxt>
  51:        0x10cbe99a8 - rustc_typeck[f85cebba5e32df34]::check::typeck
  52:        0x10d41b560 - rustc_query_system[6d9035de7c2bf143]::query::plumbing::try_execute_query::<rustc_query_impl[3c53bdc758f33676]::plumbing::QueryCtxt, rustc_query_system[6d9035de7c2bf143]::query::caches::DefaultCache<rustc_span[4a73d757271419d3]::def_id::LocalDefId, &rustc_middle[45aced61d2d8c29b]::ty::context::TypeckResults>>
  53:        0x10d4fc0a4 - rustc_query_system[6d9035de7c2bf143]::query::plumbing::get_query::<rustc_query_impl[3c53bdc758f33676]::queries::typeck, rustc_query_impl[3c53bdc758f33676]::plumbing::QueryCtxt>
  54:        0x10cc70ea8 - <rustc_middle[45aced61d2d8c29b]::hir::map::Map>::par_body_owners::<rustc_typeck[f85cebba5e32df34]::check::typeck_item_bodies::{closure#0}>
  55:        0x10cbecdcc - rustc_typeck[f85cebba5e32df34]::check::typeck_item_bodies
  56:        0x10d45e088 - rustc_query_system[6d9035de7c2bf143]::query::plumbing::try_execute_query::<rustc_query_impl[3c53bdc758f33676]::plumbing::QueryCtxt, rustc_query_system[6d9035de7c2bf143]::query::caches::DefaultCache<(), ()>>
  57:        0x10d4ca764 - rustc_query_system[6d9035de7c2bf143]::query::plumbing::get_query::<rustc_query_impl[3c53bdc758f33676]::queries::typeck_item_bodies, rustc_query_impl[3c53bdc758f33676]::plumbing::QueryCtxt>
  58:        0x10cbef764 - rustc_typeck[f85cebba5e32df34]::check_crate
  59:        0x10a827ba8 - rustc_interface[ebfef333567d6520]::passes::analysis
  60:        0x10d4538ec - rustc_query_system[6d9035de7c2bf143]::query::plumbing::try_execute_query::<rustc_query_impl[3c53bdc758f33676]::plumbing::QueryCtxt, rustc_query_system[6d9035de7c2bf143]::query::caches::DefaultCache<(), core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>>>
  61:        0x10d4fc4f4 - rustc_query_system[6d9035de7c2bf143]::query::plumbing::get_query::<rustc_query_impl[3c53bdc758f33676]::queries::analysis, rustc_query_impl[3c53bdc758f33676]::plumbing::QueryCtxt>
  62:        0x10a743ab0 - <rustc_interface[ebfef333567d6520]::passes::QueryContext>::enter::<rustc_driver[511e453c955ff185]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>>
  63:        0x10a70eff8 - <rustc_interface[ebfef333567d6520]::interface::Compiler>::enter::<rustc_driver[511e453c955ff185]::run_compiler::{closure#1}::{closure#2}, core[ced426ff7c2a0c73]::result::Result<core[ced426ff7c2a0c73]::option::Option<rustc_interface[ebfef333567d6520]::queries::Linker>, rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>>
  64:        0x10a717ad8 - rustc_span[4a73d757271419d3]::with_source_map::<core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>, rustc_interface[ebfef333567d6520]::interface::create_compiler_and_run<core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>, rustc_driver[511e453c955ff185]::run_compiler::{closure#1}>::{closure#1}>
  65:        0x10a70dea4 - rustc_interface[ebfef333567d6520]::interface::create_compiler_and_run::<core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>, rustc_driver[511e453c955ff185]::run_compiler::{closure#1}>
  66:        0x10a6f40b8 - <scoped_tls[af87e99ea2c8eff7]::ScopedKey<rustc_span[4a73d757271419d3]::SessionGlobals>>::set::<rustc_interface[ebfef333567d6520]::interface::run_compiler<core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>, rustc_driver[511e453c955ff185]::run_compiler::{closure#1}>::{closure#0}, core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>>
  67:        0x10a6f2130 - std[e23bf27bba9e92a8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ebfef333567d6520]::util::run_in_thread_pool_with_globals<rustc_interface[ebfef333567d6520]::interface::run_compiler<core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>, rustc_driver[511e453c955ff185]::run_compiler::{closure#1}>::{closure#0}, core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>>::{closure#0}, core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>>
  68:        0x10a6ed6b4 - <<std[e23bf27bba9e92a8]::thread::Builder>::spawn_unchecked_<rustc_interface[ebfef333567d6520]::util::run_in_thread_pool_with_globals<rustc_interface[ebfef333567d6520]::interface::run_compiler<core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>, rustc_driver[511e453c955ff185]::run_compiler::{closure#1}>::{closure#0}, core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>>::{closure#0}, core[ced426ff7c2a0c73]::result::Result<(), rustc_errors[f4b0e5b914d5bbb1]::ErrorReported>>::{closure#1} as core[ced426ff7c2a0c73]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  69:        0x1037287c0 - std::sys::unix::thread::Thread::new::thread_start::h3a76c8be5d1971dc
  70:        0x1a29b1240 - __pthread_deallocate

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (6a7055661 2022-02-27) running on aarch64-apple-darwin

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck] type-checking `main::{closure#0}`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
