
$ cargo run --bin quill -- -svp stdlib/core run
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/quill -svp stdlib/core run`
Apr 19 13:57:29.088  INFO quill: initialised logging with verbosity level TRACE
   Compiling quill_parser v0.1.0 (/home/sky/code/quill/src/quill_parser)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(Fingerprint(17272853890222528742, 4579056754543839504))`,
 right: `Some(Fingerprint(16699371946333824810, 9762301140808967693))`: found unstable fingerprints for predicates_of(core[ec89]::iter::traits::collect::IntoIterator): GenericPredicates { parent: None, predicates: [(Binder(TraitPredicate(<Self as std::iter::IntoIterator>), []), /home/sky/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:202:1: 202:23 (#0))] }', /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/compiler/rustc_query_system/src/query/plumbing.rs:593:5
stack backtrace:
   0:     0x7efec38ec2f0 - std::backtrace_rs::backtrace::libunwind::trace::hdcf4f90f85129e83
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7efec38ec2f0 - std::backtrace_rs::backtrace::trace_unsynchronized::h2669e30cb82f6732
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7efec38ec2f0 - std::sys_common::backtrace::_print_fmt::hfbda19e17f6db318
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7efec38ec2f0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a8751bf59281272
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7efec395d6af - core::fmt::write::h7aa6cd0067dca82a
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/core/src/fmt/mod.rs:1094:17
   5:     0x7efec38e0bb5 - std::io::Write::write_fmt::hd7dd3a1df9b6befb
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/io/mod.rs:1580:15
   6:     0x7efec38f009b - std::sys_common::backtrace::_print::h551e9ec8a9fa8106
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7efec38f009b - std::sys_common::backtrace::print::ha4b1c5e95fa040b3
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7efec38f009b - std::panicking::default_hook::{{closure}}::h0b34c9ab7fb9f857
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/panicking.rs:208:50
   9:     0x7efec38efb7d - std::panicking::default_hook::h3067e8318decd17a
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/panicking.rs:225:9
  10:     0x7efec40b24bd - rustc_driver::report_ice::h0582ed2432eb0d01
  11:     0x7efec38f07b0 - std::panicking::rust_panic_with_hook::h81b8facc50f34daa
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/panicking.rs:595:17
  12:     0x7efec38f0387 - std::panicking::begin_panic_handler::{{closure}}::ha376ab85d95a000e
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/panicking.rs:497:13
  13:     0x7efec38ec7ac - std::sys_common::backtrace::__rust_end_short_backtrace::h6795c8afdd1a77e6
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/sys_common/backtrace.rs:141:18
  14:     0x7efec38f02e9 - rust_begin_unwind
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/panicking.rs:493:5
  15:     0x7efec38be151 - core::panicking::panic_fmt::hbe99dddd3092ba3c
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/core/src/panicking.rs:92:14
  16:     0x7efec395a15e - core::panicking::assert_failed_inner::h4fdb1013d187f202
  17:     0x7efec4767b5c - core::panicking::assert_failed::h2cb4c07ec1e15c78
  18:     0x7efec5fa9d68 - rustc_query_system::query::plumbing::incremental_verify_ich::h103c82e00d4a796c
  19:     0x7efec5fbca07 - rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory::h9e56d761c3f55fd9
  20:     0x7efec547b9b0 - rustc_query_system::query::plumbing::get_query_impl::hc7fa0ab16d733c7a
  21:     0x7efec54f5139 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::predicates_of::hdd387b0bd96159fb
  22:     0x7efec5e8aa1f - rustc_typeck::outlives::explicit::ExplicitPredicatesMap::explicit_predicates_of::h7f5a65d7b7f8e549
  23:     0x7efec5e7c741 - rustc_typeck::outlives::implicit_infer::check_explicit_predicates::h292082c868c02c01
  24:     0x7efec5369d33 - <rustc_typeck::outlives::implicit_infer::InferVisitor as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h12f919059fbb748e
  25:     0x7efec5305c2d - rustc_hir::hir::Crate::visit_all_item_likes::hfab858b1d0b814e0
  26:     0x7efec5e76270 - rustc_typeck::outlives::inferred_outlives_crate::h160e6255f8a7907a
  27:     0x7efec5fc6a69 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::inferred_outlives_crate>::compute::h1a0fcac7ff623cbe
  28:     0x7efec478e822 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h4ff6601a56bd7da2
  29:     0x7efec6071929 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hc08baff31b13bc74
  30:     0x7efec5fde251 - rustc_data_structures::stack::ensure_sufficient_stack::h382a8ee98844a78b
  31:     0x7efec5f8a7da - rustc_query_system::query::plumbing::force_query_with_job::h83453c0b2eb8234a
  32:     0x7efec457fd29 - rustc_query_system::query::plumbing::force_query_impl::hba244d2d1734459e
  33:     0x7efec454a59b - rustc_query_system::query::plumbing::force_query::h913482f152e63322
  34:     0x7efec4660085 - rustc_query_impl::query_callbacks::inferred_outlives_crate::force_from_dep_node::h985057f702da451c
  35:     0x7efec54f0ed3 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::he8103ccf5a5923bc
  36:     0x7efec54f0eb9 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::he8103ccf5a5923bc
  37:     0x7efec54f0eb9 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::he8103ccf5a5923bc
  38:     0x7efec54f0cf2 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green_and_read::h5067a5afaf756be4
  39:     0x7efec5486ce2 - rustc_query_system::query::plumbing::ensure_must_run::h334a6f03cc9b27a9
  40:     0x7efec54f5173 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::predicates_of::hdd387b0bd96159fb
  41:     0x7efec538080c - rustc_typeck::collect::convert_item::h3a8ddc32236e43e8
  42:     0x7efec537c788 - <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item::hc2eef9ecb205b621
  43:     0x7efec53644bd - rustc_middle::hir::map::Map::visit_item_likes_in_module::h9ffb890de3357425
  44:     0x7efec5e9b812 - rustc_typeck::collect::collect_mod_item_types::ha83d945a3be56349
  45:     0x7efec609808c - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::hb593b4d9fe3c3cf3
  46:     0x7efec54f03be - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hfb1fec0d6fa7ea05
  47:     0x7efec5495dbe - rustc_query_system::query::plumbing::force_query_with_job::hbdb06ac2597934e2
  48:     0x7efec545d614 - rustc_query_system::query::plumbing::get_query_impl::h328bf99f81d3ad62
  49:     0x7efec6083ccd - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_mod_item_types::h73924a9da5c73b0e
  50:     0x7efec5e0668e - rustc_session::session::Session::track_errors::h632dc2a904201ad0
  51:     0x7efec5e3ca1d - rustc_typeck::check_crate::heb848e97b33b6184
  52:     0x7efec5bdac8d - rustc_interface::passes::analysis::hdf0c41e7bd20242b
  53:     0x7efec4793e9a - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h79da03c51d5eb779
  54:     0x7efec60741a8 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::he1753e46538bf70f
  55:     0x7efec5fdf317 - rustc_data_structures::stack::ensure_sufficient_stack::h55b64aa8321942c2
  56:     0x7efec5f949b1 - rustc_query_system::query::plumbing::force_query_with_job::hec25eeb08c07b506
  57:     0x7efec5f69663 - rustc_query_system::query::plumbing::get_query_impl::hd4f6191670b007e2
  58:     0x7efec608201f - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis::ha4b253866e559d53
  59:     0x7efec5bb96fa - rustc_interface::passes::QueryContext::enter::h1243d4d1902c3141
  60:     0x7efec5b9e5b5 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hedca2acec6ab04da
  61:     0x7efec5b9d554 - rustc_span::with_source_map::h67cb68e15a9ffc39
  62:     0x7efec5b9f581 - rustc_interface::interface::create_compiler_and_run::he6a465670788f848
  63:     0x7efec5bb8d08 - scoped_tls::ScopedKey<T>::set::h8de4c07d6e313e7a
  64:     0x7efec5bb9223 - std::sys_common::backtrace::__rust_begin_short_backtrace::h953d4096b984a1d5
  65:     0x7efec5bbcb15 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hf2d8af4e3cccbbb1
  66:     0x7efec3900587 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3aa31cb6360b59d9
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/alloc/src/boxed.rs:1546:9
  67:     0x7efec3900587 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7719d3c7c5841461
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/alloc/src/boxed.rs:1546:9
  68:     0x7efec3900587 - std::sys::unix::thread::Thread::new::thread_start::hfbe13ead469fd0bc
                               at /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/library/std/src/sys/unix/thread.rs:71:17
  69:     0x7efec3829609 - start_thread
  70:     0x7efec373d293 - clone
  71:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (392ba2ba1 2021-04-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [predicates_of] computing predicates of `std::iter::IntoIterator`
#1 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
#2 [collect_mod_item_types] collecting item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `quill_parser`

To learn more, run the command again with --verbose.
