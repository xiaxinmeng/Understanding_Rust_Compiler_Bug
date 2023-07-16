
   Compiling anstream v0.3.2
error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:195:90: Failed to normalize alloc::raw_vec::RawVec<<T as store::BitStore>::Mem>, maybe try to call `try_normalize_erasing_regions` instead


thread 'rustc' panicked at 'Box<dyn Any>', /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:     0x7f82137696e1 - std::backtrace_rs::backtrace::libunwind::trace::h4354896f1663baaf
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f82137696e1 - std::backtrace_rs::backtrace::trace_unsynchronized::h8ac49f89c23585dd
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f82137696e1 - std::sys_common::backtrace::_print_fmt::h9f5f16b3ef080000
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f82137696e1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcfed927151d1ad83
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f82137c9bdf - core::fmt::rt::Argument::fmt::h12fb43eea2fe23a8
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/core/src/fmt/rt.rs:138:9
   5:     0x7f82137c9bdf - core::fmt::write::hcf94a34baaaea06f
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/core/src/fmt/mod.rs:1094:21
   6:     0x7f821375c941 - std::io::Write::write_fmt::h60003491edc2e074
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/io/mod.rs:1712:15
   7:     0x7f82137694f5 - std::sys_common::backtrace::_print::h9a8311322b9d8667
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x7f82137694f5 - std::sys_common::backtrace::print::h8503eaeeae92ea08
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x7f821376c177 - std::panicking::default_hook::{{closure}}::h36a123b73b99c0f3
  10:     0x7f821376bf64 - std::panicking::default_hook::ha2efb9fc5f628e61
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/panicking.rs:288:9
  11:     0x7f82168c87db - rustc_driver_impl[64b7dd1911194932]::install_ice_hook::{closure#0}
  12:     0x7f821376c897 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h250e911c4e8d011a
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/alloc/src/boxed.rs:1999:9
  13:     0x7f821376c897 - std::panicking::rust_panic_with_hook::h5d9e02f555bc48d2
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/panicking.rs:695:13
  14:     0x7f8216dc5901 - std[d9d2839bce1dd559]::panicking::begin_panic::<rustc_errors[fcec483c501eb906]::ExplicitBug>::{closure#0}
  15:     0x7f8216dbead6 - std[d9d2839bce1dd559]::sys_common::backtrace::__rust_end_short_backtrace::<std[d9d2839bce1dd559]::panicking::begin_panic<rustc_errors[fcec483c501eb906]::ExplicitBug>::{closure#0}, !>
  16:     0x7f8216dbe406 - std[d9d2839bce1dd559]::panicking::begin_panic::<rustc_errors[fcec483c501eb906]::ExplicitBug>
  17:     0x7f8216d99334 - <rustc_errors[fcec483c501eb906]::HandlerInner>::bug::<alloc[d2b0d467f91dc298]::string::String>
  18:     0x7f8216d98f86 - <rustc_errors[fcec483c501eb906]::Handler>::bug::<alloc[d2b0d467f91dc298]::string::String>
  19:     0x7f8216e41bdc - rustc_middle[f607de435e9a4418]::util::bug::opt_span_bug_fmt::<rustc_span[f3562a9d228bc34b]::span_encoding::Span>::{closure#0}
  20:     0x7f8216e3e25a - rustc_middle[f607de435e9a4418]::ty::context::tls::with_opt::<rustc_middle[f607de435e9a4418]::util::bug::opt_span_bug_fmt<rustc_span[f3562a9d228bc34b]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0x7f8216e3e22a - rustc_middle[f607de435e9a4418]::ty::context::tls::with_context_opt::<rustc_middle[f607de435e9a4418]::ty::context::tls::with_opt<rustc_middle[f607de435e9a4418]::util::bug::opt_span_bug_fmt<rustc_span[f3562a9d228bc34b]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0x7f8214c1cb8d - rustc_middle[f607de435e9a4418]::util::bug::bug_fmt
  23:     0x7f8214e1a174 - <rustc_middle[f607de435e9a4418]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_type_ir[8300f6efcd370450]::fold::TypeFolder<rustc_middle[f607de435e9a4418]::ty::context::TyCtxt>>::fold_ty
  24:     0x7f8215b93f51 - <core[b4a8d54be4c63c12]::iter::adapters::map::Map<core[b4a8d54be4c63c12]::iter::adapters::enumerate::Enumerate<core[b4a8d54be4c63c12]::slice::iter::Iter<rustc_middle[f607de435e9a4418]::ty::FieldDef>>, <rustc_mir_dataflow[52dce49772f251c0]::elaborate_drops::DropCtxt<rustc_mir_transform[27868256649f483d]::shim::DropShimElaborator>>::move_paths_for_fields::{closure#0}> as core[b4a8d54be4c63c12]::iter::traits::iterator::Iterator>::fold::<(), core[b4a8d54be4c63c12]::iter::traits::iterator::Iterator::for_each::call<(rustc_middle[f607de435e9a4418]::mir::syntax::Place, core[b4a8d54be4c63c12]::option::Option<()>), <alloc[d2b0d467f91dc298]::vec::Vec<(rustc_middle[f607de435e9a4418]::mir::syntax::Place, core[b4a8d54be4c63c12]::option::Option<()>)>>::extend_trusted<core[b4a8d54be4c63c12]::iter::adapters::map::Map<core[b4a8d54be4c63c12]::iter::adapters::enumerate::Enumerate<core[b4a8d54be4c63c12]::slice::iter::Iter<rustc_middle[f607de435e9a4418]::ty::FieldDef>>, <rustc_mir_dataflow[52dce49772f251c0]::elaborate_drops::DropCtxt<rustc_mir_transform[27868256649f483d]::shim::DropShimElaborator>>::move_paths_for_fields::{closure#0}>>::{closure#0}>::{closure#0}>
  25:     0x7f8215b93dba - <alloc[d2b0d467f91dc298]::vec::Vec<(rustc_middle[f607de435e9a4418]::mir::syntax::Place, core[b4a8d54be4c63c12]::option::Option<()>)> as alloc[d2b0d467f91dc298]::vec::spec_from_iter::SpecFromIter<(rustc_middle[f607de435e9a4418]::mir::syntax::Place, core[b4a8d54be4c63c12]::option::Option<()>), core[b4a8d54be4c63c12]::iter::adapters::map::Map<core[b4a8d54be4c63c12]::iter::adapters::enumerate::Enumerate<core[b4a8d54be4c63c12]::slice::iter::Iter<rustc_middle[f607de435e9a4418]::ty::FieldDef>>, <rustc_mir_dataflow[52dce49772f251c0]::elaborate_drops::DropCtxt<rustc_mir_transform[27868256649f483d]::shim::DropShimElaborator>>::move_paths_for_fields::{closure#0}>>>::from_iter
  26:     0x7f8215b926b2 - <rustc_mir_dataflow[52dce49772f251c0]::elaborate_drops::DropCtxt<rustc_mir_transform[27868256649f483d]::shim::DropShimElaborator>>::elaborate_drop
  27:     0x7f8215b8da39 - rustc_mir_transform[27868256649f483d]::shim::make_shim
  28:     0x7f8215b4ed1c - rustc_query_system[7835b3e98a87b812]::query::plumbing::try_execute_query::<rustc_query_impl[29ff09dda6aeedc5]::DynamicConfig<rustc_query_system[7835b3e98a87b812]::query::caches::DefaultCache<rustc_middle[f607de435e9a4418]::ty::instance::InstanceDef, rustc_middle[f607de435e9a4418]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[29ff09dda6aeedc5]::plumbing::QueryCtxt>
  29:     0x7f821628a0a2 - rustc_query_impl[29ff09dda6aeedc5]::get_query::mir_shims
  30:     0x7f82152741d1 - <rustc_middle[f607de435e9a4418]::ty::context::TyCtxt>::instance_mir
  31:     0x7f82155d0db9 - <rustc_mir_transform[27868256649f483d]::inline::Inliner>::try_inlining
  32:     0x7f82155cd330 - <rustc_mir_transform[27868256649f483d]::inline::Inliner>::process_blocks
  33:     0x7f82155cd3ea - <rustc_mir_transform[27868256649f483d]::inline::Inliner>::process_blocks
  34:     0x7f82155ccb11 - <rustc_mir_transform[27868256649f483d]::inline::Inline as rustc_middle[f607de435e9a4418]::mir::MirPass>::run_pass
  35:     0x7f8215c3968d - rustc_mir_transform[27868256649f483d]::optimized_mir
  36:     0x7f8215c390d5 - <rustc_query_impl[29ff09dda6aeedc5]::dynamic_query::optimized_mir::{closure#2} as core[b4a8d54be4c63c12]::ops::function::FnOnce<(rustc_middle[f607de435e9a4418]::ty::context::TyCtxt, rustc_span[f3562a9d228bc34b]::def_id::DefId)>>::call_once
  37:     0x7f8214b24ae4 - rustc_query_system[7835b3e98a87b812]::query::plumbing::try_execute_query::<rustc_query_impl[29ff09dda6aeedc5]::DynamicConfig<rustc_query_system[7835b3e98a87b812]::query::caches::DefaultCache<rustc_span[f3562a9d228bc34b]::def_id::DefId, rustc_middle[f607de435e9a4418]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[29ff09dda6aeedc5]::plumbing::QueryCtxt>
  38:     0x7f82162836f6 - rustc_query_impl[29ff09dda6aeedc5]::get_query::optimized_mir
  39:     0x7f8215679ac2 - <rustc_metadata[a0efc5a83e2521c5]::rmeta::encoder::EncodeContext>::encode_crate_root
  40:     0x7f821567533d - rustc_metadata[a0efc5a83e2521c5]::rmeta::encoder::encode_metadata_impl
  41:     0x7f8215e78ab6 - rustc_metadata[a0efc5a83e2521c5]::rmeta::encoder::encode_metadata
  42:     0x7f8215e7744e - rustc_metadata[a0efc5a83e2521c5]::fs::encode_and_write_metadata
  43:     0x7f8215e67c60 - rustc_interface[d75252421fea963]::passes::start_codegen
  44:     0x7f8215e62557 - <rustc_middle[f607de435e9a4418]::ty::context::GlobalCtxt>::enter::<<rustc_interface[d75252421fea963]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[b4a8d54be4c63c12]::result::Result<alloc[d2b0d467f91dc298]::boxed::Box<dyn core[b4a8d54be4c63c12]::any::Any>, rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>
  45:     0x7f8215e609bd - <rustc_interface[d75252421fea963]::queries::Queries>::ongoing_codegen
  46:     0x7f8215e60009 - <rustc_interface[d75252421fea963]::interface::Compiler>::enter::<rustc_driver_impl[64b7dd1911194932]::run_compiler::{closure#1}::{closure#2}, core[b4a8d54be4c63c12]::result::Result<core[b4a8d54be4c63c12]::option::Option<rustc_interface[d75252421fea963]::queries::Linker>, rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>
  47:     0x7f8215e5dcfd - <scoped_tls[36ad800287692b7a]::ScopedKey<rustc_span[f3562a9d228bc34b]::SessionGlobals>>::set::<rustc_interface[d75252421fea963]::interface::run_compiler<core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>, rustc_driver_impl[64b7dd1911194932]::run_compiler::{closure#1}>::{closure#0}, core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>
  48:     0x7f8215e5d156 - std[d9d2839bce1dd559]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d75252421fea963]::util::run_in_thread_pool_with_globals<rustc_interface[d75252421fea963]::interface::run_compiler<core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>, rustc_driver_impl[64b7dd1911194932]::run_compiler::{closure#1}>::{closure#0}, core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>
  49:     0x7f8215e5cf05 - <<std[d9d2839bce1dd559]::thread::Builder>::spawn_unchecked_<rustc_interface[d75252421fea963]::util::run_in_thread_pool_with_globals<rustc_interface[d75252421fea963]::interface::run_compiler<core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>, rustc_driver_impl[64b7dd1911194932]::run_compiler::{closure#1}>::{closure#0}, core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>::{closure#1} as core[b4a8d54be4c63c12]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f8213776d45 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0ee7a0a44efbcb8b
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/alloc/src/boxed.rs:1985:9
  51:     0x7f8213776d45 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h8f984f8b1c075279
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/alloc/src/boxed.rs:1985:9
  52:     0x7f8213776d45 - std::sys::unix::thread::Thread::new::thread_start::h054937194df87b6d
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys/unix/thread.rs:108:17
  53:     0x7f82134d144b - <unknown>
  54:     0x7f8213554e40 - <unknown>
  55:                0x0 - <unknown>

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (18bfe5d8a 2023-05-14) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type rlib -C opt-level=3 -C linker-plugin-lto -Z unstable-options

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_shims] generating MIR shim for `core::ptr::drop_in_place`
#1 [optimized_mir] optimizing MIR for `vec::ops::<impl at /home/sanoj/.local/share/cargo/registry/src/index.crates.io-6f17d22bba15001f/bitvec-0.19.6/src/vec/ops.rs:141:1: 141:33>::drop::{closure#0}`
end of query stack
error: could not compile `bitvec` (lib)
warning: build failed, waiting for other jobs to finish...
