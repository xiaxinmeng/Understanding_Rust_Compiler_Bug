plain
iiiiiiii.....................i..................i....................................... 264/14093
...........................................................................F............ 352/14093
........................................................................................ 440/14093
........................................................................................ 528/14093
F...F.............F..........F.F........................................................ 616/14093
........................................................................................ 792/14093
.......................................................i................................ 880/14093
...................................................................i.................... 968/14093
........................................................................................ 1056/14093
---
.....................................................................................F.. 2904/14093
........................................................................................ 2992/14093
.........................................................................F.............. 3080/14093
........................................................................................ 3168/14093
.......................i.........................F.......F.............F......i......... 3256/14093
........................................................................................ 3432/14093
...............................................................iiiii.................... 3520/14093
........................................................................................ 3608/14093
........................................................................................ 3696/14093
---
................................................i....................................... 6864/14093
........................................................................................ 6952/14093
..............i.........................................................i.iii........i.. 7040/14093
..i......................................................................i.............. 7128/14093
...F....................F............................................................... 7216/14093
.................i..................i.............i..................................... 7392/14093
...........................i............................................................ 7480/14093
.................................................i...................................... 7568/14093
........................................................................................ 7656/14093
........................................................................................ 7656/14093
........................................................................................ 7744/14093
.......................................................ii............................... 7832/14093
..............ii.................................................................i...... 7920/14093
........................................................................................ 8008/14093
........................................................................................ 8096/14093
........ii.............................................................................. 8184/14093
.............................................................F.F....F..F..FFF........... 8272/14093
.......................................ii................i......i..ii................... 8448/14093
........................................................................................ 8536/14093
........................................................................................ 8624/14093
........................................................................................ 8712/14093
---
...............................................................FF....................... 10208/14093
........................................................................................ 10296/14093
..............................................ii...............i...iii.................. 10384/14093
........................................................................................ 10472/14093
.........................F.............................................F..........F..... 10560/14093
.................F...................................................................... 10736/14093
......................................................................F................. 10824/14093
........................................................................................ 10912/14093
...........................................................................F............ 11000/14093
...........................................................................F............ 11000/14093
........................................................................................ 11088/14093
.........................iiiii...i....i.i............................................... 11176/14093
........................................................................................ 11264/14093
............................i........................................................... 11352/14093
......................................iiiiii.i..iiiiiiiiiii.i........................... 11440/14093
........................................................................................ 11528/14093
........................................................................................ 11616/14093
F..F....F.F.....F....F.........................................F........................ 11704/14093
..............................................................................F..F...... 11792/14093
........................................................................................ 11968/14093
........................................................................................ 12056/14093
........................................................................................ 12144/14093
........................................................................................ 12232/14093
---
failures:

---- [ui] src/test/ui/associated-consts/defaults-cyclic-fail.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 11 (SIGSEGV) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/defaults-cyclic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-cyclic-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-cyclic-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when const-evaluating + checking `Tr::A`
   |
   |
LL |     const A: u8 = Self::B;
   |
   |
note: ...which requires const-evaluating + checking `Tr::B`...
   |
   |
LL |     const B: u8 = Self::A;
   |                   ^^^^^^^
   = note: ...which again requires const-evaluating + checking `Tr::A`, completing the cycle
note: cycle used when const-evaluating + checking `main::promoted[1]`
   |
   |
LL |     assert_eq!(<() as Tr>::A, 0);


thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
   0:     0x7f0d48a3d175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   0:     0x7f0d48a3d175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f0d48aad1e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f0d48a2f291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f0d48a3cf81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f0d48a40364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f0d48a4002a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f0d4948b6c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f0d48a40ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f0d48a407fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f0d48a3d694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f0d48a404e2 - rust_begin_unwind
  11:     0x7f0d489f0fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f0d489f109d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7f0d4b0c2fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7f0d4b278ce5 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<rustc_span[755eca102fd61de6]::def_id::DefId, bool>>
  15:     0x7f0d4b3591ec - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::is_foreign_item, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7f0d4aeaa955 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::is_foreign_item
  17:     0x7f0d49e58aaa - rustc_monomorphize[d88e042efd1bf97]::collector::should_codegen_locally
  18:     0x7f0d49e5d291 - rustc_monomorphize[d88e042efd1bf97]::collector::collect_items_rec
  19:     0x7f0d49e74af4 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<alloc[75b9de5db4717345]::vec::Vec<rustc_middle[cc0ab085cda1518a]::mir::mono::MonoItem>, rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  20:     0x7f0d49e9e7d4 - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items::{closure#1}>
  21:     0x7f0d49e5a376 - rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items
  22:     0x7f0d49e6cd76 - rustc_monomorphize[d88e042efd1bf97]::partitioning::collect_and_partition_mono_items
  23:     0x7f0d4b28d16a - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), (&std[5ac0ced0274dd2d1]::collections::hash::set::HashSet<rustc_span[755eca102fd61de6]::def_id::DefId, core[e5930317b0d12ce1]::hash::BuildHasherDefault<rustc_hash[ff8b1808a785fb13]::FxHasher>>, &[rustc_middle[cc0ab085cda1518a]::mir::mono::CodegenUnit])>>
  24:     0x7f0d4b3a3ac9 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::collect_and_partition_mono_items, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  25:     0x7f0d4aefe998 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::collect_and_partition_mono_items
  26:     0x7f0d4975a021 - rustc_codegen_ssa[efa17c4ad961d364]::base::codegen_crate::<rustc_codegen_llvm[3de13d325ebb1d9f]::LlvmCodegenBackend>
  27:     0x7f0d4982c557 - <rustc_codegen_llvm[3de13d325ebb1d9f]::LlvmCodegenBackend as rustc_codegen_ssa[efa17c4ad961d364]::traits::backend::CodegenBackend>::codegen_crate
  28:     0x7f0d495da98f - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<alloc[75b9de5db4717345]::boxed::Box<dyn core[e5930317b0d12ce1]::any::Any>, rustc_interface[9ae183de8b2774b4]::passes::start_codegen::{closure#0}>
  29:     0x7f0d495da3f8 - rustc_interface[9ae183de8b2774b4]::passes::start_codegen
  30:     0x7f0d495d83a6 - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<<rustc_interface[9ae183de8b2774b4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<alloc[75b9de5db4717345]::boxed::Box<dyn core[e5930317b0d12ce1]::any::Any>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  31:     0x7f0d49650225 - <rustc_interface[9ae183de8b2774b4]::queries::Queries>::ongoing_codegen
  32:     0x7f0d494f52c3 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  33:     0x7f0d4948ce6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  34:     0x7f0d494e80ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  35:     0x7f0d494aa989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  36:     0x7f0d494e9806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  37:     0x7f0d4949cfa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f0d48a4d8ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  39:     0x7f0d487e2b43 - <unknown>
  40:     0x7f0d48874a00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (c4cd43694 2022-12-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x94bce3)[0x7f0d49494ce3]
/lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7f0d48790520]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvNtCsdiqvhTzVGrU_15rustc_interface9interface21try_print_query_stack+0x6f)[0x7f0d495a816f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsl0LieaIGlMN_12rustc_driver10report_ice+0xd55)[0x7f0d4948c4b5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN3std9panicking20rust_panic_with_hook17hab0b5a9b7152bbecE+0x274)[0x7f0d48a40ad4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc57fa)[0x7f0d48a407fa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc2694)[0x7f0d48a3d694]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_begin_unwind+0x72)[0x7f0d48a404e2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking9panic_fmt17h775c62c34b432a2dE+0x33)[0x7f0d489f0fc3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking5panic17hbb25aa978ab51c1fE+0x4d)[0x7f0d489f109d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvXs0_NtCslEjRGzJr5QU_16rustc_query_impl8plumbingNtB5_9QueryCtxtNtNtCs3PKljwG4Wbl_18rustc_query_system5query12QueryContext17current_query_job+0x55)[0x7f0d4b0c2fe5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x272fce5)[0x7f0d4b278ce5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x28101ec)[0x7f0d4b3591ec]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2361955)[0x7f0d4aeaa955]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x130faaa)[0x7f0d49e58aaa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1314291)[0x7f0d49e5d291]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x132baf4)[0x7f0d49e74af4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x13557d4)[0x7f0d49e9e7d4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1311376)[0x7f0d49e5a376]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1323d76)[0x7f0d49e6cd76]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x274416a)[0x7f0d4b28d16a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x285aac9)[0x7f0d4b3a3ac9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x23b5998)[0x7f0d4aefe998]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xc11021)[0x7f0d4975a021]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvXs5_Cs5jnRH9gmQqh_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCskzy2TwEqpvc_17rustc_codegen_ssa6traits7backend14CodegenBackend13codegen_crate+0xc7)[0x7f0d4982c557]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xa9198f)[0x7f0d495da98f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvNtCsdiqvhTzVGrU_15rustc_interface6passes13start_codegen+0x278)[0x7f0d495da3f8]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xa8f3a6)[0x7f0d495d83a6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs0_NtCsdiqvhTzVGrU_15rustc_interface7queriesNtB5_7Queries15ongoing_codegen+0x65)[0x7f0d49650225]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9ac2c3)[0x7f0d494f52c3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x943e6c)[0x7f0d4948ce6c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x99f0ca)[0x7f0d494e80ca]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x961989)[0x7f0d494aa989]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9a0806)[0x7f0d494e9806]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x953fa5)[0x7f0d4949cfa5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xd28ce)[0x7f0d48a4d8ce]
/lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7f0d487e2b43]
/lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7f0d48874a00]


---- [ui] src/test/ui/associated-types/defaults-cyclic-fail-2.rs stdout ----


error: Error: expected failure status (Some(1)) but received status None.
status: signal: 11 (SIGSEGV) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-cyclic-fail-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `<bool as Tr>::B == _`
   |
   |
LL |     type A = Box<Self::B>;


thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
stack backtrace:
   0:     0x7f226a0a7175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f226a1171e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f226a099291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f226a0a6f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f226a0aa364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f226a0aa02a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f226aaf56c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f226a0aaad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f226a0aa7fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f226a0a7694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f226a0aa4e2 - rust_begin_unwind
  11:     0x7f226a05afc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f226a05b09d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7f226c72cfe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7f226c9139c8 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId, ()>>
  15:     0x7f226c9ce498 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_well_formed, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7f226c54f160 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_well_formed
  17:     0x7f226b3d6818 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ForeignItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  18:     0x7f226b28c076 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ImplItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}>>
  19:     0x7f226b47a91d - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir::ImplItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>
  20:     0x7f226b3eb721 - rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf
  21:     0x7f226c926511 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  22:     0x7f226c9ce378 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_type_wf, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  23:     0x7f226c523e90 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_type_wf
  24:     0x7f226b3d69c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  25:     0x7f226b28c0f6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  26:     0x7f226b47aedd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  27:     0x7f226b33281d - <rustc_session[a2820a0c6534fd46]::session::Session>::track_errors::<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}, ()>
  28:     0x7f226b2b0513 - rustc_hir_analysis[b021cfaa7a998ff9]::check_crate
  29:     0x7f226ac440d1 - rustc_interface[9ae183de8b2774b4]::passes::analysis
  30:     0x7f226c8e9823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  31:     0x7f226ca178b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  32:     0x7f226c4fcb5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  33:     0x7f226ab531ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  34:     0x7f226ab5f276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  35:     0x7f226aaf6e6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  36:     0x7f226ab520ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  37:     0x7f226ab14989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  38:     0x7f226ab53806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  39:     0x7f226ab06fa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f226a0b78ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  41:     0x7f2269e4cb43 - <unknown>
  42:     0x7f2269edea00 - <unknown>
  43:                0x0 - <unknown>

/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x94bce3)[0x7f226aafece3]
/lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7f2269dfa520]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xb028c6)[0x7f226acb58c6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs5_Csasxc8sCYFdb_12rustc_errorsNtB5_12HandlerInner15emit_diagnostic+0x328)[0x7f226dba3368]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs4_Csasxc8sCYFdb_12rustc_errorsNtB5_7Handler15emit_diagnostic+0x21)[0x7f226dba2281]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsl0LieaIGlMN_12rustc_driver10report_ice+0x16f)[0x7f226aaf58cf]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN3std9panicking20rust_panic_with_hook17hab0b5a9b7152bbecE+0x274)[0x7f226a0aaad4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc57fa)[0x7f226a0aa7fa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc2694)[0x7f226a0a7694]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_begin_unwind+0x72)[0x7f226a0aa4e2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking9panic_fmt17h775c62c34b432a2dE+0x33)[0x7f226a05afc3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking5panic17hbb25aa978ab51c1fE+0x4d)[0x7f226a05b09d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvXs0_NtCslEjRGzJr5QU_16rustc_query_impl8plumbingNtB5_9QueryCtxtNtNtCs3PKljwG4Wbl_18rustc_query_system5query12QueryContext17current_query_job+0x55)[0x7f226c72cfe5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x27609c8)[0x7f226c9139c8]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x281b498)[0x7f226c9ce498]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x239c160)[0x7f226c54f160]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1223818)[0x7f226b3d6818]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x10d9076)[0x7f226b28c076]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x12c791d)[0x7f226b47a91d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1238721)[0x7f226b3eb721]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2773511)[0x7f226c926511]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x281b378)[0x7f226c9ce378]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2370e90)[0x7f226c523e90]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x12239c8)[0x7f226b3d69c8]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x10d90f6)[0x7f226b28c0f6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x12c7edd)[0x7f226b47aedd]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x117f81d)[0x7f226b33281d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsf7xUi6oyiPB_18rustc_hir_analysis11check_crate+0xf3)[0x7f226b2b0513]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvNtCsdiqvhTzVGrU_15rustc_interface6passes8analysis+0x61)[0x7f226ac440d1]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2736823)[0x7f226c8e9823]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x28648b4)[0x7f226ca178b4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2349b5a)[0x7f226c4fcb5a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9a01ec)[0x7f226ab531ec]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9ac276)[0x7f226ab5f276]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x943e6c)[0x7f226aaf6e6c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x99f0ca)[0x7f226ab520ca]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x961989)[0x7f226ab14989]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9a0806)[0x7f226ab53806]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x953fa5)[0x7f226ab06fa5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xd28ce)[0x7f226a0b78ce]
/lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7f2269e4cb43]
/lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7f2269edea00]


---- [ui] src/test/ui/associated-types/defaults-cyclic-fail-1.rs stdout ----


error: Error: expected failure status (Some(1)) but received status None.
status: signal: 11 (SIGSEGV) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-cyclic-fail-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `<bool as Tr>::B == _`
   |
   |
LL |     type A = Box<Self::B>;


thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
   0:     0x7fd0f0c58175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   0:     0x7fd0f0c58175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7fd0f0cc81e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7fd0f0c4a291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7fd0f0c57f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7fd0f0c5b364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7fd0f0c5b02a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7fd0f16a66c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fd0f0c5bad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7fd0f0c5b7fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7fd0f0c58694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7fd0f0c5b4e2 - rust_begin_unwind
  11:     0x7fd0f0c0bfc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7fd0f0c0c09d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7fd0f32ddfe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7fd0f34c49c8 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId, ()>>
  15:     0x7fd0f357f498 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_well_formed, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7fd0f3100160 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_well_formed
  17:     0x7fd0f1f87818 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ForeignItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  18:     0x7fd0f1e3d076 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ImplItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}>>
  19:     0x7fd0f202b91d - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir::ImplItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>
  20:     0x7fd0f1f9c721 - rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf
  21:     0x7fd0f34d7511 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  22:     0x7fd0f357f378 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_type_wf, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  23:     0x7fd0f30d4e90 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_type_wf
  24:     0x7fd0f1f879c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  25:     0x7fd0f1e3d0f6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  26:     0x7fd0f202bedd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  27:     0x7fd0f1ee381d - <rustc_session[a2820a0c6534fd46]::session::Session>::track_errors::<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}, ()>
  28:     0x7fd0f1e61513 - rustc_hir_analysis[b021cfaa7a998ff9]::check_crate
  29:     0x7fd0f17f50d1 - rustc_interface[9ae183de8b2774b4]::passes::analysis
  30:     0x7fd0f349a823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  31:     0x7fd0f35c88b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  32:     0x7fd0f30adb5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  33:     0x7fd0f17041ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  34:     0x7fd0f1710276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  35:     0x7fd0f16a7e6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  36:     0x7fd0f17030ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  37:     0x7fd0f16c5989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  38:     0x7fd0f1704806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  39:     0x7fd0f16b7fa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7fd0f0c688ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  41:     0x7fd0f09fdb43 - <unknown>
  42:     0x7fd0f0a8fa00 - <unknown>
  43:                0x0 - <unknown>

/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x94bce3)[0x7fd0f16afce3]
/lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7fd0f09ab520]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xb028c6)[0x7fd0f18668c6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs5_Csasxc8sCYFdb_12rustc_errorsNtB5_12HandlerInner15emit_diagnostic+0x328)[0x7fd0f4754368]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs4_Csasxc8sCYFdb_12rustc_errorsNtB5_7Handler15emit_diagnostic+0x21)[0x7fd0f4753281]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsl0LieaIGlMN_12rustc_driver10report_ice+0x16f)[0x7fd0f16a68cf]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN3std9panicking20rust_panic_with_hook17hab0b5a9b7152bbecE+0x274)[0x7fd0f0c5bad4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc57fa)[0x7fd0f0c5b7fa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc2694)[0x7fd0f0c58694]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_begin_unwind+0x72)[0x7fd0f0c5b4e2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking9panic_fmt17h775c62c34b432a2dE+0x33)[0x7fd0f0c0bfc3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking5panic17hbb25aa978ab51c1fE+0x4d)[0x7fd0f0c0c09d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvXs0_NtCslEjRGzJr5QU_16rustc_query_impl8plumbingNtB5_9QueryCtxtNtNtCs3PKljwG4Wbl_18rustc_query_system5query12QueryContext17current_query_job+0x55)[0x7fd0f32ddfe5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x27609c8)[0x7fd0f34c49c8]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x281b498)[0x7fd0f357f498]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x239c160)[0x7fd0f3100160]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1223818)[0x7fd0f1f87818]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x10d9076)[0x7fd0f1e3d076]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x12c791d)[0x7fd0f202b91d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1238721)[0x7fd0f1f9c721]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2773511)[0x7fd0f34d7511]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x281b378)[0x7fd0f357f378]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2370e90)[0x7fd0f30d4e90]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x12239c8)[0x7fd0f1f879c8]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x10d90f6)[0x7fd0f1e3d0f6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x12c7edd)[0x7fd0f202bedd]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x117f81d)[0x7fd0f1ee381d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsf7xUi6oyiPB_18rustc_hir_analysis11check_crate+0xf3)[0x7fd0f1e61513]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvNtCsdiqvhTzVGrU_15rustc_interface6passes8analysis+0x61)[0x7fd0f17f50d1]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2736823)[0x7fd0f349a823]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x28648b4)[0x7fd0f35c88b4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2349b5a)[0x7fd0f30adb5a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9a01ec)[0x7fd0f17041ec]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9ac276)[0x7fd0f1710276]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x943e6c)[0x7fd0f16a7e6c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x99f0ca)[0x7fd0f17030ca]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x961989)[0x7fd0f16c5989]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9a0806)[0x7fd0f1704806]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x953fa5)[0x7fd0f16b7fa5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xd28ce)[0x7fd0f0c688ce]
/lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7fd0f09fdb43]
/lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7fd0f0a8fa00]


