
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: expected one of `,` or `>`, found `=>`
  --> /home/jyn/src/rust/rustfmt-crash.rs:56:54
   |
56 |                     assert!(cx.tcx.PrimVal::Bytes(b) => {
   |                                                      ^^ expected one of `,` or `>`
   |
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                        at ./compiler/rustc_errors/src/lib.rs:1335:29
              1: <rustc_errors::Handler>::emit_diagnostic
                        at ./compiler/rustc_errors/src/lib.rs:1124:9
              2: <rustc_span::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
                        at ./compiler/rustc_errors/src/diagnostic_builder.rs:169:28
              3: <rustc_parse::parser::Parser>::handle_ambiguous_unbraced_const_arg
                        at ./compiler/rustc_parse/src/parser/diagnostics.rs:2172:17
              4: <rustc_parse::parser::Parser>::parse_angle_args
                        at ./compiler/rustc_parse/src/parser/path.rs:566:24
              5: <rustc_parse::parser::Parser>::parse_expr_dot_or_call_with_
                        at ./compiler/rustc_parse/src/parser/diagnostics.rs:924:19
              6: <rustc_parse::parser::Parser>::parse_expr_dot_or_call_with
                        at ./compiler/rustc_parse/src/parser/expr.rs:936:19
              7: <rustc_parse::parser::Parser>::collect_tokens_for_expr::<<rustc_parse::parser::Parser>::parse_expr_dot_or_call::{closure#0}>::{closure#0}
                        at ./compiler/rustc_parse/src/parser/expr.rs:923:13
              8: <rustc_parse::parser::Parser>::collect_tokens_for_expr::<<rustc_parse::parser::Parser>::parse_expr_dot_or_call::{closure#0}>
                        at ./compiler/rustc_parse/src/parser/attr_wrapper.rs:213:23
              9: <rustc_parse::parser::Parser>::parse_expr_prefix
                        at ./compiler/rustc_parse/src/parser/expr.rs:920:9
             10: <rustc_parse::parser::Parser>::parse_expr_assoc_with
                        at ./compiler/rustc_parse/src/parser/expr.rs:174:17
             11: <rustc_parse::parser::Parser>::with_res::<core::result::Result<rustc_ast::ptr::P<rustc_ast::ast::Expr>, rustc_errors::diagnostic_builder::DiagnosticBuilder<rustc_span::ErrorGuaranteed>>, <rustc_parse::parser::Parser>::parse_expr_res::{closure#0}>
                        at ./compiler/rustc_parse/src/parser/expr.rs:153:9
             12: rustfmt_nightly::parse::macros::parse_macro_arg
                        at ./compiler/rustc_parse/src/parser/expr.rs:141:9
             13: rustfmt_nightly::parse::macros::parse_macro_args
                        at ./src/tools/rustfmt/src/parse/macros/mod.rs:110:39
             14: rustfmt_nightly::macros::rewrite_macro_inner
                        at ./src/tools/rustfmt/src/macros.rs:239:15
             15: <core::panic::unwind_safe::AssertUnwindSafe<rustfmt_nightly::macros::rewrite_macro::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
                        at ./src/tools/rustfmt/src/macros.rs:167:13
             16: std::panicking::try::<core::option::Option<alloc::string::String>, core::panic::unwind_safe::AssertUnwindSafe<rustfmt_nightly::macros::rewrite_macro::{closure#0}>>
                        at ./library/std/src/panicking.rs:485:40
             17: rustfmt_nightly::macros::rewrite_macro
                        at ./library/std/src/panic.rs:142:14
             18: <rustfmt_nightly::visitor::FmtVisitor>::with_context::<<rustfmt_nightly::visitor::FmtVisitor>::visit_mac::{closure#0}>
                        at ./src/tools/rustfmt/src/visitor.rs:686:47
             19: <rustfmt_nightly::visitor::FmtVisitor>::visit_mac
                        at ./src/tools/rustfmt/src/visitor.rs:686:23
             20: <rustfmt_nightly::visitor::FmtVisitor>::walk_stmts
                        at ./src/tools/rustfmt/src/visitor.rs:178:21
             21: <rustfmt_nightly::visitor::FmtVisitor>::visit_block
                        at ./src/tools/rustfmt/src/visitor.rs:911:9
             22: rustfmt_nightly::expr::rewrite_block_with_visitor
                        at ./src/tools/rustfmt/src/expr.rs:557:5
             23: rustfmt_nightly::expr::format_expr
                        at ./src/tools/rustfmt/src/expr.rs:152:25
             24: <rustfmt_nightly::matches::ArmWrapper as rustfmt_nightly::rewrite::Rewrite>::rewrite
                        at ./src/tools/rustfmt/src/matches.rs:472:13
             25: <rustfmt_nightly::lists::ListItems<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<rustc_ast::ast::Arm>, core::iter::adapters::chain::Chain<core::iter::adapters::take::Take<core::iter::sources::repeat::Repeat<bool>>, core::iter::sources::repeat::Repeat<bool>>>, alloc::vec::into_iter::IntoIter<core::option::Option<rustc_span::BytePos>>>, rustfmt_nightly::matches::rewrite_match_arms::{closure#0}>, rustfmt_nightly::matches::rewrite_match_arms::{closure#1}, rustfmt_nightly::matches::rewrite_match_arms::{closure#2}, rustfmt_nightly::matches::rewrite_match_arms::{closure#3}> as core::iter::traits::iterator::Iterator>::next
                        at ./src/tools/rustfmt/src/matches.rs:203:15
             26: <alloc::vec::Vec<rustfmt_nightly::lists::ListItem> as alloc::vec::spec_from_iter::SpecFromIter<rustfmt_nightly::lists::ListItem, rustfmt_nightly::lists::ListItems<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<rustc_ast::ast::Arm>, core::iter::adapters::chain::Chain<core::iter::adapters::take::Take<core::iter::sources::repeat::Repeat<bool>>, core::iter::sources::repeat::Repeat<bool>>>, alloc::vec::into_iter::IntoIter<core::option::Option<rustc_span::BytePos>>>, rustfmt_nightly::matches::rewrite_match_arms::{closure#0}>, rustfmt_nightly::matches::rewrite_match_arms::{closure#1}, rustfmt_nightly::matches::rewrite_match_arms::{closure#2}, rustfmt_nightly::matches::rewrite_match_arms::{closure#3}>>>::from_iter
                        at ./library/alloc/src/vec/mod.rs:2812:35
             27: rustfmt_nightly::matches::rewrite_match
                        at ./library/alloc/src/vec/mod.rs:2712:9
             28: rustfmt_nightly::expr::format_expr
                        at ./src/tools/rustfmt/src/expr.rs:169:13
             29: rustfmt_nightly::expr::rewrite_assign_rhs_expr::<rustc_ast::ast::Expr>
                        at ./src/tools/rustfmt/src/expr.rs:41:9
             30: rustfmt_nightly::expr::rewrite_assign_rhs_with::<alloc::string::String, rustc_ast::ast::Expr>
                        at ./src/tools/rustfmt/src/expr.rs:1994:15
             31: <rustc_ast::ast::Local as rustfmt_nightly::rewrite::Rewrite>::rewrite
                        at ./src/tools/rustfmt/src/expr.rs:1946:5
             32: rustfmt_nightly::stmt::format_stmt
                        at ./src/tools/rustfmt/src/rewrite.rs:23:9
             33: <rustfmt_nightly::stmt::Stmt as rustfmt_nightly::rewrite::Rewrite>::rewrite
                        at ./src/tools/rustfmt/src/stmt.rs:83:9
             34: <rustfmt_nightly::visitor::FmtVisitor>::with_context::<<rustfmt_nightly::visitor::FmtVisitor>::visit_stmt::{closure#2}>
                        at ./src/tools/rustfmt/src/visitor.rs:166:59
             35: <rustfmt_nightly::visitor::FmtVisitor>::walk_stmts
                        at ./src/tools/rustfmt/src/visitor.rs:166:35
             36: <rustfmt_nightly::visitor::FmtVisitor>::walk_stmts
             37: <rustfmt_nightly::visitor::FmtVisitor>::visit_block
                        at ./src/tools/rustfmt/src/visitor.rs:911:9
             38: rustfmt_nightly::expr::rewrite_block_with_visitor
                        at ./src/tools/rustfmt/src/expr.rs:557:5
             39: rustfmt_nightly::expr::format_expr
                        at ./src/tools/rustfmt/src/expr.rs:152:25
             40: <rustfmt_nightly::matches::ArmWrapper as rustfmt_nightly::rewrite::Rewrite>::rewrite
                        at ./src/tools/rustfmt/src/matches.rs:472:13
             41: <rustfmt_nightly::lists::ListItems<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<rustc_ast::ast::Arm>, core::iter::adapters::chain::Chain<core::iter::adapters::take::Take<core::iter::sources::repeat::Repeat<bool>>, core::iter::sources::repeat::Repeat<bool>>>, alloc::vec::into_iter::IntoIter<core::option::Option<rustc_span::BytePos>>>, rustfmt_nightly::matches::rewrite_match_arms::{closure#0}>, rustfmt_nightly::matches::rewrite_match_arms::{closure#1}, rustfmt_nightly::matches::rewrite_match_arms::{closure#2}, rustfmt_nightly::matches::rewrite_match_arms::{closure#3}> as core::iter::traits::iterator::Iterator>::next
                        at ./src/tools/rustfmt/src/matches.rs:203:15
             42: <alloc::vec::Vec<rustfmt_nightly::lists::ListItem> as alloc::vec::spec_from_iter::SpecFromIter<rustfmt_nightly::lists::ListItem, rustfmt_nightly::lists::ListItems<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<rustc_ast::ast::Arm>, core::iter::adapters::chain::Chain<core::iter::adapters::take::Take<core::iter::sources::repeat::Repeat<bool>>, core::iter::sources::repeat::Repeat<bool>>>, alloc::vec::into_iter::IntoIter<core::option::Option<rustc_span::BytePos>>>, rustfmt_nightly::matches::rewrite_match_arms::{closure#0}>, rustfmt_nightly::matches::rewrite_match_arms::{closure#1}, rustfmt_nightly::matches::rewrite_match_arms::{closure#2}, rustfmt_nightly::matches::rewrite_match_arms::{closure#3}>>>::from_iter
                        at ./library/alloc/src/vec/mod.rs:2812:35
             43: rustfmt_nightly::matches::rewrite_match
                        at ./library/alloc/src/vec/mod.rs:2712:9
             44: rustfmt_nightly::expr::format_expr
                        at ./src/tools/rustfmt/src/expr.rs:169:13
             45: rustfmt_nightly::stmt::format_stmt
                        at ./src/tools/rustfmt/src/stmt.rs:111:13
             46: <rustfmt_nightly::stmt::Stmt as rustfmt_nightly::rewrite::Rewrite>::rewrite
                        at ./src/tools/rustfmt/src/stmt.rs:83:9
             47: <rustfmt_nightly::visitor::FmtVisitor>::with_context::<<rustfmt_nightly::visitor::FmtVisitor>::visit_stmt::{closure#2}>
                        at ./src/tools/rustfmt/src/visitor.rs:166:59
             48: <rustfmt_nightly::visitor::FmtVisitor>::walk_stmts
                        at ./src/tools/rustfmt/src/visitor.rs:166:35
             49: <rustfmt_nightly::visitor::FmtVisitor>::walk_stmts
             50: <rustfmt_nightly::visitor::FmtVisitor>::visit_block
                        at ./src/tools/rustfmt/src/visitor.rs:911:9
             51: <rustfmt_nightly::visitor::FmtVisitor>::visit_fn
                        at ./src/tools/rustfmt/src/visitor.rs:427:9
             52: <rustfmt_nightly::visitor::FmtVisitor>::visit_item
                        at ./src/tools/rustfmt/src/visitor.rs:553:25
             53: <rustfmt_nightly::visitor::FmtVisitor>::visit_items_with_reordering
                        at ./src/tools/rustfmt/src/reorder.rs:325:17
             54: <rustfmt_nightly::visitor::FmtVisitor>::format_separate_mod
                        at ./src/tools/rustfmt/src/visitor.rs:864:9
             55: rustfmt_nightly::formatting::format_project::<rustfmt_nightly::Session<std::io::stdio::Stdout>>
                        at ./src/tools/rustfmt/src/formatting.rs:213:9
             56: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::with::<<rustfmt_nightly::Session<std::io::stdio::Stdout>>::format_input_inner::{closure#0}, core::result::Result<rustfmt_nightly::FormatReport, rustfmt_nightly::ErrorKind>>
                        at ./src/tools/rustfmt/src/formatting.rs:48:33
             57: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_span::create_session_if_not_set_then<core::result::Result<rustfmt_nightly::FormatReport, rustfmt_nightly::ErrorKind>, <rustfmt_nightly::Session<std::io::stdio::Stdout>>::format_input_inner::{closure#0}>::{closure#0}, core::result::Result<rustfmt_nightly::FormatReport, rustfmt_nightly::ErrorKind>>
                        at ./compiler/rustc_span/src/lib.rs:148:50
             58: <rustfmt_nightly::Session<std::io::stdio::Stdout>>::format
                        at ./compiler/rustc_span/src/lib.rs:148:9
             59: rustfmt::format_and_emit_report::<std::io::stdio::Stdout>
                        at ./src/tools/rustfmt/src/bin/main.rs:368:11
             60: <rustfmt_nightly::Session<std::io::stdio::Stdout>>::override_config::<rustfmt::format::{closure#0}, ()>
                        at ./src/tools/rustfmt/src/bin/main.rs:340:21
             61: rustfmt::execute
                        at ./src/tools/rustfmt/src/bin/main.rs:339:17
             62: rustfmt::main
                        at ./src/tools/rustfmt/src/bin/main.rs:35:27
             63: std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>
                        at ./library/core/src/ops/function.rs:250:5
             64: std::rt::lang_start::<()>::{closure#0}
                        at ./library/std/src/rt.rs:166:18
             65: std::panicking::try
             66: std::rt::lang_start_internal
             67: main
             68: __libc_start_call_main
                        at ./csu/../sysdeps/nptl/libc_start_call_main.h:58:16
             69: __libc_start_main_impl
                        at ./csu/../csu/libc-start.c:392:3
             70: _start
           

error: internal compiler error: expected one of `,` or `>`, found `=>`
  --> /home/jyn/src/rust/rustfmt-crash.rs:56:54
   |
56 |                     assert!(cx.tcx.PrimVal::Bytes(b) => {
   |                                                      ^^ expected one of `,` or `>`
   |
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                        at ./compiler/rustc_errors/src/lib.rs:1335:29
              1: <rustc_errors::Handler>::emit_diagnostic
                        at ./compiler/rustc_errors/src/lib.rs:1124:9
              2: <rustc_span::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
                        at ./compiler/rustc_errors/src/diagnostic_builder.rs:169:28
              3: <rustc_parse::parser::Parser>::handle_ambiguous_unbraced_const_arg
                        at ./compiler/rustc_parse/src/parser/diagnostics.rs:2172:17
              4: <rustc_parse::parser::Parser>::parse_angle_args
                        at ./compiler/rustc_parse/src/parser/path.rs:566:24
              5: <rustc_parse::parser::Parser>::parse_expr_dot_or_call_with_
                        at ./compiler/rustc_parse/src/parser/diagnostics.rs:924:19
              6: <rustc_parse::parser::Parser>::parse_expr_dot_or_call_with
                        at ./compiler/rustc_parse/src/parser/expr.rs:936:19
              7: <rustc_parse::parser::Parser>::collect_tokens_for_expr::<<rustc_parse::parser::Parser>::parse_expr_dot_or_call::{closure#0}>::{closure#0}
                        at ./compiler/rustc_parse/src/parser/expr.rs:923:13
              8: <rustc_parse::parser::Parser>::collect_tokens_for_expr::<<rustc_parse::parser::Parser>::parse_expr_dot_or_call::{closure#0}>
                        at ./compiler/rustc_parse/src/parser/attr_wrapper.rs:213:23
              9: <rustc_parse::parser::Parser>::parse_expr_prefix
                        at ./compiler/rustc_parse/src/parser/expr.rs:920:9
             10: <rustc_parse::parser::Parser>::parse_expr_assoc_with
                        at ./compiler/rustc_parse/src/parser/expr.rs:174:17
             11: <rustc_parse::parser::Parser>::with_res::<core::result::Result<rustc_ast::ptr::P<rustc_ast::ast::Expr>, rustc_errors::diagnostic_builder::DiagnosticBuilder<rustc_span::ErrorGuaranteed>>, <rustc_parse::parser::Parser>::parse_expr_res::{closure#0}>
                        at ./compiler/rustc_parse/src/parser/expr.rs:153:9
             12: rustfmt_nightly::parse::macros::parse_macro_arg
                        at ./compiler/rustc_parse/src/parser/expr.rs:141:9
             13: rustfmt_nightly::parse::macros::parse_macro_args
                        at ./src/tools/rustfmt/src/parse/macros/mod.rs:110:39
             14: rustfmt_nightly::macros::rewrite_macro_inner
                        at ./src/tools/rustfmt/src/macros.rs:239:15
             15: <core::panic::unwind_safe::AssertUnwindSafe<rustfmt_nightly::macros::rewrite_macro::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
                        at ./src/tools/rustfmt/src/macros.rs:167:13
             16: std::panicking::try::<core::option::Option<alloc::string::String>, core::panic::unwind_safe::AssertUnwindSafe<rustfmt_nightly::macros::rewrite_macro::{closure#0}>>
                        at ./library/std/src/panicking.rs:485:40
             17: rustfmt_nightly::macros::rewrite_macro
                        at ./library/std/src/panic.rs:142:14
             18: <rustfmt_nightly::visitor::FmtVisitor>::with_context::<<rustfmt_nightly::visitor::FmtVisitor>::visit_mac::{closure#0}>
                        at ./src/tools/rustfmt/src/visitor.rs:686:47
             19: <rustfmt_nightly::visitor::FmtVisitor>::visit_mac
                        at ./src/tools/rustfmt/src/visitor.rs:686:23
             20: <rustfmt_nightly::visitor::FmtVisitor>::walk_stmts
                        at ./src/tools/rustfmt/src/visitor.rs:178:21
             21: <rustfmt_nightly::visitor::FmtVisitor>::visit_block
                        at ./src/tools/rustfmt/src/visitor.rs:911:9
             22: rustfmt_nightly::expr::rewrite_block_with_visitor
                        at ./src/tools/rustfmt/src/expr.rs:557:5
             23: rustfmt_nightly::expr::format_expr
                        at ./src/tools/rustfmt/src/expr.rs:152:25
             24: <rustfmt_nightly::matches::ArmWrapper as rustfmt_nightly::rewrite::Rewrite>::rewrite
                        at ./src/tools/rustfmt/src/matches.rs:472:13
             25: <rustfmt_nightly::lists::ListItems<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<rustc_ast::ast::Arm>, core::iter::adapters::chain::Chain<core::iter::adapters::take::Take<core::iter::sources::repeat::Repeat<bool>>, core::iter::sources::repeat::Repeat<bool>>>, alloc::vec::into_iter::IntoIter<core::option::Option<rustc_span::BytePos>>>, rustfmt_nightly::matches::rewrite_match_arms::{closure#0}>, rustfmt_nightly::matches::rewrite_match_arms::{closure#1}, rustfmt_nightly::matches::rewrite_match_arms::{closure#2}, rustfmt_nightly::matches::rewrite_match_arms::{closure#3}> as core::iter::traits::iterator::Iterator>::next
                        at ./src/tools/rustfmt/src/matches.rs:203:15
             26: <alloc::vec::Vec<rustfmt_nightly::lists::ListItem> as alloc::vec::spec_from_iter::SpecFromIter<rustfmt_nightly::lists::ListItem, rustfmt_nightly::lists::ListItems<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<rustc_ast::ast::Arm>, core::iter::adapters::chain::Chain<core::iter::adapters::take::Take<core::iter::sources::repeat::Repeat<bool>>, core::iter::sources::repeat::Repeat<bool>>>, alloc::vec::into_iter::IntoIter<core::option::Option<rustc_span::BytePos>>>, rustfmt_nightly::matches::rewrite_match_arms::{closure#0}>, rustfmt_nightly::matches::rewrite_match_arms::{closure#1}, rustfmt_nightly::matches::rewrite_match_arms::{closure#2}, rustfmt_nightly::matches::rewrite_match_arms::{closure#3}>>>::from_iter
                        at ./library/alloc/src/vec/mod.rs:2812:35
             27: rustfmt_nightly::matches::rewrite_match
                        at ./library/alloc/src/vec/mod.rs:2712:9
             28: rustfmt_nightly::expr::format_expr
                        at ./src/tools/rustfmt/src/expr.rs:169:13
             29: rustfmt_nightly::expr::rewrite_assign_rhs_expr::<rustc_ast::ast::Expr>
                        at ./src/tools/rustfmt/src/expr.rs:41:9
             30: rustfmt_nightly::expr::rewrite_assign_rhs_with::<alloc::string::String, rustc_ast::ast::Expr>
                        at ./src/tools/rustfmt/src/expr.rs:1994:15
             31: <rustc_ast::ast::Local as rustfmt_nightly::rewrite::Rewrite>::rewrite
                        at ./src/tools/rustfmt/src/expr.rs:1946:5
             32: rustfmt_nightly::stmt::format_stmt
                        at ./src/tools/rustfmt/src/rewrite.rs:23:9
             33: <rustfmt_nightly::stmt::Stmt as rustfmt_nightly::rewrite::Rewrite>::rewrite
                        at ./src/tools/rustfmt/src/stmt.rs:83:9
             34: <rustfmt_nightly::visitor::FmtVisitor>::with_context::<<rustfmt_nightly::visitor::FmtVisitor>::visit_stmt::{closure#2}>
                        at ./src/tools/rustfmt/src/visitor.rs:166:59
             35: <rustfmt_nightly::visitor::FmtVisitor>::walk_stmts
                        at ./src/tools/rustfmt/src/visitor.rs:166:35
             36: <rustfmt_nightly::visitor::FmtVisitor>::walk_stmts
             37: <rustfmt_nightly::visitor::FmtVisitor>::visit_block
                        at ./src/tools/rustfmt/src/visitor.rs:911:9
             38: rustfmt_nightly::expr::rewrite_block_with_visitor
                        at ./src/tools/rustfmt/src/expr.rs:557:5
             39: rustfmt_nightly::expr::format_expr
                        at ./src/tools/rustfmt/src/expr.rs:152:25
             40: <rustfmt_nightly::matches::ArmWrapper as rustfmt_nightly::rewrite::Rewrite>::rewrite
                        at ./src/tools/rustfmt/src/matches.rs:472:13
             41: <rustfmt_nightly::lists::ListItems<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<rustc_ast::ast::Arm>, core::iter::adapters::chain::Chain<core::iter::adapters::take::Take<core::iter::sources::repeat::Repeat<bool>>, core::iter::sources::repeat::Repeat<bool>>>, alloc::vec::into_iter::IntoIter<core::option::Option<rustc_span::BytePos>>>, rustfmt_nightly::matches::rewrite_match_arms::{closure#0}>, rustfmt_nightly::matches::rewrite_match_arms::{closure#1}, rustfmt_nightly::matches::rewrite_match_arms::{closure#2}, rustfmt_nightly::matches::rewrite_match_arms::{closure#3}> as core::iter::traits::iterator::Iterator>::next
                        at ./src/tools/rustfmt/src/matches.rs:203:15
             42: <alloc::vec::Vec<rustfmt_nightly::lists::ListItem> as alloc::vec::spec_from_iter::SpecFromIter<rustfmt_nightly::lists::ListItem, rustfmt_nightly::lists::ListItems<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<rustc_ast::ast::Arm>, core::iter::adapters::chain::Chain<core::iter::adapters::take::Take<core::iter::sources::repeat::Repeat<bool>>, core::iter::sources::repeat::Repeat<bool>>>, alloc::vec::into_iter::IntoIter<core::option::Option<rustc_span::BytePos>>>, rustfmt_nightly::matches::rewrite_match_arms::{closure#0}>, rustfmt_nightly::matches::rewrite_match_arms::{closure#1}, rustfmt_nightly::matches::rewrite_match_arms::{closure#2}, rustfmt_nightly::matches::rewrite_match_arms::{closure#3}>>>::from_iter
                        at ./library/alloc/src/vec/mod.rs:2812:35
             43: rustfmt_nightly::matches::rewrite_match
                        at ./library/alloc/src/vec/mod.rs:2712:9
             44: rustfmt_nightly::expr::format_expr
                        at ./src/tools/rustfmt/src/expr.rs:169:13
             45: rustfmt_nightly::stmt::format_stmt
                        at ./src/tools/rustfmt/src/stmt.rs:111:13
             46: <rustfmt_nightly::stmt::Stmt as rustfmt_nightly::rewrite::Rewrite>::rewrite
                        at ./src/tools/rustfmt/src/stmt.rs:83:9
             47: <rustfmt_nightly::visitor::FmtVisitor>::with_context::<<rustfmt_nightly::visitor::FmtVisitor>::visit_stmt::{closure#2}>
                        at ./src/tools/rustfmt/src/visitor.rs:166:59
             48: <rustfmt_nightly::visitor::FmtVisitor>::walk_stmts
                        at ./src/tools/rustfmt/src/visitor.rs:166:35
             49: <rustfmt_nightly::visitor::FmtVisitor>::walk_stmts
             50: <rustfmt_nightly::visitor::FmtVisitor>::visit_block
                        at ./src/tools/rustfmt/src/visitor.rs:911:9
             51: <rustfmt_nightly::visitor::FmtVisitor>::visit_fn
                        at ./src/tools/rustfmt/src/visitor.rs:427:9
             52: <rustfmt_nightly::visitor::FmtVisitor>::visit_item
                        at ./src/tools/rustfmt/src/visitor.rs:553:25
             53: <rustfmt_nightly::visitor::FmtVisitor>::visit_items_with_reordering
                        at ./src/tools/rustfmt/src/reorder.rs:325:17
             54: <rustfmt_nightly::visitor::FmtVisitor>::format_separate_mod
                        at ./src/tools/rustfmt/src/visitor.rs:864:9
             55: rustfmt_nightly::formatting::format_project::<rustfmt_nightly::Session<std::io::stdio::Stdout>>
                        at ./src/tools/rustfmt/src/formatting.rs:213:9
             56: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::with::<<rustfmt_nightly::Session<std::io::stdio::Stdout>>::format_input_inner::{closure#0}, core::result::Result<rustfmt_nightly::FormatReport, rustfmt_nightly::ErrorKind>>
                        at ./src/tools/rustfmt/src/formatting.rs:48:33
             57: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_span::create_session_if_not_set_then<core::result::Result<rustfmt_nightly::FormatReport, rustfmt_nightly::ErrorKind>, <rustfmt_nightly::Session<std::io::stdio::Stdout>>::format_input_inner::{closure#0}>::{closure#0}, core::result::Result<rustfmt_nightly::FormatReport, rustfmt_nightly::ErrorKind>>
                        at ./compiler/rustc_span/src/lib.rs:148:50
             58: <rustfmt_nightly::Session<std::io::stdio::Stdout>>::format
                        at ./compiler/rustc_span/src/lib.rs:148:9
             59: rustfmt::format_and_emit_report::<std::io::stdio::Stdout>
                        at ./src/tools/rustfmt/src/bin/main.rs:368:11
             60: <rustfmt_nightly::Session<std::io::stdio::Stdout>>::override_config::<rustfmt::format::{closure#0}, ()>
                        at ./src/tools/rustfmt/src/bin/main.rs:340:21
             61: rustfmt::execute
                        at ./src/tools/rustfmt/src/bin/main.rs:339:17
             62: rustfmt::main
                        at ./src/tools/rustfmt/src/bin/main.rs:35:27
             63: std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>
                        at ./library/core/src/ops/function.rs:250:5
             64: std::rt::lang_start::<()>::{closure#0}
                        at ./library/std/src/rt.rs:166:18
             65: std::panicking::try
             66: std::rt::lang_start_internal
             67: main
             68: __libc_start_call_main
                        at ./csu/../sysdeps/nptl/libc_start_call_main.h:58:16
             69: __libc_start_main_impl
                        at ./csu/../csu/libc-start.c:392:3
             70: _start
