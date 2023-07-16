plain
........................................................................................  3608/14878
........................................................................................  3696/14878
....................................................................iiiii...............  3784/14878
........................................................................................  3872/14878
.................................F...F..FFF..F..FFFFF...................................  3960/14878
........................................................................................  4136/14878
........................................................................................  4224/14878
........................................................................................  4312/14878
...................................................................i..........i.........  4400/14878
---
---- [ui] tests/ui/dyn-star/box.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/dyn-star/box.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/box/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/box/auxiliary" "-C" "opt-level=0"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'compute_symbol_name: `_RNSNvYD*NtNtCs20tUa0bqYDv_4core3fmt7DisplayEL_B6_3fmt5reifyCskA3p4kQ8llX_3box` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:273:5
   0:     0x7fe94b65d810 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc7211feedd46988
   1:     0x7fe94b6c5138 - core::fmt::write::h57ff60867d8efd5d
   2:     0x7fe94b651ec1 - std::io::Write::write_fmt::h94be5aa5308b7f9d
   3:     0x7fe94b65d621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   3:     0x7fe94b65d621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   4:     0x7fe94b660744 - std::panicking::default_hook::{{closure}}::hba6321717a5218f9
   5:     0x7fe94b66041d - std::panicking::default_hook::h58ef156a39f86bed
   6:     0x7fe94c123035 - rustc_driver_impl[21fcdfcf54a5bd8f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe94b660e61 - std::panicking::rust_panic_with_hook::habf71243a3a021f1
   8:     0x7fe94b660bd9 - std::panicking::begin_panic_handler::{{closure}}::hbf05a4d5d42770d9
   9:     0x7fe94b65dce6 - std::sys_common::backtrace::__rust_end_short_backtrace::h05482969ce085089
  10:     0x7fe94b6608b7 - rust_begin_unwind
  11:     0x7fe94b615113 - core::panicking::panic_fmt::h4d55ca4049c5f88c
  12:     0x7fe94e12e760 - rustc_symbol_mangling[13eb4448e51775d9]::symbol_name_provider
  13:     0x7fe94de3186a - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::symbol_name, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  14:     0x7fe94dd867a7 - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::symbol_name
  15:     0x7fe94ee45dc0 - <rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>::symbol_name
  16:     0x7fe94cb06619 - <&mut rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0} as core[1762c753d0b7efe3]::ops::function::FnOnce<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem,)>>::call_once
  17:     0x7fe94cb2a4a3 - <alloc[5efc3b5c20d9d1d4]::vec::Vec<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName)> as alloc[5efc3b5c20d9d1d4]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName), core[1762c753d0b7efe3]::iter::adapters::map::Map<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>, rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7fe94cb098d8 - rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct::<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>
  19:     0x7fe94cb2b99b - <rustc_session[80078005f746a035]::session::Session>::time::<(&[rustc_middle[ceaa495fe40a5927]::mir::mono::CodegenUnit], ()), rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7fe94cb09ccb - rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items
  21:     0x7fe94df5e65d - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::collect_and_partition_mono_items, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  22:     0x7fe94ddbb38e - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::collect_and_partition_mono_items
  23:     0x7fe94c3a8110 - rustc_codegen_ssa[79ccda2df5f217f]::base::codegen_crate::<rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend>
  24:     0x7fe94c2e5200 - <rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend as rustc_codegen_ssa[79ccda2df5f217f]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7fe94c1e349f - <rustc_session[80078005f746a035]::session::Session>::time::<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_interface[6d23b5db0963900c]::passes::start_codegen::{closure#0}>
  26:     0x7fe94c1e2928 - rustc_interface[6d23b5db0963900c]::passes::start_codegen
  27:     0x7fe94c20f3ef - <rustc_middle[ceaa495fe40a5927]::ty::context::GlobalCtxt>::enter::<<rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  28:     0x7fe94c2abb14 - <rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen
  29:     0x7fe94c177533 - <rustc_interface[6d23b5db0963900c]::interface::Compiler>::enter::<rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}::{closure#2}, core[1762c753d0b7efe3]::result::Result<core[1762c753d0b7efe3]::option::Option<rustc_interface[6d23b5db0963900c]::queries::Linker>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  30:     0x7fe94c13f9a8 - rustc_span[9e3124813005d0d3]::set_source_map::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  31:     0x7fe94c136bea - <scoped_tls[1c0e552b41f63f72]::ScopedKey<rustc_span[9e3124813005d0d3]::SessionGlobals>>::set::<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  32:     0x7fe94c16f786 - std[b2076b8fda847e46]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  33:     0x7fe94c1893e8 - std[b2076b8fda847e46]::panicking::try::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, core[1762c753d0b7efe3]::panic::unwind_safe::AssertUnwindSafe<<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7fe94c133327 - <<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1} as core[1762c753d0b7efe3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7fe94b66d8ae - std::sys::unix::thread::Thread::new::thread_start::hd7d7853ca946a288
  36:     0x7fe94b405b43 - <unknown>
  37:     0x7fe94b497a00 - <unknown>
  38:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (a3fafdbe0 2023-04-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -C opt-level=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `<dyn* core::fmt::Display as core::fmt::Display>::fmt - shim(reify)`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] tests/ui/dyn-star/const.rs stdout ----
---- [ui] tests/ui/dyn-star/const.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/dyn-star/const.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/const/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/const/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCs20tUa0bqYDv_4core3ptr13drop_in_placeD*NtNtB4_3fmt5DebugEL_ECsgqINSOlOajq_5const` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:273:5
   0:     0x7f8f0ff36810 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc7211feedd46988
   1:     0x7f8f0ff9e138 - core::fmt::write::h57ff60867d8efd5d
   2:     0x7f8f0ff2aec1 - std::io::Write::write_fmt::h94be5aa5308b7f9d
   3:     0x7f8f0ff36621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   3:     0x7f8f0ff36621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   4:     0x7f8f0ff39744 - std::panicking::default_hook::{{closure}}::hba6321717a5218f9
   5:     0x7f8f0ff3941d - std::panicking::default_hook::h58ef156a39f86bed
   6:     0x7f8f109fc035 - rustc_driver_impl[21fcdfcf54a5bd8f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f8f0ff39e61 - std::panicking::rust_panic_with_hook::habf71243a3a021f1
   8:     0x7f8f0ff39bd9 - std::panicking::begin_panic_handler::{{closure}}::hbf05a4d5d42770d9
   9:     0x7f8f0ff36ce6 - std::sys_common::backtrace::__rust_end_short_backtrace::h05482969ce085089
  10:     0x7f8f0ff398b7 - rust_begin_unwind
  11:     0x7f8f0feee113 - core::panicking::panic_fmt::h4d55ca4049c5f88c
  12:     0x7f8f12a07760 - rustc_symbol_mangling[13eb4448e51775d9]::symbol_name_provider
  13:     0x7f8f1270a86a - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::symbol_name, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  14:     0x7f8f1265f7a7 - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::symbol_name
  15:     0x7f8f1371edc0 - <rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>::symbol_name
  16:     0x7f8f113df619 - <&mut rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0} as core[1762c753d0b7efe3]::ops::function::FnOnce<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem,)>>::call_once
  17:     0x7f8f114034a3 - <alloc[5efc3b5c20d9d1d4]::vec::Vec<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName)> as alloc[5efc3b5c20d9d1d4]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName), core[1762c753d0b7efe3]::iter::adapters::map::Map<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>, rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7f8f113e28d8 - rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct::<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>
  19:     0x7f8f1140499b - <rustc_session[80078005f746a035]::session::Session>::time::<(&[rustc_middle[ceaa495fe40a5927]::mir::mono::CodegenUnit], ()), rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7f8f113e2ccb - rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items
  21:     0x7f8f1283765d - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::collect_and_partition_mono_items, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  22:     0x7f8f1269438e - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::collect_and_partition_mono_items
  23:     0x7f8f10c81110 - rustc_codegen_ssa[79ccda2df5f217f]::base::codegen_crate::<rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend>
  24:     0x7f8f10bbe200 - <rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend as rustc_codegen_ssa[79ccda2df5f217f]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7f8f10abc49f - <rustc_session[80078005f746a035]::session::Session>::time::<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_interface[6d23b5db0963900c]::passes::start_codegen::{closure#0}>
  26:     0x7f8f10abb928 - rustc_interface[6d23b5db0963900c]::passes::start_codegen
  27:     0x7f8f10ae83ef - <rustc_middle[ceaa495fe40a5927]::ty::context::GlobalCtxt>::enter::<<rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  28:     0x7f8f10b84b14 - <rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen
  29:     0x7f8f10a50533 - <rustc_interface[6d23b5db0963900c]::interface::Compiler>::enter::<rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}::{closure#2}, core[1762c753d0b7efe3]::result::Result<core[1762c753d0b7efe3]::option::Option<rustc_interface[6d23b5db0963900c]::queries::Linker>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  30:     0x7f8f10a189a8 - rustc_span[9e3124813005d0d3]::set_source_map::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  31:     0x7f8f10a0fbea - <scoped_tls[1c0e552b41f63f72]::ScopedKey<rustc_span[9e3124813005d0d3]::SessionGlobals>>::set::<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  32:     0x7f8f10a48786 - std[b2076b8fda847e46]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  33:     0x7f8f10a623e8 - std[b2076b8fda847e46]::panicking::try::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, core[1762c753d0b7efe3]::panic::unwind_safe::AssertUnwindSafe<<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7f8f10a0c327 - <<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1} as core[1762c753d0b7efe3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7f8f0ff468ae - std::sys::unix::thread::Thread::new::thread_start::hd7d7853ca946a288
  36:     0x7f8f0fcdeb43 - <unknown>
  37:     0x7f8f0fd70a00 - <unknown>
  38:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (a3fafdbe0 2023-04-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::ptr::drop_in_place::<dyn* core::fmt::Debug> - shim(Some(dyn* core::fmt::Debug))`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] tests/ui/dyn-star/dont-unsize-coerce-dyn-star.rs stdout ----