---- [ui] src/test/ui/associated-types/hr-associated-type-bound-2.rs stdout ----


error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/hr-associated-type-bound-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `for<'b> u32: X<'b>`
   |
   |
LL | impl X<'_> for u32 //~ overflow evaluating the requirement `for<'b> u32: X<'b>`
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`hr_associated_type_bound_2`)
note: required for `u32` to implement `for<'b> X<'b>`
   |
   |
LL | impl X<'_> for u32 //~ overflow evaluating the requirement `for<'b> u32: X<'b>`
   = note: 128 redundant requirements hidden
   = note: 128 redundant requirements hidden
   = note: required for `u32` to implement `for<'b> X<'b>`

thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
stack backtrace:
   0:     0x7f3cd2be3175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f3cd2c531e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f3cd2bd5291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f3cd2be2f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f3cd2be6364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f3cd2be602a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f3cd36316c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3cd2be6ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f3cd2be67fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f3cd2be3694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f3cd2be64e2 - rust_begin_unwind
  11:     0x7f3cd2b96fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f3cd2b9709d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7f3cd5268fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7f3cd544f9c8 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId, ()>>
  15:     0x7f3cd550a498 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_well_formed, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7f3cd508b160 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_well_formed
  17:     0x7f3cd3f12818 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ForeignItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  18:     0x7f3cd3dc80d6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  19:     0x7f3cd3fb6d6d - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  20:     0x7f3cd3f2770a - rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf
  21:     0x7f3cd5462511 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  22:     0x7f3cd550a378 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_type_wf, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  23:     0x7f3cd505fe90 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_type_wf
  24:     0x7f3cd3f129c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  25:     0x7f3cd3dc80f6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  26:     0x7f3cd3fb6edd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  27:     0x7f3cd3e6e81d - <rustc_session[a2820a0c6534fd46]::session::Session>::track_errors::<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}, ()>
  28:     0x7f3cd3dec513 - rustc_hir_analysis[b021cfaa7a998ff9]::check_crate
  29:     0x7f3cd37800d1 - rustc_interface[9ae183de8b2774b4]::passes::analysis
  30:     0x7f3cd5425823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  31:     0x7f3cd55538b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  32:     0x7f3cd5038b5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  33:     0x7f3cd368f1ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  34:     0x7f3cd369b276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  35:     0x7f3cd3632e6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  36:     0x7f3cd368e0ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  37:     0x7f3cd3650989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  38:     0x7f3cd368f806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  39:     0x7f3cd3642fa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f3cd2bf38ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  41:     0x7f3cd2988b43 - <unknown>
  42:     0x7f3cd2a1aa00 - <unknown>
  43:                0x0 - <unknown>

thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/callbacks.rs:33:51
stack backtrace:
   0:     0x7f3cd2be3175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f3cd2c531e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f3cd2bd5291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f3cd2be2f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f3cd2be6364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f3cd2be602a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f3cd36316c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3cd2be6ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f3cd2be6839 - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f3cd2be3694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f3cd2be64e2 - rust_begin_unwind
  11:     0x7f3cd2b96fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f3cd2b974a3 - core::result::unwrap_failed::h3f88fe2aa5969013
  13:     0x7f3cd37f1da5 - rustc_interface[9ae183de8b2774b4]::callbacks::track_diagnostic
  14:     0x7f3cd66df368 - <rustc_errors[79d644afcdca68d7]::HandlerInner>::emit_diagnostic
  15:     0x7f3cd66de281 - <rustc_errors[79d644afcdca68d7]::Handler>::emit_diagnostic
  16:     0x7f3cd36318cf - rustc_driver[f4be4875f4d52ad7]::report_ice
  17:     0x7f3cd2be6ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
  18:     0x7f3cd2be67fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
  19:     0x7f3cd2be3694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  20:     0x7f3cd2be64e2 - rust_begin_unwind
  21:     0x7f3cd2b96fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  22:     0x7f3cd2b9709d - core::panicking::panic::hbb25aa978ab51c1f
  23:     0x7f3cd5268fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  24:     0x7f3cd544f9c8 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId, ()>>
  25:     0x7f3cd550a498 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_well_formed, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  26:     0x7f3cd508b160 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_well_formed
  27:     0x7f3cd3f12818 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ForeignItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  28:     0x7f3cd3dc80d6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7f3cd3fb6d6d - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  30:     0x7f3cd3f2770a - rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf
  31:     0x7f3cd5462511 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  32:     0x7f3cd550a378 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_type_wf, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  33:     0x7f3cd505fe90 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_type_wf
  34:     0x7f3cd3f129c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  35:     0x7f3cd3dc80f6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7f3cd3fb6edd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  37:     0x7f3cd3e6e81d - <rustc_session[a2820a0c6534fd46]::session::Session>::track_errors::<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}, ()>
  38:     0x7f3cd3dec513 - rustc_hir_analysis[b021cfaa7a998ff9]::check_crate
  39:     0x7f3cd37800d1 - rustc_interface[9ae183de8b2774b4]::passes::analysis
  40:     0x7f3cd5425823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  41:     0x7f3cd55538b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  42:     0x7f3cd5038b5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  43:     0x7f3cd368f1ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  44:     0x7f3cd369b276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  45:     0x7f3cd3632e6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  46:     0x7f3cd368e0ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  47:     0x7f3cd3650989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  48:     0x7f3cd368f806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  49:     0x7f3cd3642fa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f3cd2bf38ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  51:     0x7f3cd2988b43 - <unknown>
  52:     0x7f3cd2a1aa00 - <unknown>
  53:                0x0 - <unknown>
thread panicked while processing panic. aborting.
------------------------------------------



---- [ui] src/test/ui/associated-types/impl-wf-cycle-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/impl-wf-cycle-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
   |
   |
LL | impl<T: Grault> Grault for (T,)
   |
   |
note: required for `(T,)` to implement `Grault`
   |
   |
LL | impl<T: Grault> Grault for (T,)
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required for `(T,)` to implement `Grault`

thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
   0:     0x7f9ce3216175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   0:     0x7f9ce3216175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f9ce32861e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f9ce3208291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f9ce3215f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f9ce3219364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f9ce321902a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f9ce3c646c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f9ce3219ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f9ce32197fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f9ce3216694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f9ce32194e2 - rust_begin_unwind
  11:     0x7f9ce31c9fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f9ce31ca09d - core::panicking::panic::hbb25aa978ab51c1f
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (c4cd43694 2022-12-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `X`
#1 [eval_to_const_value_raw] simplifying constant for the type system `X`
#2 [eval_to_const_value_raw] simplifying constant for the type system `X`
#3 [lint_mod] linting top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   2: core::panicking::panic
   3: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::current_query_job
   4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, ()>>
   5: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_mod_privacy, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
   6: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_privacy
   7: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_interface::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   8: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_interface::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
   9: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_interface::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>
  10: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#5}::{closure#2}::{closure#0}>
  11: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#2}>, ()>
  12: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#5}>
  14: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
  15: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
  16: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  17: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  17: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  18: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
  19: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  20: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/callbacks.rs:33:51
stack backtrace:
   0:     0x7f0645f65175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f0645fd51e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f0645f57291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f0645f64f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f0645f68364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f0645f6802a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f06469b36c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f0645f68ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f0645f68839 - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f0645f65694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f0645f684e2 - rust_begin_unwind
  11:     0x7f0645f18fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f0645f194a3 - core::result::unwrap_failed::h3f88fe2aa5969013
  13:     0x7f0646b73da5 - rustc_interface[9ae183de8b2774b4]::callbacks::track_diagnostic
  14:     0x7f0649a61368 - <rustc_errors[79d644afcdca68d7]::HandlerInner>::emit_diagnostic
  15:     0x7f0649a60281 - <rustc_errors[79d644afcdca68d7]::Handler>::emit_diagnostic
  16:     0x7f06469b38cf - rustc_driver[f4be4875f4d52ad7]::report_ice
  17:     0x7f0645f68ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
  18:     0x7f0645f687fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
  19:     0x7f0645f65694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  20:     0x7f0645f684e2 - rust_begin_unwind
  21:     0x7f0645f18fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  22:     0x7f0645f1909d - core::panicking::panic::hbb25aa978ab51c1f
  23:     0x7f06485eafe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  24:     0x7f06487e4218 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  25:     0x7f064888c258 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_privacy, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  26:     0x7f06483df9a0 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_privacy
  27:     0x7f0646b715c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  28:     0x7f0646aeef26 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  29:     0x7f0646acc7fd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>
  30:     0x7f0646b05054 - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}>
  31:     0x7f0646aef115 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}>, ()>
  32:     0x7f0646b069fb - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}>
  33:     0x7f0646b0216c - rustc_interface[9ae183de8b2774b4]::passes::analysis
  34:     0x7f06487a7823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  35:     0x7f06488d58b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  36:     0x7f06483bab5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  37:     0x7f0646a111ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  38:     0x7f0646a1d276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  39:     0x7f06469b4e6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  40:     0x7f0646a100ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  41:     0x7f06469d2989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  42:     0x7f0646a11806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  43:     0x7f06469c4fa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f0645f758ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  45:     0x7f0645d0ab43 - <unknown>
  46:     0x7f0645d9ca00 - <unknown>
  47:                0x0 - <unknown>
thread panicked while processing panic. aborting.
------------------------------------------



---- [ui] src/test/ui/consts/const-size_of-cycle.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when evaluating type-level constant
   |
   |
LL |     bytes: [u8; std::mem::size_of::<Foo>()]
   |
   |
