plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error: internal compiler error: compiler/rustc_mir_build/src/build/mod.rs:872:17: no terminator on block 7
    |
    |
158 | /     fn new<T>(header: H, value: T) -> WithHeader<H> {
159 | |         let value_layout = Layout::new::<T>();
160 | |         let Ok((layout, value_offset)) = Self::alloc_layout(value_layout) else {
161 | |             // We pass an empty layout here because we do not know which layout caused the
198 | |         }
199 | |     }
    | |_____^


thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
   0:     0x7f3385815e12 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7f338587da98 - core::fmt::write::ha01458c252ca8e28
   2:     0x7f33858060e1 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7f3385819129 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7f3385818dca - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7f3386315cf4 - rustc_driver[cc240e6f4a815a62]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f338581998f - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7f33872c7333 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[81b66f48ab2827ec]::ExplicitBug>::{closure#0}
   8:     0x7f33872c5356 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[81b66f48ab2827ec]::ExplicitBug>::{closure#0}, !>
   9:     0x7f33860b4006 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[81b66f48ab2827ec]::ExplicitBug>
  10:     0x7f33872c52c6 - std[836a811975e52724]::panic::panic_any::<rustc_errors[81b66f48ab2827ec]::ExplicitBug>
  11:     0x7f33872c2833 - <rustc_errors[81b66f48ab2827ec]::HandlerInner>::span_bug::<rustc_span[ec683a5befddaf22]::span_encoding::Span, &alloc[f55ce12b9f25f528]::string::String>
  12:     0x7f33872c2100 - <rustc_errors[81b66f48ab2827ec]::Handler>::span_bug::<rustc_span[ec683a5befddaf22]::span_encoding::Span, &alloc[f55ce12b9f25f528]::string::String>
  13:     0x7f33872f9922 - rustc_middle[7c2d6da264b3b0e3]::ty::context::tls::with_opt::<rustc_middle[7c2d6da264b3b0e3]::util::bug::opt_span_bug_fmt<rustc_span[ec683a5befddaf22]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f33872f95b9 - rustc_middle[7c2d6da264b3b0e3]::util::bug::opt_span_bug_fmt::<rustc_span[ec683a5befddaf22]::span_encoding::Span>
  15:     0x7f33860b4127 - rustc_middle[7c2d6da264b3b0e3]::util::bug::span_bug_fmt::<rustc_span[ec683a5befddaf22]::span_encoding::Span>
  16:     0x7f338724caa0 - <rustc_mir_build[a7f1e46bf61e5784]::build::Builder>::finish
  17:     0x7f338724adde - rustc_mir_build[a7f1e46bf61e5784]::build::construct_fn::<core[6d9550a4e960c99f]::iter::adapters::chain::Chain<alloc[f55ce12b9f25f528]::vec::into_iter::IntoIter<rustc_mir_build[a7f1e46bf61e5784]::build::ArgInfo>, core[6d9550a4e960c99f]::iter::adapters::map::Map<core[6d9550a4e960c99f]::iter::adapters::enumerate::Enumerate<core[6d9550a4e960c99f]::slice::iter::Iter<rustc_hir[aa1b53fbe977c493]::hir::Param>>, rustc_mir_build[a7f1e46bf61e5784]::build::mir_build::{closure#1}::{closure#1}>>>
  18:     0x7f33872f0ba5 - <rustc_infer[882077c6ac31a764]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c2d6da264b3b0e3]::mir::Body, rustc_mir_build[a7f1e46bf61e5784]::build::mir_build::{closure#1}>
  19:     0x7f3387248bda - rustc_mir_build[a7f1e46bf61e5784]::build::mir_built
  20:     0x7f3387d5c65c - rustc_query_system[e920471a40e4ffdd]::query::plumbing::try_execute_query::<rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt, rustc_query_system[e920471a40e4ffdd]::query::caches::DefaultCache<rustc_middle[7c2d6da264b3b0e3]::ty::WithOptConstParam<rustc_span[ec683a5befddaf22]::def_id::LocalDefId>, &rustc_data_structures[445150d0d950bdef]::steal::Steal<rustc_middle[7c2d6da264b3b0e3]::mir::Body>>>
  21:     0x7f3387e7ccc6 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::mir_built, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  22:     0x7f33879d7fa7 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::mir_built
  23:     0x7f338699b826 - rustc_mir_transform[58eb2bf1acf89467]::check_unsafety::unsafety_check_result
  24:     0x7f33869981fc - <rustc_mir_transform[58eb2bf1acf89467]::check_unsafety::provide::{closure#0} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[7c2d6da264b3b0e3]::ty::context::TyCtxt, rustc_span[ec683a5befddaf22]::def_id::LocalDefId)>>::call_once
  25:     0x7f3387d6e604 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::try_execute_query::<rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt, rustc_query_system[e920471a40e4ffdd]::query::caches::DefaultCache<rustc_span[ec683a5befddaf22]::def_id::LocalDefId, &rustc_middle[7c2d6da264b3b0e3]::mir::query::UnsafetyCheckResult>>
  26:     0x7f3387e4ec47 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::unsafety_check_result, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  27:     0x7f33879e7a34 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::unsafety_check_result
  28:     0x7f33868fb106 - rustc_mir_transform[58eb2bf1acf89467]::mir_const
  29:     0x7f3387d5c65c - rustc_query_system[e920471a40e4ffdd]::query::plumbing::try_execute_query::<rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt, rustc_query_system[e920471a40e4ffdd]::query::caches::DefaultCache<rustc_middle[7c2d6da264b3b0e3]::ty::WithOptConstParam<rustc_span[ec683a5befddaf22]::def_id::LocalDefId>, &rustc_data_structures[445150d0d950bdef]::steal::Steal<rustc_middle[7c2d6da264b3b0e3]::mir::Body>>>
  30:     0x7f3387e7ce03 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::mir_const, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  31:     0x7f33879d84f7 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::mir_const
  32:     0x7f33868fbdae - rustc_mir_transform[58eb2bf1acf89467]::mir_promoted
  33:     0x7f3387e2c1e8 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::mir_promoted, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  34:     0x7f33879dab27 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::mir_promoted
  35:     0x7f338750342a - rustc_borrowck[1c83f97c69f3dcdb]::mir_borrowck
  36:     0x7f33874cdbec - <rustc_borrowck[1c83f97c69f3dcdb]::provide::{closure#0} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[7c2d6da264b3b0e3]::ty::context::TyCtxt, rustc_span[ec683a5befddaf22]::def_id::LocalDefId)>>::call_once
  37:     0x7f3387d6d894 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::try_execute_query::<rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt, rustc_query_system[e920471a40e4ffdd]::query::caches::DefaultCache<rustc_span[ec683a5befddaf22]::def_id::LocalDefId, &rustc_middle[7c2d6da264b3b0e3]::mir::query::BorrowCheckResult>>
  38:     0x7f3387e2bb08 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::mir_borrowck, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  39:     0x7f33879f10a4 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::mir_borrowck
  40:     0x7f33864d5f1d - <rustc_middle[7c2d6da264b3b0e3]::hir::map::Map>::par_body_owners::<rustc_interface[d24a9db69028d335]::passes::analysis::{closure#2}::{closure#0}>
  41:     0x7f338645e1d0 - <rustc_session[898bbfa7f8b71f8d]::session::Session>::time::<(), rustc_interface[d24a9db69028d335]::passes::analysis::{closure#2}>
  42:     0x7f3386446c0b - rustc_interface[d24a9db69028d335]::passes::analysis
  43:     0x7f3387da757c - rustc_query_system[e920471a40e4ffdd]::query::plumbing::try_execute_query::<rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt, rustc_query_system[e920471a40e4ffdd]::query::caches::DefaultCache<(), core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>>
  44:     0x7f3387e7aba2 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::analysis, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  45:     0x7f33879d295e - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::analysis
  46:     0x7f33863815da - <rustc_interface[d24a9db69028d335]::passes::QueryContext>::enter::<rustc_driver[cc240e6f4a815a62]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  47:     0x7f338633457d - <rustc_interface[d24a9db69028d335]::interface::Compiler>::enter::<rustc_driver[cc240e6f4a815a62]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[d24a9db69028d335]::queries::Linker>, rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  48:     0x7f3386317326 - rustc_span[ec683a5befddaf22]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_interface[d24a9db69028d335]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[cc240e6f4a815a62]::run_compiler::{closure#1}>::{closure#1}>
  49:     0x7f33863357fe - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[ec683a5befddaf22]::SessionGlobals>>::set::<rustc_interface[d24a9db69028d335]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[cc240e6f4a815a62]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  50:     0x7f3386384599 - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d24a9db69028d335]::util::run_in_thread_pool_with_globals<rustc_interface[d24a9db69028d335]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[cc240e6f4a815a62]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  51:     0x7f33863851f9 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[d24a9db69028d335]::util::run_in_thread_pool_with_globals<rustc_interface[d24a9db69028d335]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[cc240e6f4a815a62]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7f3385826353 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  53:     0x7f337fd76609 - start_thread
  54:     0x7f3385689133 - clone
  55:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (121cc6796 2022-06-28) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_built] building MIR for `boxed::thin::<impl at library/alloc/src/boxed/thin.rs:156:1: 250:2>::new`
#1 [unsafety_check_result] unsafety-checking `boxed::thin::<impl at library/alloc/src/boxed/thin.rs:156:1: 250:2>::new`
#2 [mir_const] processing MIR for `boxed::thin::<impl at library/alloc/src/boxed/thin.rs:156:1: 250:2>::new`
#3 [mir_promoted] processing `boxed::thin::<impl at library/alloc/src/boxed/thin.rs:156:1: 250:2>::new`
#4 [mir_borrowck] borrow-checking `boxed::thin::<impl at library/alloc/src/boxed/thin.rs:156:1: 250:2>::new`
#5 [analysis] running analysis passes on this crate
error: could not compile `alloc`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:04:53
