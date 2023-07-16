plain
failures:

---- [ui] src/test/ui/parser/issues/issue-84104.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-84104.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-84104" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-84104/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/issues/issue-84104.rs:3:13
   |
   |
LL | #[i=i::<ښܖ<
   |  -          ^
   |  unclosed delimiter


thread 'rustc' panicked at 'end of span not on char boundary', compiler/rustc_span/src/span_encoding.rs:107:25
   0:     0x7f55a5385d85 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2a2827d577d32bf8
   0:     0x7f55a5385d85 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2a2827d577d32bf8
   1:     0x7f55a53f5888 - core::fmt::write::h4d74d8b5c24254e8
   2:     0x7f55a5377de1 - std::io::Write::write_fmt::hca54e61e13e301a5
   3:     0x7f55a5385b91 - std::sys_common::backtrace::print::h236b59ec20dbdefc
   4:     0x7f55a5388ef4 - std::panicking::default_hook::{{closure}}::h8ef0d3bcf34a6174
   5:     0x7f55a5388bba - std::panicking::default_hook::he9ede6b46cd7b5c8
   6:     0x7f55a5de2722 - rustc_driver[e53108c937eb029]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f55a5389664 - std::panicking::rust_panic_with_hook::h1e61ac2f79c52150
   8:     0x7f55a538938a - std::panicking::begin_panic_handler::{{closure}}::h5d2b6981cf7266f0
   9:     0x7f55a53862a4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd1e8b8bb66603f95
  10:     0x7f55a5389072 - rust_begin_unwind
  11:     0x7f55a5339fe3 - core::panicking::panic_fmt::hd8d4d18069cc355a
  12:     0x7f55a914796f - <scoped_tls[d34b66ee10247130]::ScopedKey<rustc_span[a721b91a6d69a42e]::SessionGlobals>>::with::<<rustc_span[a721b91a6d69a42e]::span_encoding::Span>::new::{closure#0}, ()>
  13:     0x7f55a9130324 - <rustc_span[a721b91a6d69a42e]::source_map::SourceMap>::start_point
  14:     0x7f55a8473922 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_lit_expr
  15:     0x7f55a846ac47 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_dot_or_call_expr::{closure#0}
  16:     0x7f55a8446018 - <rustc_parse[af342ea9d185c73]::parser::Parser>::collect_tokens_trailing_token::<rustc_ast[575e47f58d92f922]::ptr::P<rustc_ast[575e47f58d92f922]::ast::Expr>, <rustc_parse[af342ea9d185c73]::parser::Parser>::collect_tokens_for_expr<<rustc_parse[af342ea9d185c73]::parser::Parser>::parse_dot_or_call_expr::{closure#0}>::{closure#0}>
  17:     0x7f55a8466bc4 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_prefix_expr
  18:     0x7f55a8462110 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_assoc_expr_with
  19:     0x7f55a84bfba0 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_generic_arg
  20:     0x7f55a84bdad4 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_angle_args
  21:     0x7f55a84bba61 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_path_segment
  22:     0x7f55a84bb2d2 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_path_segments
  23:     0x7f55a84baf88 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_path_inner
  24:     0x7f55a84cd19a - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_path_start_ty
  25:     0x7f55a84c982c - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_ty_common
  26:     0x7f55a84bf89a - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_generic_arg
  27:     0x7f55a84bdad4 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_angle_args
  28:     0x7f55a84bba61 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_path_segment
  29:     0x7f55a84bb37f - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_path_segments
  30:     0x7f55a84baf88 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_path_inner
  31:     0x7f55a8475271 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_path_start_expr
  32:     0x7f55a8469e6d - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_dot_or_call_expr::{closure#0}
  33:     0x7f55a8446018 - <rustc_parse[af342ea9d185c73]::parser::Parser>::collect_tokens_trailing_token::<rustc_ast[575e47f58d92f922]::ptr::P<rustc_ast[575e47f58d92f922]::ast::Expr>, <rustc_parse[af342ea9d185c73]::parser::Parser>::collect_tokens_for_expr<<rustc_parse[af342ea9d185c73]::parser::Parser>::parse_dot_or_call_expr::{closure#0}>::{closure#0}>
  34:     0x7f55a8466bc4 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_prefix_expr
  35:     0x7f55a8462110 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_assoc_expr_with
  36:     0x7f55a844757b - <rustc_parse[af342ea9d185c73]::parser::Parser>::collect_tokens_trailing_token::<rustc_ast[575e47f58d92f922]::ptr::P<rustc_ast[575e47f58d92f922]::ast::Expr>, <rustc_parse[af342ea9d185c73]::parser::Parser>::collect_tokens_no_attrs<rustc_ast[575e47f58d92f922]::ptr::P<rustc_ast[575e47f58d92f922]::ast::Expr>, <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_expr_force_collect::{closure#0}>::{closure#0}>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  37:     0x7f55a843ae34 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_attr_item::{closure#0}
  38:     0x7f55a8439ba9 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_attr_item
  39:     0x7f55a843800a - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_attribute
  40:     0x7f55a8437923 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_outer_attributes
  41:     0x7f55a848f648 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_mod
  42:     0x7f55a848f2a8 - <rustc_parse[af342ea9d185c73]::parser::Parser>::parse_crate_mod
  43:     0x7f55a85663a0 - rustc_parse[af342ea9d185c73]::parse_crate_from_file
  44:     0x7f55a5f1c2a3 - <rustc_session[e8387954a12e476d]::session::Session>::time::<core[7dae2715d38ec4fe]::result::Result<rustc_ast[575e47f58d92f922]::ast::Crate, rustc_errors[d607cd3d51cd8f94]::diagnostic_builder::DiagnosticBuilder<rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>>, rustc_interface[e2df7e1e4f594fcf]::passes::parse::{closure#0}>
  45:     0x7f55a5f3ad47 - rustc_interface[e2df7e1e4f594fcf]::passes::parse
  46:     0x7f55a5fbce27 - <rustc_interface[e2df7e1e4f594fcf]::queries::Queries>::parse
  47:     0x7f55a5e498bf - <rustc_interface[e2df7e1e4f594fcf]::interface::Compiler>::enter::<rustc_driver[e53108c937eb029]::run_compiler::{closure#1}::{closure#2}, core[7dae2715d38ec4fe]::result::Result<core[7dae2715d38ec4fe]::option::Option<rustc_interface[e2df7e1e4f594fcf]::queries::Linker>, rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>>
  48:     0x7f55a5de3e8c - rustc_span[a721b91a6d69a42e]::with_source_map::<core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>, rustc_interface[e2df7e1e4f594fcf]::interface::run_compiler<core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>, rustc_driver[e53108c937eb029]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  49:     0x7f55a5e3ed3a - <scoped_tls[d34b66ee10247130]::ScopedKey<rustc_span[a721b91a6d69a42e]::SessionGlobals>>::set::<rustc_interface[e2df7e1e4f594fcf]::interface::run_compiler<core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>, rustc_driver[e53108c937eb029]::run_compiler::{closure#1}>::{closure#0}, core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>>
  50:     0x7f55a5e01bcf - std[ba3549fe4e776f30]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e2df7e1e4f594fcf]::util::run_in_thread_pool_with_globals<rustc_interface[e2df7e1e4f594fcf]::interface::run_compiler<core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>, rustc_driver[e53108c937eb029]::run_compiler::{closure#1}>::{closure#0}, core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>>
  51:     0x7f55a5e601c6 - std[ba3549fe4e776f30]::panicking::try::<core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>, core[7dae2715d38ec4fe]::panic::unwind_safe::AssertUnwindSafe<<std[ba3549fe4e776f30]::thread::Builder>::spawn_unchecked_<rustc_interface[e2df7e1e4f594fcf]::util::run_in_thread_pool_with_globals<rustc_interface[e2df7e1e4f594fcf]::interface::run_compiler<core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>, rustc_driver[e53108c937eb029]::run_compiler::{closure#1}>::{closure#0}, core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  52:     0x7f55a5df4155 - <<std[ba3549fe4e776f30]::thread::Builder>::spawn_unchecked_<rustc_interface[e2df7e1e4f594fcf]::util::run_in_thread_pool_with_globals<rustc_interface[e2df7e1e4f594fcf]::interface::run_compiler<core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>, rustc_driver[e53108c937eb029]::run_compiler::{closure#1}>::{closure#0}, core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7dae2715d38ec4fe]::result::Result<(), rustc_errors[d607cd3d51cd8f94]::ErrorGuaranteed>>::{closure#1} as core[7dae2715d38ec4fe]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7f55a5395f9e - std::sys::unix::thread::Thread::new::thread_start::h067934445bb5944a
  54:     0x7f55a512bb43 - <unknown>
  55:     0x7f55a51bda00 - <unknown>
  56:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (36c3a05a2 2022-12-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
