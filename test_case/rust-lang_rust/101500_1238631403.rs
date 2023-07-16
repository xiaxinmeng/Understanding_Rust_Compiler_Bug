plain
   Compiling rand_chacha v0.2.2
   Compiling rand v0.7.3
thread 'rustc' panicked at 'attempt to subtract with overflow', compiler/rustc_mir_transform/src/liveness.rs:524:85
stack backtrace:
   0:     0x7f8ec6942acd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7aceaaec384abd8d
   1:     0x7f8ec69a81c9 - core::fmt::write::h1935094ec1b611e9
   2:     0x7f8ec6933b11 - std::io::Write::write_fmt::h924a08855ab35277
   3:     0x7f8ec6945a7e - std::panicking::default_hook::{{closure}}::hdcba8bc567463877
   4:     0x7f8ec6945747 - std::panicking::default_hook::h5e023012d652ad1c
   5:     0x7f8ec72f3184 - rustc_driver[491383e5761cfd5f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f8ec6946221 - std::panicking::rust_panic_with_hook::hc50598e1e398727d
   7:     0x7f8ec6946009 - std::panicking::begin_panic_handler::{{closure}}::h2c1a8ae8110b48ef
   8:     0x7f8ec6943034 - std::sys_common::backtrace::__rust_end_short_backtrace::h4f0e14b26c3ea417
   9:     0x7f8ec6945d22 - rust_begin_unwind
  10:     0x7f8ec68f9cc3 - core::panicking::panic_fmt::h577a3b3df57afc41
  11:     0x7f8ec68f9b8d - core::panicking::panic::h858d0a26258d295c
  12:     0x7f8ec79726eb - rustc_mir_transform[6065ec5f4cf882c4]::liveness::check_liveness
  13:     0x7f8ec8f8ddbe - rustc_query_system[b09e367728d50dde]::query::plumbing::get_query::<rustc_query_impl[2c0f045bf50d798d]::queries::check_liveness, rustc_query_impl[2c0f045bf50d798d]::plumbing::QueryCtxt>
  14:     0x7f8ec8ddd174 - <rustc_query_impl[2c0f045bf50d798d]::Queries as rustc_middle[221c3a924ceeae5d]::ty::query::QueryEngine>::check_liveness
  15:     0x7f8ec7487e21 - <core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b0c0ad648e9499f6]::sync::par_for_each_in<&[rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId], <rustc_middle[221c3a924ceeae5d]::hir::map::Map>::par_body_owners<rustc_interface[62be15c45d0bd73b]::passes::analysis::{closure#1}::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once
  16:     0x7f8ec73efa1b - rustc_data_structures[b0c0ad648e9499f6]::sync::par_for_each_in::<&[rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId], <rustc_middle[221c3a924ceeae5d]::hir::map::Map>::par_body_owners<rustc_interface[62be15c45d0bd73b]::passes::analysis::{closure#1}::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  17:     0x7f8ec748a3ab - <core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[62be15c45d0bd73b]::passes::analysis::{closure#1}::{closure#1}> as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once
  18:     0x7f8ec73ef7f9 - std[8a3c335779a4ef7b]::panic::catch_unwind::<core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[62be15c45d0bd73b]::passes::analysis::{closure#1}::{closure#1}>, ()>
  19:     0x7f8ec7471052 - <rustc_session[5e51afea9c8b884e]::session::Session>::time::<(), rustc_interface[62be15c45d0bd73b]::passes::analysis::{closure#1}>
  20:     0x7f8ec7425c34 - rustc_interface[62be15c45d0bd73b]::passes::analysis
  21:     0x7f8ec8efb3b5 - rustc_query_system[b09e367728d50dde]::query::plumbing::try_execute_query::<rustc_query_impl[2c0f045bf50d798d]::plumbing::QueryCtxt, rustc_query_system[b09e367728d50dde]::query::caches::DefaultCache<(), core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>>
  22:     0x7f8ec8fdd374 - rustc_query_system[b09e367728d50dde]::query::plumbing::get_query::<rustc_query_impl[2c0f045bf50d798d]::queries::analysis, rustc_query_impl[2c0f045bf50d798d]::plumbing::QueryCtxt>
  23:     0x7f8ec8db92ba - <rustc_query_impl[2c0f045bf50d798d]::Queries as rustc_middle[221c3a924ceeae5d]::ty::query::QueryEngine>::analysis
  24:     0x7f8ec7348b5c - <rustc_interface[62be15c45d0bd73b]::passes::QueryContext>::enter::<rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  25:     0x7f8ec72f4bbf - <rustc_interface[62be15c45d0bd73b]::interface::Compiler>::enter::<rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}::{closure#2}, core[c1e30f1bd259d119]::result::Result<core[c1e30f1bd259d119]::option::Option<rustc_interface[62be15c45d0bd73b]::queries::Linker>, rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  26:     0x7f8ec72e1ac6 - rustc_span[8c7477ded0a91ee5]::with_source_map::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_interface[62be15c45d0bd73b]::interface::create_compiler_and_run<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>::{closure#1}>
  27:     0x7f8ec72f5fee - rustc_interface[62be15c45d0bd73b]::interface::create_compiler_and_run::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>
  28:     0x7f8ec72d8b92 - <scoped_tls[5efd78fa53ce51fd]::ScopedKey<rustc_span[8c7477ded0a91ee5]::SessionGlobals>>::set::<rustc_interface[62be15c45d0bd73b]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  29:     0x7f8ec735321f - std[8a3c335779a4ef7b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[62be15c45d0bd73b]::util::run_in_thread_pool_with_globals<rustc_interface[62be15c45d0bd73b]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  30:     0x7f8ec72e324e - std[8a3c335779a4ef7b]::panicking::try::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[62be15c45d0bd73b]::util::run_in_thread_pool_with_globals<rustc_interface[62be15c45d0bd73b]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  31:     0x7f8ec7354a20 - <<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[62be15c45d0bd73b]::util::run_in_thread_pool_with_globals<rustc_interface[62be15c45d0bd73b]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1} as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f8ec6952125 - std::sys::unix::thread::Thread::new::thread_start::h3a069647ae68a533
  33:     0x7f8ec66f2b43 - <unknown>
  34:     0x7f8ec6784a00 - <unknown>
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (e066984a9 2022-09-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [check_liveness] checking liveness of variables in distributions::binomial::<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.7.3/src/distributions/binomial.rs:49:1: 49:36>::sample
#1 [analysis] running analysis passes on this crate
error: could not compile `rand`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:14:03
