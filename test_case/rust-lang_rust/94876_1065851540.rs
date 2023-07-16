plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size { raw: 8 }`,
 right: `Size { raw: 4 }`', compiler/rustc_mir_build/src/build/expr/as_constant.rs:61:21
   0:     0x7f4b38c48b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h641e0a65e2615a08
   1:     0x7f4b38cb563e - core::fmt::write::h39191aa5431a5380
   1:     0x7f4b38cb563e - core::fmt::write::h39191aa5431a5380
   2:     0x7f4b38c38a81 - std::io::Write::write_fmt::hb1861dc9906df921
   3:     0x7f4b38c4898b - std::sys_common::backtrace::print::h0ae42f20033c9262
   4:     0x7f4b38c4d1f4 - std::panicking::default_hook::{{closure}}::hd2976bf86056b49a
   5:     0x7f4b38c4cdca - std::panicking::default_hook::hd02e8d479b982aaa
   6:     0x7f4b396a7e71 - rustc_driver[7387008bc487c86e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f4b38c4d906 - std::panicking::rust_panic_with_hook::hbbba1d0bbca4f6bf
   8:     0x7f4b38c4d717 - std::panicking::begin_panic_handler::{{closure}}::h2d97bb73b3b478f0
   9:     0x7f4b38c49084 - std::sys_common::backtrace::__rust_end_short_backtrace::h61f77ae4323c082a
  10:     0x7f4b38c4d3e9 - rust_begin_unwind
  11:     0x7f4b38c04a93 - core::panicking::panic_fmt::h510f640c0e57f953
  12:     0x7f4b38cb1fe8 - core::panicking::assert_failed_inner::hd0148e7e424bf90b
  13:     0x7f4b3945c76b - core[8382fcd636289ab6]::panicking::assert_failed::<rustc_target[a9fcc2fb4eaff4cb]::abi::Size, rustc_target[a9fcc2fb4eaff4cb]::abi::Size>
  14:     0x7f4b3a44e527 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_constant
  15:     0x7f4b3a44e77c - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_operand
  16:     0x7f4b3a453afb - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_rvalue
  17:     0x7f4b3a457dd2 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest
  18:     0x7f4b3a45748c - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_temp_inner
  19:     0x7f4b3a44e98b - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_operand
  20:     0x7f4b3a44eba5 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_operand
  21:     0x7f4b3a452def - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_rvalue
  22:     0x7f4b3a457dd2 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest
  23:     0x7f4b3a45748c - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_temp_inner
  24:     0x7f4b3a44e98b - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_operand
  25:     0x7f4b3a44eba5 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_operand
  26:     0x7f4b3a452def - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_rvalue
  27:     0x7f4b3a457dd2 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest
  28:     0x7f4b3a45748c - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_temp_inner
  29:     0x7f4b3a44e98b - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_operand
  30:     0x7f4b3a44eba5 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_operand
  31:     0x7f4b3a452d89 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::as_rvalue
  32:     0x7f4b3a457dd2 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest
  33:     0x7f4b3a46f04d - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::in_scope::<<rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest::{closure#0}::{closure#0}, ()>
  34:     0x7f4b3a45917a - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest
  35:     0x7f4b3a44b534 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::ast_block_stmts
  36:     0x7f4b3a46e0b3 - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::in_scope::<<rustc_mir_build[8a8f97961e7a2025]::build::Builder>::ast_block::{closure#2}::{closure#0}, ()>
  37:     0x7f4b3a459e8f - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest
  38:     0x7f4b3a46f04d - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::in_scope::<<rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest::{closure#0}::{closure#0}, ()>
  39:     0x7f4b3a45917a - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest
  40:     0x7f4b3a46f04d - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::in_scope::<<rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest::{closure#0}::{closure#0}, ()>
  41:     0x7f4b3a45917a - <rustc_mir_build[8a8f97961e7a2025]::build::Builder>::expr_into_dest
  42:     0x7f4b3a4469dd - rustc_mir_build[8a8f97961e7a2025]::build::construct_fn::<core[8382fcd636289ab6]::iter::adapters::chain::Chain<alloc[75caa6313e26ebc1]::vec::into_iter::IntoIter<rustc_mir_build[8a8f97961e7a2025]::build::ArgInfo>, core[8382fcd636289ab6]::iter::adapters::map::Map<core[8382fcd636289ab6]::iter::adapters::enumerate::Enumerate<core[8382fcd636289ab6]::slice::iter::Iter<rustc_hir[ab985a163b28919a]::hir::Param>>, rustc_mir_build[8a8f97961e7a2025]::build::mir_build::{closure#1}::{closure#0}>>>
  43:     0x7f4b3a50bc6c - <rustc_infer[165ca10e3594e600]::infer::InferCtxtBuilder>::enter::<rustc_middle[df7eeb591dfcb37e]::mir::Body, rustc_mir_build[8a8f97961e7a2025]::build::mir_build::{closure#1}>
  44:     0x7f4b3a444849 - rustc_mir_build[8a8f97961e7a2025]::build::mir_built
  45:     0x7f4b3ab9f3ed - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<rustc_middle[df7eeb591dfcb37e]::ty::WithOptConstParam<rustc_span[f78664e4d3610ddc]::def_id::LocalDefId>, &rustc_data_structures[67c22d8712ae8803]::steal::Steal<rustc_middle[df7eeb591dfcb37e]::mir::Body>>>
  46:     0x7f4b3ace3d57 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::mir_built, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  47:     0x7f4b39c4e665 - rustc_mir_transform[4c14bca07fdccc7e]::check_unsafety::unsafety_check_result
  48:     0x7f4b39c4b86f - <rustc_mir_transform[4c14bca07fdccc7e]::check_unsafety::provide::{closure#0} as core[8382fcd636289ab6]::ops::function::FnOnce<(rustc_middle[df7eeb591dfcb37e]::ty::context::TyCtxt, rustc_span[f78664e4d3610ddc]::def_id::LocalDefId)>>::call_once
  49:     0x7f4b3abb0342 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<rustc_span[f78664e4d3610ddc]::def_id::LocalDefId, &rustc_middle[df7eeb591dfcb37e]::mir::query::UnsafetyCheckResult>>
  50:     0x7f4b3aca7cf0 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::unsafety_check_result, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  51:     0x7f4b39c1f420 - rustc_mir_transform[4c14bca07fdccc7e]::mir_const
  52:     0x7f4b3ab9f3ed - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<rustc_middle[df7eeb591dfcb37e]::ty::WithOptConstParam<rustc_span[f78664e4d3610ddc]::def_id::LocalDefId>, &rustc_data_structures[67c22d8712ae8803]::steal::Steal<rustc_middle[df7eeb591dfcb37e]::mir::Body>>>
  53:     0x7f4b3ace4113 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::mir_const, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  54:     0x7f4b39c2017c - rustc_mir_transform[4c14bca07fdccc7e]::mir_promoted
  55:     0x7f4b3ac7642a - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::mir_promoted, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  56:     0x7f4b3a760e7e - rustc_borrowck[d83276d95930d30b]::mir_borrowck
  57:     0x7f4b3a72e55f - <rustc_borrowck[d83276d95930d30b]::provide::{closure#0} as core[8382fcd636289ab6]::ops::function::FnOnce<(rustc_middle[df7eeb591dfcb37e]::ty::context::TyCtxt, rustc_span[f78664e4d3610ddc]::def_id::LocalDefId)>>::call_once
  58:     0x7f4b3abaf612 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<rustc_span[f78664e4d3610ddc]::def_id::LocalDefId, &rustc_middle[df7eeb591dfcb37e]::mir::query::BorrowCheckResult>>
  59:     0x7f4b3ac756e0 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::mir_borrowck, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  60:     0x7f4b398b589e - <rustc_middle[df7eeb591dfcb37e]::hir::map::Map>::par_body_owners::<rustc_interface[7a474fd684de44d0]::passes::analysis::{closure#2}::{closure#0}>
  61:     0x7f4b39813870 - <rustc_session[d8904b8db467f383]::session::Session>::time::<(), rustc_interface[7a474fd684de44d0]::passes::analysis::{closure#2}>
  62:     0x7f4b398ace2b - rustc_interface[7a474fd684de44d0]::passes::analysis
  63:     0x7f4b3abe942b - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<(), core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>>
  64:     0x7f4b3ace0279 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::analysis, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  65:     0x7f4b39728084 - <rustc_interface[7a474fd684de44d0]::passes::QueryContext>::enter::<rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  66:     0x7f4b39715ca2 - <rustc_interface[7a474fd684de44d0]::interface::Compiler>::enter::<rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}::{closure#2}, core[8382fcd636289ab6]::result::Result<core[8382fcd636289ab6]::option::Option<rustc_interface[7a474fd684de44d0]::queries::Linker>, rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  67:     0x7f4b397229a9 - rustc_span[f78664e4d3610ddc]::with_source_map::<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_interface[7a474fd684de44d0]::interface::create_compiler_and_run<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#1}>
  68:     0x7f4b397134dd - <scoped_tls[6532d0c6e5128321]::ScopedKey<rustc_span[f78664e4d3610ddc]::SessionGlobals>>::set::<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  69:     0x7f4b396ce639 - std[9f788053d5631f34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a474fd684de44d0]::util::run_in_thread_pool_with_globals<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  70:     0x7f4b3972c351 - std[9f788053d5631f34]::panic::catch_unwind::<core[8382fcd636289ab6]::panic::unwind_safe::AssertUnwindSafe<<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[7a474fd684de44d0]::util::run_in_thread_pool_with_globals<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  71:     0x7f4b3971d63b - <<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[7a474fd684de44d0]::util::run_in_thread_pool_with_globals<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#1} as core[8382fcd636289ab6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  72:     0x7f4b38c5c783 - std::sys::unix::thread::Thread::new::thread_start::hd363d8910f104f91
  73:     0x7f4b32fcd609 - start_thread
  74:     0x7f4b38ac2163 - clone
  75:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (c27e60f18 2022-03-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_built] building MIR for `fmt::<impl at library/core/src/fmt/mod.rs:1275:1: 2097:2>::sign_plus`
#1 [unsafety_check_result] unsafety-checking `fmt::<impl at library/core/src/fmt/mod.rs:1275:1: 2097:2>::sign_plus`
#2 [mir_const] processing MIR for `fmt::<impl at library/core/src/fmt/mod.rs:1275:1: 2097:2>::sign_plus`
#3 [mir_promoted] processing `fmt::<impl at library/core/src/fmt/mod.rs:1275:1: 2097:2>::sign_plus`
#4 [mir_borrowck] borrow-checking `fmt::<impl at library/core/src/fmt/mod.rs:1275:1: 2097:2>::sign_plus`
#5 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:04:17