note: ...which requires const-evaluating + checking `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; std::mem::size_of::<Foo>()]
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; std::mem::size_of::<Foo>()]
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing layout of `Foo`...
   = note: ...which requires computing layout of `[u8; _]`...
   = note: ...which requires normalizing `[u8; _]`...
   = note: ...which again requires evaluating type-level constant, completing the cycle
note: cycle used when checking that `Foo` is well-formed
   |
LL | struct Foo {
   | ^^^^^^^^^^


thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
stack backtrace:
   0:     0x7f8cb1c5a175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f8cb1cca1e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f8cb1c4c291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f8cb1c59f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f8cb1c5d364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f8cb1c5d02a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f8cb26a86c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f8cb1c5dad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f8cb1c5d7fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f8cb1c5a694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f8cb1c5d4e2 - rust_begin_unwind
  11:     0x7f8cb1c0dfc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f8cb1c0e09d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7f8cb42dffe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7f8cb44c69c8 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId, ()>>
  15:     0x7f8cb4581498 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_well_formed, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7f8cb4102160 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_well_formed
  17:     0x7f8cb2f89818 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ForeignItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  18:     0x7f8cb2e3f0d6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  19:     0x7f8cb302dd6d - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  20:     0x7f8cb2f9e70a - rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf
  21:     0x7f8cb44d9511 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  22:     0x7f8cb4581378 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_type_wf, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  23:     0x7f8cb40d6e90 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_type_wf
  24:     0x7f8cb2f899c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  25:     0x7f8cb2e3f0f6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  26:     0x7f8cb302dedd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  27:     0x7f8cb2ee581d - <rustc_session[a2820a0c6534fd46]::session::Session>::track_errors::<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}, ()>
  28:     0x7f8cb2e63513 - rustc_hir_analysis[b021cfaa7a998ff9]::check_crate
  29:     0x7f8cb27f70d1 - rustc_interface[9ae183de8b2774b4]::passes::analysis
  30:     0x7f8cb449c823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  31:     0x7f8cb45ca8b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  32:     0x7f8cb40afb5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  33:     0x7f8cb27061ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  34:     0x7f8cb2712276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  35:     0x7f8cb26a9e6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  36:     0x7f8cb27050ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  37:     0x7f8cb26c7989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  38:     0x7f8cb2706806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  39:     0x7f8cb26b9fa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f8cb1c6a8ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  41:     0x7f8cb19ffb43 - <unknown>
  42:     0x7f8cb1a91a00 - <unknown>
  43:                0x0 - <unknown>

thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/callbacks.rs:33:51
stack backtrace:
   0:     0x7f8cb1c5a175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f8cb1cca1e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f8cb1c4c291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f8cb1c59f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f8cb1c5d364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f8cb1c5d02a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f8cb26a86c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f8cb1c5dad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f8cb1c5d839 - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f8cb1c5a694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f8cb1c5d4e2 - rust_begin_unwind
  11:     0x7f8cb1c0dfc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f8cb1c0e4a3 - core::result::unwrap_failed::h3f88fe2aa5969013
  13:     0x7f8cb2868da5 - rustc_interface[9ae183de8b2774b4]::callbacks::track_diagnostic
  14:     0x7f8cb5756368 - <rustc_errors[79d644afcdca68d7]::HandlerInner>::emit_diagnostic
  15:     0x7f8cb5755281 - <rustc_errors[79d644afcdca68d7]::Handler>::emit_diagnostic
  16:     0x7f8cb26a88cf - rustc_driver[f4be4875f4d52ad7]::report_ice
  17:     0x7f8cb1c5dad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
  18:     0x7f8cb1c5d7fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
  19:     0x7f8cb1c5a694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  20:     0x7f8cb1c5d4e2 - rust_begin_unwind
  21:     0x7f8cb1c0dfc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  22:     0x7f8cb1c0e09d - core::panicking::panic::hbb25aa978ab51c1f
  23:     0x7f8cb42dffe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  24:     0x7f8cb44c69c8 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId, ()>>
  25:     0x7f8cb4581498 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_well_formed, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  26:     0x7f8cb4102160 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_well_formed
  27:     0x7f8cb2f89818 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ForeignItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  28:     0x7f8cb2e3f0d6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7f8cb302dd6d - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  30:     0x7f8cb2f9e70a - rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf
  31:     0x7f8cb44d9511 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  32:     0x7f8cb4581378 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_type_wf, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  33:     0x7f8cb40d6e90 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_type_wf
  34:     0x7f8cb2f899c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  35:     0x7f8cb2e3f0f6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7f8cb302dedd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  37:     0x7f8cb2ee581d - <rustc_session[a2820a0c6534fd46]::session::Session>::track_errors::<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}, ()>
  38:     0x7f8cb2e63513 - rustc_hir_analysis[b021cfaa7a998ff9]::check_crate
  39:     0x7f8cb27f70d1 - rustc_interface[9ae183de8b2774b4]::passes::analysis
  40:     0x7f8cb449c823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  41:     0x7f8cb45ca8b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  42:     0x7f8cb40afb5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  43:     0x7f8cb27061ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  44:     0x7f8cb2712276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  45:     0x7f8cb26a9e6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  46:     0x7f8cb27050ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  47:     0x7f8cb26c7989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  48:     0x7f8cb2706806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  49:     0x7f8cb26b9fa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f8cb1c6a8ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  51:     0x7f8cb19ffb43 - <unknown>
  52:     0x7f8cb1a91a00 - <unknown>
  53:                0x0 - <unknown>
thread panicked while processing panic. aborting.
------------------------------------------



---- [ui] src/test/ui/consts/issue-44415.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-44415.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-44415" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-44415/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when evaluating type-level constant
   |
   |
LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
   |
   |
note: ...which requires const-evaluating + checking `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing layout of `Foo`...
   = note: ...which requires computing layout of `[u8; _]`...
   = note: ...which requires normalizing `[u8; _]`...
   = note: ...which again requires evaluating type-level constant, completing the cycle
note: cycle used when checking that `Foo` is well-formed
   |
LL | struct Foo {
   | ^^^^^^^^^^


thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
   0:     0x7f9bbedf1175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   0:     0x7f9bbedf1175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f9bbee611e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f9bbede3291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f9bbedf0f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f9bbedf4364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f9bbedf402a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f9bbf83f6c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f9bbedf4ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f9bbedf47fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f9bbedf1694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f9bbedf44e2 - rust_begin_unwind
  11:     0x7f9bbeda4fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f9bbeda509d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7f9bc1476fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7f9bc165d9c8 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId, ()>>
  15:     0x7f9bc1718498 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_well_formed, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7f9bc1299160 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_well_formed
  17:     0x7f9bc0120818 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ForeignItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  18:     0x7f9bbffd60d6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  19:     0x7f9bc01c4d6d - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  20:     0x7f9bc013570a - rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf
  21:     0x7f9bc1670511 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  22:     0x7f9bc1718378 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_type_wf, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  23:     0x7f9bc126de90 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_type_wf
  24:     0x7f9bc01209c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  25:     0x7f9bbffd60f6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  26:     0x7f9bc01c4edd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  27:     0x7f9bc007c81d - <rustc_session[a2820a0c6534fd46]::session::Session>::track_errors::<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}, ()>
  28:     0x7f9bbfffa513 - rustc_hir_analysis[b021cfaa7a998ff9]::check_crate
  29:     0x7f9bbf98e0d1 - rustc_interface[9ae183de8b2774b4]::passes::analysis
  30:     0x7f9bc1633823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  31:     0x7f9bc17618b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  32:     0x7f9bc1246b5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  33:     0x7f9bbf89d1ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  34:     0x7f9bbf8a9276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  35:     0x7f9bbf840e6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  36:     0x7f9bbf89c0ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  37:     0x7f9bbf85e989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  38:     0x7f9bbf89d806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  39:     0x7f9bbf850fa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f9bbee018ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  41:     0x7f9bbeb96b43 - <unknown>
  42:     0x7f9bbec28a00 - <unknown>
  43:                0x0 - <unknown>

thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/callbacks.rs:33:51
   0:     0x7f9bbedf1175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   0:     0x7f9bbedf1175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f9bbee611e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f9bbede3291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f9bbedf0f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f9bbedf4364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f9bbedf402a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f9bbf83f6c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f9bbedf4ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f9bbedf4839 - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f9bbedf1694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f9bbedf44e2 - rust_begin_unwind
  11:     0x7f9bbeda4fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f9bbeda54a3 - core::result::unwrap_failed::h3f88fe2aa5969013
  13:     0x7f9bbf9ffda5 - rustc_interface[9ae183de8b2774b4]::callbacks::track_diagnostic
  14:     0x7f9bc28ed368 - <rustc_errors[79d644afcdca68d7]::HandlerInner>::emit_diagnostic
  15:     0x7f9bc28ec281 - <rustc_errors[79d644afcdca68d7]::Handler>::emit_diagnostic
  16:     0x7f9bbf83f8cf - rustc_driver[f4be4875f4d52ad7]::report_ice
  17:     0x7f9bbedf4ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
  18:     0x7f9bbedf47fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
  19:     0x7f9bbedf1694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  20:     0x7f9bbedf44e2 - rust_begin_unwind
  21:     0x7f9bbeda4fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  22:     0x7f9bbeda509d - core::panicking::panic::hbb25aa978ab51c1f
  23:     0x7f9bc1476fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  24:     0x7f9bc165d9c8 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId, ()>>
  25:     0x7f9bc1718498 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_well_formed, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  26:     0x7f9bc1299160 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_well_formed
  27:     0x7f9bc0120818 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ForeignItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  28:     0x7f9bbffd60d6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7f9bc01c4d6d - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  30:     0x7f9bc013570a - rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf
  31:     0x7f9bc1670511 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  32:     0x7f9bc1718378 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_type_wf, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  33:     0x7f9bc126de90 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_type_wf
  34:     0x7f9bc01209c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  35:     0x7f9bbffd60f6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7f9bc01c4edd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  37:     0x7f9bc007c81d - <rustc_session[a2820a0c6534fd46]::session::Session>::track_errors::<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}, ()>
  38:     0x7f9bbfffa513 - rustc_hir_analysis[b021cfaa7a998ff9]::check_crate
  39:     0x7f9bbf98e0d1 - rustc_interface[9ae183de8b2774b4]::passes::analysis
  40:     0x7f9bc1633823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  41:     0x7f9bc17618b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  42:     0x7f9bc1246b5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  43:     0x7f9bbf89d1ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  44:     0x7f9bbf8a9276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  45:     0x7f9bbf840e6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  46:     0x7f9bbf89c0ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  47:     0x7f9bbf85e989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  48:     0x7f9bbf89d806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  49:     0x7f9bbf850fa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f9bbee018ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  51:     0x7f9bbeb96b43 - <unknown>
  52:     0x7f9bbec28a00 - <unknown>
  53:                0x0 - <unknown>
