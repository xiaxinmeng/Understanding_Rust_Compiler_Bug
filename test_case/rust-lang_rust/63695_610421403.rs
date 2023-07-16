`
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
 --> ./63695.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default

warning: unnecessary braces around const expression
  --> ./63695.rs:14:25
   |
14 |   println!("{}", test::<{8i32}>());
   |                         ^^^^^^ help: remove these braces
   |
   = note: `#[warn(unused_braces)]` on by default

warning: unnecessary braces around const expression
  --> ./63695.rs:15:29
   |
15 |   println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |                             ^^^^^^^ help: remove these braces

thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', src/librustc_mir_build/hair/pattern/_match.rs:2325:13
stack backtrace:
   0:     0x7f5d7e208b24 - backtrace::backtrace::libunwind::trace::hcdd38e03c5c0ae1d
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1:     0x7f5d7e208b24 - backtrace::backtrace::trace_unsynchronized::he5bd7c616dadfd7d
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2:     0x7f5d7e208b24 - std::sys_common::backtrace::_print_fmt::h5c76d4ca71f55821
                               at src/libstd/sys_common/backtrace.rs:78
   3:     0x7f5d7e208b24 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2c8c63d62d7bea1c
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x7f5d7e2470ec - core::fmt::write::h97d981a565c87982
                               at src/libcore/fmt/mod.rs:1069
   5:     0x7f5d7e1fa5b3 - std::io::Write::write_fmt::h76b54795ca4d1941
                               at src/libstd/io/mod.rs:1439
   6:     0x7f5d7e20db25 - std::sys_common::backtrace::_print::h93fb2909159290d7
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x7f5d7e20db25 - std::sys_common::backtrace::print::ha755c3134746c2d0
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7f5d7e20db25 - std::panicking::default_hook::{{closure}}::h3a7ab24b109d5437
                               at src/libstd/panicking.rs:198
   9:     0x7f5d7e20d862 - std::panicking::default_hook::h2aa3c18a39936382
                               at src/libstd/panicking.rs:218
  10:     0x7f5d7e7b1893 - rustc_driver::report_ice::h9d751a31d8f5cb70
  11:     0x7f5d7e20e2a5 - std::panicking::rust_panic_with_hook::h5035e60b675c5c99
                               at src/libstd/panicking.rs:515
  12:     0x7f5d7e20ddbb - rust_begin_unwind
                               at src/libstd/panicking.rs:419
  13:     0x7f5d7e243d81 - core::panicking::panic_fmt::h7fe09dc6d8aa54c4
                               at src/libcore/panicking.rs:111
  14:     0x7f5d7e243d42 - core::panicking::panic_bounds_check::h455995dc3a9d6188
                               at src/libcore/panicking.rs:69
  15:     0x7f5d7f77af27 - rustc_mir_build::hair::pattern::_match::PatStack::specialize_constructor::h1594ec76531468b4
  16:     0x7f5d7f78244b - rustc_mir_build::hair::pattern::_match::is_useful_specialized::hb6a04933fc3595e9
  17:     0x7f5d7f6cdfba - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold::h1ff7bd5b8172c9d8
  18:     0x7f5d7f781465 - rustc_mir_build::hair::pattern::_match::is_useful::hcbec6f5164cb9e30
  19:     0x7f5d7f6aa017 - rustc_mir_build::hair::pattern::check_match::check_not_useful::hb67cd7eba080cf5c
  20:     0x7f5d7f6aa2f3 - rustc_mir_build::hair::pattern::check_match::check_exhaustive::h3bda1669a3d7ef3e
  21:     0x7f5d7f77d6eb - rustc_mir_build::hair::pattern::_match::MatchCheckCtxt::create_and_enter::h87762c10c625c6c4
  22:     0x7f5d7f6a8a6a - <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr::h6849508b1ee019d7
  23:     0x7f5d7f6a8998 - <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr::h6849508b1ee019d7
  24:     0x7f5d7f69c98d - rustc_hir::intravisit::walk_expr::hd0d8de0cb740ca04
  25:     0x7f5d7f6a8998 - <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr::h6849508b1ee019d7
  26:     0x7f5d7f69c98d - rustc_hir::intravisit::walk_expr::hd0d8de0cb740ca04
  27:     0x7f5d7f6a8998 - <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr::h6849508b1ee019d7
  28:     0x7f5d7f698d9a - rustc_hir::intravisit::walk_block::h9c6be5556bdcd7bb
  29:     0x7f5d7f6a8998 - <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr::h6849508b1ee019d7
  30:     0x7f5d7f698d9a - rustc_hir::intravisit::walk_block::h9c6be5556bdcd7bb
  31:     0x7f5d7f6a8998 - <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr::h6849508b1ee019d7
  32:     0x7f5d7f6a8762 - rustc_mir_build::hair::pattern::check_match::check_match::h88d733cc4c663b84
  33:     0x7f5d7eb09432 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_match>::compute::h24ee063abef3f925
  34:     0x7f5d7ea3c7c4 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hab8aed87ddfd8f0e
  35:     0x7f5d7eb1ecc3 - rustc_query_system::query::plumbing::get_query::h8370e76bdc54ab4c
  36:     0x7f5d7eb0b797 - rustc_query_system::query::plumbing::ensure_query::hf27b412ff7d91b2a
  37:     0x7f5d7ea8689a - rustc_session::utils::<impl rustc_session::session::Session>::time::hd888275f41dade91
  38:     0x7f5d7ea847f7 - rustc_session::utils::<impl rustc_session::session::Session>::time::h8a84f49d5f3a9e9a
  39:     0x7f5d7eb45614 - rustc_interface::passes::analysis::h524f56f1017c1f7e
  40:     0x7f5d7e7beabb - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute::h92269a4be7a4efde
  41:     0x7f5d7e8f783a - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hfcc7b4779663ce33
  42:     0x7f5d7e7c3968 - rustc_query_system::query::plumbing::get_query::h7724d6af1510ed57
  43:     0x7f5d7e79b457 - rustc_middle::ty::context::tls::enter_global::h2eb0eb42c8d04e67
  44:     0x7f5d7e903514 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h5d0007c9e772807e
  45:     0x7f5d7e7b978d - scoped_tls::ScopedKey<T>::set::h33da4c1fce01c783
  46:     0x7f5d7e7b6bd4 - rustc_ast::attr::with_globals::h3aa58156df1d5bbe
  47:     0x7f5d7e7c86a4 - std::sys_common::backtrace::__rust_begin_short_backtrace::hfec31279bda1a2bb
  48:     0x7f5d7e905f1e - core::ops::function::FnOnce::call_once{{vtable.shim}}::hf6fcb50b5f0a17ae
  49:     0x7f5d7e1ea72f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h0125fb8449617e8e
                               at /rustc/6dee5f1126dfd5c9314ee5ae9d9eb010e35ef257/src/liballoc/boxed.rs:1008
  50:     0x7f5d7e21e2e3 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h54cf67aadcb0c7ef
                               at /rustc/6dee5f1126dfd5c9314ee5ae9d9eb010e35ef257/src/liballoc/boxed.rs:1008
  51:     0x7f5d7e21e2e3 - std::sys::unix::thread::Thread::new::thread_start::h7b7ac7c277b0d963
                               at src/libstd/sys/unix/thread.rs:87
  52:     0x7f5d7e13046f - start_thread
  53:     0x7f5d7e0503d3 - clone
  54:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (6dee5f112 2020-04-06) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [check_match] processing `main`
