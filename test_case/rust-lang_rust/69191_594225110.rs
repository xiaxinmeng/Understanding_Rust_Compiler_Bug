txt
thread 'rustc' panicked at 'Tried to access field 0 of union TyLayout {
    ty: CSeed,
    details: LayoutDetails {
        variants: Single {
            index: 0,
        },
        fields: Union(
            0,
        ),
        abi: Uninhabited,
        largest_niche: None,
        align: AbiAndPrefAlign {
            abi: Align {
                pow2: 0,
            },
            pref: Align {
                pow2: 0,
            },
        },
        size: Size {
            raw: 0,
        },
    },
} with 0 fields', src/librustc_mir/interpret/place.rs:414:17
stack backtrace:
   0:        0x110b37025 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h69a1e3d2ca9c1e11
   1:        0x110b535f1 - core::fmt::write::hf3458d798296d204
   2:        0x110b2bf79 - std::io::Write::write_fmt::h66a47365b38681ee
   3:        0x110b36ddb - std::sys_common::backtrace::print::h02498af503ed0c92
   4:        0x110b04a0e - std::panicking::default_hook::{{closure}}::h34e746f1db241fc9
   5:        0x110b04791 - std::panicking::default_hook::h2e4333a70604ecb2
   6:        0x106060dd4 - rustc_driver::report_ice::he4e28a13284da972
   7:        0x110b05040 - std::panicking::rust_panic_with_hook::h877c0d796963b6c1
   8:        0x110b04b6a - rust_begin_unwind
   9:        0x110b729d8 - std::panicking::begin_panic_fmt::h71f3a4b90a95f3d3
  10:        0x108f0858c - rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::operand_field::h87b529a0597d5280
  11:        0x108f09aa1 - rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_place_to_op::h0badc7f16a061eb7
  12:        0x108f0aad3 - rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand::h388bdbfc5c47bf0c
  13:        0x108f2ea3a - rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place::h4f24d1525a1f30d8
  14:        0x108fcea9d - rustc_mir::transform::const_prop::ConstPropagator::use_ecx::h9e82b3a972541897
  15:        0x108ec9f11 - <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_statement::hbcbb76349a1b2af8
  16:        0x108fcc172 - rustc::mir::visit::MutVisitor::visit_body::hed185033fa2d801d
  17:        0x108ec820c - <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass::h3e923c7c053214b4
  18:        0x10910c911 - rustc_mir::transform::run_passes::hb67a5bf194d0c50b
  19:        0x10910d456 - rustc_mir::transform::run_optimization_passes::h4d95f5ca8063715d
  20:        0x10910d733 - rustc_mir::transform::optimized_mir::h2eea86349c4420ec
  21:        0x109dd673b - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute::h3776afa08b9955c7
  22:        0x109eaf462 - rustc::dep_graph::graph::DepGraph::with_task_impl::h3bcd274945700fd0
  23:        0x109ca6d3d - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h1f6b8e5f14eef5bb
  24:        0x109e5dfee - rustc::ty::<impl rustc::ty::context::TyCtxt>::instance_mir::hf27ba3a3da6bb109
  25:        0x108ddf208 - rustc_mir::monomorphize::collector::collect_items_rec::h3d134124ea30f5b1
  26:        0x108ddf449 - rustc_mir::monomorphize::collector::collect_items_rec::h3d134124ea30f5b1
  27:        0x108ddf449 - rustc_mir::monomorphize::collector::collect_items_rec::h3d134124ea30f5b1
  28:        0x108ddf449 - rustc_mir::monomorphize::collector::collect_items_rec::h3d134124ea30f5b1
  29:        0x108c2159e - rustc_session::utils::<impl rustc_session::session::Session>::time::h3d5518a7f37fd9d1
  30:        0x108dde585 - rustc_mir::monomorphize::collector::collect_crate_mono_items::h0d91d5b14a52cb4b
  31:        0x108c21a80 - rustc_session::utils::<impl rustc_session::session::Session>::time::hb1832d4b48f9a79c
  32:        0x10909de3f - rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items::h96d8f9e8b1696162
  33:        0x10639ab5c - rustc::ty::query::__query_compute::collect_and_partition_mono_items::h8b7e8e7a35df0e0f
  34:        0x10649fe56 - rustc::dep_graph::graph::DepGraph::with_task_impl::hbb6e03c47e6ffedc
  35:        0x1063d3267 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::ha0e0a8abfcf6451c
  36:        0x10638c0cd - rustc_codegen_ssa::base::codegen_crate::h3768de7321d0405f
  37:        0x106342ab9 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate::hd0d92b55e53ebbcb
  38:        0x106337090 - rustc_session::utils::<impl rustc_session::session::Session>::time::hdb245d4656aad9d6
  39:        0x106329c23 - rustc_interface::passes::start_codegen::h21caada137f8cf2e
  40:        0x10632a0ae - rustc::ty::context::tls::enter_global::h125d9d08f1491fc8
  41:        0x1061eb87f - rustc_interface::queries::Query<T>::compute::h8afa9a79dfe3203f
  42:        0x1061e7edc - rustc_interface::queries::Queries::ongoing_codegen::h8f7e07e1dac4f6c6
  43:        0x1060a8cae - rustc_interface::interface::run_compiler_in_existing_thread_pool::h424d11116d6dbfb6
  44:        0x1060935dd - scoped_tls::ScopedKey<T>::set::h56349b514de6b8d6
  45:        0x10606fb45 - syntax::with_globals::hd4aec9efb871578a
  46:        0x1060a616e - std::sys_common::backtrace::__rust_begin_short_backtrace::h9eeb9c817fb78bb0
  47:        0x1060e8b62 - std::panicking::try::do_call::h0d69788ad8d7e60a
  48:        0x110b37aca - __rust_maybe_catch_panic
  49:        0x1060e8919 - std::panicking::try::h0817a7b1e46e110f
  50:        0x10607c8ec - core::ops::function::FnOnce::call_once{{vtable.shim}}::h138312dfe0ed3c99
  51:        0x110b0c61e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hf14a404fd9d2e584
  52:        0x110b0c5a6 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h8607d8336ffda5c8
  53:        0x110b2ce3a - std::sys_common::thread::start_thread::h83b37fb852a26050
  54:        0x110b20529 - std::sys::unix::thread::Thread::new::thread_start::h843727c91d6328f4
  55:     0x7fff6b060d76 - _pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-dev running on x86_64-apple-darwin

query stack during panic:
#0 [optimized_mir] processing `<Composed as Machine>::create`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