thread panicked while processing panic. aborting.
------------------------------------------



---- [ui] src/test/ui/consts/recursive-zst-static.rs#default stdout ----

error in revision `default`: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/recursive-zst-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/recursive-zst-static.default" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/recursive-zst-static.default/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when const-evaluating + checking `FOO`
   |
   |
LL | static FOO: () = FOO; //~ cycle detected when const-evaluating + checking `FOO`
   |
   |
note: ...which requires const-evaluating + checking `FOO`...
   |
   |
LL | static FOO: () = FOO; //~ cycle detected when const-evaluating + checking `FOO`
   |                  ^^^
   = note: ...which again requires const-evaluating + checking `FOO`, completing the cycle
note: cycle used when linting top-level module
   |
   |
LL | / static FOO: () = FOO; //~ cycle detected when const-evaluating + checking `FOO`
LL | |
LL | | fn main() {
LL | |     FOO
LL | | }


thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
stack backtrace:
   0:     0x7fa43216f175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7fa4321df1e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7fa432161291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7fa43216ef81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7fa432172364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7fa43217202a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7fa432bbd6c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa432172ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7fa4321727fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7fa43216f694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7fa4321724e2 - rust_begin_unwind
  11:     0x7fa432122fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7fa43212309d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7fa4347f4fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7fa4349ee218 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  15:     0x7fa434a96258 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_privacy, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7fa4345e99a0 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_privacy
  17:     0x7fa432d7b5c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  18:     0x7fa432cf8f26 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  19:     0x7fa432cd67fd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>
  20:     0x7fa432d0f054 - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}>
  21:     0x7fa432cf9115 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}>, ()>
  22:     0x7fa432d109fb - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}>
  23:     0x7fa432d0c16c - rustc_interface[9ae183de8b2774b4]::passes::analysis
  24:     0x7fa4349b1823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  25:     0x7fa434adf8b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  26:     0x7fa4345c4b5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  27:     0x7fa432c1b1ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  28:     0x7fa432c27276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  29:     0x7fa432bbee6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  30:     0x7fa432c1a0ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  31:     0x7fa432bdc989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  32:     0x7fa432c1b806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  33:     0x7fa432bcefa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7fa43217f8ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  35:     0x7fa431f14b43 - <unknown>
  36:     0x7fa431fa6a00 - <unknown>
  37:                0x0 - <unknown>

thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/callbacks.rs:33:51
stack backtrace:
   0:     0x7fa43216f175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7fa4321df1e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7fa432161291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7fa43216ef81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7fa432172364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7fa43217202a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7fa432bbd6c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa432172ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7fa432172839 - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7fa43216f694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7fa4321724e2 - rust_begin_unwind
  11:     0x7fa432122fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7fa4321234a3 - core::result::unwrap_failed::h3f88fe2aa5969013
  13:     0x7fa432d7dda5 - rustc_interface[9ae183de8b2774b4]::callbacks::track_diagnostic
  14:     0x7fa435c6b368 - <rustc_errors[79d644afcdca68d7]::HandlerInner>::emit_diagnostic
  15:     0x7fa435c6a281 - <rustc_errors[79d644afcdca68d7]::Handler>::emit_diagnostic
  16:     0x7fa432bbd8cf - rustc_driver[f4be4875f4d52ad7]::report_ice
  17:     0x7fa432172ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
  18:     0x7fa4321727fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
  19:     0x7fa43216f694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  20:     0x7fa4321724e2 - rust_begin_unwind
  21:     0x7fa432122fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  22:     0x7fa43212309d - core::panicking::panic::hbb25aa978ab51c1f
  23:     0x7fa4347f4fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  24:     0x7fa4349ee218 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  25:     0x7fa434a96258 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_privacy, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  26:     0x7fa4345e99a0 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_privacy
  27:     0x7fa432d7b5c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  28:     0x7fa432cf8f26 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  29:     0x7fa432cd67fd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}>
  30:     0x7fa432d0f054 - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}::{closure#0}>
  31:     0x7fa432cf9115 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}::{closure#2}>, ()>
  32:     0x7fa432d109fb - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_interface[9ae183de8b2774b4]::passes::analysis::{closure#5}>
  33:     0x7fa432d0c16c - rustc_interface[9ae183de8b2774b4]::passes::analysis
  34:     0x7fa4349b1823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  35:     0x7fa434adf8b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  36:     0x7fa4345c4b5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  37:     0x7fa432c1b1ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  38:     0x7fa432c27276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  39:     0x7fa432bbee6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  40:     0x7fa432c1a0ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  41:     0x7fa432bdc989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  42:     0x7fa432c1b806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  43:     0x7fa432bcefa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7fa43217f8ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  45:     0x7fa431f14b43 - <unknown>
  46:     0x7fa431fa6a00 - <unknown>
  47:                0x0 - <unknown>
thread panicked while processing panic. aborting.
------------------------------------------



---
   |
note: required for `i32` to implement `Iterate<'_>`
  --> /checkout/src/test/ui/specialization/issue-38091-2.rs:11:13
   |
LL | impl<'a, T> Iterate<'a> for T


thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
stack backtrace:
   0:     0x7f21f1170175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f21f11e01e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f21f1162291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f21f116ff81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f21f1173364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f21f117302a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f21f1bbe6c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f21f1173ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f21f11737fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f21f1170694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f21f11734e2 - rust_begin_unwind
  11:     0x7f21f1123fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f21f112409d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7f21f37f5fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7f21f39abce5 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<rustc_span[755eca102fd61de6]::def_id::DefId, bool>>
  15:     0x7f21f3a8c1ec - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::is_foreign_item, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7f21f35dd955 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::is_foreign_item
  17:     0x7f21f258baaa - rustc_monomorphize[d88e042efd1bf97]::collector::should_codegen_locally
  18:     0x7f21f2590291 - rustc_monomorphize[d88e042efd1bf97]::collector::collect_items_rec
  19:     0x7f21f25a7af4 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<alloc[75b9de5db4717345]::vec::Vec<rustc_middle[cc0ab085cda1518a]::mir::mono::MonoItem>, rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  20:     0x7f21f25d17d4 - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items::{closure#1}>
  21:     0x7f21f258d376 - rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items
  22:     0x7f21f259fd76 - rustc_monomorphize[d88e042efd1bf97]::partitioning::collect_and_partition_mono_items
  23:     0x7f21f39c016a - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), (&std[5ac0ced0274dd2d1]::collections::hash::set::HashSet<rustc_span[755eca102fd61de6]::def_id::DefId, core[e5930317b0d12ce1]::hash::BuildHasherDefault<rustc_hash[ff8b1808a785fb13]::FxHasher>>, &[rustc_middle[cc0ab085cda1518a]::mir::mono::CodegenUnit])>>
  24:     0x7f21f3ad6ac9 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::collect_and_partition_mono_items, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  25:     0x7f21f3631998 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::collect_and_partition_mono_items
  26:     0x7f21f1e8d021 - rustc_codegen_ssa[efa17c4ad961d364]::base::codegen_crate::<rustc_codegen_llvm[3de13d325ebb1d9f]::LlvmCodegenBackend>
  27:     0x7f21f1f5f557 - <rustc_codegen_llvm[3de13d325ebb1d9f]::LlvmCodegenBackend as rustc_codegen_ssa[efa17c4ad961d364]::traits::backend::CodegenBackend>::codegen_crate
  28:     0x7f21f1d0d98f - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<alloc[75b9de5db4717345]::boxed::Box<dyn core[e5930317b0d12ce1]::any::Any>, rustc_interface[9ae183de8b2774b4]::passes::start_codegen::{closure#0}>
  29:     0x7f21f1d0d3f8 - rustc_interface[9ae183de8b2774b4]::passes::start_codegen
  30:     0x7f21f1d0b3a6 - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<<rustc_interface[9ae183de8b2774b4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<alloc[75b9de5db4717345]::boxed::Box<dyn core[e5930317b0d12ce1]::any::Any>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  31:     0x7f21f1d83225 - <rustc_interface[9ae183de8b2774b4]::queries::Queries>::ongoing_codegen
  32:     0x7f21f1c282c3 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  33:     0x7f21f1bbfe6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  34:     0x7f21f1c1b0ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  35:     0x7f21f1bdd989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  36:     0x7f21f1c1c806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  37:     0x7f21f1bcffa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f21f11808ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  39:     0x7f21f0f15b43 - <unknown>
  40:     0x7f21f0fa7a00 - <unknown>
  41:                0x0 - <unknown>

thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/callbacks.rs:33:51
stack backtrace:
   0:     0x7f21f1170175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7f21f11e01e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7f21f1162291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7f21f116ff81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7f21f1173364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7f21f117302a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7f21f1bbe6c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f21f1173ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7f21f1173839 - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7f21f1170694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7f21f11734e2 - rust_begin_unwind
  11:     0x7f21f1123fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7f21f11244a3 - core::result::unwrap_failed::h3f88fe2aa5969013
  13:     0x7f21f1d7eda5 - rustc_interface[9ae183de8b2774b4]::callbacks::track_diagnostic
  14:     0x7f21f4c6c368 - <rustc_errors[79d644afcdca68d7]::HandlerInner>::emit_diagnostic
  15:     0x7f21f4c6b281 - <rustc_errors[79d644afcdca68d7]::Handler>::emit_diagnostic
  16:     0x7f21f1bbe8cf - rustc_driver[f4be4875f4d52ad7]::report_ice
  17:     0x7f21f1173ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
  18:     0x7f21f11737fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
  19:     0x7f21f1170694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  20:     0x7f21f11734e2 - rust_begin_unwind
  21:     0x7f21f1123fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  22:     0x7f21f112409d - core::panicking::panic::hbb25aa978ab51c1f
  23:     0x7f21f37f5fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  24:     0x7f21f39abce5 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<rustc_span[755eca102fd61de6]::def_id::DefId, bool>>
  25:     0x7f21f3a8c1ec - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::is_foreign_item, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  26:     0x7f21f35dd955 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::is_foreign_item
  27:     0x7f21f258baaa - rustc_monomorphize[d88e042efd1bf97]::collector::should_codegen_locally
  28:     0x7f21f2590291 - rustc_monomorphize[d88e042efd1bf97]::collector::collect_items_rec
  29:     0x7f21f25a7af4 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<alloc[75b9de5db4717345]::vec::Vec<rustc_middle[cc0ab085cda1518a]::mir::mono::MonoItem>, rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  30:     0x7f21f25d17d4 - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items::{closure#1}>
  31:     0x7f21f258d376 - rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items
  32:     0x7f21f259fd76 - rustc_monomorphize[d88e042efd1bf97]::partitioning::collect_and_partition_mono_items
  33:     0x7f21f39c016a - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), (&std[5ac0ced0274dd2d1]::collections::hash::set::HashSet<rustc_span[755eca102fd61de6]::def_id::DefId, core[e5930317b0d12ce1]::hash::BuildHasherDefault<rustc_hash[ff8b1808a785fb13]::FxHasher>>, &[rustc_middle[cc0ab085cda1518a]::mir::mono::CodegenUnit])>>
  34:     0x7f21f3ad6ac9 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::collect_and_partition_mono_items, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  35:     0x7f21f3631998 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::collect_and_partition_mono_items
  36:     0x7f21f1e8d021 - rustc_codegen_ssa[efa17c4ad961d364]::base::codegen_crate::<rustc_codegen_llvm[3de13d325ebb1d9f]::LlvmCodegenBackend>
  37:     0x7f21f1f5f557 - <rustc_codegen_llvm[3de13d325ebb1d9f]::LlvmCodegenBackend as rustc_codegen_ssa[efa17c4ad961d364]::traits::backend::CodegenBackend>::codegen_crate
  38:     0x7f21f1d0d98f - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<alloc[75b9de5db4717345]::boxed::Box<dyn core[e5930317b0d12ce1]::any::Any>, rustc_interface[9ae183de8b2774b4]::passes::start_codegen::{closure#0}>
  39:     0x7f21f1d0d3f8 - rustc_interface[9ae183de8b2774b4]::passes::start_codegen
  40:     0x7f21f1d0b3a6 - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<<rustc_interface[9ae183de8b2774b4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<alloc[75b9de5db4717345]::boxed::Box<dyn core[e5930317b0d12ce1]::any::Any>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  41:     0x7f21f1d83225 - <rustc_interface[9ae183de8b2774b4]::queries::Queries>::ongoing_codegen
  42:     0x7f21f1c282c3 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  43:     0x7f21f1bbfe6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  44:     0x7f21f1c1b0ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  45:     0x7f21f1bdd989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  46:     0x7f21f1c1c806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  47:     0x7f21f1bcffa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:     0x7f21f11808ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  49:     0x7f21f0f15b43 - <unknown>
  50:     0x7f21f0fa7a00 - <unknown>
  51:                0x0 - <unknown>

/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x94bce3)[0x7f21f1bc7ce3]
/lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7f21f0ec3520]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xb028c6)[0x7f21f1d7e8c6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs5_Csasxc8sCYFdb_12rustc_errorsNtB5_12HandlerInner15emit_diagnostic+0x328)[0x7f21f4c6c368]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs4_Csasxc8sCYFdb_12rustc_errorsNtB5_7Handler15emit_diagnostic+0x21)[0x7f21f4c6b281]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsl0LieaIGlMN_12rustc_driver10report_ice+0x16f)[0x7f21f1bbe8cf]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN3std9panicking20rust_panic_with_hook17hab0b5a9b7152bbecE+0x274)[0x7f21f1173ad4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc5839)[0x7f21f1173839]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc2694)[0x7f21f1170694]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_begin_unwind+0x72)[0x7f21f11734e2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking9panic_fmt17h775c62c34b432a2dE+0x33)[0x7f21f1123fc3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core6result13unwrap_failed17h3f88fe2aa5969013E+0x83)[0x7f21f11244a3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xb02da5)[0x7f21f1d7eda5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs5_Csasxc8sCYFdb_12rustc_errorsNtB5_12HandlerInner15emit_diagnostic+0x328)[0x7f21f4c6c368]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs4_Csasxc8sCYFdb_12rustc_errorsNtB5_7Handler15emit_diagnostic+0x21)[0x7f21f4c6b281]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsl0LieaIGlMN_12rustc_driver10report_ice+0x16f)[0x7f21f1bbe8cf]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN3std9panicking20rust_panic_with_hook17hab0b5a9b7152bbecE+0x274)[0x7f21f1173ad4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc57fa)[0x7f21f11737fa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc2694)[0x7f21f1170694]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_begin_unwind+0x72)[0x7f21f11734e2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking9panic_fmt17h775c62c34b432a2dE+0x33)[0x7f21f1123fc3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking5panic17hbb25aa978ab51c1fE+0x4d)[0x7f21f112409d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvXs0_NtCslEjRGzJr5QU_16rustc_query_impl8plumbingNtB5_9QueryCtxtNtNtCs3PKljwG4Wbl_18rustc_query_system5query12QueryContext17current_query_job+0x55)[0x7f21f37f5fe5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x272fce5)[0x7f21f39abce5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x28101ec)[0x7f21f3a8c1ec]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2361955)[0x7f21f35dd955]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x130faaa)[0x7f21f258baaa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1314291)[0x7f21f2590291]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x132baf4)[0x7f21f25a7af4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x13557d4)[0x7f21f25d17d4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1311376)[0x7f21f258d376]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1323d76)[0x7f21f259fd76]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x274416a)[0x7f21f39c016a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x285aac9)[0x7f21f3ad6ac9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x23b5998)[0x7f21f3631998]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xc11021)[0x7f21f1e8d021]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvXs5_Cs5jnRH9gmQqh_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCskzy2TwEqpvc_17rustc_codegen_ssa6traits7backend14CodegenBackend13codegen_crate+0xc7)[0x7f21f1f5f557]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xa9198f)[0x7f21f1d0d98f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvNtCsdiqvhTzVGrU_15rustc_interface6passes13start_codegen+0x278)[0x7f21f1d0d3f8]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xa8f3a6)[0x7f21f1d0b3a6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs0_NtCsdiqvhTzVGrU_15rustc_interface7queriesNtB5_7Queries15ongoing_codegen+0x65)[0x7f21f1d83225]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9ac2c3)[0x7f21f1c282c3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x943e6c)[0x7f21f1bbfe6c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x99f0ca)[0x7f21f1c1b0ca]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x961989)[0x7f21f1bdd989]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9a0806)[0x7f21f1c1c806]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x953fa5)[0x7f21f1bcffa5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xd28ce)[0x7f21f11808ce]
/lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7f21f0f15b43]
/lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7f21f0fa7a00]


---- [ui] src/test/ui/traits/cycle-cache-err-60010.rs stdout ----


error: Error: expected failure status (Some(1)) but received status None.
status: signal: 11 (SIGSEGV) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `SalsaStorage: RefUnwindSafe`
   |
   |
LL |     _parse: <ParseQuery as Query<RootDatabase>>::Data,
   |
   |
   = note: required because it appears within the type `PhantomData<SalsaStorage>`
   = note: required because it appears within the type `Unique<SalsaStorage>`
   = note: required because it appears within the type `Box<SalsaStorage>`
note: required because it appears within the type `Runtime<RootDatabase>`
   |
   |
