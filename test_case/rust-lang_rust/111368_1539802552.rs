
error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:1038:9: no MIR available for DefId(20:166 ~ tfhe[0ecc]::boolean::engine::{impl#4}::and)

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/2f2c438dce75d8cc532c3baa849eeddc0901802c/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:        0x100a01a88 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb1d7f427abac4e67
   1:        0x100a514dc - core::fmt::write::h99b2593a967c157f
   2:        0x1009f7c94 - std::io::Write::write_fmt::hb795e309bfe1402e
   3:        0x100a018dc - std::sys_common::backtrace::print::h2210289afef2f458
   4:        0x100a042f8 - std::panicking::default_hook::{{closure}}::h2e14385d88f14166
   5:        0x100a04100 - std::panicking::default_hook::h2065fd506ae6675b
   6:        0x108e474f4 - rustc_driver_impl[578feedc5daeedcf]::install_ice_hook::{closure#0}
   7:        0x100a0490c - std::panicking::rust_panic_with_hook::h3b0873ff2ce9653d
   8:        0x10cc2be2c - std[abb65c99166164af]::panicking::begin_panic::<rustc_errors[bfc4faeb8411a782]::ExplicitBug>::{closure#0}
   9:        0x10cc28fe4 - std[abb65c99166164af]::sys_common::backtrace::__rust_end_short_backtrace::<std[abb65c99166164af]::panicking::begin_panic<rustc_errors[bfc4faeb8411a782]::ExplicitBug>::{closure#0}, !>
  10:        0x10d2233e0 - std[abb65c99166164af]::panicking::begin_panic::<rustc_errors[bfc4faeb8411a782]::ExplicitBug>
  11:        0x10cc20300 - <rustc_errors[bfc4faeb8411a782]::HandlerInner>::bug::<alloc[f5305febaccc2c84]::string::String>
  12:        0x10cc1fcbc - <rustc_errors[bfc4faeb8411a782]::Handler>::bug::<alloc[f5305febaccc2c84]::string::String>
  13:        0x10cd014f8 - rustc_middle[caf3ad0f18d6cf5a]::util::bug::opt_span_bug_fmt::<rustc_span[7819acb6a66388c3]::span_encoding::Span>::{closure#0}
  14:        0x10cd00bbc - rustc_middle[caf3ad0f18d6cf5a]::ty::context::tls::with_opt::<rustc_middle[caf3ad0f18d6cf5a]::util::bug::opt_span_bug_fmt<rustc_span[7819acb6a66388c3]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:        0x10cd00b88 - rustc_middle[caf3ad0f18d6cf5a]::ty::context::tls::with_context_opt::<rustc_middle[caf3ad0f18d6cf5a]::ty::context::tls::with_opt<rustc_middle[caf3ad0f18d6cf5a]::util::bug::opt_span_bug_fmt<rustc_span[7819acb6a66388c3]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:        0x10d22826c - rustc_middle[caf3ad0f18d6cf5a]::util::bug::bug_fmt
  17:        0x10b50a7e0 - rustc_monomorphize[eb5800245940b2a8]::collector::should_codegen_locally
  18:        0x10b50a43c - rustc_monomorphize[eb5800245940b2a8]::collector::visit_instance_use
  19:        0x10b509c7c - <rustc_monomorphize[eb5800245940b2a8]::collector::MirNeighborCollector as rustc_middle[caf3ad0f18d6cf5a]::mir::visit::Visitor>::visit_terminator
  20:        0x10b50df0c - rustc_monomorphize[eb5800245940b2a8]::collector::collect_neighbours
  21:        0x10b50cc4c - rustc_monomorphize[eb5800245940b2a8]::collector::collect_items_rec
  22:        0x10b50cf50 - rustc_monomorphize[eb5800245940b2a8]::collector::collect_items_rec
  23:        0x10b50cf50 - rustc_monomorphize[eb5800245940b2a8]::collector::collect_items_rec
  24:        0x10b50cf50 - rustc_monomorphize[eb5800245940b2a8]::collector::collect_items_rec
  25:        0x10b50cf50 - rustc_monomorphize[eb5800245940b2a8]::collector::collect_items_rec
  26:        0x10b50cf50 - rustc_monomorphize[eb5800245940b2a8]::collector::collect_items_rec
  27:        0x10b50cf50 - rustc_monomorphize[eb5800245940b2a8]::collector::collect_items_rec
  28:        0x10b52cd34 - <core[bcec2d62264d7952]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[73389b4178ce37f6]::sync::par_for_each_in<alloc[f5305febaccc2c84]::vec::Vec<rustc_middle[caf3ad0f18d6cf5a]::mir::mono::MonoItem>, rustc_monomorphize[eb5800245940b2a8]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[bcec2d62264d7952]::ops::function::FnOnce<()>>::call_once
  29:        0x10b51ff54 - rustc_data_structures[73389b4178ce37f6]::sync::par_for_each_in::<alloc[f5305febaccc2c84]::vec::Vec<rustc_middle[caf3ad0f18d6cf5a]::mir::mono::MonoItem>, rustc_monomorphize[eb5800245940b2a8]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  30:        0x10b528a60 - <rustc_session[687b389697f94a83]::session::Session>::time::<(), rustc_monomorphize[eb5800245940b2a8]::collector::collect_crate_mono_items::{closure#1}>
  31:        0x10b50b364 - rustc_monomorphize[eb5800245940b2a8]::collector::collect_crate_mono_items
  32:        0x10b51d73c - rustc_monomorphize[eb5800245940b2a8]::partitioning::collect_and_partition_mono_items
  33:        0x10c049cb8 - rustc_query_system[a2dfadba2cd10974]::query::plumbing::try_execute_query::<rustc_query_impl[a6cc2ac4118918e4]::queries::collect_and_partition_mono_items, rustc_query_impl[a6cc2ac4118918e4]::plumbing::QueryCtxt>
  34:        0x10c0fb5e0 - rustc_query_impl[a6cc2ac4118918e4]::get_query::collect_and_partition_mono_items
  35:        0x108f04b48 - rustc_codegen_ssa[aeefc1f214e73609]::base::codegen_crate::<rustc_codegen_llvm[6b91aa987c2cba1e]::LlvmCodegenBackend>
  36:        0x108f5aca4 - <rustc_codegen_llvm[6b91aa987c2cba1e]::LlvmCodegenBackend as rustc_codegen_ssa[aeefc1f214e73609]::traits::backend::CodegenBackend>::codegen_crate
  37:        0x108ef720c - rustc_interface[6e4b2cae1a32f574]::passes::start_codegen
  38:        0x108e6f930 - <rustc_middle[caf3ad0f18d6cf5a]::ty::context::GlobalCtxt>::enter::<<rustc_interface[6e4b2cae1a32f574]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[bcec2d62264d7952]::result::Result<alloc[f5305febaccc2c84]::boxed::Box<dyn core[bcec2d62264d7952]::any::Any>, rustc_span[7819acb6a66388c3]::ErrorGuaranteed>>
  39:        0x108efa848 - <rustc_interface[6e4b2cae1a32f574]::queries::Queries>::ongoing_codegen
  40:        0x108e23740 - <rustc_interface[6e4b2cae1a32f574]::interface::Compiler>::enter::<rustc_driver_impl[578feedc5daeedcf]::run_compiler::{closure#1}::{closure#2}, core[bcec2d62264d7952]::result::Result<core[bcec2d62264d7952]::option::Option<rustc_interface[6e4b2cae1a32f574]::queries::Linker>, rustc_span[7819acb6a66388c3]::ErrorGuaranteed>>
  41:        0x108debac4 - std[abb65c99166164af]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6e4b2cae1a32f574]::util::run_in_thread_pool_with_globals<rustc_interface[6e4b2cae1a32f574]::interface::run_compiler<core[bcec2d62264d7952]::result::Result<(), rustc_span[7819acb6a66388c3]::ErrorGuaranteed>, rustc_driver_impl[578feedc5daeedcf]::run_compiler::{closure#1}>::{closure#0}, core[bcec2d62264d7952]::result::Result<(), rustc_span[7819acb6a66388c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bcec2d62264d7952]::result::Result<(), rustc_span[7819acb6a66388c3]::ErrorGuaranteed>>
  42:        0x108ded8fc - <<std[abb65c99166164af]::thread::Builder>::spawn_unchecked_<rustc_interface[6e4b2cae1a32f574]::util::run_in_thread_pool_with_globals<rustc_interface[6e4b2cae1a32f574]::interface::run_compiler<core[bcec2d62264d7952]::result::Result<(), rustc_span[7819acb6a66388c3]::ErrorGuaranteed>, rustc_driver_impl[578feedc5daeedcf]::run_compiler::{closure#1}>::{closure#0}, core[bcec2d62264d7952]::result::Result<(), rustc_span[7819acb6a66388c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bcec2d62264d7952]::result::Result<(), rustc_span[7819acb6a66388c3]::ErrorGuaranteed>>::{closure#1} as core[bcec2d62264d7952]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:        0x100a0d278 - std::sys::unix::thread::Thread::new::thread_start::h38aefecedc206f89
  44:        0x1a0a63fa8 - __pthread_joiner_wake

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (2f2c438dc 2023-05-08) running on aarch64-apple-darwin

note: compiler flags: --crate-type bin -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
  