---- [ui] tests/ui/dyn-star/dont-unsize-coerce-dyn-star.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/dyn-star/dont-unsize-coerce-dyn-star.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dont-unsize-coerce-dyn-star/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dont-unsize-coerce-dyn-star/auxiliary"
stdout: none
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/dyn-star/dont-unsize-coerce-dyn-star.rs:4:12
   |
   |
LL | #![feature(dyn_star)]
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: `#[warn(incomplete_features)]` on by default


thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCs20tUa0bqYDv_4core3ptr13drop_in_placeD*NtCshrz2T9v12lj_27dont_unsize_coerce_dyn_star6AddOneEL_EBK_` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:273:5
stack backtrace:
   0:     0x7f03bb284810 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc7211feedd46988
   1:     0x7f03bb2ec138 - core::fmt::write::h57ff60867d8efd5d
   2:     0x7f03bb278ec1 - std::io::Write::write_fmt::h94be5aa5308b7f9d
   3:     0x7f03bb284621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   4:     0x7f03bb287744 - std::panicking::default_hook::{{closure}}::hba6321717a5218f9
   5:     0x7f03bb28741d - std::panicking::default_hook::h58ef156a39f86bed
   6:     0x7f03bbd4a035 - rustc_driver_impl[21fcdfcf54a5bd8f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f03bb287e61 - std::panicking::rust_panic_with_hook::habf71243a3a021f1
   8:     0x7f03bb287bd9 - std::panicking::begin_panic_handler::{{closure}}::hbf05a4d5d42770d9
   9:     0x7f03bb284ce6 - std::sys_common::backtrace::__rust_end_short_backtrace::h05482969ce085089
  10:     0x7f03bb2878b7 - rust_begin_unwind
  11:     0x7f03bb23c113 - core::panicking::panic_fmt::h4d55ca4049c5f88c
  12:     0x7f03bdd55760 - rustc_symbol_mangling[13eb4448e51775d9]::symbol_name_provider
  13:     0x7f03bda5886a - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::symbol_name, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  14:     0x7f03bd9ad7a7 - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::symbol_name
  15:     0x7f03bea6cdc0 - <rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>::symbol_name
  16:     0x7f03bc72d619 - <&mut rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0} as core[1762c753d0b7efe3]::ops::function::FnOnce<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem,)>>::call_once
  17:     0x7f03bc7514a3 - <alloc[5efc3b5c20d9d1d4]::vec::Vec<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName)> as alloc[5efc3b5c20d9d1d4]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName), core[1762c753d0b7efe3]::iter::adapters::map::Map<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>, rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7f03bc7308d8 - rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct::<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>
  19:     0x7f03bc75299b - <rustc_session[80078005f746a035]::session::Session>::time::<(&[rustc_middle[ceaa495fe40a5927]::mir::mono::CodegenUnit], ()), rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7f03bc730ccb - rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items
  21:     0x7f03bdb8565d - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::collect_and_partition_mono_items, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  22:     0x7f03bd9e238e - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::collect_and_partition_mono_items
  23:     0x7f03bbfcf110 - rustc_codegen_ssa[79ccda2df5f217f]::base::codegen_crate::<rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend>
  24:     0x7f03bbf0c200 - <rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend as rustc_codegen_ssa[79ccda2df5f217f]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7f03bbe0a49f - <rustc_session[80078005f746a035]::session::Session>::time::<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_interface[6d23b5db0963900c]::passes::start_codegen::{closure#0}>
  26:     0x7f03bbe09928 - rustc_interface[6d23b5db0963900c]::passes::start_codegen
  27:     0x7f03bbe363ef - <rustc_middle[ceaa495fe40a5927]::ty::context::GlobalCtxt>::enter::<<rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  28:     0x7f03bbed2b14 - <rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen
  29:     0x7f03bbd9e533 - <rustc_interface[6d23b5db0963900c]::interface::Compiler>::enter::<rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}::{closure#2}, core[1762c753d0b7efe3]::result::Result<core[1762c753d0b7efe3]::option::Option<rustc_interface[6d23b5db0963900c]::queries::Linker>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  30:     0x7f03bbd669a8 - rustc_span[9e3124813005d0d3]::set_source_map::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  31:     0x7f03bbd5dbea - <scoped_tls[1c0e552b41f63f72]::ScopedKey<rustc_span[9e3124813005d0d3]::SessionGlobals>>::set::<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  32:     0x7f03bbd96786 - std[b2076b8fda847e46]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  33:     0x7f03bbdb03e8 - std[b2076b8fda847e46]::panicking::try::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, core[1762c753d0b7efe3]::panic::unwind_safe::AssertUnwindSafe<<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7f03bbd5a327 - <<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1} as core[1762c753d0b7efe3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7f03bb2948ae - std::sys::unix::thread::Thread::new::thread_start::hd7d7853ca946a288
  36:     0x7f03bb02cb43 - <unknown>
  37:     0x7f03bb0bea00 - <unknown>
  38:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (a3fafdbe0 2023-04-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::ptr::drop_in_place::<dyn* AddOne> - shim(Some(dyn* AddOne))`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/dyn-star/drop.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/dyn-star/drop.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/drop/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/drop/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCs20tUa0bqYDv_4core3ptr13drop_in_placeD*NtNtB4_3fmt5DebugEL_ECs40Cl1W8Lz6x_4drop` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:273:5
   0:     0x7fe4062f8810 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc7211feedd46988
   1:     0x7fe406360138 - core::fmt::write::h57ff60867d8efd5d
   2:     0x7fe4062ecec1 - std::io::Write::write_fmt::h94be5aa5308b7f9d
   3:     0x7fe4062f8621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   3:     0x7fe4062f8621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   4:     0x7fe4062fb744 - std::panicking::default_hook::{{closure}}::hba6321717a5218f9
   5:     0x7fe4062fb41d - std::panicking::default_hook::h58ef156a39f86bed
   6:     0x7fe406dbe035 - rustc_driver_impl[21fcdfcf54a5bd8f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe4062fbe61 - std::panicking::rust_panic_with_hook::habf71243a3a021f1
   8:     0x7fe4062fbbd9 - std::panicking::begin_panic_handler::{{closure}}::hbf05a4d5d42770d9
   9:     0x7fe4062f8ce6 - std::sys_common::backtrace::__rust_end_short_backtrace::h05482969ce085089
  10:     0x7fe4062fb8b7 - rust_begin_unwind
  11:     0x7fe4062b0113 - core::panicking::panic_fmt::h4d55ca4049c5f88c
  12:     0x7fe408dc9760 - rustc_symbol_mangling[13eb4448e51775d9]::symbol_name_provider
  13:     0x7fe408acc86a - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::symbol_name, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  14:     0x7fe408a217a7 - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::symbol_name
  15:     0x7fe409ae0dc0 - <rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>::symbol_name
  16:     0x7fe4077a1619 - <&mut rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0} as core[1762c753d0b7efe3]::ops::function::FnOnce<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem,)>>::call_once
  17:     0x7fe4077c54a3 - <alloc[5efc3b5c20d9d1d4]::vec::Vec<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName)> as alloc[5efc3b5c20d9d1d4]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName), core[1762c753d0b7efe3]::iter::adapters::map::Map<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>, rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7fe4077a48d8 - rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct::<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>
  19:     0x7fe4077c699b - <rustc_session[80078005f746a035]::session::Session>::time::<(&[rustc_middle[ceaa495fe40a5927]::mir::mono::CodegenUnit], ()), rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7fe4077a4ccb - rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items
  21:     0x7fe408bf965d - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::collect_and_partition_mono_items, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  22:     0x7fe408a5638e - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::collect_and_partition_mono_items
  23:     0x7fe407043110 - rustc_codegen_ssa[79ccda2df5f217f]::base::codegen_crate::<rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend>
  24:     0x7fe406f80200 - <rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend as rustc_codegen_ssa[79ccda2df5f217f]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7fe406e7e49f - <rustc_session[80078005f746a035]::session::Session>::time::<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_interface[6d23b5db0963900c]::passes::start_codegen::{closure#0}>
  26:     0x7fe406e7d928 - rustc_interface[6d23b5db0963900c]::passes::start_codegen
  27:     0x7fe406eaa3ef - <rustc_middle[ceaa495fe40a5927]::ty::context::GlobalCtxt>::enter::<<rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  28:     0x7fe406f46b14 - <rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen
  29:     0x7fe406e12533 - <rustc_interface[6d23b5db0963900c]::interface::Compiler>::enter::<rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}::{closure#2}, core[1762c753d0b7efe3]::result::Result<core[1762c753d0b7efe3]::option::Option<rustc_interface[6d23b5db0963900c]::queries::Linker>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  30:     0x7fe406dda9a8 - rustc_span[9e3124813005d0d3]::set_source_map::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  31:     0x7fe406dd1bea - <scoped_tls[1c0e552b41f63f72]::ScopedKey<rustc_span[9e3124813005d0d3]::SessionGlobals>>::set::<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  32:     0x7fe406e0a786 - std[b2076b8fda847e46]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  33:     0x7fe406e243e8 - std[b2076b8fda847e46]::panicking::try::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, core[1762c753d0b7efe3]::panic::unwind_safe::AssertUnwindSafe<<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7fe406dce327 - <<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1} as core[1762c753d0b7efe3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7fe4063088ae - std::sys::unix::thread::Thread::new::thread_start::hd7d7853ca946a288
  36:     0x7fe4060a0b43 - <unknown>
  37:     0x7fe406132a00 - <unknown>
  38:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (a3fafdbe0 2023-04-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::ptr::drop_in_place::<dyn* core::fmt::Debug> - shim(Some(dyn* core::fmt::Debug))`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] tests/ui/dyn-star/dispatch-on-pin-mut.rs stdout ----
