
$ RUST_BACKTRACE=1 cargo test -p time_steward_integer_math --verbose
       Fresh autocfg v1.0.1
       Fresh cfg-if v1.0.0
       Fresh rand_core v0.4.2
       Fresh autocfg v0.1.7
       Fresh unicode-xid v0.2.2
       Fresh quick-error v1.2.3
       Fresh ppv-lite86 v0.2.10
       Fresh cc v1.0.67
       Fresh adler v1.0.2
       Fresh remove_dir_all v0.5.3
       Fresh gimli v0.23.0
       Fresh regex-syntax v0.6.23
       Fresh fnv v1.0.7
       Fresh bit-vec v0.6.3
       Fresh termcolor v1.1.2
       Fresh rustc-demangle v0.1.18
       Fresh object v0.23.0
       Fresh rawpointer v0.2.1
       Fresh itoa v0.4.7
       Fresh lazy_static v1.4.0
       Fresh regex-syntax v0.4.2
       Fresh nodrop v0.1.14
       Fresh array_ext v0.2.0
       Fresh rand_core v0.3.1
       Fresh rand_jitter v0.1.4
       Fresh humantime v1.3.0
       Fresh addr2line v0.14.1
       Fresh bit-set v0.5.2
       Fresh matrixmultiply v0.2.4
       Fresh libc v0.2.94
       Fresh libm v0.2.1
       Fresh rand_xorshift v0.1.1
       Fresh rand_hc v0.1.0
       Fresh rand_isaac v0.1.1
       Fresh proc-macro2 v1.0.26
       Fresh serde v1.0.125
       Fresh memchr v2.3.4
       Fresh log v0.4.14
       Fresh typenum v1.13.0
       Fresh maybe-uninit v2.0.0
       Fresh ryu v1.0.5
       Fresh bitflags v1.2.1
       Fresh arrayvec v0.4.12
       Fresh miniz_oxide v0.4.4
       Fresh num-traits v0.2.14
       Fresh getrandom v0.2.2
       Fresh rand_os v0.1.3
       Fresh atty v0.2.14
       Fresh rand v0.4.6
       Fresh rand_chacha v0.1.1
       Fresh wait-timeout v0.2.0
       Fresh rand v0.5.6
       Fresh quote v1.0.9
       Fresh aho-corasick v0.7.15
       Fresh rand_pcg v0.1.2
       Fresh generic-array v0.12.4
       Fresh smallvec v0.6.14
       Fresh backtrace v0.3.58
       Fresh serde_json v1.0.64
       Fresh rand_core v0.6.2
       Fresh num-integer v0.1.44
       Fresh num-complex v0.2.4
       Fresh approx v0.3.2
       Fresh syn v1.0.71
       Fresh regex v1.4.6
       Fresh rand v0.6.5
       Fresh rand_chacha v0.3.0
       Fresh num-bigint v0.2.6
       Fresh synstructure v0.12.4
       Fresh num-iter v0.1.42
       Fresh serde_derive v1.0.125
       Fresh alga v0.9.3
       Fresh rand v0.8.3
       Fresh num-rational v0.2.4
       Fresh env_logger v0.5.13
       Fresh failure_derive v0.1.8
       Fresh tempfile v3.2.0
       Fresh num v0.2.1
       Fresh failure v0.1.8
       Fresh quickcheck v0.6.2
       Fresh nalgebra v0.18.1
       Fresh rusty-fork v0.2.2
       Fresh proptest v0.7.2
   Compiling time_steward_integer_math v0.1.0 (/n/autobuild/time-steward/build/integer-math)
     Running `rustc --crate-name time_steward_integer_math --edition=2018 integer-math/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test -C metadata=e392076af31eab96 -C extra-filename=-e392076af31eab96 --out-dir /n/autobuild/time-steward/build/target/debug/deps -C incremental=/n/autobuild/time-steward/build/target/debug/incremental -L dependency=/n/autobuild/time-steward/build/target/debug/deps --extern array_ext=/n/autobuild/time-steward/build/target/debug/deps/libarray_ext-47d3e5cfb7b3228f.rlib --extern arrayvec=/n/autobuild/time-steward/build/target/debug/deps/libarrayvec-f28918546fa5fd9d.rlib --extern failure=/n/autobuild/time-steward/build/target/debug/deps/libfailure-8f57cc1d23951cc4.rlib --extern nalgebra=/n/autobuild/time-steward/build/target/debug/deps/libnalgebra-74d664a57db5edb4.rlib --extern num=/n/autobuild/time-steward/build/target/debug/deps/libnum-42f75ee379f121e8.rlib --extern proptest=/n/autobuild/time-steward/build/target/debug/deps/libproptest-b64410933db44bd1.rlib --extern quickcheck=/n/autobuild/time-steward/build/target/debug/deps/libquickcheck-884295518ecc1f5e.rlib --extern rand=/n/autobuild/time-steward/build/target/debug/deps/librand-5bb4359a6331ad11.rlib --extern serde=/n/autobuild/time-steward/build/target/debug/deps/libserde-103797c98b946c95.rlib --extern serde_derive=/n/autobuild/time-steward/build/target/debug/deps/libserde_derive-6d072d8e1a850d0f.so --extern serde_json=/n/autobuild/time-steward/build/target/debug/deps/libserde_json-59bd01ada351a934.rlib --extern smallvec=/n/autobuild/time-steward/build/target/debug/deps/libsmallvec-5181b5897ba42111.rlib`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(Fingerprint(8537439170242672706, 4648092694241280842))`,
 right: `Some(Fingerprint(212923312148573672, 18320173992410164289))`: found unstable fingerprints for evaluate_obligation(56b37730cf1b7b6-4e6095387770cf20): Ok(EvaluatedToOk)', /rustc/42816d61ead7e46d462df997958ccfd514f8c21c/compiler/rustc_query_system/src/query/plumbing.rs:593:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/42816d61ead7e46d462df997958ccfd514f8c21c/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /rustc/42816d61ead7e46d462df997958ccfd514f8c21c/library/core/src/panicking.rs:92:14
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed
   4: rustc_query_system::query::plumbing::incremental_verify_ich
   5: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
   6: rustc_data_structures::stack::ensure_sufficient_stack
   7: rustc_query_system::query::plumbing::get_query_impl
   8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::evaluate_obligation
   9: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  10: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  11: rustc_trait_selection::traits::fulfill::FulfillProcessor::process_trait_obligation
  12: rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations
  13: rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
  14: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
  15: <rustc_infer::infer::InferCtxtBuilder as rustc_trait_selection::infer::InferCtxtBuilderExt>::enter_canonical_trait_query
  16: rustc_traits::normalize_projection_ty::normalize_projection_ty
  17: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::normalize_projection_ty>::compute
  18: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  19: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
  20: rustc_data_structures::stack::ensure_sufficient_stack
  21: rustc_query_system::query::plumbing::get_query_impl
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_projection_ty
  23: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  24: rustc_middle::ty::fold::TypeFoldable::fold_with
  25: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
  26: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  27: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
  28: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  29: rustc_middle::ty::fold::TypeFoldable::fold_with
  30: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
  31: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  32: rustc_middle::ty::util::fold_list
  33: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
  34: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  35: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
  36: rustc_middle::ty::fold::TypeFoldable::fold_with
  37: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
  38: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  39: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
  40: rustc_middle::ty::fold::TypeFoldable::fold_with
  41: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
  42: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  43: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
  44: rustc_infer::infer::InferCtxtBuilder::enter
  45: core::ops::function::FnOnce::call_once
  46: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  47: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
  48: rustc_data_structures::stack::ensure_sufficient_stack
  49: rustc_query_system::query::plumbing::get_query_impl
  50: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_generic_arg_after_erasing_regions
  51: <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty
  52: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_terminator
  53: rustc_mir::monomorphize::collector::collect_neighbours
  54: rustc_mir::monomorphize::collector::collect_items_rec
  55: rustc_mir::monomorphize::collector::collect_items_rec
  56: rustc_mir::monomorphize::collector::collect_items_rec
  57: rustc_mir::monomorphize::collector::collect_items_rec
  58: rustc_mir::monomorphize::collector::collect_items_rec
  59: rustc_session::utils::<impl rustc_session::session::Session>::time
  60: rustc_mir::monomorphize::collector::collect_crate_mono_items
  61: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  62: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::collect_and_partition_mono_items>::compute
  63: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  64: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  65: rustc_data_structures::stack::ensure_sufficient_stack
  66: rustc_query_system::query::plumbing::force_query_with_job
  67: rustc_query_system::query::plumbing::get_query_impl
  68: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  69: rustc_codegen_ssa::base::codegen_crate
  70: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  71: rustc_interface::passes::QueryContext::enter
  72: rustc_interface::queries::Queries::ongoing_codegen
  73: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  74: rustc_span::with_source_map
  75: rustc_interface::interface::create_compiler_and_run
  76: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (42816d61e 2021-04-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `proptest::strategy::Map<proptest::strategy::BoxedStrategy<(std::vec::Vec<i64>, u32)>, [closure@integer-math/src/polynomial.rs:966:123: 971:6]>: proptest::strategy::Strategy`
#1 [normalize_projection_ty] normalizing `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: ProjectionTy { substs: [proptest::strategy::Map<proptest::strategy::NoShrink<proptest::strategy::Flatten<proptest::strategy::Map<proptest::strategy::BoxedStrategy<(std::vec::Vec<i64>, u32)>, [closure@integer-math/src/polynomial.rs:966:123: 971:6]>>>, [closure@/n/HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/proptest-0.7.2/src/sugar.rs:94:25: 94:78]>], item_def_id: DefId(56:1038 ~ proptest[e66f]::strategy::traits::Strategy::Value) } } }`
#2 [normalize_generic_arg_after_erasing_regions] normalizing `for<'r, 's> fn(&'r mut proptest::test_runner::TestRunner, &'s proptest::strategy::Map<proptest::strategy::NoShrink<proptest::strategy::Flatten<proptest::strategy::Map<proptest::strategy::BoxedStrategy<(std::vec::Vec<i64>, u32)>, [closure@integer-math/src/polynomial.rs:966:123: 971:6]>>>, [closure@/n/HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/proptest-0.7.2/src/sugar.rs:94:25: 94:78]>, [closure@proptest::test_runner::TestRunner::run_in_fork<proptest::strategy::Map<proptest::strategy::NoShrink<proptest::strategy::Flatten<proptest::strategy::Map<proptest::strategy::BoxedStrategy<(std::vec::Vec<i64>, u32)>, [closure@integer-math/src/polynomial.rs:966:123: 971:6]>>>, [closure@/n/HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/proptest-0.7.2/src/sugar.rs:94:25: 94:78]>, [closure@/n/HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/proptest-0.7.2/src/sugar.rs:95:21: 100:22]>::{closure#3}], std::vec::IntoIter<std::result::Result<(), proptest::test_runner::TestCaseError>>, proptest::test_runner::runner::ForkOutput) -> std::result::Result<(), proptest::test_runner::TestError<<<proptest::strategy::Map<proptest::strategy::NoShrink<proptest::strategy::Flatten<proptest::strategy::Map<proptest::strategy::BoxedStrategy<(std::vec::Vec<i64>, u32)>, [closure@integer-math/src/polynomial.rs:966:123: 971:6]>>>, [closure@/n/HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/proptest-0.7.2/src/sugar.rs:94:25: 94:78]> as proptest::strategy::Strategy>::Value as proptest::strategy::ValueTree>::Value>> {proptest::test_runner::TestRunner::run_in_process_with_replay::<proptest::strategy::Map<proptest::strategy::NoShrink<proptest::strategy::Flatten<proptest::strategy::Map<proptest::strategy::BoxedStrategy<(std::vec::Vec<i64>, u32)>, [closure@integer-math/src/polynomial.rs:966:123: 971:6]>>>, [closure@/n/HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/proptest-0.7.2/src/sugar.rs:94:25: 94:78]>, [closure@proptest::test_runner::TestRunner::run_in_fork<proptest::strategy::Map<proptest::strategy::NoShrink<proptest::strategy::Flatten<proptest::strategy::Map<proptest::strategy::BoxedStrategy<(std::vec::Vec<i64>, u32)>, [closure@integer-math/src/polynomial.rs:966:123: 971:6]>>>, [closure@/n/HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/proptest-0.7.2/src/sugar.rs:94:25: 94:78]>, [closure@/n/HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/proptest-0.7.2/src/sugar.rs:95:21: 100:22]>::{closure#3}], std::vec::IntoIter<std::result::Result<(), proptest::test_runner::TestCaseError>>>}`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: could not compile `time_steward_integer_math`

Caused by:
  process didn't exit successfully: `rustc --crate-name time_steward_integer_math --edition=2018 integer-math/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test -C metadata=e392076af31eab96 -C extra-filename=-e392076af31eab96 --out-dir /n/autobuild/time-steward/build/target/debug/deps -C incremental=/n/autobuild/time-steward/build/target/debug/incremental -L dependency=/n/autobuild/time-steward/build/target/debug/deps --extern array_ext=/n/autobuild/time-steward/build/target/debug/deps/libarray_ext-47d3e5cfb7b3228f.rlib --extern arrayvec=/n/autobuild/time-steward/build/target/debug/deps/libarrayvec-f28918546fa5fd9d.rlib --extern failure=/n/autobuild/time-steward/build/target/debug/deps/libfailure-8f57cc1d23951cc4.rlib --extern nalgebra=/n/autobuild/time-steward/build/target/debug/deps/libnalgebra-74d664a57db5edb4.rlib --extern num=/n/autobuild/time-steward/build/target/debug/deps/libnum-42f75ee379f121e8.rlib --extern proptest=/n/autobuild/time-steward/build/target/debug/deps/libproptest-b64410933db44bd1.rlib --extern quickcheck=/n/autobuild/time-steward/build/target/debug/deps/libquickcheck-884295518ecc1f5e.rlib --extern rand=/n/autobuild/time-steward/build/target/debug/deps/librand-5bb4359a6331ad11.rlib --extern serde=/n/autobuild/time-steward/build/target/debug/deps/libserde-103797c98b946c95.rlib --extern serde_derive=/n/autobuild/time-steward/build/target/debug/deps/libserde_derive-6d072d8e1a850d0f.so --extern serde_json=/n/autobuild/time-steward/build/target/debug/deps/libserde_json-59bd01ada351a934.rlib --extern smallvec=/n/autobuild/time-steward/build/target/debug/deps/libsmallvec-5181b5897ba42111.rlib` (exit status: 101)
