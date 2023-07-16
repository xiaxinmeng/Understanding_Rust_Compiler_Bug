plain

running 11553 tests
.................................................................................................... 100/11553
........................................i........ii................................................. 200/11553
...................................F....................F........................................... 300/11553
.................................................................................................... 500/11553
.................................................................................................... 600/11553
.................i............................................................................ii.... 700/11553
.................................................................................................... 800/11553
---
.................................................................................................... 3800/11553
.................................................................................................... 3900/11553
.................................................................................................... 4000/11553
.................................................................................................... 4100/11553
...............................F.........F.F...F...F..............................F.FF..F..F........ 4200/11553
........F........................................................................................... 4300/11553
.................................................................i.................................. 4500/11553
.................................................................................................... 4600/11553
.................................................................................................... 4700/11553
.................................................................................................... 4800/11553
---
.................................................................................................... 9300/11553
.................................................................................................... 9400/11553
.................................................................................i......i........... 9500/11553
.................................................................................................... 9600/11553
...........................iiiiiii..iiiiii.i........................................................ 9700/11553
.................................................................................................... 9900/11553
.................................................................................................... 10000/11553
.................................................................................................... 10100/11553
.................................................................................................... 10200/11553
---
failures:

---- [ui] ui/associated-type-bounds/duplicate.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/duplicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/duplicate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/duplicate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(impl_trait_in_bindings)] //~ WARN the feature `impl_trait_in_bindings` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information


thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(HirId { owner: DefId(0:108 ~ duplicate[317d]::lit1), local_id: 8 })`,
 right: `None`', compiler/rustc_middle/src/hir/map/collector.rs:253:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (943932dcf 2021-03-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:481:16
stack backtrace:
   0:     0x7f3ded2cad0f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h04b0513813036258
   1:     0x7f3ded341dbc - core::fmt::write::h49737a8d16636d2f
   2:     0x7f3ded2bdce5 - std::io::Write::write_fmt::h86abb269454b5813
   3:     0x7f3ded2cef42 - std::panicking::default_hook::{{closure}}::hee08368e4d643c82
   4:     0x7f3ded2ce9a5 - std::panicking::default_hook::h3769a51ff330fe0d
   5:     0x7f3dedb9afcd - rustc_driver::report_ice::hd8a3ee3d5a5d29aa
   6:     0x7f3ded2cf933 - std::panicking::rust_panic_with_hook::h7ef07342e0b50b09
   7:     0x7f3ded2cf3b7 - std::panicking::begin_panic_handler::{{closure}}::he29adb739e0ef6ba
   8:     0x7f3ded2cb1bc - std::sys_common::backtrace::__rust_end_short_backtrace::h7c454ee9dc08d417
   9:     0x7f3ded2cf319 - rust_begin_unwind
  10:     0x7f3ded33e091 - core::panicking::panic_fmt::h022cdf391b19da3f
  11:     0x7f3ded33dd03 - core::option::expect_none_failed::haadfd47035530d1a
  12:     0x7f3dee90c908 - rustc_query_system::query::plumbing::get_query_impl::hc180b7aa6d95cc02
  13:     0x7f3dee9ffb65 - rustc_query_system::query::plumbing::get_query::h8361c233700a1223
  14:     0x7f3df01e7571 - rustc_middle::hir::map::Map::find::h70a658da26161b78
  15:     0x7f3df01eace8 - rustc_middle::hir::map::Map::opt_span::h511a9117507e2ecd
  16:     0x7f3df01eb018 - rustc_middle::hir::map::Map::span_if_local::h6860811b8d0a48b5
  17:     0x7f3df01549ee - core::ops::function::FnOnce::call_once::h12f2425414bad2ec
  18:     0x7f3deec0bc6b - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h493df0a6a2e8c934
  19:     0x7f3deedcae31 - rustc_data_structures::stack::ensure_sufficient_stack::h78eed2ba79f99368
  20:     0x7f3dee98077d - rustc_query_system::query::plumbing::force_query_with_job::h806d5d6b03929a83
  21:     0x7f3dee8ecea6 - rustc_query_system::query::plumbing::get_query_impl::h86431913c36a4620
  22:     0x7f3deea11d6f - rustc_query_system::query::plumbing::get_query::he21116711871010a
  23:     0x7f3deeba20d4 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h25ad7cb690fab6dd
  24:     0x7f3deeba1fb7 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::hf3aee7487f1def3f
  25:     0x7f3deed3a7a2 - rustc_query_impl::make_query::hir_owner::h01cdb16762db99cf
  26:     0x7f3deea5abed - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h30e0f845ffbef49f
  27:     0x7f3deea2d5a1 - <hashbrown::map::HashMap<K,V,S> as core::iter::traits::collect::Extend<(K,V)>>::extend::h82a63ec80161653d
  28:     0x7f3dee9b608c - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h208fdbe356063833
  29:     0x7f3deed2779e - rustc_query_impl::Queries::try_collect_active_jobs::h102db0095b58c89f
  30:     0x7f3deeae2531 - rustc_query_system::query::job::print_query_stack::h63ae39855d03ca03
  31:     0x7f3dedcd2566 - rustc_interface::interface::try_print_query_stack::hfc796884182ec2d9
  32:     0x7f3dedb9b83a - rustc_driver::report_ice::hd8a3ee3d5a5d29aa
  33:     0x7f3ded2cf933 - std::panicking::rust_panic_with_hook::h7ef07342e0b50b09
  34:     0x7f3ded2cf3b7 - std::panicking::begin_panic_handler::{{closure}}::he29adb739e0ef6ba
  35:     0x7f3ded2cb1bc - std::sys_common::backtrace::__rust_end_short_backtrace::h7c454ee9dc08d417
  36:     0x7f3ded2cf319 - rust_begin_unwind
  37:     0x7f3ded33e091 - core::panicking::panic_fmt::h022cdf391b19da3f
  38:     0x7f3ded33e327 - core::panicking::assert_failed_inner::h0a9618eab19d1866
  39:     0x7f3df0127b7b - core::panicking::assert_failed::h6de5b22d9ba47401
  40:     0x7f3df00d2028 - rustc_middle::hir::map::collector::NodeCollector::insert_nested::hb01096ae601d0b54
  41:     0x7f3df00d2086 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hb560a0392a48c4d8
  42:     0x7f3df01e3454 - rustc_hir::intravisit::walk_ty::hbfd7e2d08ef2e6a1
  43:     0x7f3df00d391b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_ty::h03252e2e2be6c5dd
  44:     0x7f3df00d3acb - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_local::h0d5d9392ed8818d9
  45:     0x7f3df00d385b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_stmt::hc15183026e4572ec
  46:     0x7f3df01e0a1d - rustc_hir::intravisit::walk_block::h4f24a5170d23d22b
  47:     0x7f3df00d3a6b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_block::he73f0faf8fad17d5
  48:     0x7f3df00d37fb - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_expr::h615b2aa5b1c16e7e
  49:     0x7f3df00d39e8 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_fn::h36bc3f9ea3658581
  50:     0x7f3df01e53b7 - rustc_hir::intravisit::walk_item::h6c20ac08465c623e
  51:     0x7f3df00d29b0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::h2c863cf544b5a23a
  52:     0x7f3df00d20a0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hb560a0392a48c4d8
  53:     0x7f3df01ec2bb - rustc_middle::hir::map::index_hir::hfb1bc6e46cf48bdc
  54:     0x7f3deec15c76 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h61cdb4110ea9e392
  55:     0x7f3deedc3743 - rustc_data_structures::stack::ensure_sufficient_stack::h385b9831336d23ec
  56:     0x7f3dee98489c - rustc_query_system::query::plumbing::force_query_with_job::h9dca503f6b4e249f
  57:     0x7f3dee91500d - rustc_query_system::query::plumbing::get_query_impl::hd04a15da98712620
  58:     0x7f3deea0f851 - rustc_query_system::query::plumbing::get_query::hd861e13791cd5745
  59:     0x7f3df0155f73 - core::ops::function::FnOnce::call_once::hc28cfa375019fe41
  60:     0x7f3deec20fd6 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h80d3653663b55e82
  61:     0x7f3deedcad30 - rustc_data_structures::stack::ensure_sufficient_stack::h78eadc4091105c47
  62:     0x7f3dee98ab4c - rustc_query_system::query::plumbing::force_query_with_job::hc80f80f0695330e6
  63:     0x7f3dee90c69d - rustc_query_system::query::plumbing::get_query_impl::hc180b7aa6d95cc02
  64:     0x7f3dee9ffb65 - rustc_query_system::query::plumbing::get_query::h8361c233700a1223
  65:     0x7f3df01e7571 - rustc_middle::hir::map::Map::find::h70a658da26161b78
  66:     0x7f3df01e7b1c - <rustc_middle::hir::map::Map as rustc_hir::intravisit::Map>::item::hf7b611d2f93ac53b
  67:     0x7f3deeea4ca8 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h46b9b86e8354ce4d
  68:     0x7f3deeeb9708 - rustc_passes::hir_id_validator::check_crate::h04cc2cad9a1b14cb
  69:     0x7f3dedceb911 - rustc_interface::passes::analysis::hc8c7237f1303a31b
  70:     0x7f3deec2c2c6 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h99c2c6801a185a63
  71:     0x7f3deec5eae9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h062950a4429fc35e
  72:     0x7f3deedc5ef2 - rustc_data_structures::stack::ensure_sufficient_stack::h4a75b70e61f492c2
  73:     0x7f3dee976077 - rustc_query_system::query::plumbing::force_query_with_job::h49fe36ea94646291
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  74:     0x7f3dee8c85ed - rustc_query_system::query::plumbing::get_query_impl::h44cf89f25e980ca3
  75:     0x7f3deea0a503 - rustc_query_system::query::plumbing::get_query::hbcf31c55d03cc888
  76:     0x7f3dedc19f8a - rustc_interface::passes::QueryContext::enter::hb8c73b4c4992abee
  77:     0x7f3dedbb5390 - std::sys_common::backtrace::__rust_begin_short_backtrace::hba6016a8d8d41d98
  78:     0x7f3dedbb6f46 - std::panicking::try::h65e818210fe68b07
  79:     0x7f3dedc0ab2a - core::ops::function::FnOnce::call_once{{vtable.shim}}::he46c1ffb80480db8
  80:     0x7f3ded2e0ed8 - std::sys::unix::thread::Thread::new::thread_start::ha79ef4f882a2473c
  81:     0x7f3de841e6db - start_thread
  82:     0x7f3decf5e71f - __clone
  83:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (943932dcf 2021-03-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
------------------------------------------


---- [ui] ui/associated-type-bounds/lcsit.rs stdout ----
---- [ui] ui/associated-type-bounds/lcsit.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/lcsit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/lcsit/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/lcsit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(impl_trait_in_bindings)]
   |            ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(HirId { owner: DefId(0:22 ~ lcsit[317d]::cdef_et1), local_id: 7 })`,
 right: `None`', compiler/rustc_middle/src/hir/map/collector.rs:253:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (943932dcf 2021-03-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:481:16
