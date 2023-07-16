plain
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
[RUSTC-TIMING] rustc_attr test:false 7.988
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_monomorphize/src/collector.rs:940:13
   0:     0x7ffa4e46c8b0 - std::backtrace_rs::backtrace::libunwind::trace::hd86347086d4d9848
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7ffa4e46c8b0 - std::backtrace_rs::backtrace::trace_unsynchronized::h471f843ea3342d4e
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7ffa4e46c8b0 - std::sys_common::backtrace::_print_fmt::ha5b71df9f208c9e1
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7ffa4e46c8b0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4f8935cce220fb7c
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7ffa4e4cba5c - core::fmt::write::hf970a4d7eb447230
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/core/src/fmt/mod.rs:1202:17
   5:     0x7ffa4e45cf95 - std::io::Write::write_fmt::ha54260ad7296009e
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/io/mod.rs:1679:15
   6:     0x7ffa4e46f7e1 - std::sys_common::backtrace::_print::hdfdcc8a9593c812b
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7ffa4e46f7e1 - std::sys_common::backtrace::print::h4fc4cbbeedef9395
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7ffa4e46f7e1 - std::panicking::default_hook::{{closure}}::h9e815e095d29d239
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/panicking.rs:267:22
   9:     0x7ffa4e46f49e - std::panicking::default_hook::hc19acee535433356
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/panicking.rs:286:9
  10:     0x7ffa4b437b86 - rustc_driver[e88ab1e0ee5011ab]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7ffa4e470058 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hdc766bdbadd67bed
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/alloc/src/boxed.rs:1954:9
  12:     0x7ffa4e470058 - std::panicking::rust_panic_with_hook::hbe4a665e90dad3db
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/panicking.rs:673:13
  13:     0x7ffa4e46fe59 - std::panicking::begin_panic_handler::{{closure}}::h1f0acd1fe5441dca
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/panicking.rs:558:13
  14:     0x7ffa4e46cdd4 - std::sys_common::backtrace::__rust_end_short_backtrace::hca6b38c154bab897
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/sys_common/backtrace.rs:138:18
  15:     0x7ffa4e46fb92 - rust_begin_unwind
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/panicking.rs:556:5
  16:     0x7ffa4e4c85f3 - core::panicking::panic_fmt::h7574974d27e60ac2
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/core/src/panicking.rs:142:14
  17:     0x7ffa4e4c843d - core::panicking::panic::h3d77d8ba5e42b73d
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/core/src/panicking.rs:48:5
  18:     0x7ffa4b8b3462 - <rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::MirNeighborCollector as rustc_middle[c82706f5e99477f9]::mir::visit::Visitor>::visit_terminator
  19:     0x7ffa4b8b9741 - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_neighbours
  20:     0x7ffa4b8b71bb - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  21:     0x7ffa4b8b78dd - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  22:     0x7ffa4b8b78dd - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  23:     0x7ffa4b8b78dd - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  24:     0x7ffa4b8b78dd - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  25:     0x7ffa4b8b78dd - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  26:     0x7ffa4b8b78dd - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  27:     0x7ffa4b8b78dd - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  28:     0x7ffa4b8b78dd - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  29:     0x7ffa4b8b78dd - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  30:     0x7ffa4b8f071d - <core[2bb8d0aa17313d7]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[75a2095c119064ec]::sync::par_for_each_in<alloc[50cd12edb6646b8b]::vec::Vec<rustc_middle[c82706f5e99477f9]::mir::mono::MonoItem>, rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[2bb8d0aa17313d7]::ops::function::FnOnce<()>>::call_once
  31:     0x7ffa4b8ebfe5 - std[734a5d07e7dea0b5]::panicking::try::<(), core[2bb8d0aa17313d7]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[75a2095c119064ec]::sync::par_for_each_in<alloc[50cd12edb6646b8b]::vec::Vec<rustc_middle[c82706f5e99477f9]::mir::mono::MonoItem>, rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  32:     0x7ffa4b8cf481 - <rustc_session[860b62581d5940c8]::session::Session>::time::<(), rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_crate_mono_items::{closure#1}>
  33:     0x7ffa4b8b4fe2 - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_crate_mono_items
  34:     0x7ffa4b8ca935 - rustc_monomorphize[b6fb6c5a7eb33ed1]::partitioning::collect_and_partition_mono_items
  35:     0x7ffa4cc4942d - rustc_query_system[ff407481ba3fdf0a]::query::plumbing::try_execute_query::<rustc_query_impl[352f51d90f435fc8]::plumbing::QueryCtxt, rustc_query_system[ff407481ba3fdf0a]::query::caches::DefaultCache<(), (&std[734a5d07e7dea0b5]::collections::hash::set::HashSet<rustc_span[834d17fc4ad3406c]::def_id::DefId, core[2bb8d0aa17313d7]::hash::BuildHasherDefault<rustc_hash[5034897b462db3a3]::FxHasher>>, &[rustc_middle[c82706f5e99477f9]::mir::mono::CodegenUnit])>>
  36:     0x7ffa4ccd9f9e - rustc_query_system[ff407481ba3fdf0a]::query::plumbing::get_query::<rustc_query_impl[352f51d90f435fc8]::queries::collect_and_partition_mono_items, rustc_query_impl[352f51d90f435fc8]::plumbing::QueryCtxt>
  37:     0x7ffa4cea098b - <rustc_query_impl[352f51d90f435fc8]::Queries as rustc_middle[c82706f5e99477f9]::ty::query::QueryEngine>::collect_and_partition_mono_items
  38:     0x7ffa4b6f4807 - rustc_codegen_ssa[8a38c4617f0ae3b7]::base::codegen_crate::<rustc_codegen_llvm[e64b4b2756031fdd]::LlvmCodegenBackend>
  39:     0x7ffa4b7a7bbd - <rustc_codegen_llvm[e64b4b2756031fdd]::LlvmCodegenBackend as rustc_codegen_ssa[8a38c4617f0ae3b7]::traits::backend::CodegenBackend>::codegen_crate
  40:     0x7ffa4b5b4b65 - <rustc_interface[af109c0bf5e7882b]::passes::QueryContext>::enter::<<rustc_interface[af109c0bf5e7882b]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[2bb8d0aa17313d7]::result::Result<alloc[50cd12edb6646b8b]::boxed::Box<dyn core[2bb8d0aa17313d7]::any::Any>, rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>
  41:     0x7ffa4b59edeb - <rustc_interface[af109c0bf5e7882b]::queries::Queries>::ongoing_codegen
  42:     0x7ffa4b46b4c0 - rustc_interface[af109c0bf5e7882b]::interface::create_compiler_and_run::<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>, rustc_driver[e88ab1e0ee5011ab]::run_compiler::{closure#1}>
  43:     0x7ffa4b4db682 - <scoped_tls[da5adb31d26bd224]::ScopedKey<rustc_span[834d17fc4ad3406c]::SessionGlobals>>::set::<rustc_interface[af109c0bf5e7882b]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>, rustc_driver[e88ab1e0ee5011ab]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>
  44:     0x7ffa4b48b9ef - std[734a5d07e7dea0b5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[af109c0bf5e7882b]::util::run_in_thread_pool_with_globals<rustc_interface[af109c0bf5e7882b]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>, rustc_driver[e88ab1e0ee5011ab]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>
  45:     0x7ffa4b48cc09 - <<std[734a5d07e7dea0b5]::thread::Builder>::spawn_unchecked_<rustc_interface[af109c0bf5e7882b]::util::run_in_thread_pool_with_globals<rustc_interface[af109c0bf5e7882b]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>, rustc_driver[e88ab1e0ee5011ab]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>::{closure#1} as core[2bb8d0aa17313d7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:     0x7ffa4e47a563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb278eb53bcad769e
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/alloc/src/boxed.rs:1940:9
  47:     0x7ffa4e47a563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf56f5a379975cbe0
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/alloc/src/boxed.rs:1940:9
  48:     0x7ffa4e47a563 - std::sys::unix::thread::Thread::new::thread_start::h5bbd14dda35b01a2
                               at /rustc/2f926d4179897884e344ddec25c14ae770818d18/library/std/src/sys/unix/thread.rs:108:17
  49:     0x7ffa49c14ea5 - start_thread
  50:     0x7ffa4993db0d - clone
  51:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (2f926d417 2022-09-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C link-args=-Wl,--icf=all -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
[RUSTC-TIMING] rustc_expand test:false 2.445
error: could not compile `rustc_expand`
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_errors test:false 14.587
