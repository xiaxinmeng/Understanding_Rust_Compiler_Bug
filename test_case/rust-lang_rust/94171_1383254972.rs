text
thread 'rustc' panicked at 'Span must not be empty and have no suggestion', /home/lukas/code/rust/compiler/rustc_errors/src/diagnostic.rs:633:9
stack backtrace:
   0:     0x7f5b78ee5781 - std::backtrace_rs::backtrace::libunwind::trace::hefedb5c08a08a590
                               at /home/lukas/code/rust/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f5b78ee5781 - std::backtrace_rs::backtrace::trace_unsynchronized::he7f90cd9da6767af
                               at /home/lukas/code/rust/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f5b78ee5781 - std::sys_common::backtrace::_print_fmt::habb43519b808ac64
                               at /home/lukas/code/rust/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f5b78ee5781 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99e90d53f85ba8d7
                               at /home/lukas/code/rust/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f5b78f5d2d8 - core::fmt::write::h094ad263467a053c
                               at /home/lukas/code/rust/library/core/src/fmt/mod.rs:1213:17
   5:     0x7f5b78ebbd71 - std::io::Write::write_fmt::h1b8e58ae77b425e4
                               at /home/lukas/code/rust/library/std/src/io/mod.rs:1682:15
   6:     0x7f5b78ee55aa - std::sys_common::backtrace::_print::he4982fa331fb96a3
                               at /home/lukas/code/rust/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f5b78ee55aa - std::sys_common::backtrace::print::h1558a7fc8286ead3
                               at /home/lukas/code/rust/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f5b78ed96a7 - std::panicking::default_hook::{{closure}}::h7801bb9b93655c98
   9:     0x7f5b78ed949b - std::panicking::default_hook::h135831dab6184daa
                               at /home/lukas/code/rust/library/std/src/panicking.rs:286:9
  10:     0x7f5b798febd2 - <alloc[48d7b30605060536]::boxed::Box<dyn for<'a, 'b> core[672e3947e150d6c6]::ops::function::Fn<(&'a core[672e3947e150d6c6]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[672e3947e150d6c6]::marker::Send + core[672e3947e150d6c6]::marker::Sync> as core[672e3947e150d6c6]::ops::function::Fn<(&core[672e3947e150d6c6]::panic::panic_info::PanicInfo,)>>::call
                               at /home/lukas/code/rust/library/alloc/src/boxed.rs:2002:9
  11:     0x7f5b798febd2 - rustc_driver[6d8976894bfa954f]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_driver/src/lib.rs:1206:17
  12:     0x7f5b78ed9bf3 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hfe67b972cbd1b6fb
                               at /home/lukas/code/rust/library/alloc/src/boxed.rs:2002:9
  13:     0x7f5b78ed9bf3 - std::panicking::rust_panic_with_hook::h7d878098db373449
                               at /home/lukas/code/rust/library/std/src/panicking.rs:692:13
  14:     0x7f5b78ede032 - std::panicking::begin_panic_handler::{{closure}}::ha548179c7608e527
                               at /home/lukas/code/rust/library/std/src/panicking.rs:577:13
  15:     0x7f5b78eddfac - std::sys_common::backtrace::__rust_end_short_backtrace::hbe746217fadbf4b8
                               at /home/lukas/code/rust/library/std/src/sys_common/backtrace.rs:137:18
  16:     0x7f5b78ed977a - rust_begin_unwind
                               at /home/lukas/code/rust/library/std/src/panicking.rs:575:5
  17:     0x7f5b78e95723 - core::panicking::panic_fmt::h195b27650e929019
                               at /home/lukas/code/rust/library/core/src/panicking.rs:64:14
  18:     0x7f5b7bb92cc9 - <rustc_errors[52317f54776bd715]::diagnostic::Diagnostic>::multipart_suggestion_with_style::<rustc_error_messages[79b22b3bdd90c67f]::SubdiagnosticMessage>
                               at /home/lukas/code/rust/compiler/rustc_errors/src/diagnostic.rs:633:9
  19:     0x7f5b7bc68068 - <rustc_parse[c8cd746a65b62aef]::errors::NoFieldsForFnCall as rustc_errors[52317f54776bd715]::diagnostic::AddToDiagnostic>::add_to_diagnostic_with::<<rustc_parse[c8cd746a65b62aef]::errors::NoFieldsForFnCall as rustc_errors[52317f54776bd715]::diagnostic::AddToDiagnostic>::add_to_diagnostic::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_parse/src/errors.rs:584:10
  20:     0x7f5b7bc68068 - <rustc_parse[c8cd746a65b62aef]::errors::NoFieldsForFnCall as rustc_errors[52317f54776bd715]::diagnostic::AddToDiagnostic>::add_to_diagnostic
                               at /home/lukas/code/rust/compiler/rustc_errors/src/diagnostic.rs:80:9
  21:     0x7f5b7bc70371 - <rustc_errors[52317f54776bd715]::diagnostic::Diagnostic>::subdiagnostic::<rustc_parse[c8cd746a65b62aef]::errors::NoFieldsForFnCall>
                               at /home/lukas/code/rust/compiler/rustc_errors/src/diagnostic.rs:899:9
  22:     0x7f5b7bc70371 - <rustc_errors[52317f54776bd715]::diagnostic_builder::DiagnosticBuilder<rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::subdiagnostic::<rustc_parse[c8cd746a65b62aef]::errors::NoFieldsForFnCall>
                               at /home/lukas/code/rust/compiler/rustc_errors/src/diagnostic_builder.rs:428:13
  23:     0x7f5b7bc70371 - <rustc_parse[c8cd746a65b62aef]::errors::ParenthesesWithStructFields as rustc_errors[52317f54776bd715]::diagnostic_builder::IntoDiagnostic>::into_diagnostic
                               at /home/lukas/code/rust/compiler/rustc_parse/src/errors.rs:563:10
  24:     0x7f5b7bc2467a - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::maybe_recover_struct_lit_bad_delims
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:1214:55
  25:     0x7f5b7bc2467a - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_fn_call_expr
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:1180:13
  26:     0x7f5b7bc2467a - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_dot_or_call_expr_with_
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:1001:61
  27:     0x7f5b7bc22b7c - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_dot_or_call_expr_with
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:957:19
  28:     0x7f5b7bbd70bc - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_dot_or_call_expr::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:944:13
  29:     0x7f5b7bbd70bc - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_for_expr::<<rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_dot_or_call_expr::{closure#0}>::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:3175:23
  30:     0x7f5b7bbd47c3 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_trailing_token::<rustc_ast[164d9789c5234b58]::ptr::P<rustc_ast[164d9789c5234b58]::ast::Expr>, <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_for_expr<<rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_dot_or_call_expr::{closure#0}>::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/attr_wrapper.rs:213:23
  31:     0x7f5b7bbd47c3 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_for_expr::<<rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_dot_or_call_expr::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:3174:9
  32:     0x7f5b7bc2038a - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_dot_or_call_expr
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:941:9
  33:     0x7f5b7bc2038a - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_prefix_expr
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:617:25
  34:     0x7f5b7bc1d271 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_assoc_expr_with
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:188:17
  35:     0x7f5b7bc008c4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_assoc_expr
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:167:9
  36:     0x7f5b7bc008c4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_expr_res::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:155:33
  37:     0x7f5b7bc008c4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::with_res::<core[672e3947e150d6c6]::result::Result<rustc_ast[164d9789c5234b58]::ptr::P<rustc_ast[164d9789c5234b58]::ast::Expr>, rustc_errors[52317f54776bd715]::diagnostic_builder::DiagnosticBuilder<rustc_errors[52317f54776bd715]::ErrorGuaranteed>>, <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_expr_res::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/mod.rs:1372:19
  38:     0x7f5b7bc008c4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_expr_res
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:155:9
  39:     0x7f5b7bc008c4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_expr
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:114:9
  40:     0x7f5b7bc008c4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_expr_force_collect::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:119:45
  41:     0x7f5b7bc008c4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_no_attrs::<rustc_ast[164d9789c5234b58]::ptr::P<rustc_ast[164d9789c5234b58]::ast::Expr>, <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_expr_force_collect::{closure#0}>::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/mod.rs:1497:32
  42:     0x7f5b7bc008c4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_trailing_token::<rustc_ast[164d9789c5234b58]::ptr::P<rustc_ast[164d9789c5234b58]::ast::Expr>, <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_no_attrs<rustc_ast[164d9789c5234b58]::ptr::P<rustc_ast[164d9789c5234b58]::ast::Expr>, <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_expr_force_collect::{closure#0}>::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/attr_wrapper.rs:223:19
  43:     0x7f5b7bc008c4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_no_attrs::<rustc_ast[164d9789c5234b58]::ptr::P<rustc_ast[164d9789c5234b58]::ast::Expr>, <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_expr_force_collect::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/mod.rs:1494:9
  44:     0x7f5b7bc5da2e - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_expr_force_collect
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/expr.rs:119:9
  45:     0x7f5b7bc5da2e - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_attr_args
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/mod.rs:1272:55
  46:     0x7f5b7bc0e535 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_attr_item::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/attr.rs:260:28
  47:     0x7f5b7bc0e535 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_attr_item
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/attr.rs:264:81
  48:     0x7f5b7bc034fc - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_attribute::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/attr.rs:124:24
  49:     0x7f5b7bc034fc - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_no_attrs::<rustc_ast[164d9789c5234b58]::ast::Attribute, <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_attribute::{closure#0}>::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/mod.rs:1497:32
  50:     0x7f5b7bc034fc - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_trailing_token::<rustc_ast[164d9789c5234b58]::ast::Attribute, <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_no_attrs<rustc_ast[164d9789c5234b58]::ast::Attribute, <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_attribute::{closure#0}>::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/attr_wrapper.rs:223:19
  51:     0x7f5b7bc034fc - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::collect_tokens_no_attrs::<rustc_ast[164d9789c5234b58]::ast::Attribute, <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_attribute::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/mod.rs:1494:9
  52:     0x7f5b7bc0d73a - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_attribute
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/attr.rs:117:9
  53:     0x7f5b7bc0d455 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_outer_attributes
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/attr.rs:51:22
  54:     0x7f5b7bc35f0e - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_item_
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/item.rs:102:21
  55:     0x7f5b7bc356d4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_item
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/item.rs:93:9
  56:     0x7f5b7bc356d4 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_mod
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/item.rs:62:32
  57:     0x7f5b7bc35358 - <rustc_parse[c8cd746a65b62aef]::parser::Parser>::parse_crate_mod
                               at /home/lukas/code/rust/compiler/rustc_parse/src/parser/item.rs:31:37
  58:     0x7f5b7bbada20 - rustc_parse[c8cd746a65b62aef]::parse_crate_from_file
                               at /home/lukas/code/rust/compiler/rustc_parse/src/lib.rs:62:5
  59:     0x7f5b79a9e6c2 - rustc_interface[ed5c3e7d6b9d9752]::passes::parse::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_interface/src/passes.rs:56:30
  60:     0x7f5b79a9e6c2 - <rustc_data_structures[24675f911328660]::profiling::VerboseTimingGuard>::run::<core[672e3947e150d6c6]::result::Result<rustc_ast[164d9789c5234b58]::ast::Crate, rustc_errors[52317f54776bd715]::diagnostic_builder::DiagnosticBuilder<rustc_errors[52317f54776bd715]::ErrorGuaranteed>>, rustc_interface[ed5c3e7d6b9d9752]::passes::parse::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_data_structures/src/profiling.rs:723:9
  61:     0x7f5b79a9e6c2 - <rustc_session[19274890c3d55ee9]::session::Session>::time::<core[672e3947e150d6c6]::result::Result<rustc_ast[164d9789c5234b58]::ast::Crate, rustc_errors[52317f54776bd715]::diagnostic_builder::DiagnosticBuilder<rustc_errors[52317f54776bd715]::ErrorGuaranteed>>, rustc_interface[ed5c3e7d6b9d9752]::passes::parse::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_session/src/utils.rs:10:9
  62:     0x7f5b79a82c87 - rustc_interface[ed5c3e7d6b9d9752]::passes::parse
                               at /home/lukas/code/rust/compiler/rustc_interface/src/passes.rs:55:17
  63:     0x7f5b79ab45ca - <rustc_interface[ed5c3e7d6b9d9752]::queries::Queries>::parse::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_interface/src/queries.rs:134:13
  64:     0x7f5b79ab45ca - <rustc_interface[ed5c3e7d6b9d9752]::queries::Query<rustc_ast[164d9789c5234b58]::ast::Crate>>::compute::<<rustc_interface[ed5c3e7d6b9d9752]::queries::Queries>::parse::{closure#0}>::{closure#0}::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_interface/src/queries.rs:43:41
  65:     0x7f5b79ab45ca - <core[672e3947e150d6c6]::option::Option<core[672e3947e150d6c6]::result::Result<rustc_data_structures[24675f911328660]::steal::Steal<rustc_ast[164d9789c5234b58]::ast::Crate>, rustc_errors[52317f54776bd715]::ErrorGuaranteed>>>::get_or_insert_with::<<rustc_interface[ed5c3e7d6b9d9752]::queries::Query<rustc_ast[164d9789c5234b58]::ast::Crate>>::compute<<rustc_interface[ed5c3e7d6b9d9752]::queries::Queries>::parse::{closure#0}>::{closure#0}::{closure#0}>
                               at /home/lukas/code/rust/library/core/src/option.rs:1591:49
  66:     0x7f5b79ab45ca - <rustc_interface[ed5c3e7d6b9d9752]::queries::Query<rustc_ast[164d9789c5234b58]::ast::Crate>>::compute::<<rustc_interface[ed5c3e7d6b9d9752]::queries::Queries>::parse::{closure#0}>::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_interface/src/queries.rs:43:17
  67:     0x7f5b79ab45ca - <core[672e3947e150d6c6]::cell::RefMut<core[672e3947e150d6c6]::option::Option<core[672e3947e150d6c6]::result::Result<rustc_data_structures[24675f911328660]::steal::Steal<rustc_ast[164d9789c5234b58]::ast::Crate>, rustc_errors[52317f54776bd715]::ErrorGuaranteed>>>>::filter_map::<rustc_data_structures[24675f911328660]::steal::Steal<rustc_ast[164d9789c5234b58]::ast::Crate>, <rustc_interface[ed5c3e7d6b9d9752]::queries::Query<rustc_ast[164d9789c5234b58]::ast::Crate>>::compute<<rustc_interface[ed5c3e7d6b9d9752]::queries::Queries>::parse::{closure#0}>::{closure#0}>
                               at /home/lukas/code/rust/library/core/src/cell.rs:1578:15
  68:     0x7f5b79ab45ca - <rustc_interface[ed5c3e7d6b9d9752]::queries::Query<rustc_ast[164d9789c5234b58]::ast::Crate>>::compute::<<rustc_interface[ed5c3e7d6b9d9752]::queries::Queries>::parse::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_interface/src/queries.rs:40:9
  69:     0x7f5b799239bc - rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}::{closure#2}
                               at /home/lukas/code/rust/compiler/rustc_driver/src/lib.rs:312:13
  70:     0x7f5b799239bc - <rustc_interface[ed5c3e7d6b9d9752]::interface::Compiler>::enter::<rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}::{closure#2}, core[672e3947e150d6c6]::result::Result<core[672e3947e150d6c6]::option::Option<rustc_interface[ed5c3e7d6b9d9752]::queries::Linker>, rustc_errors[52317f54776bd715]::ErrorGuaranteed>>
                               at /home/lukas/code/rust/compiler/rustc_interface/src/queries.rs:391:19
  71:     0x7f5b7997d72b - rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}
                               at /home/lukas/code/rust/compiler/rustc_driver/src/lib.rs:310:22
  72:     0x7f5b7997d72b - rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler::<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_interface/src/interface.rs:325:21
  73:     0x7f5b7997d72b - rustc_span[b471945038545fa4]::with_source_map::<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_span/src/lib.rs:1026:5
  74:     0x7f5b79978f0a - rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler::<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_interface/src/interface.rs:319:13
  75:     0x7f5b79978f0a - <scoped_tls[9108c8ebfd9b9de3]::ScopedKey<rustc_span[b471945038545fa4]::SessionGlobals>>::set::<rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>
                               at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  76:     0x7f5b798f776a - rustc_span[b471945038545fa4]::create_session_globals_then::<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}>
                               at /home/lukas/code/rust/compiler/rustc_span/src/lib.rs:111:5
  77:     0x7f5b798f776a - rustc_interface[ed5c3e7d6b9d9752]::util::run_in_thread_pool_with_globals::<rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#0}::{closure#0}
                               at /home/lukas/code/rust/compiler/rustc_interface/src/util.rs:145:38
  78:     0x7f5b798f776a - std[3b2069d10db46603]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed5c3e7d6b9d9752]::util::run_in_thread_pool_with_globals<rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>
                               at /home/lukas/code/rust/library/std/src/sys_common/backtrace.rs:121:18
  79:     0x7f5b798e5658 - <std[3b2069d10db46603]::thread::Builder>::spawn_unchecked_::<rustc_interface[ed5c3e7d6b9d9752]::util::run_in_thread_pool_with_globals<rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /home/lukas/code/rust/library/std/src/thread/mod.rs:558:17
  80:     0x7f5b798e5658 - <core[672e3947e150d6c6]::panic::unwind_safe::AssertUnwindSafe<<std[3b2069d10db46603]::thread::Builder>::spawn_unchecked_<rustc_interface[ed5c3e7d6b9d9752]::util::run_in_thread_pool_with_globals<rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[672e3947e150d6c6]::ops::function::FnOnce<()>>::call_once
                               at /home/lukas/code/rust/library/core/src/panic/unwind_safe.rs:271:9
  81:     0x7f5b798e5658 - std[3b2069d10db46603]::panicking::try::do_call::<core[672e3947e150d6c6]::panic::unwind_safe::AssertUnwindSafe<<std[3b2069d10db46603]::thread::Builder>::spawn_unchecked_<rustc_interface[ed5c3e7d6b9d9752]::util::run_in_thread_pool_with_globals<rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>
                               at /home/lukas/code/rust/library/std/src/panicking.rs:483:40
  82:     0x7f5b798e5658 - std[3b2069d10db46603]::panicking::try::<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, core[672e3947e150d6c6]::panic::unwind_safe::AssertUnwindSafe<<std[3b2069d10db46603]::thread::Builder>::spawn_unchecked_<rustc_interface[ed5c3e7d6b9d9752]::util::run_in_thread_pool_with_globals<rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /home/lukas/code/rust/library/std/src/panicking.rs:447:19
  83:     0x7f5b79978606 - std[3b2069d10db46603]::panic::catch_unwind::<core[672e3947e150d6c6]::panic::unwind_safe::AssertUnwindSafe<<std[3b2069d10db46603]::thread::Builder>::spawn_unchecked_<rustc_interface[ed5c3e7d6b9d9752]::util::run_in_thread_pool_with_globals<rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>
                               at /home/lukas/code/rust/library/std/src/panic.rs:140:14
  84:     0x7f5b79978606 - <std[3b2069d10db46603]::thread::Builder>::spawn_unchecked_::<rustc_interface[ed5c3e7d6b9d9752]::util::run_in_thread_pool_with_globals<rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#1}
                               at /home/lukas/code/rust/library/std/src/thread/mod.rs:557:30
  85:     0x7f5b79978606 - <<std[3b2069d10db46603]::thread::Builder>::spawn_unchecked_<rustc_interface[ed5c3e7d6b9d9752]::util::run_in_thread_pool_with_globals<rustc_interface[ed5c3e7d6b9d9752]::interface::run_compiler<core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>, rustc_driver[6d8976894bfa954f]::run_compiler::{closure#1}>::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[672e3947e150d6c6]::result::Result<(), rustc_errors[52317f54776bd715]::ErrorGuaranteed>>::{closure#1} as core[672e3947e150d6c6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /home/lukas/code/rust/library/core/src/ops/function.rs:250:5
  86:     0x7f5b78ec05e8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he0efd4d07605773b
                               at /home/lukas/code/rust/library/alloc/src/boxed.rs:1988:9
  87:     0x7f5b78ec05e8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h68e98ff725fb3b40
                               at /home/lukas/code/rust/library/alloc/src/boxed.rs:1988:9
  88:     0x7f5b78eca00c - std::sys::unix::thread::Thread::new::thread_start::he3fbd5537a4f57c6
                               at /home/lukas/code/rust/library/std/src/sys/unix/thread.rs:108:17
  89:     0x7f5b78cb98fd - <unknown>
  90:     0x7f5b78d3ba60 - <unknown>
  91:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