stack backtrace:
   0:     0x7f1980c5bd0f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h04b0513813036258
   1:     0x7f1980cd2dbc - core::fmt::write::h49737a8d16636d2f
   2:     0x7f1980c4ece5 - std::io::Write::write_fmt::h86abb269454b5813
   3:     0x7f1980c5ff42 - std::panicking::default_hook::{{closure}}::hee08368e4d643c82
   4:     0x7f1980c5f9a5 - std::panicking::default_hook::h3769a51ff330fe0d
   5:     0x7f198152bfcd - rustc_driver::report_ice::hd8a3ee3d5a5d29aa
   6:     0x7f1980c60933 - std::panicking::rust_panic_with_hook::h7ef07342e0b50b09
   7:     0x7f1980c603b7 - std::panicking::begin_panic_handler::{{closure}}::he29adb739e0ef6ba
   8:     0x7f1980c5c1bc - std::sys_common::backtrace::__rust_end_short_backtrace::h7c454ee9dc08d417
   9:     0x7f1980c60319 - rust_begin_unwind
  10:     0x7f1980ccf091 - core::panicking::panic_fmt::h022cdf391b19da3f
  11:     0x7f1980cced03 - core::option::expect_none_failed::haadfd47035530d1a
  12:     0x7f198229d908 - rustc_query_system::query::plumbing::get_query_impl::hc180b7aa6d95cc02
  13:     0x7f1982390b65 - rustc_query_system::query::plumbing::get_query::h8361c233700a1223
  14:     0x7f1983b78571 - rustc_middle::hir::map::Map::find::h70a658da26161b78
  15:     0x7f1983b7bce8 - rustc_middle::hir::map::Map::opt_span::h511a9117507e2ecd
  16:     0x7f1983b7c018 - rustc_middle::hir::map::Map::span_if_local::h6860811b8d0a48b5
  17:     0x7f1983ae59ee - core::ops::function::FnOnce::call_once::h12f2425414bad2ec
  18:     0x7f198259cc6b - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h493df0a6a2e8c934
  19:     0x7f198275be31 - rustc_data_structures::stack::ensure_sufficient_stack::h78eed2ba79f99368
  20:     0x7f198231177d - rustc_query_system::query::plumbing::force_query_with_job::h806d5d6b03929a83
  21:     0x7f198227dea6 - rustc_query_system::query::plumbing::get_query_impl::h86431913c36a4620
  22:     0x7f19823a2d6f - rustc_query_system::query::plumbing::get_query::he21116711871010a
  23:     0x7f19825330d4 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h25ad7cb690fab6dd
  24:     0x7f1982532fb7 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::hf3aee7487f1def3f
  25:     0x7f19826cb7a2 - rustc_query_impl::make_query::hir_owner::h01cdb16762db99cf
  26:     0x7f19823ebbed - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h30e0f845ffbef49f
  27:     0x7f19823be5a1 - <hashbrown::map::HashMap<K,V,S> as core::iter::traits::collect::Extend<(K,V)>>::extend::h82a63ec80161653d
  28:     0x7f198234708c - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h208fdbe356063833
  29:     0x7f19826b879e - rustc_query_impl::Queries::try_collect_active_jobs::h102db0095b58c89f
  30:     0x7f1982473531 - rustc_query_system::query::job::print_query_stack::h63ae39855d03ca03
  31:     0x7f1981663566 - rustc_interface::interface::try_print_query_stack::hfc796884182ec2d9
  32:     0x7f198152c83a - rustc_driver::report_ice::hd8a3ee3d5a5d29aa
  33:     0x7f1980c60933 - std::panicking::rust_panic_with_hook::h7ef07342e0b50b09
  34:     0x7f1980c603b7 - std::panicking::begin_panic_handler::{{closure}}::he29adb739e0ef6ba
  35:     0x7f1980c5c1bc - std::sys_common::backtrace::__rust_end_short_backtrace::h7c454ee9dc08d417
  36:     0x7f1980c60319 - rust_begin_unwind
  37:     0x7f1980ccf091 - core::panicking::panic_fmt::h022cdf391b19da3f
  38:     0x7f1980ccf327 - core::panicking::assert_failed_inner::h0a9618eab19d1866
  39:     0x7f1983ab8b7b - core::panicking::assert_failed::h6de5b22d9ba47401
  40:     0x7f1983a63028 - rustc_middle::hir::map::collector::NodeCollector::insert_nested::hb01096ae601d0b54
  41:     0x7f1983a63086 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hb560a0392a48c4d8
  42:     0x7f1983b74454 - rustc_hir::intravisit::walk_ty::hbfd7e2d08ef2e6a1
  43:     0x7f1983a6491b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_ty::h03252e2e2be6c5dd
  44:     0x7f1983a64acb - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_local::h0d5d9392ed8818d9
  45:     0x7f1983a6485b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_stmt::hc15183026e4572ec
  46:     0x7f1983b71a1d - rustc_hir::intravisit::walk_block::h4f24a5170d23d22b
  47:     0x7f1983a64a6b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_block::he73f0faf8fad17d5
  48:     0x7f1983a647fb - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_expr::h615b2aa5b1c16e7e
  49:     0x7f1983a639b0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::h2c863cf544b5a23a
  50:     0x7f1983a630a0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hb560a0392a48c4d8
  51:     0x7f1983b7d2bb - rustc_middle::hir::map::index_hir::hfb1bc6e46cf48bdc
  52:     0x7f19825a6c76 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h61cdb4110ea9e392
  53:     0x7f1982754743 - rustc_data_structures::stack::ensure_sufficient_stack::h385b9831336d23ec
  54:     0x7f198231589c - rustc_query_system::query::plumbing::force_query_with_job::h9dca503f6b4e249f
  55:     0x7f19822a600d - rustc_query_system::query::plumbing::get_query_impl::hd04a15da98712620
  56:     0x7f19823a0851 - rustc_query_system::query::plumbing::get_query::hd861e13791cd5745
  57:     0x7f1983ae6f73 - core::ops::function::FnOnce::call_once::hc28cfa375019fe41
  58:     0x7f19825b1fd6 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h80d3653663b55e82
  59:     0x7f198275bd30 - rustc_data_structures::stack::ensure_sufficient_stack::h78eadc4091105c47
  60:     0x7f198231bb4c - rustc_query_system::query::plumbing::force_query_with_job::hc80f80f0695330e6
  61:     0x7f198229d69d - rustc_query_system::query::plumbing::get_query_impl::hc180b7aa6d95cc02
  62:     0x7f1982390b65 - rustc_query_system::query::plumbing::get_query::h8361c233700a1223
  63:     0x7f1983b78571 - rustc_middle::hir::map::Map::find::h70a658da26161b78
  64:     0x7f1983b78b1c - <rustc_middle::hir::map::Map as rustc_hir::intravisit::Map>::item::hf7b611d2f93ac53b
  65:     0x7f1982835ca8 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h46b9b86e8354ce4d
  66:     0x7f198284a708 - rustc_passes::hir_id_validator::check_crate::h04cc2cad9a1b14cb
  67:     0x7f198167c911 - rustc_interface::passes::analysis::hc8c7237f1303a31b
  68:     0x7f19825bd2c6 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h99c2c6801a185a63
  69:     0x7f19825efae9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h062950a4429fc35e
  70:     0x7f1982756ef2 - rustc_data_structures::stack::ensure_sufficient_stack::h4a75b70e61f492c2
  71:     0x7f1982307077 - rustc_query_system::query::plumbing::force_query_with_job::h49fe36ea94646291
  72:     0x7f19822595ed - rustc_query_system::query::plumbing::get_query_impl::h44cf89f25e980ca3
  73:     0x7f198239b503 - rustc_query_system::query::plumbing::get_query::hbcf31c55d03cc888
  74:     0x7f19815aaf8a - rustc_interface::passes::QueryContext::enter::hb8c73b4c4992abee
  75:     0x7f1981546390 - std::sys_common::backtrace::__rust_begin_short_backtrace::hba6016a8d8d41d98
  76:     0x7f1981547f46 - std::panicking::try::h65e818210fe68b07
  77:     0x7f198159bb2a - core::ops::function::FnOnce::call_once{{vtable.shim}}::he46c1ffb80480db8
  78:     0x7f1980c71ed8 - std::sys::unix::thread::Thread::new::thread_start::ha79ef4f882a2473c
  79:     0x7f197bdaf6db - start_thread
  80:     0x7f19808ef71f - __clone
  81:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (943932dcf 2021-03-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
------------------------------------------


---- [ui] ui/impl-trait-in-bindings.rs stdout ----
---- [ui] ui/impl-trait-in-bindings.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait-in-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait-in-bindings/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait-in-bindings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(impl_trait_in_bindings)]
   |            ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(HirId { owner: DefId(0:10 ~ impl_trait_in_bindings[317d]::a), local_id: 8 })`,
 right: `None`', compiler/rustc_middle/src/hir/map/collector.rs:253:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (943932dcf 2021-03-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:481:16
stack backtrace:
   0:     0x7fcd1a828d0f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h04b0513813036258
   1:     0x7fcd1a89fdbc - core::fmt::write::h49737a8d16636d2f
   2:     0x7fcd1a81bce5 - std::io::Write::write_fmt::h86abb269454b5813
   3:     0x7fcd1a82cf42 - std::panicking::default_hook::{{closure}}::hee08368e4d643c82
   4:     0x7fcd1a82c9a5 - std::panicking::default_hook::h3769a51ff330fe0d
   5:     0x7fcd1b0f8fcd - rustc_driver::report_ice::hd8a3ee3d5a5d29aa
   6:     0x7fcd1a82d933 - std::panicking::rust_panic_with_hook::h7ef07342e0b50b09
   7:     0x7fcd1a82d3b7 - std::panicking::begin_panic_handler::{{closure}}::he29adb739e0ef6ba
   8:     0x7fcd1a8291bc - std::sys_common::backtrace::__rust_end_short_backtrace::h7c454ee9dc08d417
   9:     0x7fcd1a82d319 - rust_begin_unwind
  10:     0x7fcd1a89c091 - core::panicking::panic_fmt::h022cdf391b19da3f
  11:     0x7fcd1a89bd03 - core::option::expect_none_failed::haadfd47035530d1a
  12:     0x7fcd1be6a908 - rustc_query_system::query::plumbing::get_query_impl::hc180b7aa6d95cc02
  13:     0x7fcd1bf5db65 - rustc_query_system::query::plumbing::get_query::h8361c233700a1223
  14:     0x7fcd1d745571 - rustc_middle::hir::map::Map::find::h70a658da26161b78
  15:     0x7fcd1d748ce8 - rustc_middle::hir::map::Map::opt_span::h511a9117507e2ecd
  16:     0x7fcd1d749018 - rustc_middle::hir::map::Map::span_if_local::h6860811b8d0a48b5
  17:     0x7fcd1d6b29ee - core::ops::function::FnOnce::call_once::h12f2425414bad2ec
  18:     0x7fcd1c169c6b - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h493df0a6a2e8c934
  19:     0x7fcd1c328e31 - rustc_data_structures::stack::ensure_sufficient_stack::h78eed2ba79f99368
  20:     0x7fcd1bede77d - rustc_query_system::query::plumbing::force_query_with_job::h806d5d6b03929a83
  21:     0x7fcd1be4aea6 - rustc_query_system::query::plumbing::get_query_impl::h86431913c36a4620
  22:     0x7fcd1bf6fd6f - rustc_query_system::query::plumbing::get_query::he21116711871010a
  23:     0x7fcd1c1000d4 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h25ad7cb690fab6dd
  24:     0x7fcd1c0fffb7 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::hf3aee7487f1def3f
  25:     0x7fcd1c2987a2 - rustc_query_impl::make_query::hir_owner::h01cdb16762db99cf
  26:     0x7fcd1bfb8bed - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h30e0f845ffbef49f
  27:     0x7fcd1bf8b5a1 - <hashbrown::map::HashMap<K,V,S> as core::iter::traits::collect::Extend<(K,V)>>::extend::h82a63ec80161653d
  28:     0x7fcd1bf1408c - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h208fdbe356063833
  29:     0x7fcd1c28579e - rustc_query_impl::Queries::try_collect_active_jobs::h102db0095b58c89f
  30:     0x7fcd1c040531 - rustc_query_system::query::job::print_query_stack::h63ae39855d03ca03
  31:     0x7fcd1b230566 - rustc_interface::interface::try_print_query_stack::hfc796884182ec2d9
  32:     0x7fcd1b0f983a - rustc_driver::report_ice::hd8a3ee3d5a5d29aa
  33:     0x7fcd1a82d933 - std::panicking::rust_panic_with_hook::h7ef07342e0b50b09
  34:     0x7fcd1a82d3b7 - std::panicking::begin_panic_handler::{{closure}}::he29adb739e0ef6ba
  35:     0x7fcd1a8291bc - std::sys_common::backtrace::__rust_end_short_backtrace::h7c454ee9dc08d417
  36:     0x7fcd1a82d319 - rust_begin_unwind
  37:     0x7fcd1a89c091 - core::panicking::panic_fmt::h022cdf391b19da3f
  38:     0x7fcd1a89c327 - core::panicking::assert_failed_inner::h0a9618eab19d1866
  39:     0x7fcd1d685b7b - core::panicking::assert_failed::h6de5b22d9ba47401
  40:     0x7fcd1d630028 - rustc_middle::hir::map::collector::NodeCollector::insert_nested::hb01096ae601d0b54
  41:     0x7fcd1d630086 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hb560a0392a48c4d8
  42:     0x7fcd1d741454 - rustc_hir::intravisit::walk_ty::hbfd7e2d08ef2e6a1
  43:     0x7fcd1d63191b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_ty::h03252e2e2be6c5dd
  44:     0x7fcd1d631acb - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_local::h0d5d9392ed8818d9
  45:     0x7fcd1d63185b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_stmt::hc15183026e4572ec
  46:     0x7fcd1d73ea1d - rustc_hir::intravisit::walk_block::h4f24a5170d23d22b
  47:     0x7fcd1d631a6b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_block::he73f0faf8fad17d5
  48:     0x7fcd1d6317fb - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_expr::h615b2aa5b1c16e7e
  49:     0x7fcd1d6319e8 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_fn::h36bc3f9ea3658581
  50:     0x7fcd1d7433b7 - rustc_hir::intravisit::walk_item::h6c20ac08465c623e
  51:     0x7fcd1d6309b0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::h2c863cf544b5a23a
  52:     0x7fcd1d6300a0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hb560a0392a48c4d8
  53:     0x7fcd1d74a2bb - rustc_middle::hir::map::index_hir::hfb1bc6e46cf48bdc
  54:     0x7fcd1c173c76 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h61cdb4110ea9e392
  55:     0x7fcd1c321743 - rustc_data_structures::stack::ensure_sufficient_stack::h385b9831336d23ec
  56:     0x7fcd1bee289c - rustc_query_system::query::plumbing::force_query_with_job::h9dca503f6b4e249f
  57:     0x7fcd1be7300d - rustc_query_system::query::plumbing::get_query_impl::hd04a15da98712620
  58:     0x7fcd1bf6d851 - rustc_query_system::query::plumbing::get_query::hd861e13791cd5745
  59:     0x7fcd1d6b3f73 - core::ops::function::FnOnce::call_once::hc28cfa375019fe41
  60:     0x7fcd1c17efd6 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h80d3653663b55e82
  61:     0x7fcd1c328d30 - rustc_data_structures::stack::ensure_sufficient_stack::h78eadc4091105c47
  62:     0x7fcd1bee8b4c - rustc_query_system::query::plumbing::force_query_with_job::hc80f80f0695330e6
  63:     0x7fcd1be6a69d - rustc_query_system::query::plumbing::get_query_impl::hc180b7aa6d95cc02
  64:     0x7fcd1bf5db65 - rustc_query_system::query::plumbing::get_query::h8361c233700a1223
  65:     0x7fcd1d745571 - rustc_middle::hir::map::Map::find::h70a658da26161b78
  66:     0x7fcd1d745b1c - <rustc_middle::hir::map::Map as rustc_hir::intravisit::Map>::item::hf7b611d2f93ac53b
  67:     0x7fcd1c402ca8 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h46b9b86e8354ce4d
  68:     0x7fcd1c417708 - rustc_passes::hir_id_validator::check_crate::h04cc2cad9a1b14cb
  69:     0x7fcd1b249911 - rustc_interface::passes::analysis::hc8c7237f1303a31b
  70:     0x7fcd1c18a2c6 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h99c2c6801a185a63
  71:     0x7fcd1c1bcae9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h062950a4429fc35e
  72:     0x7fcd1c323ef2 - rustc_data_structures::stack::ensure_sufficient_stack::h4a75b70e61f492c2
  73:     0x7fcd1bed4077 - rustc_query_system::query::plumbing::force_query_with_job::h49fe36ea94646291
  74:     0x7fcd1be265ed - rustc_query_system::query::plumbing::get_query_impl::h44cf89f25e980ca3
  75:     0x7fcd1bf68503 - rustc_query_system::query::plumbing::get_query::hbcf31c55d03cc888
  76:     0x7fcd1b177f8a - rustc_interface::passes::QueryContext::enter::hb8c73b4c4992abee
  77:     0x7fcd1b113390 - std::sys_common::backtrace::__rust_begin_short_backtrace::hba6016a8d8d41d98
  78:     0x7fcd1b114f46 - std::panicking::try::h65e818210fe68b07
  79:     0x7fcd1b168b2a - core::ops::function::FnOnce::call_once{{vtable.shim}}::he46c1ffb80480db8
  80:     0x7fcd1a83eed8 - std::sys::unix::thread::Thread::new::thread_start::ha79ef4f882a2473c
  81:     0x7fcd1597c6db - start_thread
  82:     0x7fcd1a4bc71f - __clone
  83:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (943932dcf 2021-03-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
------------------------------------------


---- [ui] ui/impl-trait/binding-without-value.rs stdout ----
---- [ui] ui/impl-trait/binding-without-value.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/binding-without-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/binding-without-value" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/binding-without-value/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(HirId { owner: DefId(0:3 ~ binding_without_value[317d]::foo), local_id: 4 })`,
 right: `None`', compiler/rustc_middle/src/hir/map/collector.rs:253:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (943932dcf 2021-03-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:481:16
