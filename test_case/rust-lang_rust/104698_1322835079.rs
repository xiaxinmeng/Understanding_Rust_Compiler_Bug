plain
.................i...................................................................... 3256/13883
........................................................................................ 3344/13883
.......................................................................................i 3432/13883
iiii.................................................................................... 3520/13883
..............................................F..FFF...FFFF............................. 3608/13883
........................................................................................ 3784/13883
........................................................................................ 3872/13883
........................................................................................ 3960/13883
...............................................i..........i..........i.................. 4048/13883
---
---- [ui] src/test/ui/dyn-star/box.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/box/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/box/auxiliary" "-C" "opt-level=0"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'compute_symbol_name: `_RNSNvYD*NtNtCslOdBNxM6kjx_4core3fmt7DisplayEL_B6_3fmt5reifyCsfp0ikrovvI6_3box` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:270:5
   0:     0x7f97281ecbfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aeaa56d090e9ab9
   1:     0x7f972825c608 - core::fmt::write::h2490a8ab4a12c52c
   2:     0x7f97281de9c1 - std::io::Write::write_fmt::h2be4f304211cf72f
   3:     0x7f97281eca01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   3:     0x7f97281eca01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   4:     0x7f97281efd64 - std::panicking::default_hook::{{closure}}::hf69241d7d2047960
   5:     0x7f97281efa29 - std::panicking::default_hook::h868c757edf60ac4c
   6:     0x7f9728c2f434 - rustc_driver[8e9e515da644f252]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f97281f04b4 - std::panicking::rust_panic_with_hook::h7849717bc638f116
   8:     0x7f97281f0217 - std::panicking::begin_panic_handler::{{closure}}::h81e29c4a2d1ece5a
   9:     0x7f97281ed134 - std::sys_common::backtrace::__rust_end_short_backtrace::h41e40edd0dad5c05
  10:     0x7f97281efee2 - rust_begin_unwind
  11:     0x7f97281a0f83 - core::panicking::panic_fmt::hcc69528c6f8ff6df
  12:     0x7f972ace291c - rustc_symbol_mangling[c05fe8d1b64523d5]::symbol_name_provider
  13:     0x7f972aab7e4a - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::symbol_name, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  14:     0x7f972a6805a6 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::symbol_name
  15:     0x7f972ba71c56 - <rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>::symbol_name
  16:     0x7f97296051c9 - <&mut rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0} as core[fe08d2e032e98c7d]::ops::function::FnOnce<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem,)>>::call_once
  17:     0x7f97295f85e3 - <alloc[41f84813ad0978d7]::vec::Vec<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName)> as alloc[41f84813ad0978d7]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName), core[fe08d2e032e98c7d]::iter::adapters::map::Map<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>, rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7f9729607916 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct::<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>
  19:     0x7f9729628c83 - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<(&[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit], ()), rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7f9729607d56 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items
  21:     0x7f972aa2e6ee - rustc_query_system[9b53d02561cb34fe]::query::plumbing::try_execute_query::<rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt, rustc_query_system[9b53d02561cb34fe]::query::caches::DefaultCache<(), (&std[f0a5d13aac131859]::collections::hash::set::HashSet<rustc_span[876e10d6949604c1]::def_id::DefId, core[fe08d2e032e98c7d]::hash::BuildHasherDefault<rustc_hash[f01bda883cb38193]::FxHasher>>, &[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit])>>
  22:     0x7f972ab04f30 - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::collect_and_partition_mono_items, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  23:     0x7f972a6b55f8 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::collect_and_partition_mono_items
  24:     0x7f9728edc9f8 - rustc_codegen_ssa[918cf1c24d969c28]::base::codegen_crate::<rustc_codegen_llvm[b1c9d938d58e8979]::LlvmCodegenBackend>
  25:     0x7f9728e6de67 - <rustc_codegen_llvm[b1c9d938d58e8979]::LlvmCodegenBackend as rustc_codegen_ssa[918cf1c24d969c28]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7f9728d5a76f - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_interface[2e7e71dcda075f32]::passes::start_codegen::{closure#0}>
  27:     0x7f9728d910fb - rustc_interface[2e7e71dcda075f32]::passes::start_codegen
  28:     0x7f9728d90710 - <rustc_interface[2e7e71dcda075f32]::passes::QueryContext>::enter::<<rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  29:     0x7f9728d77112 - <rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen
  30:     0x7f9728c9e35a - <rustc_interface[2e7e71dcda075f32]::interface::Compiler>::enter::<rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}::{closure#2}, core[fe08d2e032e98c7d]::result::Result<core[fe08d2e032e98c7d]::option::Option<rustc_interface[2e7e71dcda075f32]::queries::Linker>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  31:     0x7f9728c30b7e - rustc_span[876e10d6949604c1]::with_source_map::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  32:     0x7f9728c90a5c - <scoped_tls[4702e4e5b19ae72a]::ScopedKey<rustc_span[876e10d6949604c1]::SessionGlobals>>::set::<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  33:     0x7f9728c4e1a9 - std[f0a5d13aac131859]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  34:     0x7f9728c92686 - std[f0a5d13aac131859]::panicking::try::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, core[fe08d2e032e98c7d]::panic::unwind_safe::AssertUnwindSafe<<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f9728c3f79a - <<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1} as core[fe08d2e032e98c7d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f97281fcbde - std::sys::unix::thread::Thread::new::thread_start::ha6decd6a9e4610bb
  37:     0x7f9727f92b43 - <unknown>
  38:     0x7f9728024a00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (9c8b16934 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -C opt-level=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `<dyn* core::fmt::Display as core::fmt::Display>::fmt - shim(reify)`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/dyn-star/const.rs stdout ----
---- [ui] src/test/ui/dyn-star/const.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/const/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/const/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCslOdBNxM6kjx_4core3ptr13drop_in_placeD*NtNtB4_3fmt5DebugEL_ECsaLsv8ANlPj4_5const` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:270:5
   0:     0x7f5bcacb4bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aeaa56d090e9ab9
   1:     0x7f5bcad24608 - core::fmt::write::h2490a8ab4a12c52c
   2:     0x7f5bcaca69c1 - std::io::Write::write_fmt::h2be4f304211cf72f
   3:     0x7f5bcacb4a01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   3:     0x7f5bcacb4a01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   4:     0x7f5bcacb7d64 - std::panicking::default_hook::{{closure}}::hf69241d7d2047960
   5:     0x7f5bcacb7a29 - std::panicking::default_hook::h868c757edf60ac4c
   6:     0x7f5bcb6f7434 - rustc_driver[8e9e515da644f252]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5bcacb84b4 - std::panicking::rust_panic_with_hook::h7849717bc638f116
   8:     0x7f5bcacb8217 - std::panicking::begin_panic_handler::{{closure}}::h81e29c4a2d1ece5a
   9:     0x7f5bcacb5134 - std::sys_common::backtrace::__rust_end_short_backtrace::h41e40edd0dad5c05
  10:     0x7f5bcacb7ee2 - rust_begin_unwind
  11:     0x7f5bcac68f83 - core::panicking::panic_fmt::hcc69528c6f8ff6df
  12:     0x7f5bcd7aa91c - rustc_symbol_mangling[c05fe8d1b64523d5]::symbol_name_provider
  13:     0x7f5bcd57fe4a - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::symbol_name, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  14:     0x7f5bcd1485a6 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::symbol_name
  15:     0x7f5bce539c56 - <rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>::symbol_name
  16:     0x7f5bcc0cd1c9 - <&mut rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0} as core[fe08d2e032e98c7d]::ops::function::FnOnce<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem,)>>::call_once
  17:     0x7f5bcc0c05e3 - <alloc[41f84813ad0978d7]::vec::Vec<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName)> as alloc[41f84813ad0978d7]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName), core[fe08d2e032e98c7d]::iter::adapters::map::Map<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>, rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7f5bcc0cf916 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct::<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>
  19:     0x7f5bcc0f0c83 - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<(&[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit], ()), rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7f5bcc0cfd56 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items
  21:     0x7f5bcd4f66ee - rustc_query_system[9b53d02561cb34fe]::query::plumbing::try_execute_query::<rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt, rustc_query_system[9b53d02561cb34fe]::query::caches::DefaultCache<(), (&std[f0a5d13aac131859]::collections::hash::set::HashSet<rustc_span[876e10d6949604c1]::def_id::DefId, core[fe08d2e032e98c7d]::hash::BuildHasherDefault<rustc_hash[f01bda883cb38193]::FxHasher>>, &[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit])>>
  22:     0x7f5bcd5ccf30 - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::collect_and_partition_mono_items, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  23:     0x7f5bcd17d5f8 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::collect_and_partition_mono_items
  24:     0x7f5bcb9a49f8 - rustc_codegen_ssa[918cf1c24d969c28]::base::codegen_crate::<rustc_codegen_llvm[b1c9d938d58e8979]::LlvmCodegenBackend>
  25:     0x7f5bcb935e67 - <rustc_codegen_llvm[b1c9d938d58e8979]::LlvmCodegenBackend as rustc_codegen_ssa[918cf1c24d969c28]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7f5bcb82276f - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_interface[2e7e71dcda075f32]::passes::start_codegen::{closure#0}>
  27:     0x7f5bcb8590fb - rustc_interface[2e7e71dcda075f32]::passes::start_codegen
  28:     0x7f5bcb858710 - <rustc_interface[2e7e71dcda075f32]::passes::QueryContext>::enter::<<rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  29:     0x7f5bcb83f112 - <rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen
  30:     0x7f5bcb76635a - <rustc_interface[2e7e71dcda075f32]::interface::Compiler>::enter::<rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}::{closure#2}, core[fe08d2e032e98c7d]::result::Result<core[fe08d2e032e98c7d]::option::Option<rustc_interface[2e7e71dcda075f32]::queries::Linker>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  31:     0x7f5bcb6f8b7e - rustc_span[876e10d6949604c1]::with_source_map::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  32:     0x7f5bcb758a5c - <scoped_tls[4702e4e5b19ae72a]::ScopedKey<rustc_span[876e10d6949604c1]::SessionGlobals>>::set::<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  33:     0x7f5bcb7161a9 - std[f0a5d13aac131859]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  34:     0x7f5bcb75a686 - std[f0a5d13aac131859]::panicking::try::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, core[fe08d2e032e98c7d]::panic::unwind_safe::AssertUnwindSafe<<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f5bcb70779a - <<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1} as core[fe08d2e032e98c7d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f5bcacc4bde - std::sys::unix::thread::Thread::new::thread_start::ha6decd6a9e4610bb
  37:     0x7f5bcaa5ab43 - <unknown>
  38:     0x7f5bcaaeca00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (9c8b16934 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::ptr::drop_in_place::<dyn* core::fmt::Debug> - shim(Some(dyn* core::fmt::Debug))`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/dyn-star/no-explicit-dyn-star.rs stdout ----
---- [ui] src/test/ui/dyn-star/no-explicit-dyn-star.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/dyn-star/auxiliary/dyn-star-foreign.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/auxiliary/dyn-star-foreign.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-explicit-dyn-star/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-explicit-dyn-star/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function `works_locally` is never used
  --> /checkout/src/test/ui/dyn-star/auxiliary/dyn-star-foreign.rs:8:4
   |
LL | fn works_locally() {
   |
   = note: `#[warn(dead_code)]` on by default


thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCslOdBNxM6kjx_4core3ptr13drop_in_placeD*NtNtB4_3fmt7DisplayEL_ECsd20lmC0NepD_16dyn_star_foreign` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:270:5
stack backtrace:
   0:     0x7f9b44ccebfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aeaa56d090e9ab9
   1:     0x7f9b44d3e608 - core::fmt::write::h2490a8ab4a12c52c
   2:     0x7f9b44cc09c1 - std::io::Write::write_fmt::h2be4f304211cf72f
   3:     0x7f9b44ccea01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   4:     0x7f9b44cd1d64 - std::panicking::default_hook::{{closure}}::hf69241d7d2047960
   5:     0x7f9b44cd1a29 - std::panicking::default_hook::h868c757edf60ac4c
   6:     0x7f9b45711434 - rustc_driver[8e9e515da644f252]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f9b44cd24b4 - std::panicking::rust_panic_with_hook::h7849717bc638f116
   8:     0x7f9b44cd2217 - std::panicking::begin_panic_handler::{{closure}}::h81e29c4a2d1ece5a
   9:     0x7f9b44ccf134 - std::sys_common::backtrace::__rust_end_short_backtrace::h41e40edd0dad5c05
  10:     0x7f9b44cd1ee2 - rust_begin_unwind
  11:     0x7f9b44c82f83 - core::panicking::panic_fmt::hcc69528c6f8ff6df
  12:     0x7f9b477c491c - rustc_symbol_mangling[c05fe8d1b64523d5]::symbol_name_provider
  13:     0x7f9b47599e4a - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::symbol_name, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  14:     0x7f9b471625a6 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::symbol_name
  15:     0x7f9b48553c56 - <rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>::symbol_name
  16:     0x7f9b460e71c9 - <&mut rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0} as core[fe08d2e032e98c7d]::ops::function::FnOnce<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem,)>>::call_once
  17:     0x7f9b460da5e3 - <alloc[41f84813ad0978d7]::vec::Vec<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName)> as alloc[41f84813ad0978d7]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName), core[fe08d2e032e98c7d]::iter::adapters::map::Map<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>, rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7f9b460e9916 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct::<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>
  19:     0x7f9b4610ac83 - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<(&[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit], ()), rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7f9b460e9d56 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items
  21:     0x7f9b475106ee - rustc_query_system[9b53d02561cb34fe]::query::plumbing::try_execute_query::<rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt, rustc_query_system[9b53d02561cb34fe]::query::caches::DefaultCache<(), (&std[f0a5d13aac131859]::collections::hash::set::HashSet<rustc_span[876e10d6949604c1]::def_id::DefId, core[fe08d2e032e98c7d]::hash::BuildHasherDefault<rustc_hash[f01bda883cb38193]::FxHasher>>, &[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit])>>
  22:     0x7f9b475e6f30 - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::collect_and_partition_mono_items, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  23:     0x7f9b471975f8 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::collect_and_partition_mono_items
  24:     0x7f9b476ec281 - rustc_codegen_ssa[918cf1c24d969c28]::back::symbol_export::exported_symbols_provider_local
  25:     0x7f9b474f9b2c - rustc_query_system[9b53d02561cb34fe]::query::plumbing::try_execute_query::<rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt, rustc_query_system[9b53d02561cb34fe]::query::caches::DefaultCache<rustc_span[876e10d6949604c1]::def_id::CrateNum, &[(rustc_middle[34ad2455d841ea9f]::middle::exported_symbols::ExportedSymbol, rustc_middle[34ad2455d841ea9f]::middle::exported_symbols::SymbolExportInfo)]>>
  26:     0x7f9b475a9aec - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::exported_symbols, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  27:     0x7f9b47196e7e - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::exported_symbols
  28:     0x7f9b479c62d0 - <rustc_metadata[d95911049bc96afc]::rmeta::encoder::EncodeContext>::encode_crate_root
  29:     0x7f9b479d8963 - rustc_metadata[d95911049bc96afc]::rmeta::encoder::encode_metadata_impl
  30:     0x7f9b479def74 - rustc_data_structures[c9445ef981c0b573]::sync::join::<rustc_metadata[d95911049bc96afc]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[d95911049bc96afc]::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
  31:     0x7f9b479d7ec5 - rustc_metadata[d95911049bc96afc]::rmeta::encoder::encode_metadata
  32:     0x7f9b47a282ac - rustc_metadata[d95911049bc96afc]::fs::encode_and_write_metadata
  33:     0x7f9b45873091 - rustc_interface[2e7e71dcda075f32]::passes::start_codegen
  34:     0x7f9b45872710 - <rustc_interface[2e7e71dcda075f32]::passes::QueryContext>::enter::<<rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  35:     0x7f9b45859112 - <rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen
  36:     0x7f9b4578035a - <rustc_interface[2e7e71dcda075f32]::interface::Compiler>::enter::<rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}::{closure#2}, core[fe08d2e032e98c7d]::result::Result<core[fe08d2e032e98c7d]::option::Option<rustc_interface[2e7e71dcda075f32]::queries::Linker>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  37:     0x7f9b45712b7e - rustc_span[876e10d6949604c1]::with_source_map::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  38:     0x7f9b45772a5c - <scoped_tls[4702e4e5b19ae72a]::ScopedKey<rustc_span[876e10d6949604c1]::SessionGlobals>>::set::<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  39:     0x7f9b457301a9 - std[f0a5d13aac131859]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  40:     0x7f9b45774686 - std[f0a5d13aac131859]::panicking::try::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, core[fe08d2e032e98c7d]::panic::unwind_safe::AssertUnwindSafe<<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7f9b4572179a - <<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1} as core[fe08d2e032e98c7d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7f9b44cdebde - std::sys::unix::thread::Thread::new::thread_start::ha6decd6a9e4610bb
  43:     0x7f9b44a74b43 - <unknown>
  44:     0x7f9b44b06a00 - <unknown>
  45:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (9c8b16934 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::ptr::drop_in_place::<dyn* core::fmt::Display> - shim(Some(dyn* core::fmt::Display))`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
#2 [exported_symbols] collecting exported symbols for crate `0`
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dyn-star/no-implicit-dyn-star.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/dyn-star/auxiliary/dyn-star-foreign.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/auxiliary/dyn-star-foreign.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-implicit-dyn-star/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-implicit-dyn-star/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function `works_locally` is never used
  --> /checkout/src/test/ui/dyn-star/auxiliary/dyn-star-foreign.rs:8:4
   |
LL | fn works_locally() {
   |
   = note: `#[warn(dead_code)]` on by default


thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCslOdBNxM6kjx_4core3ptr13drop_in_placeD*NtNtB4_3fmt7DisplayEL_ECsd20lmC0NepD_16dyn_star_foreign` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:270:5
   0:     0x7f48decdcbfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aeaa56d090e9ab9
   1:     0x7f48ded4c608 - core::fmt::write::h2490a8ab4a12c52c
   2:     0x7f48decce9c1 - std::io::Write::write_fmt::h2be4f304211cf72f
   3:     0x7f48decdca01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   3:     0x7f48decdca01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   4:     0x7f48decdfd64 - std::panicking::default_hook::{{closure}}::hf69241d7d2047960
   5:     0x7f48decdfa29 - std::panicking::default_hook::h868c757edf60ac4c
   6:     0x7f48df71f434 - rustc_driver[8e9e515da644f252]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f48dece04b4 - std::panicking::rust_panic_with_hook::h7849717bc638f116
   8:     0x7f48dece0217 - std::panicking::begin_panic_handler::{{closure}}::h81e29c4a2d1ece5a
   9:     0x7f48decdd134 - std::sys_common::backtrace::__rust_end_short_backtrace::h41e40edd0dad5c05
  10:     0x7f48decdfee2 - rust_begin_unwind
  11:     0x7f48dec90f83 - core::panicking::panic_fmt::hcc69528c6f8ff6df
  12:     0x7f48e17d291c - rustc_symbol_mangling[c05fe8d1b64523d5]::symbol_name_provider
  13:     0x7f48e15a7e4a - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::symbol_name, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  14:     0x7f48e11705a6 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::symbol_name
  15:     0x7f48e2561c56 - <rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>::symbol_name
  16:     0x7f48e00f51c9 - <&mut rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0} as core[fe08d2e032e98c7d]::ops::function::FnOnce<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem,)>>::call_once
  17:     0x7f48e00e84d6 - <alloc[41f84813ad0978d7]::vec::Vec<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName)> as alloc[41f84813ad0978d7]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName), core[fe08d2e032e98c7d]::iter::adapters::map::Map<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>, rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7f48e00f7916 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct::<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>
  19:     0x7f48e0118c83 - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<(&[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit], ()), rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7f48e00f7d56 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items
  21:     0x7f48e151e6ee - rustc_query_system[9b53d02561cb34fe]::query::plumbing::try_execute_query::<rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt, rustc_query_system[9b53d02561cb34fe]::query::caches::DefaultCache<(), (&std[f0a5d13aac131859]::collections::hash::set::HashSet<rustc_span[876e10d6949604c1]::def_id::DefId, core[fe08d2e032e98c7d]::hash::BuildHasherDefault<rustc_hash[f01bda883cb38193]::FxHasher>>, &[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit])>>
  22:     0x7f48e15f4f30 - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::collect_and_partition_mono_items, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  23:     0x7f48e11a55f8 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::collect_and_partition_mono_items
  24:     0x7f48e16fa281 - rustc_codegen_ssa[918cf1c24d969c28]::back::symbol_export::exported_symbols_provider_local
  25:     0x7f48e1507b2c - rustc_query_system[9b53d02561cb34fe]::query::plumbing::try_execute_query::<rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt, rustc_query_system[9b53d02561cb34fe]::query::caches::DefaultCache<rustc_span[876e10d6949604c1]::def_id::CrateNum, &[(rustc_middle[34ad2455d841ea9f]::middle::exported_symbols::ExportedSymbol, rustc_middle[34ad2455d841ea9f]::middle::exported_symbols::SymbolExportInfo)]>>
  26:     0x7f48e15b7aec - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::exported_symbols, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  27:     0x7f48e11a4e7e - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::exported_symbols
  28:     0x7f48e19d42d0 - <rustc_metadata[d95911049bc96afc]::rmeta::encoder::EncodeContext>::encode_crate_root
  29:     0x7f48e19e6963 - rustc_metadata[d95911049bc96afc]::rmeta::encoder::encode_metadata_impl
  30:     0x7f48e19ecf74 - rustc_data_structures[c9445ef981c0b573]::sync::join::<rustc_metadata[d95911049bc96afc]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[d95911049bc96afc]::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
  31:     0x7f48e19e5ec5 - rustc_metadata[d95911049bc96afc]::rmeta::encoder::encode_metadata
  32:     0x7f48e1a362ac - rustc_metadata[d95911049bc96afc]::fs::encode_and_write_metadata
  33:     0x7f48df881091 - rustc_interface[2e7e71dcda075f32]::passes::start_codegen
  34:     0x7f48df880710 - <rustc_interface[2e7e71dcda075f32]::passes::QueryContext>::enter::<<rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  35:     0x7f48df867112 - <rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen
  36:     0x7f48df78e35a - <rustc_interface[2e7e71dcda075f32]::interface::Compiler>::enter::<rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}::{closure#2}, core[fe08d2e032e98c7d]::result::Result<core[fe08d2e032e98c7d]::option::Option<rustc_interface[2e7e71dcda075f32]::queries::Linker>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  37:     0x7f48df720b7e - rustc_span[876e10d6949604c1]::with_source_map::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  38:     0x7f48df780a5c - <scoped_tls[4702e4e5b19ae72a]::ScopedKey<rustc_span[876e10d6949604c1]::SessionGlobals>>::set::<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  39:     0x7f48df73e1a9 - std[f0a5d13aac131859]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  40:     0x7f48df782686 - std[f0a5d13aac131859]::panicking::try::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, core[fe08d2e032e98c7d]::panic::unwind_safe::AssertUnwindSafe<<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7f48df72f79a - <<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1} as core[fe08d2e032e98c7d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7f48dececbde - std::sys::unix::thread::Thread::new::thread_start::ha6decd6a9e4610bb
  43:     0x7f48dea82b43 - <unknown>
  44:     0x7f48deb14a00 - <unknown>
  45:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (9c8b16934 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::ptr::drop_in_place::<dyn* core::fmt::Display> - shim(Some(dyn* core::fmt::Display))`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
#2 [exported_symbols] collecting exported symbols for crate `0`
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dyn-star/make-dyn-star.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/make-dyn-star.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/make-dyn-star/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/make-dyn-star/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCslOdBNxM6kjx_4core3ptr13drop_in_placeD*NtNtB4_3fmt5DebugEL_ECsbcIYje1oVNH_13make_dyn_star` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:270:5
   0:     0x7f1d562d1bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aeaa56d090e9ab9
   1:     0x7f1d56341608 - core::fmt::write::h2490a8ab4a12c52c
   2:     0x7f1d562c39c1 - std::io::Write::write_fmt::h2be4f304211cf72f
   3:     0x7f1d562d1a01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   3:     0x7f1d562d1a01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   4:     0x7f1d562d4d64 - std::panicking::default_hook::{{closure}}::hf69241d7d2047960
   5:     0x7f1d562d4a29 - std::panicking::default_hook::h868c757edf60ac4c
   6:     0x7f1d56d14434 - rustc_driver[8e9e515da644f252]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1d562d54b4 - std::panicking::rust_panic_with_hook::h7849717bc638f116
   8:     0x7f1d562d5217 - std::panicking::begin_panic_handler::{{closure}}::h81e29c4a2d1ece5a
   9:     0x7f1d562d2134 - std::sys_common::backtrace::__rust_end_short_backtrace::h41e40edd0dad5c05
  10:     0x7f1d562d4ee2 - rust_begin_unwind
  11:     0x7f1d56285f83 - core::panicking::panic_fmt::hcc69528c6f8ff6df
  12:     0x7f1d58dc791c - rustc_symbol_mangling[c05fe8d1b64523d5]::symbol_name_provider
  13:     0x7f1d58b9ce4a - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::symbol_name, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  14:     0x7f1d587655a6 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::symbol_name
  15:     0x7f1d59b56c56 - <rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>::symbol_name
  16:     0x7f1d576ea1c9 - <&mut rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0} as core[fe08d2e032e98c7d]::ops::function::FnOnce<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem,)>>::call_once
  17:     0x7f1d576dd4d6 - <alloc[41f84813ad0978d7]::vec::Vec<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName)> as alloc[41f84813ad0978d7]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName), core[fe08d2e032e98c7d]::iter::adapters::map::Map<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>, rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7f1d576ec916 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct::<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>
  19:     0x7f1d5770dc83 - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<(&[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit], ()), rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7f1d576ecd56 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items
  21:     0x7f1d58b136ee - rustc_query_system[9b53d02561cb34fe]::query::plumbing::try_execute_query::<rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt, rustc_query_system[9b53d02561cb34fe]::query::caches::DefaultCache<(), (&std[f0a5d13aac131859]::collections::hash::set::HashSet<rustc_span[876e10d6949604c1]::def_id::DefId, core[fe08d2e032e98c7d]::hash::BuildHasherDefault<rustc_hash[f01bda883cb38193]::FxHasher>>, &[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit])>>
  22:     0x7f1d58be9f30 - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::collect_and_partition_mono_items, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  23:     0x7f1d5879a5f8 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::collect_and_partition_mono_items
  24:     0x7f1d56fc19f8 - rustc_codegen_ssa[918cf1c24d969c28]::base::codegen_crate::<rustc_codegen_llvm[b1c9d938d58e8979]::LlvmCodegenBackend>
  25:     0x7f1d56f52e67 - <rustc_codegen_llvm[b1c9d938d58e8979]::LlvmCodegenBackend as rustc_codegen_ssa[918cf1c24d969c28]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7f1d56e3f76f - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_interface[2e7e71dcda075f32]::passes::start_codegen::{closure#0}>
  27:     0x7f1d56e760fb - rustc_interface[2e7e71dcda075f32]::passes::start_codegen
  28:     0x7f1d56e75710 - <rustc_interface[2e7e71dcda075f32]::passes::QueryContext>::enter::<<rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  29:     0x7f1d56e5c112 - <rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen
  30:     0x7f1d56d8335a - <rustc_interface[2e7e71dcda075f32]::interface::Compiler>::enter::<rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}::{closure#2}, core[fe08d2e032e98c7d]::result::Result<core[fe08d2e032e98c7d]::option::Option<rustc_interface[2e7e71dcda075f32]::queries::Linker>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  31:     0x7f1d56d15b7e - rustc_span[876e10d6949604c1]::with_source_map::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  32:     0x7f1d56d75a5c - <scoped_tls[4702e4e5b19ae72a]::ScopedKey<rustc_span[876e10d6949604c1]::SessionGlobals>>::set::<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  33:     0x7f1d56d331a9 - std[f0a5d13aac131859]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  34:     0x7f1d56d77686 - std[f0a5d13aac131859]::panicking::try::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, core[fe08d2e032e98c7d]::panic::unwind_safe::AssertUnwindSafe<<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f1d56d2479a - <<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1} as core[fe08d2e032e98c7d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f1d562e1bde - std::sys::unix::thread::Thread::new::thread_start::ha6decd6a9e4610bb
  37:     0x7f1d56077b43 - <unknown>
  38:     0x7f1d56109a00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (9c8b16934 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::ptr::drop_in_place::<dyn* core::fmt::Debug> - shim(Some(dyn* core::fmt::Debug))`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/dyn-star/drop.rs stdout ----
---- [ui] src/test/ui/dyn-star/drop.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/drop/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/drop/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCslOdBNxM6kjx_4core3ptr13drop_in_placeD*NtNtB4_3fmt5DebugEL_ECskRQVQDCFBe7_4drop` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:270:5
   0:     0x7fc175039bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aeaa56d090e9ab9
   1:     0x7fc1750a9608 - core::fmt::write::h2490a8ab4a12c52c
   2:     0x7fc17502b9c1 - std::io::Write::write_fmt::h2be4f304211cf72f
   3:     0x7fc175039a01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   3:     0x7fc175039a01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   4:     0x7fc17503cd64 - std::panicking::default_hook::{{closure}}::hf69241d7d2047960
   5:     0x7fc17503ca29 - std::panicking::default_hook::h868c757edf60ac4c
   6:     0x7fc175a7c434 - rustc_driver[8e9e515da644f252]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc17503d4b4 - std::panicking::rust_panic_with_hook::h7849717bc638f116
   8:     0x7fc17503d217 - std::panicking::begin_panic_handler::{{closure}}::h81e29c4a2d1ece5a
   9:     0x7fc17503a134 - std::sys_common::backtrace::__rust_end_short_backtrace::h41e40edd0dad5c05
  10:     0x7fc17503cee2 - rust_begin_unwind
  11:     0x7fc174fedf83 - core::panicking::panic_fmt::hcc69528c6f8ff6df
  12:     0x7fc177b2f91c - rustc_symbol_mangling[c05fe8d1b64523d5]::symbol_name_provider
  13:     0x7fc177904e4a - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::symbol_name, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  14:     0x7fc1774cd5a6 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::symbol_name
  15:     0x7fc1788bec56 - <rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>::symbol_name
  16:     0x7fc1764521c9 - <&mut rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0} as core[fe08d2e032e98c7d]::ops::function::FnOnce<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem,)>>::call_once
  17:     0x7fc1764455e3 - <alloc[41f84813ad0978d7]::vec::Vec<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName)> as alloc[41f84813ad0978d7]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName), core[fe08d2e032e98c7d]::iter::adapters::map::Map<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>, rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7fc176454916 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct::<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>
  19:     0x7fc176475c83 - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<(&[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit], ()), rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7fc176454d56 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items
  21:     0x7fc17787b6ee - rustc_query_system[9b53d02561cb34fe]::query::plumbing::try_execute_query::<rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt, rustc_query_system[9b53d02561cb34fe]::query::caches::DefaultCache<(), (&std[f0a5d13aac131859]::collections::hash::set::HashSet<rustc_span[876e10d6949604c1]::def_id::DefId, core[fe08d2e032e98c7d]::hash::BuildHasherDefault<rustc_hash[f01bda883cb38193]::FxHasher>>, &[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit])>>
  22:     0x7fc177951f30 - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::collect_and_partition_mono_items, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  23:     0x7fc1775025f8 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::collect_and_partition_mono_items
  24:     0x7fc175d299f8 - rustc_codegen_ssa[918cf1c24d969c28]::base::codegen_crate::<rustc_codegen_llvm[b1c9d938d58e8979]::LlvmCodegenBackend>
  25:     0x7fc175cbae67 - <rustc_codegen_llvm[b1c9d938d58e8979]::LlvmCodegenBackend as rustc_codegen_ssa[918cf1c24d969c28]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7fc175ba776f - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_interface[2e7e71dcda075f32]::passes::start_codegen::{closure#0}>
  27:     0x7fc175bde0fb - rustc_interface[2e7e71dcda075f32]::passes::start_codegen
  28:     0x7fc175bdd710 - <rustc_interface[2e7e71dcda075f32]::passes::QueryContext>::enter::<<rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  29:     0x7fc175bc4112 - <rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen
  30:     0x7fc175aeb35a - <rustc_interface[2e7e71dcda075f32]::interface::Compiler>::enter::<rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}::{closure#2}, core[fe08d2e032e98c7d]::result::Result<core[fe08d2e032e98c7d]::option::Option<rustc_interface[2e7e71dcda075f32]::queries::Linker>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  31:     0x7fc175a7db7e - rustc_span[876e10d6949604c1]::with_source_map::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  32:     0x7fc175adda5c - <scoped_tls[4702e4e5b19ae72a]::ScopedKey<rustc_span[876e10d6949604c1]::SessionGlobals>>::set::<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  33:     0x7fc175a9b1a9 - std[f0a5d13aac131859]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  34:     0x7fc175adf686 - std[f0a5d13aac131859]::panicking::try::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, core[fe08d2e032e98c7d]::panic::unwind_safe::AssertUnwindSafe<<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7fc175a8c79a - <<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1} as core[fe08d2e032e98c7d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fc175049bde - std::sys::unix::thread::Thread::new::thread_start::ha6decd6a9e4610bb
  37:     0x7fc174ddfb43 - <unknown>
  38:     0x7fc174e71a00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (9c8b16934 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::ptr::drop_in_place::<dyn* core::fmt::Debug> - shim(Some(dyn* core::fmt::Debug))`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/dyn-star/method.rs stdout ----
---- [ui] src/test/ui/dyn-star/method.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/method/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/method/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCslOdBNxM6kjx_4core3ptr13drop_in_placeD*NtCseS6erWYsVdg_6method3FooEL_EBK_` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:270:5
   0:     0x7fadf6781bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aeaa56d090e9ab9
   1:     0x7fadf67f1608 - core::fmt::write::h2490a8ab4a12c52c
   2:     0x7fadf67739c1 - std::io::Write::write_fmt::h2be4f304211cf72f
   3:     0x7fadf6781a01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   3:     0x7fadf6781a01 - std::sys_common::backtrace::print::h2e255cf2296e58dd
   4:     0x7fadf6784d64 - std::panicking::default_hook::{{closure}}::hf69241d7d2047960
   5:     0x7fadf6784a29 - std::panicking::default_hook::h868c757edf60ac4c
   6:     0x7fadf71c4434 - rustc_driver[8e9e515da644f252]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fadf67854b4 - std::panicking::rust_panic_with_hook::h7849717bc638f116
   8:     0x7fadf6785217 - std::panicking::begin_panic_handler::{{closure}}::h81e29c4a2d1ece5a
   9:     0x7fadf6782134 - std::sys_common::backtrace::__rust_end_short_backtrace::h41e40edd0dad5c05
  10:     0x7fadf6784ee2 - rust_begin_unwind
  11:     0x7fadf6735f83 - core::panicking::panic_fmt::hcc69528c6f8ff6df
  12:     0x7fadf927791c - rustc_symbol_mangling[c05fe8d1b64523d5]::symbol_name_provider
  13:     0x7fadf904ce4a - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::symbol_name, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  14:     0x7fadf8c155a6 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::symbol_name
  15:     0x7fadfa006c56 - <rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>::symbol_name
  16:     0x7fadf7b9a1c9 - <&mut rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0} as core[fe08d2e032e98c7d]::ops::function::FnOnce<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem,)>>::call_once
  17:     0x7fadf7b8d5e3 - <alloc[41f84813ad0978d7]::vec::Vec<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName)> as alloc[41f84813ad0978d7]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem, rustc_middle[34ad2455d841ea9f]::ty::SymbolName), core[fe08d2e032e98c7d]::iter::adapters::map::Map<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>, rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7fadf7b9c916 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::assert_symbols_are_distinct::<std[f0a5d13aac131859]::collections::hash::set::Iter<rustc_middle[34ad2455d841ea9f]::mir::mono::MonoItem>>
  19:     0x7fadf7bbdc83 - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<(&[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit], ()), rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7fadf7b9cd56 - rustc_monomorphize[17f299ae1d9f44f9]::partitioning::collect_and_partition_mono_items
  21:     0x7fadf8fc36ee - rustc_query_system[9b53d02561cb34fe]::query::plumbing::try_execute_query::<rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt, rustc_query_system[9b53d02561cb34fe]::query::caches::DefaultCache<(), (&std[f0a5d13aac131859]::collections::hash::set::HashSet<rustc_span[876e10d6949604c1]::def_id::DefId, core[fe08d2e032e98c7d]::hash::BuildHasherDefault<rustc_hash[f01bda883cb38193]::FxHasher>>, &[rustc_middle[34ad2455d841ea9f]::mir::mono::CodegenUnit])>>
  22:     0x7fadf9099f30 - rustc_query_system[9b53d02561cb34fe]::query::plumbing::get_query::<rustc_query_impl[9d9f179661dd1faa]::queries::collect_and_partition_mono_items, rustc_query_impl[9d9f179661dd1faa]::plumbing::QueryCtxt>
  23:     0x7fadf8c4a5f8 - <rustc_query_impl[9d9f179661dd1faa]::Queries as rustc_middle[34ad2455d841ea9f]::ty::query::QueryEngine>::collect_and_partition_mono_items
  24:     0x7fadf74719f8 - rustc_codegen_ssa[918cf1c24d969c28]::base::codegen_crate::<rustc_codegen_llvm[b1c9d938d58e8979]::LlvmCodegenBackend>
  25:     0x7fadf7402e67 - <rustc_codegen_llvm[b1c9d938d58e8979]::LlvmCodegenBackend as rustc_codegen_ssa[918cf1c24d969c28]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7fadf72ef76f - <rustc_session[17ba04133c3a20ca]::session::Session>::time::<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_interface[2e7e71dcda075f32]::passes::start_codegen::{closure#0}>
  27:     0x7fadf73260fb - rustc_interface[2e7e71dcda075f32]::passes::start_codegen
  28:     0x7fadf7325710 - <rustc_interface[2e7e71dcda075f32]::passes::QueryContext>::enter::<<rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<alloc[41f84813ad0978d7]::boxed::Box<dyn core[fe08d2e032e98c7d]::any::Any>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  29:     0x7fadf730c112 - <rustc_interface[2e7e71dcda075f32]::queries::Queries>::ongoing_codegen
  30:     0x7fadf723335a - <rustc_interface[2e7e71dcda075f32]::interface::Compiler>::enter::<rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}::{closure#2}, core[fe08d2e032e98c7d]::result::Result<core[fe08d2e032e98c7d]::option::Option<rustc_interface[2e7e71dcda075f32]::queries::Linker>, rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  31:     0x7fadf71c5b7e - rustc_span[876e10d6949604c1]::with_source_map::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  32:     0x7fadf7225a5c - <scoped_tls[4702e4e5b19ae72a]::ScopedKey<rustc_span[876e10d6949604c1]::SessionGlobals>>::set::<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  33:     0x7fadf71e31a9 - std[f0a5d13aac131859]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>
  34:     0x7fadf7227686 - std[f0a5d13aac131859]::panicking::try::<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, core[fe08d2e032e98c7d]::panic::unwind_safe::AssertUnwindSafe<<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7fadf71d479a - <<std[f0a5d13aac131859]::thread::Builder>::spawn_unchecked_<rustc_interface[2e7e71dcda075f32]::util::run_in_thread_pool_with_globals<rustc_interface[2e7e71dcda075f32]::interface::run_compiler<core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>, rustc_driver[8e9e515da644f252]::run_compiler::{closure#1}>::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fe08d2e032e98c7d]::result::Result<(), rustc_errors[ca36a921d33e348a]::ErrorGuaranteed>>::{closure#1} as core[fe08d2e032e98c7d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fadf6791bde - std::sys::unix::thread::Thread::new::thread_start::ha6decd6a9e4610bb
  37:     0x7fadf6527b43 - <unknown>
  38:     0x7fadf65b9a00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (9c8b16934 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::ptr::drop_in_place::<dyn* Foo> - shim(Some(dyn* Foo))`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------

