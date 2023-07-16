
thread 'rustc' panicked at 'Unable to fulfill trait DefId(2:1944 ~ core[6c88]::marker[0]::Unpin[0]) for 'lines::Lines<'a, 'b, L, F, S>': [FulfillmentError(Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { substs: [S], item_def_id: DefId(2:4966 ~ core[6c88]::iter[0]::traits[0]::iterator[0]::Iterator[0]::Item[0]) }, section::SectionText<'_>)), depth=3),Ambiguity)]', /rustc/c367798cfd3817ca6ae908ce675d1d99242af148/src/libstd/macros.rs:16:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1076
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:486
  11: rust_begin_unwind
             at src/libstd/panicking.rs:388
  12: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:342
  13: rustc_trait_selection::traits::auto_trait::AutoTraitFinder::find_auto_trait_generics::{{closure}}::{{closure}}
  14: rustc_middle::ty::context::GlobalCtxt::enter_local
  15: rustc_trait_selection::traits::auto_trait::AutoTraitFinder::find_auto_trait_generics
  16: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
  17: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  18: rustdoc::clean::utils::get_auto_trait_and_blanket_impls
  19: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector as rustdoc::fold::DocFolder>::fold_item
  20: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  21: rustdoc::fold::DocFolder::fold_inner_recur
  22: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector as rustdoc::fold::DocFolder>::fold_item
  23: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  24: rustdoc::fold::DocFolder::fold_inner_recur
  25: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector as rustdoc::fold::DocFolder>::fold_item
  26: rustdoc::passes::collect_trait_impls::collect_trait_impls
  27: rustc_middle::ty::context::tls::enter_global
  28: rustc_interface::interface::run_compiler_in_existing_thread_pool
  29: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  30: rustc_driver::catch_fatal_errors
  31: rustdoc::main_options
  32: scoped_tls::ScopedKey<T>::set
  33: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: Could not document `glyph_brush_layout`.