stack backtrace:
   0:     0x7f05a9138d0f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h04b0513813036258
   1:     0x7f05a91afdbc - core::fmt::write::h49737a8d16636d2f
   2:     0x7f05a912bce5 - std::io::Write::write_fmt::h86abb269454b5813
   3:     0x7f05a913cf42 - std::panicking::default_hook::{{closure}}::hee08368e4d643c82
   4:     0x7f05a913c9a5 - std::panicking::default_hook::h3769a51ff330fe0d
   5:     0x7f05a9a08fcd - rustc_driver::report_ice::hd8a3ee3d5a5d29aa
   6:     0x7f05a913d933 - std::panicking::rust_panic_with_hook::h7ef07342e0b50b09
   7:     0x7f05a913d3b7 - std::panicking::begin_panic_handler::{{closure}}::he29adb739e0ef6ba
   8:     0x7f05a91391bc - std::sys_common::backtrace::__rust_end_short_backtrace::h7c454ee9dc08d417
   9:     0x7f05a913d319 - rust_begin_unwind
  10:     0x7f05a91ac091 - core::panicking::panic_fmt::h022cdf391b19da3f
  11:     0x7f05a91abd03 - core::option::expect_none_failed::haadfd47035530d1a
  12:     0x7f05aa77a908 - rustc_query_system::query::plumbing::get_query_impl::hc180b7aa6d95cc02
  13:     0x7f05aa86db65 - rustc_query_system::query::plumbing::get_query::h8361c233700a1223
  14:     0x7f05ac055571 - rustc_middle::hir::map::Map::find::h70a658da26161b78
  15:     0x7f05ac058ce8 - rustc_middle::hir::map::Map::opt_span::h511a9117507e2ecd
  16:     0x7f05ac059018 - rustc_middle::hir::map::Map::span_if_local::h6860811b8d0a48b5
  17:     0x7f05abfc29ee - core::ops::function::FnOnce::call_once::h12f2425414bad2ec
  18:     0x7f05aaa79c6b - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h493df0a6a2e8c934
  19:     0x7f05aac38e31 - rustc_data_structures::stack::ensure_sufficient_stack::h78eed2ba79f99368
  20:     0x7f05aa7ee77d - rustc_query_system::query::plumbing::force_query_with_job::h806d5d6b03929a83
  21:     0x7f05aa75aea6 - rustc_query_system::query::plumbing::get_query_impl::h86431913c36a4620
  22:     0x7f05aa87fd6f - rustc_query_system::query::plumbing::get_query::he21116711871010a
  23:     0x7f05aaa100d4 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h25ad7cb690fab6dd
  24:     0x7f05aaa0ffb7 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::hf3aee7487f1def3f
  25:     0x7f05aaba87a2 - rustc_query_impl::make_query::hir_owner::h01cdb16762db99cf
  26:     0x7f05aa8c8bed - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h30e0f845ffbef49f
  27:     0x7f05aa89b5a1 - <hashbrown::map::HashMap<K,V,S> as core::iter::traits::collect::Extend<(K,V)>>::extend::h82a63ec80161653d
  28:     0x7f05aa82408c - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h208fdbe356063833
  29:     0x7f05aab9579e - rustc_query_impl::Queries::try_collect_active_jobs::h102db0095b58c89f
  30:     0x7f05aa950531 - rustc_query_system::query::job::print_query_stack::h63ae39855d03ca03
  31:     0x7f05a9b40566 - rustc_interface::interface::try_print_query_stack::hfc796884182ec2d9
  32:     0x7f05a9a0983a - rustc_driver::report_ice::hd8a3ee3d5a5d29aa
  33:     0x7f05a913d933 - std::panicking::rust_panic_with_hook::h7ef07342e0b50b09
  34:     0x7f05a913d3b7 - std::panicking::begin_panic_handler::{{closure}}::he29adb739e0ef6ba
  35:     0x7f05a91391bc - std::sys_common::backtrace::__rust_end_short_backtrace::h7c454ee9dc08d417
  36:     0x7f05a913d319 - rust_begin_unwind
  37:     0x7f05a91ac091 - core::panicking::panic_fmt::h022cdf391b19da3f
  38:     0x7f05a91ac327 - core::panicking::assert_failed_inner::h0a9618eab19d1866
  39:     0x7f05abf95b7b - core::panicking::assert_failed::h6de5b22d9ba47401
  40:     0x7f05abf40028 - rustc_middle::hir::map::collector::NodeCollector::insert_nested::hb01096ae601d0b54
  41:     0x7f05abf40086 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hb560a0392a48c4d8
  42:     0x7f05ac051454 - rustc_hir::intravisit::walk_ty::hbfd7e2d08ef2e6a1
  43:     0x7f05abf4191b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_ty::h03252e2e2be6c5dd
  44:     0x7f05abf41acb - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_local::h0d5d9392ed8818d9
  45:     0x7f05abf4185b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_stmt::hc15183026e4572ec
  46:     0x7f05ac04ea1d - rustc_hir::intravisit::walk_block::h4f24a5170d23d22b
  47:     0x7f05abf41a6b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_block::he73f0faf8fad17d5
  48:     0x7f05abf417fb - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_expr::h615b2aa5b1c16e7e
  49:     0x7f05abf419e8 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_fn::h36bc3f9ea3658581
---
test result: FAILED. 11447 passed; 13 failed; 93 ignored; 0 measured; 0 filtered out; finished in 131.36s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:48