#1 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: unexpected const parent in type_of_def_id(): Expr(Expr { hir_id: HirId { owner: DefId(0:11 ~ 63695[317d]::main[0]), local_id: 82 }, kind: MethodCall(PathSegment { ident: test#0, hir_id: Some(HirId { owner: DefId(0:11 ~ 63695[317d]::main[0]), local_id: 79 }), res: Some(Err), args: Some(GenericArgs { args: [Const(ConstArg { value: AnonConst { hir_id: HirId { owner: DefId(0:11 ~ 63695[317d]::main[0]), local_id: 75 }, body: BodyId { hir_id: HirId { owner: DefId(0:11 ~ 63695[317d]::main[0]), local_id: 78 } } }, span: ./63695.rs:15:29: 15:36 })], bindings: [], parenthesized: false }), infer_args: false }, ./63695.rs:15:22: 15:26, [Expr { hir_id: HirId { owner: DefId(0:11 ~ 63695[317d]::main[0]), local_id: 81 }, kind: Path(Resolved(None, Path { span: ./63695.rs:15:18: 15:21, res: Local(HirId { owner: DefId(0:11 ~ 63695[317d]::main[0]), local_id: 1 }), segments: [PathSegment { ident: foo#0, hir_id: Some(HirId { owner: DefId(0:11 ~ 63695[317d]::main[0]), local_id: 80 }), res: Some(Local(HirId { owner: DefId(0:11 ~ 63695[317d]::main[0]), local_id: 1 })), args: None, infer_args: true }] })), attrs: ThinVec(None), span: ./63695.rs:15:18: 15:21 }]), attrs: ThinVec(None), span: ./63695.rs:15:18: 15:39 })

