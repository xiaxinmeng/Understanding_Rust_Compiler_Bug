
ef➤  bt
#0  std::panicking::begin_panic_handler () at library/std/src/panicking.rs:493
#1  0x00005555557be471 in core::panicking::panic_fmt () at library/core/src/panicking.rs:92
#2  0x00005555557be432 in core::panicking::panic_bounds_check () at library/core/src/panicking.rs:69
#3  0x00005555564acb2e in core::slice::index::{{impl}}::index<salsa::interned::InternValue<hir_def::AssocItemLoc<hir_def::item_tree::Const>>> (self=0xf7c2b2df, slice=...) at /home/aaron/repos/rust/library/core/src/slice/index.rs:182
#4  0x000055555679a8a7 in core::slice::index::{{impl}}::index<salsa::interned::InternValue<hir_def::AssocItemLoc<hir_def::item_tree::Const>>,usize> (self=..., index=0xf7c2b2df) at /home/aaron/repos/rust/library/core/src/slice/index.rs:15
#5  0x00005555564ea29f in alloc::vec::{{impl}}::index<salsa::interned::InternValue<hir_def::AssocItemLoc<hir_def::item_tree::Const>>,usize,alloc::alloc::Global> (self=0x7ffff0001e48, index=0xf7c2b2df) at /home/aaron/repos/rust/library/alloc/src/vec/mod.rs:2285
#6  0x000055555679dd2f in salsa::interned::InternTables<hir_def::AssocItemLoc<hir_def::item_tree::Const>>::slot_for_index<hir_def::AssocItemLoc<hir_def::item_tree::Const>> (self=0x7ffff0001e28, index=..., revision_now=...) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/interned.rs:138
#7  0x00005555567a9a5b in salsa::interned::InternedStorage<hir_def::db::InternConstQuery>::lookup_value<hir_def::db::InternConstQuery> (self=0x7ffff0001e20, db=..., index=...) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/interned.rs:278
#8  0x0000555556799bf7 in salsa::interned::{{impl}}::try_fetch<hir_def::db::InternConstLookupQuery,hir_def::db::InternConstQuery> (self=0x7ffff0001e90, db=..., key=0x7ffff7c2909c) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/interned.rs:492
#9  0x0000555556657221 in salsa::QueryTable<hir_def::db::InternConstLookupQuery>::try_get<hir_def::db::InternConstLookupQuery> (self=0x7ffff7c29138, key=...) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/lib.rs:494
#10 0x0000555556653931 in salsa::QueryTable<hir_def::db::InternConstLookupQuery>::get<hir_def::db::InternConstLookupQuery> (self=0x7ffff7c29138, key=...) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/lib.rs:490
#11 0x000055555637345f in hir_def::db::{{impl}}::lookup_intern_const::__shim (db=..., key0=...) at crates/hir_def/src/db.rs:25
#12 0x0000555555982083 in hir_def::db::{{impl}}::lookup_intern_const<ide_db::RootDatabase> (self=0x7ffff7c2b2e0, key0=...) at /home/aaron/repos/rust-analyzer/crates/hir_def/src/db.rs:25
#13 0x00005555562fc0ee in hir_def::{{impl}}::lookup (self=0x7ffff7c2920c, db=...) at crates/hir_def/src/lib.rs:181
#14 0x0000555556370d5d in hir_def::data::ConstData::const_data_query (db=..., konst=...) at crates/hir_def/src/data.rs:179
#15 0x0000555556376737 in hir_def::db::{{impl}}::execute (db=..., key0=...) at crates/hir_def/src/db.rs:99
#16 0x00005555567ec8d0 in salsa::derived::slot::{{impl}}::read_upgrade::{{closure}}<hir_def::db::ConstDataQuery,salsa::derived::AlwaysMemoizeValue> () at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/derived/slot.rs:218
#17 0x00005555568adcde in salsa::runtime::Runtime::execute_query_implementation<DefDatabase,alloc::sync::Arc<hir_def::data::ConstData>,closure-0> (self=0x7ffff7c2b2e8, db=..., database_key_index=..., execute=...) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/runtime.rs:330
#18 0x00005555567e8661 in salsa::derived::slot::Slot<hir_def::db::ConstDataQuery, salsa::derived::AlwaysMemoizeValue>::read_upgrade<hir_def::db::ConstDataQuery,salsa::derived::AlwaysMemoizeValue> (self=0x7ffff00096c0, db=..., revision_now=...) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/derived/slot.rs:215
#19 0x000055555681ba18 in salsa::derived::slot::Slot<hir_def::db::ConstDataQuery, salsa::derived::AlwaysMemoizeValue>::read<hir_def::db::ConstDataQuery,salsa::derived::AlwaysMemoizeValue> (self=0x7ffff00096c0, db=...) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/derived/slot.rs:148
#20 0x0000555556452d67 in salsa::derived::{{impl}}::try_fetch<hir_def::db::ConstDataQuery,salsa::derived::AlwaysMemoizeValue> (self=0x7ffff0003020, db=..., key=0x7ffff7c2a90c) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/derived.rs:170
#21 0x0000555556656881 in salsa::QueryTable<hir_def::db::ConstDataQuery>::try_get<hir_def::db::ConstDataQuery> (self=0x7ffff7c2a9a8, key=...) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/lib.rs:494
#22 0x0000555556653e1d in salsa::QueryTable<hir_def::db::ConstDataQuery>::get<hir_def::db::ConstDataQuery> (self=0x7ffff7c2a9a8, key=...) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/lib.rs:490
#23 0x0000555556375f78 in hir_def::db::{{impl}}::const_data::__shim (db=..., key0=...) at crates/hir_def/src/db.rs:49
#24 0x000055555598155c in hir_def::db::{{impl}}::const_data<ide_db::RootDatabase> (self=0x7ffff7c2b2e0, key0=...) at /home/aaron/repos/rust-analyzer/crates/hir_def/src/db.rs:49
#25 0x0000555556004650 in salsa::plumbing::get_query_table<hir_ty::db::InferQueryQuery> (db=...) at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/salsa-0.16.0/src/plumbing.rs:95
#26 0x0000555555c9aad1 in hir_ty::db::{{impl}}::infer_query::__shim (db=..., key0=...) at crates/hir_ty/src/db.rs:20
#27 0x0000555555980bc2 in hir_ty::db::{{impl}}::infer_query<ide_db::RootDatabase> (self=0x7ffff7c2b2e0, key0=...) at /home/aaron/repos/rust-analyzer/crates/hir_ty/src/db.rs:20
#28 0x0000555555c9aa4c in hir_ty::db::infer_wait (db=..., def=...) at crates/hir_ty/src/db.rs:143
#29 0x00005555559814c2 in hir_ty::db::{{impl}}::infer<ide_db::RootDatabase> (self=0x7ffff7c2b2e0, key0=...) at /home/aaron/repos/rust-analyzer/crates/hir_ty/src/db.rs:20
#30 0x0000555555af0c5d in hir::source_analyzer::SourceAnalyzer::new_for_body (db=..., def=..., node=..., offset=...) at crates/hir/src/source_analyzer.rs:66
#31 0x0000555555b2a6d5 in hir::semantics::SemanticsImpl::analyze_impl (self=0x7ffff7c2b370, node=0x7ffff7c2b0d0, offset=...) at crates/hir/src/semantics.rs:578
#32 0x0000555555b2a3ac in hir::semantics::SemanticsImpl::analyze (self=0x7ffff7c2b370, node=0x7ffff7c2b0d0) at crates/hir/src/semantics.rs:561
#33 0x0000555555b28870 in hir::semantics::SemanticsImpl::descend_into_macros (self=0x7ffff7c2b370, token=...) at crates/hir/src/semantics.rs:341
#34 0x00005555558ec9ec in hir::semantics::Semantics<ide_db::RootDatabase>::descend_into_macros<ide_db::RootDatabase> (self=0x7ffff7c2b368, token=...) at /home/aaron/repos/rust-analyzer/crates/hir/src/semantics.rs:127
#35 0x00005555557f21cc in ide_completion::render::tests::sets_deprecated_flag_in_items () at crates/ide_completion/src/render.rs:543
#36 0x00005555558549bd in ide_completion::render::tests::sets_deprecated_flag_in_items::{{closure}} () at crates/ide_completion/src/render.rs:531
#37 0x0000555555828a8e in core::ops::function::FnOnce::call_once<closure-0,()> () at /home/aaron/repos/rust/library/core/src/ops/function.rs:227
#38 0x00005555558a1d96 in core::ops::function::FnOnce::call_once<fn(),()> () at /home/aaron/repos/rust/library/core/src/ops/function.rs:227
#39 test::__rust_begin_short_backtrace<fn()> () at library/test/src/lib.rs:567
#40 0x000055555589890e in alloc::boxed::{{impl}}::call_once<(),FnOnce<()>,alloc::alloc::Global> () at /home/aaron/repos/rust/library/alloc/src/boxed.rs:1546
#41 std::panic::{{impl}}::call_once<(),alloc::boxed::Box<FnOnce<()>, alloc::alloc::Global>> () at /home/aaron/repos/rust/library/std/src/panic.rs:344
#42 std::panicking::try::do_call<std::panic::AssertUnwindSafe<alloc::boxed::Box<FnOnce<()>, alloc::alloc::Global>>,()> () at /home/aaron/repos/rust/library/std/src/panicking.rs:379
#43 std::panicking::try<(),std::panic::AssertUnwindSafe<alloc::boxed::Box<FnOnce<()>, alloc::alloc::Global>>> () at /home/aaron/repos/rust/library/std/src/panicking.rs:343
#44 0x000055555588d772 in std::panic::catch_unwind<std::panic::AssertUnwindSafe<alloc::boxed::Box<FnOnce<()>, alloc::alloc::Global>>,()> () at /home/aaron/repos/rust/library/std/src/panic.rs:431
#45 test::run_test_in_process () at library/test/src/lib.rs:589
#46 0x0000555555892634 in test::run_test::run_test_inner::{{closure}} () at library/test/src/lib.rs:486
#47 test::run_test::run_test_inner::{{closure}} () at library/test/src/lib.rs:511
#48 std::sys_common::backtrace::__rust_begin_short_backtrace<closure-1,()> () at /home/aaron/repos/rust/library/std/src/sys_common/backtrace.rs:125
#49 0x0000555555898959 in std::thread::{{impl}}::spawn_unchecked::{{closure}}::{{closure}}<closure-1,()> () at /home/aaron/repos/rust/library/std/src/thread/mod.rs:474
#50 std::panic::{{impl}}::call_once<(),closure-0> () at /home/aaron/repos/rust/library/std/src/panic.rs:344
#51 std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure-0>,()> () at /home/aaron/repos/rust/library/std/src/panicking.rs:379
#52 std::panicking::try<(),std::panic::AssertUnwindSafe<closure-0>> () at /home/aaron/repos/rust/library/std/src/panicking.rs:343
#53 0x00005555558add1f in std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure-0>,()> () at /home/aaron/repos/rust/library/std/src/panic.rs:431
#54 std::thread::{{impl}}::spawn_unchecked::{{closure}}<closure-1,()> () at /home/aaron/repos/rust/library/std/src/thread/mod.rs:473
#55 core::ops::function::FnOnce::call_once<closure-0,()> () at /home/aaron/repos/rust/library/core/src/ops/function.rs:227
#56 0x0000555556d60a1b in alloc::boxed::{{impl}}::call_once<(),FnOnce<()>,alloc::alloc::Global> () at /home/aaron/repos/rust/library/alloc/src/boxed.rs:1546
#57 alloc::boxed::{{impl}}::call_once<(),alloc::boxed::Box<FnOnce<()>, alloc::alloc::Global>,alloc::alloc::Global> () at /home/aaron/repos/rust/library/alloc/src/boxed.rs:1546
#58 0x0000555556d7323a in std::sys::unix::thread::{{impl}}::new::thread_start () at library/std/src/sys/unix/thread.rs:71
#59 0x00007ffff7f54299 in start_thread () from /usr/lib/libpthread.so.0
#60 0x00007ffff7d31053 in clone () from /usr/lib/libc.so.6
