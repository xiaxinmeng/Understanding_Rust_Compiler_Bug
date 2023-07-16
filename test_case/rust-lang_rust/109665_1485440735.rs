plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:13c1b4e09b845ddb9664cee13d03879444a1054d)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
   Compiling askama_derive v0.12.0
   Compiling askama v0.12.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:1454 ~ rustdoc[1802]::core::create_config::{closure#3}::{closure#2}::EMPTY_SET) (src/librustdoc/core.rs:265:82: 265:99 (#0)): ascribe_user_type `fn() -> rustc_data_structures::unord::UnordSet<rustc_span::def_id::LocalDefId> {<rustc_data_structures::unord::UnordSet<rustc_span::def_id::LocalDefId> as std::default::Default>::default}==TypeOf(DefId(2:2581 ~ core[6e08]::default::Default::default), UserSubsts { substs: [rustc_data_structures::unord::UnordSet<_>], user_self_ty: None })` failed with `NoSolution`
  = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
             1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &str>
             1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &str>
             2: <rustc_borrowck::type_check::TypeChecker>::ascribe_user_type
             3: rustc_borrowck::nll::compute_regions
             4: rustc_borrowck::do_mir_borrowck
             5: rustc_borrowck::mir_borrowck
             6: <rustc_borrowck::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
             8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
             8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
             9: rustc_data_structures::sync::par_for_each_in::<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_interface::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
            10: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
            12: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            13: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
            14: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            15: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
            15: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
            16: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            17: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            18: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            20: <unknown>
            21: <unknown>
          


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (93797456c 2023-03-27) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z binary-dep-depinfo -Z tls-model=initial-exec
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