LL | struct Runtime<DB: Database> {
   |        ^^^^^^^
note: required because it appears within the type `RootDatabase`
   |
   |
LL | struct RootDatabase {
   |        ^^^^^^^^^^^^
note: required for `RootDatabase` to implement `SourceDatabase`
   |
   |
LL | impl<T> SourceDatabase for T
   |         ^^^^^^^^^^^^^^     ^
note: required for `ParseQuery` to implement `Query<RootDatabase>`
   |
   |
LL | impl<DB> Query<DB> for ParseQuery


thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
stack backtrace:
   0:     0x7fbce3edb175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7fbce3f4b1e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7fbce3ecd291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7fbce3edaf81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7fbce3ede364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7fbce3ede02a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7fbce49296c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fbce3edead4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7fbce3ede7fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7fbce3edb694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7fbce3ede4e2 - rust_begin_unwind
  11:     0x7fbce3e8efc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7fbce3e8f09d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7fbce6560fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7fbce67479c8 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId, ()>>
  15:     0x7fbce6802498 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_well_formed, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7fbce6383160 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_well_formed
  17:     0x7fbce520a818 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ForeignItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  18:     0x7fbce50c00d6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  19:     0x7fbce52aed6d - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir::ItemId], <rustc_middle[cc0ab085cda1518a]::hir::ModuleItems>::par_items<rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  20:     0x7fbce521f70a - rustc_hir_analysis[b021cfaa7a998ff9]::check::wfcheck::check_mod_type_wf
  21:     0x7fbce675a511 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, ()>>
  22:     0x7fbce6802378 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::check_mod_type_wf, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  23:     0x7fbce6357e90 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::check_mod_type_wf
  24:     0x7fbce520a9c8 - <core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once
  25:     0x7fbce50c00f6 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  26:     0x7fbce52aeedd - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_hir[4f921811ff0d42d2]::hir_id::OwnerId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  27:     0x7fbce516681d - <rustc_session[a2820a0c6534fd46]::session::Session>::track_errors::<rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#5}, ()>
  28:     0x7fbce50e4513 - rustc_hir_analysis[b021cfaa7a998ff9]::check_crate
  29:     0x7fbce4a780d1 - rustc_interface[9ae183de8b2774b4]::passes::analysis
  30:     0x7fbce671d823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  31:     0x7fbce684b8b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  32:     0x7fbce6330b5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  33:     0x7fbce49871ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  34:     0x7fbce4993276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  35:     0x7fbce492ae6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  36:     0x7fbce49860ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  37:     0x7fbce4948989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  38:     0x7fbce4987806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  39:     0x7fbce493afa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7fbce3eeb8ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  41:     0x7fbce3c80b43 - <unknown>
  42:     0x7fbce3d12a00 - <unknown>
  43:                0x0 - <unknown>

/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x94bce3)[0x7fbce4932ce3]
/lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7fbce3c2e520]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xb028c6)[0x7fbce4ae98c6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs5_Csasxc8sCYFdb_12rustc_errorsNtB5_12HandlerInner15emit_diagnostic+0x328)[0x7fbce79d7368]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs4_Csasxc8sCYFdb_12rustc_errorsNtB5_7Handler15emit_diagnostic+0x21)[0x7fbce79d6281]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsl0LieaIGlMN_12rustc_driver10report_ice+0x16f)[0x7fbce49298cf]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN3std9panicking20rust_panic_with_hook17hab0b5a9b7152bbecE+0x274)[0x7fbce3edead4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc57fa)[0x7fbce3ede7fa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc2694)[0x7fbce3edb694]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_begin_unwind+0x72)[0x7fbce3ede4e2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking9panic_fmt17h775c62c34b432a2dE+0x33)[0x7fbce3e8efc3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking5panic17hbb25aa978ab51c1fE+0x4d)[0x7fbce3e8f09d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvXs0_NtCslEjRGzJr5QU_16rustc_query_impl8plumbingNtB5_9QueryCtxtNtNtCs3PKljwG4Wbl_18rustc_query_system5query12QueryContext17current_query_job+0x55)[0x7fbce6560fe5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x27609c8)[0x7fbce67479c8]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x281b498)[0x7fbce6802498]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x239c160)[0x7fbce6383160]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1223818)[0x7fbce520a818]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x10d90d6)[0x7fbce50c00d6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x12c7d6d)[0x7fbce52aed6d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x123870a)[0x7fbce521f70a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2773511)[0x7fbce675a511]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x281b378)[0x7fbce6802378]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2370e90)[0x7fbce6357e90]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x12239c8)[0x7fbce520a9c8]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x10d90f6)[0x7fbce50c00f6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x12c7edd)[0x7fbce52aeedd]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x117f81d)[0x7fbce516681d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsf7xUi6oyiPB_18rustc_hir_analysis11check_crate+0xf3)[0x7fbce50e4513]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvNtCsdiqvhTzVGrU_15rustc_interface6passes8analysis+0x61)[0x7fbce4a780d1]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2736823)[0x7fbce671d823]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x28648b4)[0x7fbce684b8b4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2349b5a)[0x7fbce6330b5a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9a01ec)[0x7fbce49871ec]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9ac276)[0x7fbce4993276]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x943e6c)[0x7fbce492ae6c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x99f0ca)[0x7fbce49860ca]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x961989)[0x7fbce4948989]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9a0806)[0x7fbce4987806]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x953fa5)[0x7fbce493afa5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xd28ce)[0x7fbce3eeb8ce]
/lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7fbce3c80b43]
/lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7fbce3d12a00]


---- [ui] src/test/ui/traits/mutual-recursion-issue-75860.rs stdout ----


error: Error: expected failure status (Some(1)) but received status None.
status: signal: 11 (SIGSEGV) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/mutual-recursion-issue-75860.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/mutual-recursion-issue-75860" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/mutual-recursion-issue-75860/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `Option<_>: Sized`
   |
LL |     iso(left, right)
   |     ^^^
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`mutual_recursion_issue_75860`)
note: required by a bound in `Option`
  --> /rustc/FAKE_PREFIX/library/core/src/option.rs:562:1

thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
stack backtrace:
   0:     0x7fcd8431e175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7fcd8438e1e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7fcd84310291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7fcd8431df81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7fcd84321364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7fcd8432102a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7fcd84d6c6c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fcd84321ad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7fcd843217fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7fcd8431e694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7fcd843214e2 - rust_begin_unwind
  11:     0x7fcd842d1fc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7fcd842d209d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7fcd869a3fe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7fcd86b98874 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::VecCache<rustc_span[755eca102fd61de6]::def_id::LocalDefId, &rustc_middle[cc0ab085cda1518a]::ty::typeck_results::TypeckResults>>
  15:     0x7fcd86c8e4fb - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::typeck, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7fcd8679cc30 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::typeck
  17:     0x7fcd8533e144 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<&[rustc_span[755eca102fd61de6]::def_id::LocalDefId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_body_owners<rustc_hir_typeck[f001529be8c0c5]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  18:     0x7fcd8534022d - rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in::<&[rustc_span[755eca102fd61de6]::def_id::LocalDefId], <rustc_middle[cc0ab085cda1518a]::hir::map::Map>::par_body_owners<rustc_hir_typeck[f001529be8c0c5]::typeck_item_bodies::{closure#0}>::{closure#0}>
  19:     0x7fcd853cd9c7 - rustc_hir_typeck[f001529be8c0c5]::typeck_item_bodies
  20:     0x7fcd86b6f2f4 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), ()>>
  21:     0x7fcd86c4ec04 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::typeck_item_bodies, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  22:     0x7fcd8679c4ca - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::typeck_item_bodies
  23:     0x7fcd855a718b - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_hir_analysis[b021cfaa7a998ff9]::check_crate::{closure#7}>
  24:     0x7fcd855275a3 - rustc_hir_analysis[b021cfaa7a998ff9]::check_crate
  25:     0x7fcd84ebb0d1 - rustc_interface[9ae183de8b2774b4]::passes::analysis
  26:     0x7fcd86b60823 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>>
  27:     0x7fcd86c8e8b4 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::analysis, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  28:     0x7fcd86773b5a - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::analysis
  29:     0x7fcd84dca1ec - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  30:     0x7fcd84dd6276 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  31:     0x7fcd84d6de6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  32:     0x7fcd84dc90ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  33:     0x7fcd84d8b989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  34:     0x7fcd84dca806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  35:     0x7fcd84d7dfa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fcd8432e8ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  37:     0x7fcd840c3b43 - <unknown>
  38:     0x7fcd84155a00 - <unknown>
  39:                0x0 - <unknown>

/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x94bce3)[0x7fcd84d75ce3]
/lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7fcd84071520]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xb028c6)[0x7fcd84f2c8c6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs5_Csasxc8sCYFdb_12rustc_errorsNtB5_12HandlerInner15emit_diagnostic+0x328)[0x7fcd87e1a368]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvMs4_Csasxc8sCYFdb_12rustc_errorsNtB5_7Handler15emit_diagnostic+0x21)[0x7fcd87e19281]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsl0LieaIGlMN_12rustc_driver10report_ice+0x16f)[0x7fcd84d6c8cf]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN3std9panicking20rust_panic_with_hook17hab0b5a9b7152bbecE+0x274)[0x7fcd84321ad4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc57fa)[0x7fcd843217fa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc2694)[0x7fcd8431e694]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_begin_unwind+0x72)[0x7fcd843214e2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking9panic_fmt17h775c62c34b432a2dE+0x33)[0x7fcd842d1fc3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking5panic17hbb25aa978ab51c1fE+0x4d)[0x7fcd842d209d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvXs0_NtCslEjRGzJr5QU_16rustc_query_impl8plumbingNtB5_9QueryCtxtNtNtCs3PKljwG4Wbl_18rustc_query_system5query12QueryContext17current_query_job+0x55)[0x7fcd869a3fe5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x276e874)[0x7fcd86b98874]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x28644fb)[0x7fcd86c8e4fb]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2372c30)[0x7fcd8679cc30]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xf14144)[0x7fcd8533e144]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xf1622d)[0x7fcd8534022d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xfa39c7)[0x7fcd853cd9c7]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x27452f4)[0x7fcd86b6f2f4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2824c04)[0x7fcd86c4ec04]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x23724ca)[0x7fcd8679c4ca]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x117d18b)[0x7fcd855a718b]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsf7xUi6oyiPB_18rustc_hir_analysis11check_crate+0x183)[0x7fcd855275a3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvNtCsdiqvhTzVGrU_15rustc_interface6passes8analysis+0x61)[0x7fcd84ebb0d1]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2736823)[0x7fcd86b60823]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x28648b4)[0x7fcd86c8e8b4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2349b5a)[0x7fcd86773b5a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9a01ec)[0x7fcd84dca1ec]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9ac276)[0x7fcd84dd6276]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x943e6c)[0x7fcd84d6de6c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x99f0ca)[0x7fcd84dc90ca]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x961989)[0x7fcd84d8b989]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x9a0806)[0x7fcd84dca806]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x953fa5)[0x7fcd84d7dfa5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xd28ce)[0x7fcd8432e8ce]
/lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7fcd840c3b43]
/lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7fcd84155a00]


---- [ui] src/test/ui/traits/issue-91949-hangs-on-recursion.rs stdout ----


error: Error: expected failure status (Some(1)) but received status None.
status: signal: 11 (SIGSEGV) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-91949-hangs-on-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-91949-hangs-on-recursion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-91949-hangs-on-recursion/auxiliary" "-Zinline-mir=no"
stdout: none
--- stderr -------------------------------
warning: function cannot return without recursing
   |
   |
