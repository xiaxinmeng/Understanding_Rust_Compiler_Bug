plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: matches!(item.top_elts, Tt(TokenTree :: Sequence(..)))', compiler/rustc_expand/src/mbe/macro_parser.rs:520:13
   0:     0x7fa2ffcaa16c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h641e0a65e2615a08
   0:     0x7fa2ffcaa16c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h641e0a65e2615a08
   1:     0x7fa2ffd16d1e - core::fmt::write::h39191aa5431a5380
   2:     0x7fa2ffc992e1 - std::io::Write::write_fmt::hb1861dc9906df921
   3:     0x7fa2ffca9f9b - std::sys_common::backtrace::print::h0ae42f20033c9262
   4:     0x7fa2ffcae804 - std::panicking::default_hook::{{closure}}::hd2976bf86056b49a
   5:     0x7fa2ffcae3da - std::panicking::default_hook::hd02e8d479b982aaa
   6:     0x7fa300706e81 - rustc_driver[7387008bc487c86e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa2ffcaef15 - std::panicking::rust_panic_with_hook::hbbba1d0bbca4f6bf
   8:     0x7fa2ffcaece9 - std::panicking::begin_panic_handler::{{closure}}::h2d97bb73b3b478f0
   9:     0x7fa2ffcaa694 - std::sys_common::backtrace::__rust_end_short_backtrace::h61f77ae4323c082a
  10:     0x7fa2ffcae9f9 - rust_begin_unwind
  11:     0x7fa2ffc65a93 - core::panicking::panic_fmt::h510f640c0e57f953
  12:     0x7fa2ffc6595d - core::panicking::panic::h5aad8a7117313b35
  13:     0x7fa302539d7f - rustc_expand[efa6bc5acd30c03b]::mbe::macro_parser::parse_tt
  14:     0x7fa302529ff8 - rustc_expand[efa6bc5acd30c03b]::mbe::macro_rules::compile_declarative_macro
  15:     0x7fa30127676b - <rustc_resolve[62267bb9da75df5a]::Resolver>::compile_macro
  16:     0x7fa3012e5cfd - <rustc_resolve[62267bb9da75df5a]::build_reduced_graph::BuildReducedGraphVisitor>::define_macro
  17:     0x7fa3012e69f5 - <rustc_resolve[62267bb9da75df5a]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[4c5af1b4f56e9b2b]::visit::Visitor>::visit_item
  18:     0x7fa3012401ed - rustc_ast[4c5af1b4f56e9b2b]::visit::walk_item::<rustc_resolve[62267bb9da75df5a]::build_reduced_graph::BuildReducedGraphVisitor>
  19:     0x7fa3012e81ce - <rustc_resolve[62267bb9da75df5a]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[4c5af1b4f56e9b2b]::visit::Visitor>::visit_item
  20:     0x7fa30123d34d - rustc_ast[4c5af1b4f56e9b2b]::visit::walk_crate::<rustc_resolve[62267bb9da75df5a]::build_reduced_graph::BuildReducedGraphVisitor>
  21:     0x7fa3012e97cc - <rustc_resolve[62267bb9da75df5a]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[4c5af1b4f56e9b2b]::visit::Visitor>::visit_crate
  22:     0x7fa30126db29 - <rustc_resolve[62267bb9da75df5a]::Resolver as rustc_expand[efa6bc5acd30c03b]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  23:     0x7fa3025c4025 - <rustc_expand[efa6bc5acd30c03b]::expand::MacroExpander>::collect_invocations
  24:     0x7fa3025bf468 - <rustc_expand[efa6bc5acd30c03b]::expand::MacroExpander>::fully_expand_fragment
  25:     0x7fa3025bf118 - <rustc_expand[efa6bc5acd30c03b]::expand::MacroExpander>::expand_crate
  26:     0x7fa300868b8f - <rustc_session[d8904b8db467f383]::session::Session>::time::<core[8382fcd636289ab6]::result::Result<rustc_ast[4c5af1b4f56e9b2b]::ast::Crate, rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_interface[7a474fd684de44d0]::passes::configure_and_expand::{closure#1}>
  27:     0x7fa3008fe98e - rustc_interface[7a474fd684de44d0]::passes::configure_and_expand
  28:     0x7fa30090fc80 - <rustc_interface[7a474fd684de44d0]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[7a474fd684de44d0]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[8382fcd636289ab6]::result::Result<rustc_ast[4c5af1b4f56e9b2b]::ast::Crate, rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  29:     0x7fa30089e2bd - <rustc_interface[7a474fd684de44d0]::queries::Queries>::expansion
  30:     0x7fa3007725f1 - rustc_interface[7a474fd684de44d0]::interface::create_compiler_and_run::<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>
  31:     0x7fa30072cdfc - std[9f788053d5631f34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a474fd684de44d0]::util::run_in_thread_pool_with_globals<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  32:     0x7fa300780591 - std[9f788053d5631f34]::panic::catch_unwind::<core[8382fcd636289ab6]::panic::unwind_safe::AssertUnwindSafe<<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[7a474fd684de44d0]::util::run_in_thread_pool_with_globals<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  33:     0x7fa300722c52 - <<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[7a474fd684de44d0]::util::run_in_thread_pool_with_globals<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#1} as core[8382fcd636289ab6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7fa2ffcbdd93 - std::sys::unix::thread::Thread::new::thread_start::hd363d8910f104f91
  35:     0x7fa2fa02e609 - start_thread
  36:     0x7fa2ffb23163 - clone
  37:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (40885a21c 2022-03-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
