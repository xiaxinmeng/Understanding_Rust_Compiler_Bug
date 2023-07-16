`
error: environment variable `    ` not defined at compile time
 --> treereduce.out:1:1
  |
1 | env!{"\t"}
  | ^^^^^^^^^^
  |
thread 'rustc' panicked at 'assertion failed: self.lines.iter().all(|r| !r.iter().any(|sc| sc.chr == \'\\t\'))', compiler/rustc_errors/src/styled_buffer.rs:32:9
stack backtrace:
   0:     0x7ffa0bcfc744 - std::backtrace_rs::backtrace::libunwind::trace::h56757e7e25601569
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7ffa0bcfc744 - std::backtrace_rs::backtrace::trace_unsynchronized::hd676671100e3ba6d
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7ffa0bcfc744 - std::sys_common::backtrace::_print_fmt::h1360db735fde7c44
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7ffa0bcfc744 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h874d21e1a2ca9982
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7ffa0bdb3f68 - core::fmt::write::hd4eb0d8409af39da
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/fmt/mod.rs:1254:17
   5:     0x7ffa0bd4580f - std::io::Write::write_fmt::h3d807647408c9030
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/io/mod.rs:1698:15
   6:     0x7ffa0bcfc545 - std::sys_common::backtrace::_print::h57daf8ffede52753
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7ffa0bcfc545 - std::sys_common::backtrace::print::hdaea9ba05299927c
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7ffa0bd2ca34 - std::panicking::default_hook::{{closure}}::h4c6d7d557c0df471
   9:     0x7ffa0bd2c69a - std::panicking::default_hook::hac355d5a2e2eb547
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:288:9
  10:     0x7ffa0e77cdf5 - <alloc[32b43e9536c01a3]::boxed::Box<dyn for<'a, 'b> core[c5d5d662f7508502]::ops::function::Fn<(&'a core[c5d5d662f7508502]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[c5d5d662f7508502]::marker::Send + core[c5d5d662f7508502]::marker::Sync> as core[c5d5d662f7508502]::ops::function::Fn<(&core[c5d5d662f7508502]::panic::panic_info::PanicInfo,)>>::call
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1987:9
  11:     0x7ffa0e77cdf5 - rustc_driver_impl[ccb753bc1017b924]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver_impl/src/lib.rs:1208:17
  12:     0x7ffa0bd2d291 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc42bd19225fc85a2
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1987:9
  13:     0x7ffa0bd2d291 - std::panicking::rust_panic_with_hook::hc60a1f91e87ff481
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:695:13
  14:     0x7ffa0bcfca22 - std::panicking::begin_panic_handler::{{closure}}::h635104bd7a006c44
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:580:13
  15:     0x7ffa0bcfc826 - std::sys_common::backtrace::__rust_end_short_backtrace::h23f5845d88aa41e7
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:150:18
  16:     0x7ffa0bd2cdf2 - rust_begin_unwind
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:578:5
  17:     0x7ffa0bdd8203 - core::panicking::panic_fmt::h5863111baae9147e
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panicking.rs:67:14
  18:     0x7ffa0bdd829d - core::panicking::panic::he5f7c6779e1d5164
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panicking.rs:117:5
  19:     0x7ffa0e86dd8c - <rustc_errors[1332105c0c19f354]::styled_buffer::StyledBuffer>::render
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/styled_buffer.rs:32:9
  20:     0x7ffa0e855163 - <rustc_errors[1332105c0c19f354]::emitter::EmitterWriter>::emit_message_default::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/emitter.rs:1428:41
  21:     0x7ffa0e853692 - <rustc_errors[1332105c0c19f354]::emitter::EmitterWriter>::emit_message_default
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/emitter.rs:1365:25
  22:     0x7ffa0e846702 - <rustc_errors[1332105c0c19f354]::emitter::EmitterWriter>::emit_messages_default
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/emitter.rs:2158:43
  23:     0x7ffa0e846702 - <rustc_errors[1332105c0c19f354]::emitter::EmitterWriter as rustc_errors[1332105c0c19f354]::emitter::Emitter>::emit_diagnostic
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/emitter.rs:556:9
  24:     0x7ffa0e878959 - <rustc_errors[1332105c0c19f354]::HandlerInner>::emit_diagnostic::{closure#2}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:1400:17
  25:     0x7ffa0f2ed3f4 - rustc_interface[75cb3db337003cb3]::callbacks::track_diagnostic::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/callbacks.rs:41:53
  26:     0x7ffa0f2ed3f4 - rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context::<rustc_interface[75cb3db337003cb3]::callbacks::track_diagnostic::{closure#0}::{closure#0}, ()>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:82:9
  27:     0x7ffa0f2ed3f4 - <std[6efab6f90a3b1c4f]::thread::local::LocalKey<core[c5d5d662f7508502]::cell::Cell<*const ()>>>::try_with::<rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context<rustc_interface[75cb3db337003cb3]::callbacks::track_diagnostic::{closure#0}::{closure#0}, ()>::{closure#0}, ()>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/local.rs:252:16
  28:     0x7ffa0f2ed3f4 - <std[6efab6f90a3b1c4f]::thread::local::LocalKey<core[c5d5d662f7508502]::cell::Cell<*const ()>>>::with::<rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context<rustc_interface[75cb3db337003cb3]::callbacks::track_diagnostic::{closure#0}::{closure#0}, ()>::{closure#0}, ()>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/local.rs:228:9
  29:     0x7ffa0f2ed3f4 - rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context::<rustc_interface[75cb3db337003cb3]::callbacks::track_diagnostic::{closure#0}::{closure#0}, ()>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:79:5
  30:     0x7ffa0f2ed3f4 - rustc_interface[75cb3db337003cb3]::callbacks::track_diagnostic::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/callbacks.rs:41:20
  31:     0x7ffa0f2ed3f4 - rustc_middle[ab1e559699b6e831]::ty::context::tls::with_context_opt::<rustc_interface[75cb3db337003cb3]::callbacks::track_diagnostic::{closure#0}, ()>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:100:18
  32:     0x7ffa0f2ed3f4 - rustc_interface[75cb3db337003cb3]::callbacks::track_diagnostic
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/callbacks.rs:31:5
  33:     0x7ffa0e877d43 - <rustc_errors[1332105c0c19f354]::HandlerInner>::emit_diagnostic
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:1369:9
  34:     0x7ffa0e881c16 - <rustc_errors[1332105c0c19f354]::Handler>::emit_diagnostic
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:1120:9
  35:     0x7ffa0e881c16 - <rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed as rustc_errors[1332105c0c19f354]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic_builder.rs:169:28
  36:     0x7ffa0e161a14 - <rustc_errors[1332105c0c19f354]::diagnostic_builder::DiagnosticBuilder<rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::emit
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic_builder.rs:505:9
  37:     0x7ffa0e161a14 - <rustc_session[b31df004a6d1cc2b]::parse::ParseSess>::emit_err::<rustc_builtin_macros[23d9227d4ae0f4da]::errors::EnvNotDefined>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_session/src/parse.rs:347:30
  38:     0x7ffa0e247efc - <rustc_session[b31df004a6d1cc2b]::session::Session>::emit_err::<rustc_builtin_macros[23d9227d4ae0f4da]::errors::EnvNotDefined>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_session/src/session.rs:551:9
  39:     0x7ffa0e247efc - <rustc_expand[5eeed1c0db39db62]::base::ExtCtxt>::emit_err::<rustc_builtin_macros[23d9227d4ae0f4da]::errors::EnvNotDefined>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_expand/src/base.rs:1128:19
  40:     0x7ffa0e247efc - rustc_builtin_macros[23d9227d4ae0f4da]::env::expand_env
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_builtin_macros/src/env.rs:83:16
  41:     0x7ffa0e91453b - <rustc_expand[5eeed1c0db39db62]::expand::MacroExpander>::expand_invoc
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_expand/src/expand.rs:662:38
  42:     0x7ffa0e91453b - <rustc_expand[5eeed1c0db39db62]::expand::MacroExpander>::fully_expand_fragment
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_expand/src/expand.rs:475:62
  43:     0x7ffa0e9136b1 - <rustc_expand[5eeed1c0db39db62]::expand::MacroExpander>::expand_crate
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_expand/src/expand.rs:402:21
  44:     0x7ffa0f3928bc - rustc_interface[75cb3db337003cb3]::passes::configure_and_expand::{closure#1}::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:243:50
  45:     0x7ffa0f3928bc - <rustc_data_structures[e0305e891f62af14]::profiling::VerboseTimingGuard>::run::<rustc_ast[920a8b2931d178ae]::ast::Crate, rustc_interface[75cb3db337003cb3]::passes::configure_and_expand::{closure#1}::{closure#1}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/profiling.rs:752:9
  46:     0x7ffa0f3928bc - <rustc_session[b31df004a6d1cc2b]::session::Session>::time::<rustc_ast[920a8b2931d178ae]::ast::Crate, rustc_interface[75cb3db337003cb3]::passes::configure_and_expand::{closure#1}::{closure#1}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_session/src/utils.rs:11:50
  47:     0x7ffa0f3928bc - rustc_interface[75cb3db337003cb3]::passes::configure_and_expand::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:243:21
  48:     0x7ffa0f3928bc - <rustc_data_structures[e0305e891f62af14]::profiling::VerboseTimingGuard>::run::<rustc_ast[920a8b2931d178ae]::ast::Crate, rustc_interface[75cb3db337003cb3]::passes::configure_and_expand::{closure#1}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/profiling.rs:752:9
  49:     0x7ffa0f3928bc - <rustc_session[b31df004a6d1cc2b]::session::Session>::time::<rustc_ast[920a8b2931d178ae]::ast::Crate, rustc_interface[75cb3db337003cb3]::passes::configure_and_expand::{closure#1}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_session/src/utils.rs:11:50
  50:     0x7ffa0f2db950 - rustc_interface[75cb3db337003cb3]::passes::configure_and_expand
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:194:13
  51:     0x7ffa0f2db950 - rustc_interface[75cb3db337003cb3]::passes::resolver_for_lowering
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:566:17
  52:     0x7ffa102a085d - <rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering as rustc_query_system[475b93d7b2c8ae41]::query::config::QueryConfig<rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>>::compute
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:524:21
  53:     0x7ffa102a085d - rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr::<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:445:72
  54:     0x7ffa102a085d - rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context::<rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:82:9
  55:     0x7ffa102a085d - <std[6efab6f90a3b1c4f]::thread::local::LocalKey<core[c5d5d662f7508502]::cell::Cell<*const ()>>>::try_with::<rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context<rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/local.rs:252:16
  56:     0x7ffa102a085d - <std[6efab6f90a3b1c4f]::thread::local::LocalKey<core[c5d5d662f7508502]::cell::Cell<*const ()>>>::with::<rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context<rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/local.rs:228:9
  57:     0x7ffa102a085d - rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context::<rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:79:5
  58:     0x7ffa102a085d - <rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt as rustc_query_system[475b93d7b2c8ae41]::query::QueryContext>::start_query::<rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:127:13
  59:     0x7ffa102a085d - rustc_middle[ab1e559699b6e831]::ty::context::tls::with_related_context::<<rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt as rustc_query_system[475b93d7b2c8ae41]::query::QueryContext>::start_query<rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:133:9
  60:     0x7ffa102a085d - rustc_middle[ab1e559699b6e831]::ty::context::tls::with_context::<rustc_middle[ab1e559699b6e831]::ty::context::tls::with_related_context<<rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt as rustc_query_system[475b93d7b2c8ae41]::query::QueryContext>::start_query<rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:111:36
  61:     0x7ffa102a085d - rustc_middle[ab1e559699b6e831]::ty::context::tls::with_context_opt::<rustc_middle[ab1e559699b6e831]::ty::context::tls::with_context<rustc_middle[ab1e559699b6e831]::ty::context::tls::with_related_context<<rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt as rustc_query_system[475b93d7b2c8ae41]::query::QueryContext>::start_query<rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:100:18
  62:     0x7ffa102a085d - rustc_middle[ab1e559699b6e831]::ty::context::tls::with_context::<rustc_middle[ab1e559699b6e831]::ty::context::tls::with_related_context<<rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt as rustc_query_system[475b93d7b2c8ae41]::query::QueryContext>::start_query<rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:111:5
  63:     0x7ffa102a085d - rustc_middle[ab1e559699b6e831]::ty::context::tls::with_related_context::<<rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt as rustc_query_system[475b93d7b2c8ae41]::query::QueryContext>::start_query<rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:124:5
  64:     0x7ffa102a085d - <rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt as rustc_query_system[475b93d7b2c8ae41]::query::QueryContext>::start_query::<rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:112:9
  65:     0x7ffa102a085d - rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job_non_incr::<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:445:18
  66:     0x7ffa102a085d - rustc_query_system[475b93d7b2c8ae41]::query::plumbing::execute_job::<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:402:17
  67:     0x7ffa102a085d - rustc_query_system[475b93d7b2c8ae41]::query::plumbing::try_execute_query::<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:358:13
  68:     0x7ffa106d3e56 - rustc_query_system[475b93d7b2c8ae41]::query::plumbing::get_query::<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:805:36
  69:     0x7ffa106d3e56 - stacker[f871904c967749]::maybe_grow::<(rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>, core[c5d5d662f7508502]::option::Option<rustc_query_system[475b93d7b2c8ae41]::dep_graph::graph::DepNodeIndex>), rustc_query_system[475b93d7b2c8ae41]::query::plumbing::get_query<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  70:     0x7ffa106d3e56 - rustc_data_structures[e0305e891f62af14]::stack::ensure_sufficient_stack::<(rustc_middle[ab1e559699b6e831]::query::erase::Erased<[u8; 8usize]>, core[c5d5d662f7508502]::option::Option<rustc_query_system[475b93d7b2c8ae41]::dep_graph::graph::DepNodeIndex>), rustc_query_system[475b93d7b2c8ae41]::query::plumbing::get_query<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  71:     0x7ffa106d3e56 - rustc_query_system[475b93d7b2c8ae41]::query::plumbing::get_query::<rustc_query_impl[2c629a8f2053622a]::queries::resolver_for_lowering, rustc_query_impl[2c629a8f2053622a]::plumbing::QueryCtxt>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:805:9
  72:     0x7ffa106d3e56 - <rustc_query_impl[2c629a8f2053622a]::Queries as rustc_middle[ab1e559699b6e831]::ty::query::QueryEngine>::resolver_for_lowering
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:829:17
  73:     0x7ffa0e7e1813 - <rustc_middle[ab1e559699b6e831]::ty::query::TyCtxtAt>::resolver_for_lowering
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/query.rs:396:29
  74:     0x7ffa0e7e1813 - <rustc_middle[ab1e559699b6e831]::ty::context::TyCtxt>::resolver_for_lowering
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/query.rs:383:35
  75:     0x7ffa0e7e1813 - rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}::{closure#2}::{closure#2}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver_impl/src/lib.rs:370:52
  76:     0x7ffa0e7e1813 - <rustc_middle[ab1e559699b6e831]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:557:37
  77:     0x7ffa0e7e1813 - rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context::<<rustc_middle[ab1e559699b6e831]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>::{closure#0}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:82:9
  78:     0x7ffa0e7e1813 - <std[6efab6f90a3b1c4f]::thread::local::LocalKey<core[c5d5d662f7508502]::cell::Cell<*const ()>>>::try_with::<rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context<<rustc_middle[ab1e559699b6e831]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>::{closure#0}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>::{closure#0}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/local.rs:252:16
  79:     0x7ffa0e7e1813 - <std[6efab6f90a3b1c4f]::thread::local::LocalKey<core[c5d5d662f7508502]::cell::Cell<*const ()>>>::with::<rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context<<rustc_middle[ab1e559699b6e831]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>::{closure#0}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>::{closure#0}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/local.rs:228:9
  80:     0x7ffa0e7e1813 - rustc_middle[ab1e559699b6e831]::ty::context::tls::enter_context::<<rustc_middle[ab1e559699b6e831]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>::{closure#0}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context/tls.rs:79:5
  81:     0x7ffa0e7e1813 - <rustc_middle[ab1e559699b6e831]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:557:9
  82:     0x7ffa0e7e1813 - <rustc_interface[75cb3db337003cb3]::queries::QueryResult<&rustc_middle[ab1e559699b6e831]::ty::context::GlobalCtxt>>::enter::<&rustc_data_structures[e0305e891f62af14]::steal::Steal<(rustc_middle[ab1e559699b6e831]::ty::ResolverAstLowering, alloc[32b43e9536c01a3]::rc::Rc<rustc_ast[920a8b2931d178ae]::ast::Crate>)>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}::{closure#2}::{closure#2}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:71:9
  83:     0x7ffa0e7f1c87 - rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}::{closure#2}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver_impl/src/lib.rs:370:13
  84:     0x7ffa0e7f1c87 - <rustc_interface[75cb3db337003cb3]::interface::Compiler>::enter::<rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}::{closure#2}, core[c5d5d662f7508502]::result::Result<core[c5d5d662f7508502]::option::Option<rustc_interface[75cb3db337003cb3]::queries::Linker>, rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:394:19
  85:     0x7ffa0e79087b - rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver_impl/src/lib.rs:331:22
  86:     0x7ffa0e79087b - rustc_interface[75cb3db337003cb3]::interface::run_compiler::<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:301:21
  87:     0x7ffa0e79087b - rustc_span[60d9bd9abeb54baa]::set_source_map::<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:1040:5
  88:     0x7ffa0e79087b - rustc_interface[75cb3db337003cb3]::interface::run_compiler::<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:295:13
  89:     0x7ffa0e79087b - <scoped_tls[c6512a5451dfceef]::ScopedKey<rustc_span[60d9bd9abeb54baa]::SessionGlobals>>::set::<rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  90:     0x7ffa0e79087b - rustc_span[60d9bd9abeb54baa]::create_session_globals_then::<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:120:5
  91:     0x7ffa0e79087b - rustc_interface[75cb3db337003cb3]::util::run_in_thread_pool_with_globals::<rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/util.rs:152:38
  92:     0x7ffa0e79087b - std[6efab6f90a3b1c4f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75cb3db337003cb3]::util::run_in_thread_pool_with_globals<rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:134:18
  93:     0x7ffa0e792f91 - <std[6efab6f90a3b1c4f]::thread::Builder>::spawn_unchecked_::<rustc_interface[75cb3db337003cb3]::util::run_in_thread_pool_with_globals<rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:526:17
  94:     0x7ffa0e792f91 - <core[c5d5d662f7508502]::panic::unwind_safe::AssertUnwindSafe<<std[6efab6f90a3b1c4f]::thread::Builder>::spawn_unchecked_<rustc_interface[75cb3db337003cb3]::util::run_in_thread_pool_with_globals<rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[c5d5d662f7508502]::ops::function::FnOnce<()>>::call_once
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panic/unwind_safe.rs:271:9
  95:     0x7ffa0e792f91 - std[6efab6f90a3b1c4f]::panicking::try::do_call::<core[c5d5d662f7508502]::panic::unwind_safe::AssertUnwindSafe<<std[6efab6f90a3b1c4f]::thread::Builder>::spawn_unchecked_<rustc_interface[75cb3db337003cb3]::util::run_in_thread_pool_with_globals<rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:485:40
  96:     0x7ffa0e792f91 - std[6efab6f90a3b1c4f]::panicking::try::<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, core[c5d5d662f7508502]::panic::unwind_safe::AssertUnwindSafe<<std[6efab6f90a3b1c4f]::thread::Builder>::spawn_unchecked_<rustc_interface[75cb3db337003cb3]::util::run_in_thread_pool_with_globals<rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:449:19
  97:     0x7ffa0e792f91 - std[6efab6f90a3b1c4f]::panic::catch_unwind::<core[c5d5d662f7508502]::panic::unwind_safe::AssertUnwindSafe<<std[6efab6f90a3b1c4f]::thread::Builder>::spawn_unchecked_<rustc_interface[75cb3db337003cb3]::util::run_in_thread_pool_with_globals<rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:140:14
  98:     0x7ffa0e792f91 - <std[6efab6f90a3b1c4f]::thread::Builder>::spawn_unchecked_::<rustc_interface[75cb3db337003cb3]::util::run_in_thread_pool_with_globals<rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:525:30
  99:     0x7ffa0e792f91 - <<std[6efab6f90a3b1c4f]::thread::Builder>::spawn_unchecked_<rustc_interface[75cb3db337003cb3]::util::run_in_thread_pool_with_globals<rustc_interface[75cb3db337003cb3]::interface::run_compiler<core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>, rustc_driver_impl[ccb753bc1017b924]::run_compiler::{closure#1}>::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c5d5d662f7508502]::result::Result<(), rustc_span[60d9bd9abeb54baa]::ErrorGuaranteed>>::{closure#1} as core[c5d5d662f7508502]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:250:5
 100:     0x7ffa0bd4175a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc593816cbfe333a6
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1973:9
 101:     0x7ffa0bd4175a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf0a0a0c078b006cc
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1973:9
 102:     0x7ffa0bd028e5 - std::sys::unix::thread::Thread::new::thread_start::h56978627721898d3
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys/unix/thread.rs:108:17
 103:     0x7ffa0be9ebb5 - <unknown>
 104:     0x7ffa0bf20d90 - <unknown>
 105:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