LL | / fn recurse<T>(elements: T) -> Vec<char>
LL | | where
LL | |     T: Iterator<Item = ()>,
   | |___________________________^ cannot return without recursing
LL |   {
LL |       recurse(IteratorOfWrapped(elements).map(|t| t.0))
   |       ------------------------------------------------- recursive call site
   |
   = help: a `loop` may express intention better if this is on purpose
   = note: `#[warn(unconditional_recursion)]` on by default

error[E0275]: overflow evaluating the requirement `(): Sized`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "512"]` attribute to your crate (`issue_91949_hangs_on_recursion`)
   = note: required for `std::iter::Empty<()>` to implement `Iterator`
   = note: 171 redundant requirements hidden
   = note: required for `IteratorOfWrapped<(), Map<IteratorOfWrapped<(), Map<IteratorOfWrapped<(), Map<..., ...>>, ...>>, ...>>` to implement `Iterator`
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-91949-hangs-on-recursion/issue-91949-hangs-on-recursion.long-type-14699345298625251075.txt'

thread 'rustc' panicked at 'assertion failed: ptr_eq(context.tcx.gcx, tcx.gcx)', /checkout/compiler/rustc_middle/src/ty/context.rs:1385:13
   0:     0x7faec1577175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   0:     0x7faec1577175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd648b3f50d029b41
   1:     0x7faec15e71e8 - core::fmt::write::h14c8e176bb50a85a
   2:     0x7faec1569291 - std::io::Write::write_fmt::h84dc0a9dcd908589
   3:     0x7faec1576f81 - std::sys_common::backtrace::print::hc5b9308644794758
   4:     0x7faec157a364 - std::panicking::default_hook::{{closure}}::he027b9b1ad5e1735
   5:     0x7faec157a02a - std::panicking::default_hook::h4a8f10339377e18c
   6:     0x7faec1fc56c2 - rustc_driver[f4be4875f4d52ad7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7faec157aad4 - std::panicking::rust_panic_with_hook::hab0b5a9b7152bbec
   8:     0x7faec157a7fa - std::panicking::begin_panic_handler::{{closure}}::ha1f34c86bd321c28
   9:     0x7faec1577694 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b862906d88d53e4
  10:     0x7faec157a4e2 - rust_begin_unwind
  11:     0x7faec152afc3 - core::panicking::panic_fmt::h775c62c34b432a2d
  12:     0x7faec152b09d - core::panicking::panic::hbb25aa978ab51c1f
  13:     0x7faec3bfcfe5 - <rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt as rustc_query_system[2caa153941c85269]::query::QueryContext>::current_query_job
  14:     0x7faec3db2ce5 - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<rustc_span[755eca102fd61de6]::def_id::DefId, bool>>
  15:     0x7faec3e931ec - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::is_foreign_item, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  16:     0x7faec39e4955 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::is_foreign_item
  17:     0x7faec2992aaa - rustc_monomorphize[d88e042efd1bf97]::collector::should_codegen_locally
  18:     0x7faec2997291 - rustc_monomorphize[d88e042efd1bf97]::collector::collect_items_rec
  19:     0x7faec29aeaf4 - std[5ac0ced0274dd2d1]::panicking::try::<(), core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f86d1f5f016714cf]::sync::par_for_each_in<alloc[75b9de5db4717345]::vec::Vec<rustc_middle[cc0ab085cda1518a]::mir::mono::MonoItem>, rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  20:     0x7faec29d87d4 - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<(), rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items::{closure#1}>
  21:     0x7faec2994376 - rustc_monomorphize[d88e042efd1bf97]::collector::collect_crate_mono_items
  22:     0x7faec29a6d76 - rustc_monomorphize[d88e042efd1bf97]::partitioning::collect_and_partition_mono_items
  23:     0x7faec3dc716a - rustc_query_system[2caa153941c85269]::query::plumbing::try_execute_query::<rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_query_system[2caa153941c85269]::query::caches::DefaultCache<(), (&std[5ac0ced0274dd2d1]::collections::hash::set::HashSet<rustc_span[755eca102fd61de6]::def_id::DefId, core[e5930317b0d12ce1]::hash::BuildHasherDefault<rustc_hash[ff8b1808a785fb13]::FxHasher>>, &[rustc_middle[cc0ab085cda1518a]::mir::mono::CodegenUnit])>>
  24:     0x7faec3eddac9 - rustc_query_system[2caa153941c85269]::query::plumbing::get_query::<rustc_query_impl[fc2cbe26f209b17e]::queries::collect_and_partition_mono_items, rustc_query_impl[fc2cbe26f209b17e]::plumbing::QueryCtxt, rustc_middle[cc0ab085cda1518a]::dep_graph::dep_node::DepKind>
  25:     0x7faec3a38998 - <rustc_query_impl[fc2cbe26f209b17e]::Queries as rustc_middle[cc0ab085cda1518a]::ty::query::QueryEngine>::collect_and_partition_mono_items
  26:     0x7faec2294021 - rustc_codegen_ssa[efa17c4ad961d364]::base::codegen_crate::<rustc_codegen_llvm[3de13d325ebb1d9f]::LlvmCodegenBackend>
  27:     0x7faec2366557 - <rustc_codegen_llvm[3de13d325ebb1d9f]::LlvmCodegenBackend as rustc_codegen_ssa[efa17c4ad961d364]::traits::backend::CodegenBackend>::codegen_crate
  28:     0x7faec211498f - <rustc_session[a2820a0c6534fd46]::session::Session>::time::<alloc[75b9de5db4717345]::boxed::Box<dyn core[e5930317b0d12ce1]::any::Any>, rustc_interface[9ae183de8b2774b4]::passes::start_codegen::{closure#0}>
  29:     0x7faec21143f8 - rustc_interface[9ae183de8b2774b4]::passes::start_codegen
  30:     0x7faec21123a6 - <rustc_interface[9ae183de8b2774b4]::passes::QueryContext>::enter::<<rustc_interface[9ae183de8b2774b4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<alloc[75b9de5db4717345]::boxed::Box<dyn core[e5930317b0d12ce1]::any::Any>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  31:     0x7faec218a225 - <rustc_interface[9ae183de8b2774b4]::queries::Queries>::ongoing_codegen
  32:     0x7faec202f2c3 - <rustc_interface[9ae183de8b2774b4]::interface::Compiler>::enter::<rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}::{closure#2}, core[e5930317b0d12ce1]::result::Result<core[e5930317b0d12ce1]::option::Option<rustc_interface[9ae183de8b2774b4]::queries::Linker>, rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  33:     0x7faec1fc6e6c - rustc_span[755eca102fd61de6]::with_source_map::<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  34:     0x7faec20220ca - <scoped_tls[79c893eafc108340]::ScopedKey<rustc_span[755eca102fd61de6]::SessionGlobals>>::set::<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  35:     0x7faec1fe4989 - std[5ac0ced0274dd2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  36:     0x7faec2023806 - std[5ac0ced0274dd2d1]::panic::catch_unwind::<core[e5930317b0d12ce1]::panic::unwind_safe::AssertUnwindSafe<<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>
  37:     0x7faec1fd6fa5 - <<std[5ac0ced0274dd2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[9ae183de8b2774b4]::util::run_in_thread_pool_with_globals<rustc_interface[9ae183de8b2774b4]::interface::run_compiler<core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>, rustc_driver[f4be4875f4d52ad7]::run_compiler::{closure#1}>::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5930317b0d12ce1]::result::Result<(), rustc_errors[79d644afcdca68d7]::ErrorGuaranteed>>::{closure#1} as core[e5930317b0d12ce1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7faec15878ce - std::sys::unix::thread::Thread::new::thread_start::hc26797c06573547a
  39:     0x7faec131cb43 - <unknown>
  40:     0x7faec13aea00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (c4cd43694 2022-12-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z inline-mir=no
query stack during panic:
query stack during panic:
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x94bce3)[0x7faec1fcece3]
/lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7faec12ca520]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvNtCsdiqvhTzVGrU_15rustc_interface9interface21try_print_query_stack+0x6f)[0x7faec20e216f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvCsl0LieaIGlMN_12rustc_driver10report_ice+0xd55)[0x7faec1fc64b5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN3std9panicking20rust_panic_with_hook17hab0b5a9b7152bbecE+0x274)[0x7faec157aad4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc57fa)[0x7faec157a7fa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_metadata_std_5ac0ced0274dd2d1+0xc2694)[0x7faec1577694]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(rust_begin_unwind+0x72)[0x7faec157a4e2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking9panic_fmt17h775c62c34b432a2dE+0x33)[0x7faec152afc3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-94494efee446a1e1.so(_ZN4core9panicking5panic17hbb25aa978ab51c1fE+0x4d)[0x7faec152b09d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvXs0_NtCslEjRGzJr5QU_16rustc_query_impl8plumbingNtB5_9QueryCtxtNtNtCs3PKljwG4Wbl_18rustc_query_system5query12QueryContext17current_query_job+0x55)[0x7faec3bfcfe5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x272fce5)[0x7faec3db2ce5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x28101ec)[0x7faec3e931ec]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x2361955)[0x7faec39e4955]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x130faaa)[0x7faec2992aaa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1314291)[0x7faec2997291]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x132baf4)[0x7faec29aeaf4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x13557d4)[0x7faec29d87d4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1311376)[0x7faec2994376]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x1323d76)[0x7faec29a6d76]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x274416a)[0x7faec3dc716a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x285aac9)[0x7faec3eddac9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0x23b5998)[0x7faec3a38998]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(+0xc11021)[0x7faec2294021]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-7c240f60853011e4.so(_RNvXs5_Cs5jnRH9gmQqh_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCskzy2TwEqpvc_17rustc_codegen_ssa6traits7backend14CodegenBackend13codegen_crate+0xc7)[0x7faec2366557]
