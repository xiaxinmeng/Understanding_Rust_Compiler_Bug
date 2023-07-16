plain
To only update this specific test, also pass `--test-args fmt/respanned-literal-issue-106191.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/fmt/respanned-literal-issue-106191.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/respanned-literal-issue-106191" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/respanned-literal-issue-106191/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32', compiler/rustc_span/src/lib.rs:1717:17
   0:     0x7f3f1837b8c5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hafd2362bb59f7a82
   1:     0x7f3f183ebd78 - core::fmt::write::h73b6cb44dde7af98
   2:     0x7f3f1836d771 - std::io::Write::write_fmt::h3a1cd06415644190
   3:     0x7f3f1837b6d1 - std::sys_common::backtrace::print::hdb11aa480a1b3d30
   3:     0x7f3f1837b6d1 - std::sys_common::backtrace::print::hdb11aa480a1b3d30
   4:     0x7f3f1837eab4 - std::panicking::default_hook::{{closure}}::hade95e8b04f1ea4f
   5:     0x7f3f1837e77a - std::panicking::default_hook::hc3019ca99442fa67
   6:     0x7f3f18deeac2 - rustc_driver[14c416efc6e6a180]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3f1837f221 - std::panicking::rust_panic_with_hook::h2dc0ccd6508d8611
   8:     0x7f3f1837ef4a - std::panicking::begin_panic_handler::{{closure}}::h2d73dcc6f6c7a3fa
   9:     0x7f3f1837bde4 - std::sys_common::backtrace::__rust_end_short_backtrace::h871d4fc3eafd9e50
  10:     0x7f3f1837ec32 - rust_begin_unwind
  11:     0x7f3f1832efe3 - core::panicking::panic_fmt::hde4d95546c337da9
  12:     0x7f3f1832f07d - core::panicking::panic::h5d0d4e83057ccc61
  13:     0x7f3f1c147b13 - <rustc_span[f27fc1ab68a007e2]::SourceFile>::bytepos_to_file_charpos
  14:     0x7f3f1c147bf1 - <rustc_span[f27fc1ab68a007e2]::SourceFile>::lookup_file_pos
  15:     0x7f3f1c148662 - <rustc_span[f27fc1ab68a007e2]::SourceFile>::lookup_file_pos_with_col_display
  16:     0x7f3f1c1315e1 - <rustc_span[f27fc1ab68a007e2]::source_map::SourceMap>::lookup_char_pos
  17:     0x7f3f1be7bb9b - <rustc_errors[4343c6d7cfd845e6]::emitter::FileWithAnnotatedLines>::collect_annotations
  18:     0x7f3f1be806ca - <rustc_errors[4343c6d7cfd845e6]::emitter::EmitterWriter>::emit_message_default::{closure#0}
  19:     0x7f3f1be7f631 - <rustc_errors[4343c6d7cfd845e6]::emitter::EmitterWriter>::emit_message_default
  20:     0x7f3f1be751bd - <rustc_errors[4343c6d7cfd845e6]::emitter::EmitterWriter as rustc_errors[4343c6d7cfd845e6]::emitter::Emitter>::emit_diagnostic
  21:     0x7f3f1be5ffd5 - <rustc_errors[4343c6d7cfd845e6]::json::Diagnostic>::from_errors_diagnostic
  22:     0x7f3f1be5eadc - <rustc_errors[4343c6d7cfd845e6]::json::JsonEmitter as rustc_errors[4343c6d7cfd845e6]::emitter::Emitter>::emit_diagnostic
  23:     0x7f3f1be5259b - <rustc_errors[4343c6d7cfd845e6]::HandlerInner>::emit_diagnostic::{closure#2}
  24:     0x7f3f1be51c54 - <rustc_errors[4343c6d7cfd845e6]::HandlerInner>::emit_diagnostic
  25:     0x7f3f1be4bb66 - <rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed as rustc_errors[4343c6d7cfd845e6]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  26:     0x7f3f19c55141 - rustc_builtin_macros[f0af0907478a3a78]::format::make_format_args
  27:     0x7f3f19c58e1b - rustc_builtin_macros[f0af0907478a3a78]::format::expand_format_args_impl
  28:     0x7f3f1b3992b0 - <rustc_expand[aa9ad2f00bcc18b]::expand::MacroExpander>::fully_expand_fragment
  29:     0x7f3f1b398202 - <rustc_expand[aa9ad2f00bcc18b]::expand::MacroExpander>::expand_crate
  30:     0x7f3f18f5d493 - rustc_interface[39f9c77e8400c3f2]::passes::configure_and_expand
  31:     0x7f3f18fb941b - <rustc_interface[39f9c77e8400c3f2]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[39f9c77e8400c3f2]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[48fcb276faa09488]::result::Result<rustc_ast[37a5ef26ec10248e]::ast::Crate, rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>>
  32:     0x7f3f18f20390 - <rustc_interface[39f9c77e8400c3f2]::queries::Queries>::expansion
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  33:     0x7f3f18e63b47 - <rustc_interface[39f9c77e8400c3f2]::interface::Compiler>::enter::<rustc_driver[14c416efc6e6a180]::run_compiler::{closure#1}::{closure#2}, core[48fcb276faa09488]::result::Result<core[48fcb276faa09488]::option::Option<rustc_interface[39f9c77e8400c3f2]::queries::Linker>, rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>>
  34:     0x7f3f18ddbe2c - rustc_span[f27fc1ab68a007e2]::with_source_map::<core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>, rustc_interface[39f9c77e8400c3f2]::interface::run_compiler<core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>, rustc_driver[14c416efc6e6a180]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  35:     0x7f3f18e5656a - <scoped_tls[bcbc94d7ad93acd4]::ScopedKey<rustc_span[f27fc1ab68a007e2]::SessionGlobals>>::set::<rustc_interface[39f9c77e8400c3f2]::interface::run_compiler<core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>, rustc_driver[14c416efc6e6a180]::run_compiler::{closure#1}>::{closure#0}, core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>>
  36:     0x7f3f18e4ff69 - std[cbea79e2adf59f62]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[39f9c77e8400c3f2]::util::run_in_thread_pool_with_globals<rustc_interface[39f9c77e8400c3f2]::interface::run_compiler<core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>, rustc_driver[14c416efc6e6a180]::run_compiler::{closure#1}>::{closure#0}, core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>>
  37:     0x7f3f18e4fdb8 - std[cbea79e2adf59f62]::panic::catch_unwind::<core[48fcb276faa09488]::panic::unwind_safe::AssertUnwindSafe<<std[cbea79e2adf59f62]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9c77e8400c3f2]::util::run_in_thread_pool_with_globals<rustc_interface[39f9c77e8400c3f2]::interface::run_compiler<core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>, rustc_driver[14c416efc6e6a180]::run_compiler::{closure#1}>::{closure#0}, core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>>
  38:     0x7f3f18dfeea7 - <<std[cbea79e2adf59f62]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9c77e8400c3f2]::util::run_in_thread_pool_with_globals<rustc_interface[39f9c77e8400c3f2]::interface::run_compiler<core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>, rustc_driver[14c416efc6e6a180]::run_compiler::{closure#1}>::{closure#0}, core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[48fcb276faa09488]::result::Result<(), rustc_errors[4343c6d7cfd845e6]::ErrorGuaranteed>>::{closure#1} as core[48fcb276faa09488]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f3f1838c0be - std::sys::unix::thread::Thread::new::thread_start::hebac7909bce22b13
  40:     0x7f3f18120b43 - <unknown>
  41:     0x7f3f181b2a00 - <unknown>
  42:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (5c19b9c9e 2023-01-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

