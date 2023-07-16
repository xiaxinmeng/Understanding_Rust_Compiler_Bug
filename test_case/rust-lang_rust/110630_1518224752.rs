
#0 92.22 error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:195:90: Failed to normalize alloc::raw_vec::RawVec<<T as store::BitStore>::Mem>, maybe try to call `try_normalize_erasing_regions` instead
#0 92.22
#0 92.22 thread 'rustc' panicked at 'Box<dyn Any>', /rustc/39c6804b92aa202369e402525cee329556bc1db0/compiler/rustc_errors/src/lib.rs:1643:9
#0 92.22 stack backtrace:
#0 92.23    0:     0xffff7f92d174 - std::backtrace_rs::backtrace::libunwind::trace::h12be91c7b6627c6d
#0 92.24                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
#0 92.24    1:     0xffff7f92d174 - std::backtrace_rs::backtrace::trace_unsynchronized::h4bdab7ec283fe3be
#0 92.24                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
#0 92.24    2:     0xffff7f92d174 - std::sys_common::backtrace::_print_fmt::he3e23b7445a93ab7
#0 92.24                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/std/src/sys_common/backtrace.rs:65:5
#0 92.24    3:     0xffff7f92d174 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0b8e6c555078be05
#0 92.24                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/std/src/sys_common/backtrace.rs:44:22
#0 92.24    4:     0xffff7f984638 - core::fmt::write::h552f8b618807d728
#0 92.24                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/core/src/fmt/mod.rs:1254:17
#0 92.24    5:     0xffff7f922548 - std::io::Write::write_fmt::h152cf3f5e88b0a25
#0 92.24                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/std/src/io/mod.rs:1698:15
#0 92.24    6:     0xffff7f92cfc8 - std::sys_common::backtrace::_print::h9f38c7a813da777d
#0 92.24                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/std/src/sys_common/backtrace.rs:47:5
#0 92.24    7:     0xffff7f92cfc8 - std::sys_common::backtrace::print::hbf1753281d02ce4f
#0 92.24                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/std/src/sys_common/backtrace.rs:34:9
#0 92.24    8:     0xffff7f92f8ec - std::panicking::default_hook::{{closure}}::hb1ad0964de2ba130
#0 92.24    9:     0xffff7f92f6dc - std::panicking::default_hook::h7cdfed1a64e285f9
#0 92.24                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/std/src/panicking.rs:288:9
#0 92.26   10:     0xffff806c6a64 - rustc_driver_impl[834bde73645836c3]::DEFAULT_HOOK::{closure#0}::{closure#0}
#0 92.26   11:     0xffff7f92ff68 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hed64ebfa41cbc02a
#0 92.26                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/alloc/src/boxed.rs:1976:9
#0 92.27   12:     0xffff7f92ff68 - std::panicking::rust_panic_with_hook::h317e3aa4f76fcfc7
#0 92.27                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/std/src/panicking.rs:695:13
#0 92.27   13:     0xffff851795e8 - std[ee9b954e96d984cc]::panicking::begin_panic::<rustc_errors[f6729713c501e706]::ExplicitBug>::{closure#0}
#0 92.27   14:     0xffff8517723c - std[ee9b954e96d984cc]::sys_common::backtrace::__rust_end_short_backtrace::<std[ee9b954e96d984cc]::panicking::begin_panic<rustc_errors[f6729713c501e706]::ExplicitBug>::{closure#0}, !>
#0 92.27   15:     0xffff805a4c34 - std[ee9b954e96d984cc]::panicking::begin_panic::<rustc_errors[f6729713c501e706]::ExplicitBug>
#0 92.27   16:     0xffff852545e8 - <rustc_errors[f6729713c501e706]::HandlerInner>::bug::<&alloc[73ea3e3568c2066d]::string::String>
#0 92.27   17:     0xffff85254298 - <rustc_errors[f6729713c501e706]::Handler>::bug::<&alloc[73ea3e3568c2066d]::string::String>
#0 92.27   18:     0xffff852198a0 - rustc_middle[8ab10741ae762805]::util::bug::opt_span_bug_fmt::<rustc_span[566bd16bcf1e2cd5]::span_encoding::Span>::{closure#0}
#0 92.27   19:     0xffff85217248 - rustc_middle[8ab10741ae762805]::ty::context::tls::with_opt::<rustc_middle[8ab10741ae762805]::util::bug::opt_span_bug_fmt<rustc_span[566bd16bcf1e2cd5]::span_encoding::Span>::{closure#0}, !>::{closure#0}
#0 92.27   20:     0xffff85217214 - rustc_middle[8ab10741ae762805]::ty::context::tls::with_context_opt::<rustc_middle[8ab10741ae762805]::ty::context::tls::with_opt<rustc_middle[8ab10741ae762805]::util::bug::opt_span_bug_fmt<rustc_span[566bd16bcf1e2cd5]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
#0 92.27   21:     0xffff805a4ef4 - rustc_middle[8ab10741ae762805]::util::bug::bug_fmt
#0 92.27   22:     0xffff851e7404 - <rustc_middle[8ab10741ae762805]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::normalize_generic_arg_after_erasing_regions
#0 92.27   23:     0xffff851e7444 - <rustc_middle[8ab10741ae762805]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_type_ir[91f2e8daaf6907fc]::fold::TypeFolder<rustc_middle[8ab10741ae762805]::ty::context::TyCtxt>>::fold_ty
#0 92.27   24:     0xffff83783a30 - <core[2d48004a85dc5565]::iter::adapters::map::Map<core[2d48004a85dc5565]::iter::adapters::enumerate::Enumerate<core[2d48004a85dc5565]::slice::iter::Iter<rustc_middle[8ab10741ae762805]::ty::FieldDef>>, <rustc_mir_dataflow[509d19dec06e5cb0]::elaborate_drops::DropCtxt<rustc_mir_transform[2a96cee0d5989ce9]::shim::DropShimElaborator>>::move_paths_for_fields::{closure#0}> as core[2d48004a85dc5565]::iter::traits::iterator::Iterator>::fold::<(), core[2d48004a85dc5565]::iter::traits::iterator::Iterator::for_each::call<(rustc_middle[8ab10741ae762805]::mir::syntax::Place, core[2d48004a85dc5565]::option::Option<()>), <alloc[73ea3e3568c2066d]::vec::Vec<(rustc_middle[8ab10741ae762805]::mir::syntax::Place, core[2d48004a85dc5565]::option::Option<()>)>>::extend_trusted<core[2d48004a85dc5565]::iter::adapters::map::Map<core[2d48004a85dc5565]::iter::adapters::enumerate::Enumerate<core[2d48004a85dc5565]::slice::iter::Iter<rustc_middle[8ab10741ae762805]::ty::FieldDef>>, <rustc_mir_dataflow[509d19dec06e5cb0]::elaborate_drops::DropCtxt<rustc_mir_transform[2a96cee0d5989ce9]::shim::DropShimElaborator>>::move_paths_for_fields::{closure#0}>>::{closure#0}>::{closure#0}>
#0 92.27   25:     0xffff838b3d04 - <alloc[73ea3e3568c2066d]::vec::Vec<(rustc_middle[8ab10741ae762805]::mir::syntax::Place, core[2d48004a85dc5565]::option::Option<()>)> as alloc[73ea3e3568c2066d]::vec::spec_from_iter::SpecFromIter<(rustc_middle[8ab10741ae762805]::mir::syntax::Place, core[2d48004a85dc5565]::option::Option<()>), core[2d48004a85dc5565]::iter::adapters::map::Map<core[2d48004a85dc5565]::iter::adapters::enumerate::Enumerate<core[2d48004a85dc5565]::slice::iter::Iter<rustc_middle[8ab10741ae762805]::ty::FieldDef>>, <rustc_mir_dataflow[509d19dec06e5cb0]::elaborate_drops::DropCtxt<rustc_mir_transform[2a96cee0d5989ce9]::shim::DropShimElaborator>>::move_paths_for_fields::{closure#0}>>>::from_iter
#0 92.27   26:     0xffff83790cdc - <rustc_mir_dataflow[509d19dec06e5cb0]::elaborate_drops::DropCtxt<rustc_mir_transform[2a96cee0d5989ce9]::shim::DropShimElaborator>>::open_drop_for_adt
#0 92.27   27:     0xffff83790648 - <rustc_mir_dataflow[509d19dec06e5cb0]::elaborate_drops::DropCtxt<rustc_mir_transform[2a96cee0d5989ce9]::shim::DropShimElaborator>>::elaborate_drop
#0 92.27   28:     0xffff838d8028 - rustc_mir_transform[2a96cee0d5989ce9]::shim::make_shim
#0 92.27   29:     0xffff843fc124 - <std[ee9b954e96d984cc]::thread::local::LocalKey<core[2d48004a85dc5565]::cell::Cell<*const ()>>>::with::<rustc_middle[8ab10741ae762805]::ty::context::tls::enter_context<rustc_query_system[d43ab53be31e41e1]::query::plumbing::execute_job_non_incr<rustc_query_impl[d86a9383aa5ae983]::queries::mir_shims, rustc_query_impl[d86a9383aa5ae983]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[8ab10741ae762805]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[8ab10741ae762805]::query::erase::Erased<[u8; 8usize]>>
#0 92.27   30:     0xffff8467a4cc - rustc_query_system[d43ab53be31e41e1]::query::plumbing::try_execute_query::<rustc_query_impl[d86a9383aa5ae983]::queries::mir_shims, rustc_query_impl[d86a9383aa5ae983]::plumbing::QueryCtxt>
#0 92.27   31:     0xffff84511e78 - <rustc_query_impl[d86a9383aa5ae983]::Queries as rustc_middle[8ab10741ae762805]::ty::query::QueryEngine>::mir_shims
#0 92.27   32:     0xffff852b8e24 - <rustc_middle[8ab10741ae762805]::ty::context::TyCtxt>::instance_mir
#0 92.27   33:     0xffff838986ec - <rustc_mir_transform[2a96cee0d5989ce9]::inline::Inliner>::try_inlining
#0 92.27   34:     0xffff83897fc4 - <rustc_mir_transform[2a96cee0d5989ce9]::inline::Inliner>::process_blocks
#0 92.27   35:     0xffff838980dc - <rustc_mir_transform[2a96cee0d5989ce9]::inline::Inliner>::process_blocks
#0 92.27   36:     0xffff83897a88 - <rustc_mir_transform[2a96cee0d5989ce9]::inline::Inline as rustc_middle[8ab10741ae762805]::mir::MirPass>::run_pass
#0 92.27   37:     0xffff838d23a4 - rustc_mir_transform[2a96cee0d5989ce9]::pass_manager::run_passes_inner
#0 92.27   38:     0xffff83819ec4 - rustc_mir_transform[2a96cee0d5989ce9]::optimized_mir
#0 92.28   39:     0xffff84607c04 - rustc_query_system[d43ab53be31e41e1]::query::plumbing::try_execute_query::<rustc_query_impl[d86a9383aa5ae983]::queries::optimized_mir, rustc_query_impl[d86a9383aa5ae983]::plumbing::QueryCtxt>
#0 92.28   40:     0xffff84508368 - <rustc_query_impl[d86a9383aa5ae983]::Queries as rustc_middle[8ab10741ae762805]::ty::query::QueryEngine>::optimized_mir
#0 92.28   41:     0xffff84a03614 - <rustc_metadata[6fb9eab109fb1453]::rmeta::encoder::EncodeContext>::encode_crate_root
#0 92.28   42:     0xffff84a10ba4 - rustc_metadata[6fb9eab109fb1453]::rmeta::encoder::encode_metadata_impl
#0 92.28   43:     0xffff849b2480 - rustc_data_structures[3d18d37d82609441]::sync::join::<rustc_metadata[6fb9eab109fb1453]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[6fb9eab109fb1453]::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
#0 92.28   44:     0xffff84a103c4 - rustc_metadata[6fb9eab109fb1453]::rmeta::encoder::encode_metadata
#0 92.28   45:     0xffff8499a980 - rustc_metadata[6fb9eab109fb1453]::fs::encode_and_write_metadata
#0 92.28   46:     0xffff8078b2a0 - rustc_interface[7da19b7ad2a25e97]::passes::start_codegen
#0 92.28   47:     0xffff80776d34 - <rustc_middle[8ab10741ae762805]::ty::context::GlobalCtxt>::enter::<<rustc_interface[7da19b7ad2a25e97]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[2d48004a85dc5565]::result::Result<alloc[73ea3e3568c2066d]::boxed::Box<dyn core[2d48004a85dc5565]::any::Any>, rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>>
#0 92.28   48:     0xffff8073fd58 - <rustc_interface[7da19b7ad2a25e97]::queries::Queries>::ongoing_codegen
#0 92.28   49:     0xffff8069b230 - <rustc_interface[7da19b7ad2a25e97]::interface::Compiler>::enter::<rustc_driver_impl[834bde73645836c3]::run_compiler::{closure#1}::{closure#2}, core[2d48004a85dc5565]::result::Result<core[2d48004a85dc5565]::option::Option<rustc_interface[7da19b7ad2a25e97]::queries::Linker>, rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>>
#0 92.28   50:     0xffff806d10f0 - rustc_span[566bd16bcf1e2cd5]::set_source_map::<core[2d48004a85dc5565]::result::Result<(), rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>, rustc_interface[7da19b7ad2a25e97]::interface::run_compiler<core[2d48004a85dc5565]::result::Result<(), rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>, rustc_driver_impl[834bde73645836c3]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
#0 92.29   51:     0xffff806799bc - <scoped_tls[922f07b1016f6b55]::ScopedKey<rustc_span[566bd16bcf1e2cd5]::SessionGlobals>>::set::<rustc_interface[7da19b7ad2a25e97]::interface::run_compiler<core[2d48004a85dc5565]::result::Result<(), rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>, rustc_driver_impl[834bde73645836c3]::run_compiler::{closure#1}>::{closure#0}, core[2d48004a85dc5565]::result::Result<(), rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>>
#0 92.29   52:     0xffff806d6464 - std[ee9b954e96d984cc]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7da19b7ad2a25e97]::util::run_in_thread_pool_with_globals<rustc_interface[7da19b7ad2a25e97]::interface::run_compiler<core[2d48004a85dc5565]::result::Result<(), rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>, rustc_driver_impl[834bde73645836c3]::run_compiler::{closure#1}>::{closure#0}, core[2d48004a85dc5565]::result::Result<(), rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2d48004a85dc5565]::result::Result<(), rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>>
#0 92.29   53:     0xffff806e3d1c - <<std[ee9b954e96d984cc]::thread::Builder>::spawn_unchecked_<rustc_interface[7da19b7ad2a25e97]::util::run_in_thread_pool_with_globals<rustc_interface[7da19b7ad2a25e97]::interface::run_compiler<core[2d48004a85dc5565]::result::Result<(), rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>, rustc_driver_impl[834bde73645836c3]::run_compiler::{closure#1}>::{closure#0}, core[2d48004a85dc5565]::result::Result<(), rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2d48004a85dc5565]::result::Result<(), rustc_span[566bd16bcf1e2cd5]::ErrorGuaranteed>>::{closure#1} as core[2d48004a85dc5565]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
#0 92.29   54:     0xffff7f939620 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h82ef39943f2999bd
#0 92.29                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/alloc/src/boxed.rs:1962:9
#0 92.29   55:     0xffff7f939620 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hae7c2ed10f6de47e
#0 92.29                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/alloc/src/boxed.rs:1962:9
#0 92.29   56:     0xffff7f939620 - std::sys::unix::thread::Thread::new::thread_start::h5463bf79a61ca5d5
#0 92.29                                at /rustc/39c6804b92aa202369e402525cee329556bc1db0/library/std/src/sys/unix/thread.rs:108:17
#0 92.29   57:     0xffff7f6b1648 - start_thread
#0 92.29   58:     0xffff7f7e5c1c - <unknown>
#0 92.29   59:                0x0 - <unknown>
#0 92.29
#0 92.29 note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
#0 92.29
#0 92.29 note: rustc 1.71.0-nightly (39c6804b9 2023-04-19) running on aarch64-unknown-linux-gnu
#0 92.29
#0 92.29 note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no
#0 92.29
#0 92.29 note: some of the compiler flags provided by cargo are hidden
#0 92.29
#0 92.29 query stack during panic:
#0 92.29 #0 [mir_shims] generating MIR shim for `core::ptr::drop_in_place`
#0 92.29 #1 [optimized_mir] optimizing MIR for `vec::ops::<impl at /usr/local/cargo/registry/src/index.crates.io-6f17d22bba15001f/bitvec-0.18.5/src/vec/ops.rs:141:1: 141:33>::drop::{closure#0}`
#0 92.29 end of query stack
#0 92.34 error: could not compile `bitvec` (lib)
#0 92.34 warning: build failed, waiting for other jobs to finish.