---- [ui] tests/ui/dyn-star/dispatch-on-pin-mut.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/dyn-star/dispatch-on-pin-mut.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dispatch-on-pin-mut/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dispatch-on-pin-mut/auxiliary" "--edition=2021"
stdout: none
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/dyn-star/dispatch-on-pin-mut.rs:5:12
   |
   |
LL | #![feature(dyn_star)]
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: `#[warn(incomplete_features)]` on by default


thread 'rustc' panicked at 'compute_symbol_name: `_RNvMs4_NtCs20tUa0bqYDv_4core3pinINtB5_3PinQD*NtNtNtB7_6future6future6Futurep6OutputlEL_E13new_uncheckedCsiuHUYT06xzq_19dispatch_on_pin_mut` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:273:5
stack backtrace:
   0:     0x7fadd3626810 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc7211feedd46988
   1:     0x7fadd368e138 - core::fmt::write::h57ff60867d8efd5d
   2:     0x7fadd361aec1 - std::io::Write::write_fmt::h94be5aa5308b7f9d
   3:     0x7fadd3626621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   4:     0x7fadd3629744 - std::panicking::default_hook::{{closure}}::hba6321717a5218f9
   5:     0x7fadd362941d - std::panicking::default_hook::h58ef156a39f86bed
   6:     0x7fadd40ec035 - rustc_driver_impl[21fcdfcf54a5bd8f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fadd3629e61 - std::panicking::rust_panic_with_hook::habf71243a3a021f1
   8:     0x7fadd3629bd9 - std::panicking::begin_panic_handler::{{closure}}::hbf05a4d5d42770d9
   9:     0x7fadd3626ce6 - std::sys_common::backtrace::__rust_end_short_backtrace::h05482969ce085089
  10:     0x7fadd36298b7 - rust_begin_unwind
  11:     0x7fadd35de113 - core::panicking::panic_fmt::h4d55ca4049c5f88c
  12:     0x7fadd60f7760 - rustc_symbol_mangling[13eb4448e51775d9]::symbol_name_provider
  13:     0x7fadd5dfa86a - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::symbol_name, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  14:     0x7fadd5d4f7a7 - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::symbol_name
  15:     0x7fadd6e0edc0 - <rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>::symbol_name
  16:     0x7fadd4acf619 - <&mut rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0} as core[1762c753d0b7efe3]::ops::function::FnOnce<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem,)>>::call_once
  17:     0x7fadd4af34a3 - <alloc[5efc3b5c20d9d1d4]::vec::Vec<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName)> as alloc[5efc3b5c20d9d1d4]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName), core[1762c753d0b7efe3]::iter::adapters::map::Map<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>, rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7fadd4ad28d8 - rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct::<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>
  19:     0x7fadd4af499b - <rustc_session[80078005f746a035]::session::Session>::time::<(&[rustc_middle[ceaa495fe40a5927]::mir::mono::CodegenUnit], ()), rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7fadd4ad2ccb - rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items
  21:     0x7fadd5f2765d - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::collect_and_partition_mono_items, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  22:     0x7fadd5d8438e - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::collect_and_partition_mono_items
  23:     0x7fadd4371110 - rustc_codegen_ssa[79ccda2df5f217f]::base::codegen_crate::<rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend>
  24:     0x7fadd42ae200 - <rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend as rustc_codegen_ssa[79ccda2df5f217f]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7fadd41ac49f - <rustc_session[80078005f746a035]::session::Session>::time::<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_interface[6d23b5db0963900c]::passes::start_codegen::{closure#0}>
  26:     0x7fadd41ab928 - rustc_interface[6d23b5db0963900c]::passes::start_codegen
  27:     0x7fadd41d83ef - <rustc_middle[ceaa495fe40a5927]::ty::context::GlobalCtxt>::enter::<<rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  28:     0x7fadd4274b14 - <rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen
  29:     0x7fadd4140533 - <rustc_interface[6d23b5db0963900c]::interface::Compiler>::enter::<rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}::{closure#2}, core[1762c753d0b7efe3]::result::Result<core[1762c753d0b7efe3]::option::Option<rustc_interface[6d23b5db0963900c]::queries::Linker>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  30:     0x7fadd41089a8 - rustc_span[9e3124813005d0d3]::set_source_map::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  31:     0x7fadd40ffbea - <scoped_tls[1c0e552b41f63f72]::ScopedKey<rustc_span[9e3124813005d0d3]::SessionGlobals>>::set::<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  32:     0x7fadd4138786 - std[b2076b8fda847e46]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  33:     0x7fadd41523e8 - std[b2076b8fda847e46]::panicking::try::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, core[1762c753d0b7efe3]::panic::unwind_safe::AssertUnwindSafe<<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7fadd40fc327 - <<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1} as core[1762c753d0b7efe3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7fadd36368ae - std::sys::unix::thread::Thread::new::thread_start::hd7d7853ca946a288
  36:     0x7fadd33ceb43 - <unknown>
  37:     0x7fadd3460a00 - <unknown>
  38:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (a3fafdbe0 2023-04-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::pin::Pin::<&mut dyn* core::future::future::Future<Output = i32>>::new_unchecked`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/dyn-star/dyn-star-to-dyn.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/dyn-star/dyn-star-to-dyn.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dyn-star-to-dyn/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dyn-star-to-dyn/auxiliary"
stdout: none
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/dyn-star/dyn-star-to-dyn.rs:3:12
   |
   |
LL | #![feature(dyn_star)]
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: `#[warn(incomplete_features)]` on by default


thread 'rustc' panicked at 'compute_symbol_name: `_RNSNvYD*NtNtCs20tUa0bqYDv_4core3fmt5DebugEL_B6_3fmt5reifyCs4oCdeGFJcoB_15dyn_star_to_dyn` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:273:5
   0:     0x7f8ce1532810 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc7211feedd46988
   0:     0x7f8ce1532810 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc7211feedd46988
   1:     0x7f8ce159a138 - core::fmt::write::h57ff60867d8efd5d
   2:     0x7f8ce1526ec1 - std::io::Write::write_fmt::h94be5aa5308b7f9d
   3:     0x7f8ce1532621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   4:     0x7f8ce1535744 - std::panicking::default_hook::{{closure}}::hba6321717a5218f9
   5:     0x7f8ce153541d - std::panicking::default_hook::h58ef156a39f86bed
   6:     0x7f8ce1ff8035 - rustc_driver_impl[21fcdfcf54a5bd8f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f8ce1535e61 - std::panicking::rust_panic_with_hook::habf71243a3a021f1
   8:     0x7f8ce1535bd9 - std::panicking::begin_panic_handler::{{closure}}::hbf05a4d5d42770d9
   9:     0x7f8ce1532ce6 - std::sys_common::backtrace::__rust_end_short_backtrace::h05482969ce085089
  10:     0x7f8ce15358b7 - rust_begin_unwind
  11:     0x7f8ce14ea113 - core::panicking::panic_fmt::h4d55ca4049c5f88c
  12:     0x7f8ce4003760 - rustc_symbol_mangling[13eb4448e51775d9]::symbol_name_provider
  13:     0x7f8ce3d0686a - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::symbol_name, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  14:     0x7f8ce3c5b7a7 - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::symbol_name
  15:     0x7f8ce4d1adc0 - <rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>::symbol_name
  16:     0x7f8ce29db619 - <&mut rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0} as core[1762c753d0b7efe3]::ops::function::FnOnce<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem,)>>::call_once
  17:     0x7f8ce29ff4a3 - <alloc[5efc3b5c20d9d1d4]::vec::Vec<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName)> as alloc[5efc3b5c20d9d1d4]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName), core[1762c753d0b7efe3]::iter::adapters::map::Map<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>, rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7f8ce29de8d8 - rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct::<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>
  19:     0x7f8ce2a0099b - <rustc_session[80078005f746a035]::session::Session>::time::<(&[rustc_middle[ceaa495fe40a5927]::mir::mono::CodegenUnit], ()), rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7f8ce29deccb - rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items
  21:     0x7f8ce3e3365d - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::collect_and_partition_mono_items, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  22:     0x7f8ce3c9038e - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::collect_and_partition_mono_items
  23:     0x7f8ce227d110 - rustc_codegen_ssa[79ccda2df5f217f]::base::codegen_crate::<rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend>
  24:     0x7f8ce21ba200 - <rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend as rustc_codegen_ssa[79ccda2df5f217f]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7f8ce20b849f - <rustc_session[80078005f746a035]::session::Session>::time::<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_interface[6d23b5db0963900c]::passes::start_codegen::{closure#0}>
  26:     0x7f8ce20b7928 - rustc_interface[6d23b5db0963900c]::passes::start_codegen
  27:     0x7f8ce20e43ef - <rustc_middle[ceaa495fe40a5927]::ty::context::GlobalCtxt>::enter::<<rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  28:     0x7f8ce2180b14 - <rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen
  29:     0x7f8ce204c533 - <rustc_interface[6d23b5db0963900c]::interface::Compiler>::enter::<rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}::{closure#2}, core[1762c753d0b7efe3]::result::Result<core[1762c753d0b7efe3]::option::Option<rustc_interface[6d23b5db0963900c]::queries::Linker>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  30:     0x7f8ce20149a8 - rustc_span[9e3124813005d0d3]::set_source_map::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  31:     0x7f8ce200bbea - <scoped_tls[1c0e552b41f63f72]::ScopedKey<rustc_span[9e3124813005d0d3]::SessionGlobals>>::set::<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  32:     0x7f8ce2044786 - std[b2076b8fda847e46]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  33:     0x7f8ce205e3e8 - std[b2076b8fda847e46]::panicking::try::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, core[1762c753d0b7efe3]::panic::unwind_safe::AssertUnwindSafe<<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7f8ce2008327 - <<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1} as core[1762c753d0b7efe3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7f8ce15428ae - std::sys::unix::thread::Thread::new::thread_start::hd7d7853ca946a288
  36:     0x7f8ce12dab43 - <unknown>
  37:     0x7f8ce136ca00 - <unknown>
  38:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (a3fafdbe0 2023-04-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `<dyn* core::fmt::Debug as core::fmt::Debug>::fmt - shim(reify)`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/dyn-star/make-dyn-star.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/dyn-star/make-dyn-star.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/make-dyn-star/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/make-dyn-star/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCs20tUa0bqYDv_4core3ptr13drop_in_placeD*NtNtB4_3fmt5DebugEL_ECs2saRO9vAFuO_13make_dyn_star` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:273:5
   0:     0x7ff970e18810 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc7211feedd46988
   1:     0x7ff970e80138 - core::fmt::write::h57ff60867d8efd5d
   2:     0x7ff970e0cec1 - std::io::Write::write_fmt::h94be5aa5308b7f9d
   3:     0x7ff970e18621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   3:     0x7ff970e18621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   4:     0x7ff970e1b744 - std::panicking::default_hook::{{closure}}::hba6321717a5218f9
   5:     0x7ff970e1b41d - std::panicking::default_hook::h58ef156a39f86bed
   6:     0x7ff9718de035 - rustc_driver_impl[21fcdfcf54a5bd8f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff970e1be61 - std::panicking::rust_panic_with_hook::habf71243a3a021f1
   8:     0x7ff970e1bbd9 - std::panicking::begin_panic_handler::{{closure}}::hbf05a4d5d42770d9
   9:     0x7ff970e18ce6 - std::sys_common::backtrace::__rust_end_short_backtrace::h05482969ce085089
  10:     0x7ff970e1b8b7 - rust_begin_unwind
  11:     0x7ff970dd0113 - core::panicking::panic_fmt::h4d55ca4049c5f88c
  12:     0x7ff9738e9760 - rustc_symbol_mangling[13eb4448e51775d9]::symbol_name_provider
  13:     0x7ff9735ec86a - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::symbol_name, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  14:     0x7ff9735417a7 - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::symbol_name
  15:     0x7ff974600dc0 - <rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>::symbol_name
  16:     0x7ff9722c1619 - <&mut rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0} as core[1762c753d0b7efe3]::ops::function::FnOnce<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem,)>>::call_once
  17:     0x7ff9722e54a3 - <alloc[5efc3b5c20d9d1d4]::vec::Vec<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName)> as alloc[5efc3b5c20d9d1d4]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem, rustc_middle[ceaa495fe40a5927]::ty::SymbolName), core[1762c753d0b7efe3]::iter::adapters::map::Map<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>, rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  18:     0x7ff9722c48d8 - rustc_monomorphize[11c174f91c6a5300]::partitioning::assert_symbols_are_distinct::<std[b2076b8fda847e46]::collections::hash::set::Iter<rustc_middle[ceaa495fe40a5927]::mir::mono::MonoItem>>
  19:     0x7ff9722e699b - <rustc_session[80078005f746a035]::session::Session>::time::<(&[rustc_middle[ceaa495fe40a5927]::mir::mono::CodegenUnit], ()), rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items::{closure#0}>
  20:     0x7ff9722c4ccb - rustc_monomorphize[11c174f91c6a5300]::partitioning::collect_and_partition_mono_items
  21:     0x7ff97371965d - rustc_query_system[a8d62d1144c352bc]::query::plumbing::try_execute_query::<rustc_query_impl[2cb8e18627cdfe6d]::queries::collect_and_partition_mono_items, rustc_query_impl[2cb8e18627cdfe6d]::plumbing::QueryCtxt>
  22:     0x7ff97357638e - <rustc_query_impl[2cb8e18627cdfe6d]::Queries as rustc_middle[ceaa495fe40a5927]::ty::query::QueryEngine>::collect_and_partition_mono_items
  23:     0x7ff971b63110 - rustc_codegen_ssa[79ccda2df5f217f]::base::codegen_crate::<rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend>
  24:     0x7ff971aa0200 - <rustc_codegen_llvm[8a84db41dc531aee]::LlvmCodegenBackend as rustc_codegen_ssa[79ccda2df5f217f]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7ff97199e49f - <rustc_session[80078005f746a035]::session::Session>::time::<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_interface[6d23b5db0963900c]::passes::start_codegen::{closure#0}>
  26:     0x7ff97199d928 - rustc_interface[6d23b5db0963900c]::passes::start_codegen
  27:     0x7ff9719ca3ef - <rustc_middle[ceaa495fe40a5927]::ty::context::GlobalCtxt>::enter::<<rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<alloc[5efc3b5c20d9d1d4]::boxed::Box<dyn core[1762c753d0b7efe3]::any::Any>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  28:     0x7ff971a66b14 - <rustc_interface[6d23b5db0963900c]::queries::Queries>::ongoing_codegen
  29:     0x7ff971932533 - <rustc_interface[6d23b5db0963900c]::interface::Compiler>::enter::<rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}::{closure#2}, core[1762c753d0b7efe3]::result::Result<core[1762c753d0b7efe3]::option::Option<rustc_interface[6d23b5db0963900c]::queries::Linker>, rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  30:     0x7ff9718fa9a8 - rustc_span[9e3124813005d0d3]::set_source_map::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  31:     0x7ff9718f1bea - <scoped_tls[1c0e552b41f63f72]::ScopedKey<rustc_span[9e3124813005d0d3]::SessionGlobals>>::set::<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  32:     0x7ff97192a786 - std[b2076b8fda847e46]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>
  33:     0x7ff9719443e8 - std[b2076b8fda847e46]::panicking::try::<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, core[1762c753d0b7efe3]::panic::unwind_safe::AssertUnwindSafe<<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7ff9718ee327 - <<std[b2076b8fda847e46]::thread::Builder>::spawn_unchecked_<rustc_interface[6d23b5db0963900c]::util::run_in_thread_pool_with_globals<rustc_interface[6d23b5db0963900c]::interface::run_compiler<core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>, rustc_driver_impl[21fcdfcf54a5bd8f]::run_compiler::{closure#1}>::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1762c753d0b7efe3]::result::Result<(), rustc_span[9e3124813005d0d3]::ErrorGuaranteed>>::{closure#1} as core[1762c753d0b7efe3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7ff970e288ae - std::sys::unix::thread::Thread::new::thread_start::hd7d7853ca946a288
  36:     0x7ff970bc0b43 - <unknown>
  37:     0x7ff970c52a00 - <unknown>
  38:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (a3fafdbe0 2023-04-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `core::ptr::drop_in_place::<dyn* core::fmt::Debug> - shim(Some(dyn* core::fmt::Debug))`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] tests/ui/dyn-star/no-explicit-dyn-star.rs stdout ----
---- [ui] tests/ui/dyn-star/no-explicit-dyn-star.rs stdout ----

error: auxiliary build of "/checkout/tests/ui/dyn-star/auxiliary/dyn-star-foreign.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/dyn-star/auxiliary/dyn-star-foreign.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-explicit-dyn-star/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-explicit-dyn-star/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function `works_locally` is never used
  --> fake-test-src-base/dyn-star/auxiliary/dyn-star-foreign.rs:8:4
   |
LL | fn works_locally() {
   |
   = note: `#[warn(dead_code)]` on by default


thread 'rustc' panicked at 'compute_symbol_name: `_RINvNtCs20tUa0bqYDv_4core3ptr13drop_in_placeD*NtNtB4_3fmt7DisplayEL_ECs7dVpw2d4hBU_16dyn_star_foreign` cannot be demangled', compiler/rustc_symbol_mangling/src/lib.rs:273:5
stack backtrace:
   0:     0x7f13f58a4810 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc7211feedd46988
   1:     0x7f13f590c138 - core::fmt::write::h57ff60867d8efd5d
   2:     0x7f13f5898ec1 - std::io::Write::write_fmt::h94be5aa5308b7f9d
   3:     0x7f13f58a4621 - std::sys_common::backtrace::print::hcb884d9cf00567d3
   4:     0x7f13f58a7744 - std::panicking::default_hook::{{closure}}::hba6321717a5218f9
