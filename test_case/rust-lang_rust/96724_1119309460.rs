plain
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling object v0.26.2
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.12.0
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Diagnostic { level: Error { lint: false }, message: [(Str("expected one of `/*»*/` or `::`, found `=`"), NoStyle)], code: None, span: MultiSpan { primary_spans: [library/stdarch/crates/std_detect/src/detect/arch/x86.rs:99:5: 99:67 (#0)], span_labels: [(library/stdarch/crates/std_detect/src/detect/arch/x86.rs:99:5: 99:67 (#0), Str("expected one of `/*»*/` or `::`"))] }, children: [], suggestions: Ok([]), args: [], sort_span: library/stdarch/crates/std_detect/src/detect/arch/x86.rs:99:5: 99:67 (#0), is_lint: false }', compiler/rustc_builtin_macros/src/cfg_eval.rs:151:81
   0:     0x7f45d7219c52 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   1:     0x7f45d7281618 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f45d720a191 - std::io::Write::write_fmt::hf3faa85fa7d28190
   2:     0x7f45d720a191 - std::io::Write::write_fmt::hf3faa85fa7d28190
   3:     0x7f45d721cf96 - std::panicking::default_hook::{{closure}}::h243e0a014f6b15da
   4:     0x7f45d721cb8d - std::panicking::default_hook::hdf681f01978f1e20
   5:     0x7f45d7d6b5a1 - rustc_driver[6cf9c77043e56e2d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f45d721d930 - std::panicking::rust_panic_with_hook::h1c127668bc0f49d8
   7:     0x7f45d721d747 - std::panicking::begin_panic_handler::{{closure}}::hdc297c549f81c3b7
   8:     0x7f45d721a1f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h7b90b067d1e7c19a
   9:     0x7f45d721d439 - rust_begin_unwind
  10:     0x7f45d71d10b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7f45d71d1263 - core::result::unwrap_failed::hf38d50bcb33515da
  12:     0x7f45d855a317 - <<rustc_builtin_macros[2e4ff2045d3ca1bb]::cfg_eval::CfgEval>::configure_annotatable::{closure#0} as core[10878fb91fc84a80]::ops::function::FnOnce<(&mut rustc_parse[a80a94cce78dd7db]::parser::Parser,)>>::call_once
  13:     0x7f45d855b068 - rustc_builtin_macros[2e4ff2045d3ca1bb]::cfg_eval::cfg_eval
  14:     0x7f45d855d20e - <rustc_builtin_macros[2e4ff2045d3ca1bb]::derive::Expander as rustc_expand[fd7db300d97bc99a]::base::MultiItemModifier>::expand::{closure#0}
  15:     0x7f45d8a3390d - <rustc_resolve[a544d2b375f69c09]::Resolver as rustc_expand[fd7db300d97bc99a]::base::ResolverExpand>::resolve_derives
  16:     0x7f45d855cdff - <rustc_builtin_macros[2e4ff2045d3ca1bb]::derive::Expander as rustc_expand[fd7db300d97bc99a]::base::MultiItemModifier>::expand
  17:     0x7f45d9e878a6 - <rustc_expand[fd7db300d97bc99a]::expand::MacroExpander>::fully_expand_fragment
  18:     0x7f45d9e85de2 - <rustc_expand[fd7db300d97bc99a]::expand::MacroExpander>::expand_crate
  19:     0x7f45d7e865a8 - <rustc_session[bb98eeba4fc964d8]::session::Session>::time::<core[10878fb91fc84a80]::result::Result<rustc_ast[480f26339f3853fc]::ast::Crate, rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_interface[575397cbf047e88c]::passes::configure_and_expand::{closure#1}>
  20:     0x7f45d7e76f0e - rustc_interface[575397cbf047e88c]::passes::configure_and_expand
  21:     0x7f45d7e97e3f - <rustc_interface[575397cbf047e88c]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[575397cbf047e88c]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[10878fb91fc84a80]::result::Result<rustc_ast[480f26339f3853fc]::ast::Crate, rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>
  22:     0x7f45d7e5c2b3 - <rustc_interface[575397cbf047e88c]::queries::Queries>::expansion
  23:     0x7f45d7d00ef8 - <rustc_interface[575397cbf047e88c]::interface::Compiler>::enter::<rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[575397cbf047e88c]::queries::Linker>, rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>
  24:     0x7f45d7ce3126 - rustc_span[188b5a9c74ea64c9]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_interface[575397cbf047e88c]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7f45d7d024af - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[188b5a9c74ea64c9]::SessionGlobals>>::set::<rustc_interface[575397cbf047e88c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>
  26:     0x7f45d7d56f79 - std[38ff3720b7fd637]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[575397cbf047e88c]::util::run_in_thread_pool_with_globals<rustc_interface[575397cbf047e88c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>
  27:     0x7f45d7d15ce1 - std[38ff3720b7fd637]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[575397cbf047e88c]::util::run_in_thread_pool_with_globals<rustc_interface[575397cbf047e88c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  28:     0x7f45d7d59202 - <<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[575397cbf047e88c]::util::run_in_thread_pool_with_globals<rustc_interface[575397cbf047e88c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  29:     0x7f45d722a333 - std::sys::unix::thread::Thread::new::thread_start::h38902d511e7013ce
  30:     0x7f45d177a609 - start_thread
  31:     0x7f45d708d163 - clone
  32:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (1ea7ccb08 2022-05-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
