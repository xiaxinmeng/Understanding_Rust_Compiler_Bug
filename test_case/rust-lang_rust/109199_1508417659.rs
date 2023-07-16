
Compiling sol-did v0.2.0
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7fbc02ae4b9a - std::backtrace_rs::backtrace::libunwind::trace::h33a8ebdd070a2849
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fbc02ae4b9a - std::backtrace_rs::backtrace::trace_unsynchronized::hdcf243f187b50d8d
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fbc02ae4b9a - std::sys_common::backtrace::_print_fmt::h447195462962d9fa
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fbc02ae4b9a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h030527b6b1cd3ea0
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fbc02b48b3f - core::fmt::write::h820832a7aab88d9e
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/core/src/fmt/mod.rs:1254:17
   5:     0x7fbc02ad76d5 - std::io::Write::write_fmt::h8f101bdb654e5017
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/io/mod.rs:1698:15
   6:     0x7fbc02ae4965 - std::sys_common::backtrace::_print::h774dcd3ec539df8d
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fbc02ae4965 - std::sys_common::backtrace::print::hfb7ed7d7ba09efbc
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fbc02ae760e - std::panicking::default_hook::{{closure}}::h1b562f555ed8e3e1
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/panicking.rs:271:22
   9:     0x7fbc02ae73b5 - std::panicking::default_hook::hc7a03bfdd14960de
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/panicking.rs:290:9
  10:     0x7fbc05d9a025 - <rustc_driver_impl[92a15bc3e068e4f2]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[a3d8d6709fc4f528]::ops::function::FnOnce<(&core[a3d8d6709fc4f528]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7fbc02ae7e04 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::ha85a7263ce5c1565
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/alloc/src/boxed.rs:2002:9
  12:     0x7fbc02ae7e04 - std::panicking::rust_panic_with_hook::hfd9594253b2174d9
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/panicking.rs:696:13
  13:     0x7fbc062e3251 - std[1bf61b140f607083]::panicking::begin_panic::<rustc_errors[7ae47b583acaa0c7]::ExplicitBug>::{closure#0}
  14:     0x7fbc062dfe26 - std[1bf61b140f607083]::sys_common::backtrace::__rust_end_short_backtrace::<std[1bf61b140f607083]::panicking::begin_panic<rustc_errors[7ae47b583acaa0c7]::ExplicitBug>::{closure#0}, !>
  15:     0x7fbc062cc176 - std[1bf61b140f607083]::panicking::begin_panic::<rustc_errors[7ae47b583acaa0c7]::ExplicitBug>
  16:     0x7fbc06301536 - std[1bf61b140f607083]::panic::panic_any::<rustc_errors[7ae47b583acaa0c7]::ExplicitBug>
  17:     0x7fbc062fffc6 - <rustc_errors[7ae47b583acaa0c7]::HandlerInner>::bug::<&alloc[d60a5f910632ae0c]::string::String>
  18:     0x7fbc062ffc90 - <rustc_errors[7ae47b583acaa0c7]::Handler>::bug::<&alloc[d60a5f910632ae0c]::string::String>
  19:     0x7fbc062fbc9b - rustc_middle[e79c5042f5680516]::util::bug::opt_span_bug_fmt::<rustc_span[9f00524050a95850]::span_encoding::Span>::{closure#0}
  20:     0x7fbc062fab5a - rustc_middle[e79c5042f5680516]::ty::context::tls::with_opt::<rustc_middle[e79c5042f5680516]::util::bug::opt_span_bug_fmt<rustc_span[9f00524050a95850]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0x7fbc062fab26 - rustc_middle[e79c5042f5680516]::ty::context::tls::with_context_opt::<rustc_middle[e79c5042f5680516]::ty::context::tls::with_opt<rustc_middle[e79c5042f5680516]::util::bug::opt_span_bug_fmt<rustc_span[9f00524050a95850]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0x7fbc062fbbe6 - rustc_middle[e79c5042f5680516]::util::bug::opt_span_bug_fmt::<rustc_span[9f00524050a95850]::span_encoding::Span>
  23:     0x7fbc04065f53 - rustc_middle[e79c5042f5680516]::util::bug::bug_fmt
  24:     0x7fbc056f3233 - <rustc_middle[e79c5042f5680516]::ty::instance::Instance>::expect_resolve
  25:     0x7fbc05153d56 - rustc_monomorphize[a31546c071a6ebc4]::collector::collect_roots
  26:     0x7fbc0514ea72 - <rustc_session[46f71b3d93718d91]::session::Session>::time::<alloc[d60a5f910632ae0c]::vec::Vec<rustc_middle[e79c5042f5680516]::mir::mono::MonoItem>, rustc_monomorphize[a31546c071a6ebc4]::collector::collect_crate_mono_items::{closure#0}>
  27:     0x7fbc0514e6b8 - rustc_monomorphize[a31546c071a6ebc4]::collector::collect_crate_mono_items
  28:     0x7fbc0514cfc7 - rustc_monomorphize[a31546c071a6ebc4]::partitioning::collect_and_partition_mono_items
  29:     0x7fbc053e0c41 - rustc_query_system[c3c23202dfcd6b3e]::query::plumbing::try_execute_query::<rustc_query_impl[eb7c07aa1bc4986c]::queries::collect_and_partition_mono_items, rustc_query_impl[eb7c07aa1bc4986c]::plumbing::QueryCtxt>
  30:     0x7fbc053e091d - <rustc_query_impl[eb7c07aa1bc4986c]::Queries as rustc_middle[e79c5042f5680516]::ty::query::QueryEngine>::collect_and_partition_mono_items
  31:     0x7fbc04d639b3 - rustc_codegen_ssa[7dece96c95dcc8d]::back::symbol_export::exported_symbols_provider_local
  32:     0x7fbc054c90fe - rustc_query_system[c3c23202dfcd6b3e]::query::plumbing::try_execute_query::<rustc_query_impl[eb7c07aa1bc4986c]::queries::exported_symbols, rustc_query_impl[eb7c07aa1bc4986c]::plumbing::QueryCtxt>
  33:     0x7fbc054c8931 - <rustc_query_impl[eb7c07aa1bc4986c]::Queries as rustc_middle[e79c5042f5680516]::ty::query::QueryEngine>::exported_symbols
  34:     0x7fbc04fb1dec - <rustc_metadata[a1d83b82903af0b2]::rmeta::encoder::EncodeContext>::encode_crate_root
  35:     0x7fbc04f47fa2 - rustc_metadata[a1d83b82903af0b2]::rmeta::encoder::encode_metadata_impl
  36:     0x7fbc04f3d29c - rustc_data_structures[70b35e3a66f9208f]::sync::join::<rustc_metadata[a1d83b82903af0b2]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[a1d83b82903af0b2]::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
  37:     0x7fbc04f3cfbf - rustc_metadata[a1d83b82903af0b2]::rmeta::encoder::encode_metadata
  38:     0x7fbc04f3c216 - rustc_metadata[a1d83b82903af0b2]::fs::encode_and_write_metadata
  39:     0x7fbc04f33e73 - rustc_interface[8a54472d067ff82c]::passes::start_codegen
  40:     0x7fbc04f2ef64 - <rustc_middle[e79c5042f5680516]::ty::context::GlobalCtxt>::enter::<<rustc_interface[8a54472d067ff82c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[a3d8d6709fc4f528]::result::Result<alloc[d60a5f910632ae0c]::boxed::Box<dyn core[a3d8d6709fc4f528]::any::Any>, rustc_span[9f00524050a95850]::ErrorGuaranteed>>
  41:     0x7fbc04f2d348 - <rustc_interface[8a54472d067ff82c]::queries::Queries>::ongoing_codegen
  42:     0x7fbc04f2cb64 - <rustc_interface[8a54472d067ff82c]::interface::Compiler>::enter::<rustc_driver_impl[92a15bc3e068e4f2]::run_compiler::{closure#1}::{closure#2}, core[a3d8d6709fc4f528]::result::Result<core[a3d8d6709fc4f528]::option::Option<rustc_interface[8a54472d067ff82c]::queries::Linker>, rustc_span[9f00524050a95850]::ErrorGuaranteed>>
  43:     0x7fbc04f2ace1 - rustc_span[9f00524050a95850]::with_source_map::<core[a3d8d6709fc4f528]::result::Result<(), rustc_span[9f00524050a95850]::ErrorGuaranteed>, rustc_interface[8a54472d067ff82c]::interface::run_compiler<core[a3d8d6709fc4f528]::result::Result<(), rustc_span[9f00524050a95850]::ErrorGuaranteed>, rustc_driver_impl[92a15bc3e068e4f2]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  44:     0x7fbc04f2a28f - std[1bf61b140f607083]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8a54472d067ff82c]::util::run_in_thread_pool_with_globals<rustc_interface[8a54472d067ff82c]::interface::run_compiler<core[a3d8d6709fc4f528]::result::Result<(), rustc_span[9f00524050a95850]::ErrorGuaranteed>, rustc_driver_impl[92a15bc3e068e4f2]::run_compiler::{closure#1}>::{closure#0}, core[a3d8d6709fc4f528]::result::Result<(), rustc_span[9f00524050a95850]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a3d8d6709fc4f528]::result::Result<(), rustc_span[9f00524050a95850]::ErrorGuaranteed>>
  45:     0x7fbc05643cbe - <<std[1bf61b140f607083]::thread::Builder>::spawn_unchecked_<rustc_interface[8a54472d067ff82c]::util::run_in_thread_pool_with_globals<rustc_interface[8a54472d067ff82c]::interface::run_compiler<core[a3d8d6709fc4f528]::result::Result<(), rustc_span[9f00524050a95850]::ErrorGuaranteed>, rustc_driver_impl[92a15bc3e068e4f2]::run_compiler::{closure#1}>::{closure#0}, core[a3d8d6709fc4f528]::result::Result<(), rustc_span[9f00524050a95850]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a3d8d6709fc4f528]::result::Result<(), rustc_span[9f00524050a95850]::ErrorGuaranteed>>::{closure#1} as core[a3d8d6709fc4f528]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:     0x7fbc02af1ed5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h009afec0196bee04
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/alloc/src/boxed.rs:1988:9
  47:     0x7fbc02af1ed5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::ha6c4bb25884feef5
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/alloc/src/boxed.rs:1988:9
  48:     0x7fbc02af1ed5 - std::sys::unix::thread::Thread::new::thread_start::h6c0e7fa9b5646d29
                               at /rustc/0c61c7a978fe9f7b77a1d667c77d2202dadd1c10/library/std/src/sys/unix/thread.rs:108:17
  49:     0x7fbc02851bb5 - <unknown>
  50:     0x7fbc028d3d90 - <unknown>
  51:                0x0 - <unknown>

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (0c61c7a97 2023-03-25) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type cdylib --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C debuginfo=2 -C link-dead-code

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [exported_symbols] collecting exported symbols for crate `0`
end of query stack
error: Broken pipe (os error 32)
warning: build failed, waiting for other jobs to finish...
error: could not compile `sol-did` (lib) due to previous error
Apr 14 13:06:29.745 ERROR cargo_tarpaulin: Failed to compile tests!
error: internal compiler error: compiler/rustc_middle/src/ty/instance.rs:401:18: failed to resolve instance for <DecentralizedIdentifier<'_> as BorshDeserialize>::try_from_slice


Error: "Failed to compile tests!\nerror: internal compiler error: compiler/rustc_middle/src/ty/instance.rs:401:18: failed to resolve instance for <DecentralizedIdentifier<'_> as BorshDeserialize>::try_from_slice\n\n"
