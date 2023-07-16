
--- tests::test_completion_detail_from_macro_generated_struct_fn_doc_comment stdout ----
thread 'tests::test_completion_detail_from_macro_generated_struct_fn_doc_comment' panicked at 'index out of bounds: the len is 0 but the index is 2958863903', /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/interned.rs:138:16
stack backtrace:
   0: rust_begin_unwind
             at /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/core/src/panicking.rs:69:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
   6: salsa::interned::InternTables<K>::slot_for_index
   7: salsa::interned::InternedStorage<Q>::lookup_value
   8: <salsa::interned::LookupInternedStorage<Q,IQ> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
   9: salsa::QueryTable<Q>::try_get
  10: salsa::QueryTable<Q>::get
  11: <DB as hir_def::db::InternDatabase>::lookup_intern_const::__shim
  12: <DB as hir_def::db::InternDatabase>::lookup_intern_const
  13: <hir_def::ConstId as hir_def::Lookup>::lookup
  14: hir_def::data::ConstData::const_data_query
  15: <hir_def::db::ConstDataQuery as salsa::plumbing::QueryFunction>::execute
  16: salsa::derived::slot::Slot<Q,MP>::read_upgrade::{{closure}}
  17: salsa::runtime::Runtime::execute_query_implementation
  18: salsa::derived::slot::Slot<Q,MP>::read_upgrade
  19: salsa::derived::slot::Slot<Q,MP>::read
  20: <salsa::derived::DerivedStorage<Q,MP> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
  21: salsa::QueryTable<Q>::try_get
  22: salsa::QueryTable<Q>::get
  23: <DB as hir_def::db::DefDatabase>::const_data::__shim
  24: <DB as hir_def::db::DefDatabase>::const_data
  25: salsa::plumbing::get_query_table
  26: <DB as hir_ty::db::HirDatabase>::infer_query::__shim
  27: <DB as hir_ty::db::HirDatabase>::infer_query
  28: hir_ty::db::infer_wait
  29: <DB as hir_ty::db::HirDatabase>::infer
  30: hir::source_analyzer::SourceAnalyzer::new_for_body
  31: hir::semantics::SemanticsImpl::analyze_impl
  32: hir::semantics::SemanticsImpl::analyze
  33: hir::semantics::SemanticsImpl::descend_into_macros
  34: hir::semantics::Semantics<DB>::descend_into_macros
  35: ide_completion::context::CompletionContext::new
             at ./src/context.rs:121:21
  36: ide_completion::completions
             at ./src/lib.rs:107:15
  37: ide_completion::tests::check_detail_and_documentation
             at ./src/lib.rs:175:35
  38: ide_completion::tests::test_completion_detail_from_macro_generated_struct_fn_doc_comment
             at ./src/lib.rs:235:9
  39: ide_completion::tests::test_completion_detail_from_macro_generated_struct_fn_doc_comment::{{closure}}
             at ./src/lib.rs:234:5
  40: core::ops::function::FnOnce::call_once
             at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227:5
  41: core::ops::function::FnOnce::call_once
             at /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