error: internal compiler error: cat_expr Errd
  --> ./63695.rs:12:11
   |
12 |   fn main() {
   |  ___________^
13 | |   let foo = S();
14 | |   println!("{}", test::<{8i32}>());
15 | |   println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
16 | | }
   | |_^

error: internal compiler error: cat_expr Errd
  --> ./63695.rs:15:3
   |
15 |   println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> ./63695.rs:15:3
   |
15 |   println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> ./63695.rs:15:18
   |
15 |   println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |                  ^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: cat_expr Errd
  --> ./63695.rs:15:18
   |
15 |   println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |                  ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> ./63695.rs:15:29
   |
15 |   println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |                             ^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
stack backtrace:
   0:     0x7f5d7e208b24 - backtrace::backtrace::libunwind::trace::hcdd38e03c5c0ae1d
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1:     0x7f5d7e208b24 - backtrace::backtrace::trace_unsynchronized::he5bd7c616dadfd7d
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2:     0x7f5d7e208b24 - std::sys_common::backtrace::_print_fmt::h5c76d4ca71f55821
                               at src/libstd/sys_common/backtrace.rs:78
   3:     0x7f5d7e208b24 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2c8c63d62d7bea1c
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x7f5d7e2470ec - core::fmt::write::h97d981a565c87982
                               at src/libcore/fmt/mod.rs:1069
   5:     0x7f5d7e1fa5b3 - std::io::Write::write_fmt::h76b54795ca4d1941
                               at src/libstd/io/mod.rs:1439
   6:     0x7f5d7e20db25 - std::sys_common::backtrace::_print::h93fb2909159290d7
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x7f5d7e20db25 - std::sys_common::backtrace::print::ha755c3134746c2d0
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7f5d7e20db25 - std::panicking::default_hook::{{closure}}::h3a7ab24b109d5437
                               at src/libstd/panicking.rs:198
   9:     0x7f5d7e20d862 - std::panicking::default_hook::h2aa3c18a39936382
                               at src/libstd/panicking.rs:218
  10:     0x7f5d7e7b1893 - rustc_driver::report_ice::h9d751a31d8f5cb70
  11:     0x7f5d7e20e2a5 - std::panicking::rust_panic_with_hook::h5035e60b675c5c99
                               at src/libstd/panicking.rs:515
  12:     0x7f5d80f3f42e - std::panicking::begin_panic::h634b5f0eb0b90992
  13:     0x7f5d80f77a02 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h5759019f31a3ed3d
  14:     0x7f5d7e8e70c6 - core::ptr::drop_in_place::hd3d25bd1d04868da
  15:     0x7f5d7e8eb4f2 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h41dbd5209bf268e4
  16:     0x7f5d7e906c6d - core::ptr::drop_in_place::h0ad6eaa53f54c79b
  17:     0x7f5d7e9042dd - rustc_interface::interface::run_compiler_in_existing_thread_pool::h5d0007c9e772807e
  18:     0x7f5d7e7b978d - scoped_tls::ScopedKey<T>::set::h33da4c1fce01c783
  19:     0x7f5d7e7b6bd4 - rustc_ast::attr::with_globals::h3aa58156df1d5bbe
  20:     0x7f5d7e7c86a4 - std::sys_common::backtrace::__rust_begin_short_backtrace::hfec31279bda1a2bb
  21:     0x7f5d7e905f1e - core::ops::function::FnOnce::call_once{{vtable.shim}}::hf6fcb50b5f0a17ae
  22:     0x7f5d7e1ea72f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h0125fb8449617e8e
                               at /rustc/6dee5f1126dfd5c9314ee5ae9d9eb010e35ef257/src/liballoc/boxed.rs:1008
  23:     0x7f5d7e21e2e3 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h54cf67aadcb0c7ef
                               at /rustc/6dee5f1126dfd5c9314ee5ae9d9eb010e35ef257/src/liballoc/boxed.rs:1008
  24:     0x7f5d7e21e2e3 - std::sys::unix::thread::Thread::new::thread_start::h7b7ac7c277b0d963
                               at src/libstd/sys/unix/thread.rs:87
  25:     0x7f5d7e13046f - start_thread
  26:     0x7f5d7e0503d3 - clone
  27:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (6dee5f112 2020-04-06) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
[1]    609374 illegal hardware instruction (core dumped)  RUST_BACKTRACE=full rustc ./63695.rs
