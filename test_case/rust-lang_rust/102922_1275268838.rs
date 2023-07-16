plain
failures:

---- [ui] src/test/ui/macros/issue-86865.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-86865.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-86865" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-86865/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Span must not be empty and have no suggestion', /checkout/compiler/rustc_errors/src/diagnostic.rs:784:9
stack backtrace:
   0:     0x7fae209e0d9e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he352ef98fd35b01f
   1:     0x7fae20a49ac8 - core::fmt::write::hf9c5411474171eae
   2:     0x7fae209d2371 - std::io::Write::write_fmt::h9063bed4b4ea99e6
   3:     0x7fae209e3e2e - std::panicking::default_hook::{{closure}}::he6eb1ee2e5f237d8
   4:     0x7fae209e3ae9 - std::panicking::default_hook::h56c0d9d54054fbb0
   5:     0x7fae213b6aa4 - rustc_driver[544b89b38ee44b13]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fae209e45ed - std::panicking::rust_panic_with_hook::h7e8dd6b677d89e56
   7:     0x7fae209e43c9 - std::panicking::begin_panic_handler::{{closure}}::h84afd1e5b5891920
   8:     0x7fae209e12d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hacd84c8effdeec6a
   9:     0x7fae209e40d2 - rust_begin_unwind
  10:     0x7fae20993b93 - core::panicking::panic_fmt::h05d8aed612463af4
  11:     0x7fae2376423e - <rustc_errors[204f3fd1c034177c]::diagnostic::Diagnostic>::span_suggestion_with_style::<&str, &str>
  12:     0x7fae2378192a - rustc_expand[9d211a62688cc6c]::base::expr_to_spanned_string
  13:     0x7fae220fbb8e - rustc_builtin_macros[78daf3299f285573]::format::make_format_args
  14:     0x7fae2210150b - rustc_builtin_macros[78daf3299f285573]::format::expand_format_args_impl
  15:     0x7fae2379091e - <rustc_expand[9d211a62688cc6c]::expand::MacroExpander>::fully_expand_fragment
  16:     0x7fae2378fc5b - <rustc_expand[9d211a62688cc6c]::expand::MacroExpander>::expand_crate
  17:     0x7fae214ce544 - <rustc_session[7dd59c7819efa57d]::session::Session>::time::<core[bcb91f7603f89409]::result::Result<rustc_ast[898ea8ae941a9db1]::ast::Crate, rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_interface[674247669d84820a]::passes::configure_and_expand::{closure#1}>
  18:     0x7fae214f2762 - rustc_interface[674247669d84820a]::passes::configure_and_expand
  19:     0x7fae214ff856 - <rustc_interface[674247669d84820a]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[674247669d84820a]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[bcb91f7603f89409]::result::Result<rustc_ast[898ea8ae941a9db1]::ast::Crate, rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>
  20:     0x7fae214ebff8 - <rustc_interface[674247669d84820a]::queries::Queries>::expansion
  21:     0x7fae213b8596 - <rustc_interface[674247669d84820a]::interface::Compiler>::enter::<rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}::{closure#2}, core[bcb91f7603f89409]::result::Result<core[bcb91f7603f89409]::option::Option<rustc_interface[674247669d84820a]::queries::Linker>, rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>
  22:     0x7fae213a39fe - rustc_span[fd66e77ae260c9b6]::with_source_map::<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_interface[674247669d84820a]::interface::create_compiler_and_run<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>::{closure#1}>
  23:     0x7fae213b9a42 - rustc_interface[674247669d84820a]::interface::create_compiler_and_run::<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>
  24:     0x7fae21428dbf - <scoped_tls[2204431335e08ca0]::ScopedKey<rustc_span[fd66e77ae260c9b6]::SessionGlobals>>::set::<rustc_interface[674247669d84820a]::interface::run_compiler<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>
  25:     0x7fae213e5aef - std[df62b41bba7199b5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[674247669d84820a]::util::run_in_thread_pool_with_globals<rustc_interface[674247669d84820a]::interface::run_compiler<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>
  26:     0x7fae213a6f16 - std[df62b41bba7199b5]::panicking::try::<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, core[bcb91f7603f89409]::panic::unwind_safe::AssertUnwindSafe<<std[df62b41bba7199b5]::thread::Builder>::spawn_unchecked_<rustc_interface[674247669d84820a]::util::run_in_thread_pool_with_globals<rustc_interface[674247669d84820a]::interface::run_compiler<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  27:     0x7fae213d9f7a - <<std[df62b41bba7199b5]::thread::Builder>::spawn_unchecked_<rustc_interface[674247669d84820a]::util::run_in_thread_pool_with_globals<rustc_interface[674247669d84820a]::interface::run_compiler<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>::{closure#1} as core[bcb91f7603f89409]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  28:     0x7fae209f0f75 - std::sys::unix::thread::Thread::new::thread_start::hfa3396d24620fd31
  29:     0x7fae20789b43 - <unknown>
  30:     0x7fae2081ba00 - <unknown>
  31:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (60e36c0ff 2022-10-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/parser/increment-notfixed.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/increment-notfixed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/increment-notfixed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/increment-notfixed/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Span must not be empty and have no suggestion', /checkout/compiler/rustc_errors/src/diagnostic.rs:856:9
   0:     0x7f1a510b7d9e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he352ef98fd35b01f
   1:     0x7f1a51120ac8 - core::fmt::write::hf9c5411474171eae
   2:     0x7f1a510a9371 - std::io::Write::write_fmt::h9063bed4b4ea99e6
   2:     0x7f1a510a9371 - std::io::Write::write_fmt::h9063bed4b4ea99e6
   3:     0x7f1a510bae2e - std::panicking::default_hook::{{closure}}::he6eb1ee2e5f237d8
   4:     0x7f1a510baae9 - std::panicking::default_hook::h56c0d9d54054fbb0
   5:     0x7f1a51a8daa4 - rustc_driver[544b89b38ee44b13]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1a510bb5ed - std::panicking::rust_panic_with_hook::h7e8dd6b677d89e56
   7:     0x7f1a510bb3c9 - std::panicking::begin_panic_handler::{{closure}}::h84afd1e5b5891920
   8:     0x7f1a510b82d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hacd84c8effdeec6a
   9:     0x7f1a510bb0d2 - rust_begin_unwind
  10:     0x7f1a5106ab93 - core::panicking::panic_fmt::h05d8aed612463af4
  11:     0x7f1a5402b372 - <rustc_errors[204f3fd1c034177c]::diagnostic::Diagnostic>::multipart_suggestions::<&str, core[bcb91f7603f89409]::iter::adapters::map::Map<core[bcb91f7603f89409]::array::iter::IntoIter<rustc_parse[ebf7f0ca9409c037]::parser::diagnostics::MultiSugg, 2usize>, <rustc_parse[ebf7f0ca9409c037]::parser::diagnostics::MultiSugg>::emit_many<core[bcb91f7603f89409]::array::iter::IntoIter<rustc_parse[ebf7f0ca9409c037]::parser::diagnostics::MultiSugg, 2usize>>::{closure#0}>>
  12:     0x7f1a53f36c24 - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::recover_from_inc_dec
  13:     0x7f1a53f436f7 - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::parse_assoc_expr_with
  14:     0x7f1a53f9f7a2 - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::parse_stmt_path_start
  15:     0x7f1a53f9e9b7 - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::parse_stmt_without_recovery
  16:     0x7f1a53fa443a - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::parse_full_stmt
  17:     0x7f1a53fa410f - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::parse_block_tail
  18:     0x7f1a53fa4007 - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::parse_block_common
  19:     0x7f1a53f85368 - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::parse_fn
  20:     0x7f1a53f6f9aa - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::parse_item_common
  21:     0x7f1a53f6ea9e - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::parse_mod
  22:     0x7f1a53f6e6b8 - <rustc_parse[ebf7f0ca9409c037]::parser::Parser>::parse_crate_mod
  23:     0x7f1a5401506d - rustc_parse[ebf7f0ca9409c037]::parse_crate_from_file
  24:     0x7f1a51ba4eb5 - <rustc_session[7dd59c7819efa57d]::session::Session>::time::<core[bcb91f7603f89409]::result::Result<rustc_ast[898ea8ae941a9db1]::ast::Crate, rustc_errors[204f3fd1c034177c]::diagnostic_builder::DiagnosticBuilder<rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>, rustc_interface[674247669d84820a]::passes::parse::{closure#0}>
  25:     0x7f1a51bc8a97 - rustc_interface[674247669d84820a]::passes::parse
  26:     0x7f1a51bc20e5 - <rustc_interface[674247669d84820a]::queries::Queries>::parse
  27:     0x7f1a51a8f0ba - <rustc_interface[674247669d84820a]::interface::Compiler>::enter::<rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}::{closure#2}, core[bcb91f7603f89409]::result::Result<core[bcb91f7603f89409]::option::Option<rustc_interface[674247669d84820a]::queries::Linker>, rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>
  28:     0x7f1a51a7a9fe - rustc_span[fd66e77ae260c9b6]::with_source_map::<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_interface[674247669d84820a]::interface::create_compiler_and_run<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7f1a51a90a42 - rustc_interface[674247669d84820a]::interface::create_compiler_and_run::<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>
  30:     0x7f1a51affdbf - <scoped_tls[2204431335e08ca0]::ScopedKey<rustc_span[fd66e77ae260c9b6]::SessionGlobals>>::set::<rustc_interface[674247669d84820a]::interface::run_compiler<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>
  31:     0x7f1a51abcaef - std[df62b41bba7199b5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[674247669d84820a]::util::run_in_thread_pool_with_globals<rustc_interface[674247669d84820a]::interface::run_compiler<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>
  32:     0x7f1a51a7df16 - std[df62b41bba7199b5]::panicking::try::<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, core[bcb91f7603f89409]::panic::unwind_safe::AssertUnwindSafe<<std[df62b41bba7199b5]::thread::Builder>::spawn_unchecked_<rustc_interface[674247669d84820a]::util::run_in_thread_pool_with_globals<rustc_interface[674247669d84820a]::interface::run_compiler<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7f1a51ab0f7a - <<std[df62b41bba7199b5]::thread::Builder>::spawn_unchecked_<rustc_interface[674247669d84820a]::util::run_in_thread_pool_with_globals<rustc_interface[674247669d84820a]::interface::run_compiler<core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>, rustc_driver[544b89b38ee44b13]::run_compiler::{closure#1}>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>::{closure#0}, core[bcb91f7603f89409]::result::Result<(), rustc_errors[204f3fd1c034177c]::ErrorGuaranteed>>::{closure#1} as core[bcb91f7603f89409]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f1a510c7f75 - std::sys::unix::thread::Thread::new::thread_start::hfa3396d24620fd31
  35:     0x7f1a50e60b43 - <unknown>
  36:     0x7f1a50ef2a00 - <unknown>
  37:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (60e36c0ff 2022-10-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

