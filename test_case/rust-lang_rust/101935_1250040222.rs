plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
486 |                         let mut err = self.ecx.struct_span_err(
    |                                                ^^^^^^^^^^^^^^^
    |
    |
note: the lint level is defined here
   --> compiler/rustc_builtin_macros/src/lib.rs:17:9
    |
17  | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
486 |                         let mut err = self.ecx.struct_span_err(
    |                                                ^^^^^^^^^^^^^^^
    |
    |
note: the lint level is defined here
   --> compiler/rustc_builtin_macros/src/lib.rs:16:9
    |
16  | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
593 |             e = self.ecx.struct_span_err(


error: diagnostics should be created using translatable messages
    |
    |
593 |             e = self.ecx.struct_span_err(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
625 |             e = self.ecx.struct_span_err(


error: diagnostics should be created using translatable messages
    |
    |
625 |             e = self.ecx.struct_span_err(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
782 | ...                   let mut err = self.ecx.struct_span_err(sp, &msg);


error: diagnostics should be created using translatable messages
    |
    |
782 | ...                   let mut err = self.ecx.struct_span_err(sp, &msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1282 |         let mut e = ecx.struct_span_err(sp, &format!("invalid format string: {}", err.description));


error: diagnostics should be created using translatable messages
     |
     |
1282 |         let mut e = ecx.struct_span_err(sp, &format!("invalid format string: {}", err.description));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1408 |                 let mut diag = cx.ecx.struct_span_err(*sp, *msg);


error: diagnostics should be created using translatable messages
     |
     |
1408 |                 let mut diag = cx.ecx.struct_span_err(*sp, *msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1412 |                 let mut diag = cx.ecx.struct_span_err(


error: diagnostics should be created using translatable messages
     |
     |
1412 |                 let mut diag = cx.ecx.struct_span_err(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
24 |         ecx.sess.parse_sess.span_diagnostic.span_err(item.span(), "allocators must be statics");


error: diagnostics should be created using translatable messages
   |
   |
24 |         ecx.sess.parse_sess.span_diagnostic.span_err(item.span(), "allocators must be statics");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_builtin_macros/src/source_util.rs:153:36
    |
153 | ...                   self.p.struct_span_err(self.p.token.span, &msg).emit();


error: diagnostics should be created using translatable messages
   --> compiler/rustc_builtin_macros/src/source_util.rs:153:36
    |
153 | ...                   self.p.struct_span_err(self.p.token.span, &msg).emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_builtin_macros/src/source_util.rs:191:20
    |
191 |                 cx.span_err(sp, &format!("{} wasn't a utf-8 file", file.display()));


error: diagnostics should be created using translatable messages
   --> compiler/rustc_builtin_macros/src/source_util.rs:191:20
    |
191 |                 cx.span_err(sp, &format!("{} wasn't a utf-8 file", file.display()));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_builtin_macros/src/source_util.rs:196:16
    |
196 |             cx.span_err(sp, &format!("couldn't read {}: {}", file.display(), e));


error: diagnostics should be created using translatable messages
   --> compiler/rustc_builtin_macros/src/source_util.rs:196:16
    |
196 |             cx.span_err(sp, &format!("couldn't read {}: {}", file.display(), e));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_builtin_macros/src/source_util.rs:221:16
    |
221 |             cx.span_err(sp, &format!("couldn't read {}: {}", file.display(), e));


error: diagnostics should be created using translatable messages
   --> compiler/rustc_builtin_macros/src/source_util.rs:221:16
    |
221 |             cx.span_err(sp, &format!("couldn't read {}: {}", file.display(), e));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
96 |             cx.struct_span_err(
   |                ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
   |
96 |             cx.struct_span_err(
   |                ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
112 |             ast::ItemKind::MacCall(_) => diag.struct_span_warn(attr_sp, msg),


error: diagnostics should be created using translatable messages
    |
    |
112 |             ast::ItemKind::MacCall(_) => diag.struct_span_warn(attr_sp, msg),


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
116 |             _ => diag.struct_span_err(attr_sp, msg).forget_guarantee(),


error: diagnostics should be created using translatable messages
    |
    |
116 |             _ => diag.struct_span_err(attr_sp, msg).forget_guarantee(),


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
407 |                         sd.struct_span_warn(
    |                            ^^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
407 |                         sd.struct_span_warn(
    |                            ^^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
464 |             sd.struct_span_err(i.span, "unsafe functions cannot be used for tests")


error: diagnostics should be created using translatable messages
    |
    |
464 |             sd.struct_span_err(i.span, "unsafe functions cannot be used for tests")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
470 |             sd.struct_span_err(i.span, "async functions cannot be used for tests")


error: diagnostics should be created using translatable messages
    |
    |
470 |             sd.struct_span_err(i.span, "async functions cannot be used for tests")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
485 |             sd.span_err(i.span, "functions used as tests can not have any arguments");


error: diagnostics should be created using translatable messages
    |
    |
485 |             sd.span_err(i.span, "functions used as tests can not have any arguments");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
491 |                 sd.span_err(i.span, "functions using `#[should_panic]` must return `()`");


error: diagnostics should be created using translatable messages
    |
    |
491 |                 sd.span_err(i.span, "functions using `#[should_panic]` must return `()`");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
496 |                     sd.span_err(i.span, "functions used as tests must have signature fn() -> ()");


error: diagnostics should be created using translatable messages
    |
    |
496 |                     sd.span_err(i.span, "functions used as tests must have signature fn() -> ()");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
520 |         cx.sess.parse_sess.span_diagnostic.span_err(


error: diagnostics should be created using translatable messages
    |
    |
520 |         cx.sess.parse_sess.span_diagnostic.span_err(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
23 |         cx.span_err(sp, "trace_macros! accepts only `true` or `false`")


error: diagnostics should be created using translatable messages
   |
   |
23 |         cx.span_err(sp, "trace_macros! accepts only `true` or `false`")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
592 |             let mut e = ecx.struct_span_err(err_sp, msg);


error: diagnostics should be created using translatable messages
    |
    |
592 |             let mut e = ecx.struct_span_err(err_sp, msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
626 | ...                   let mut err = ecx.struct_span_err(span, &msg);


error: diagnostics should be created using translatable messages
    |
    |
626 | ...                   let mut err = ecx.struct_span_err(span, &msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
738 |             let mut err = ecx.struct_span_err(sp, msg);


error: diagnostics should be created using translatable messages
    |
    |
738 |             let mut err = ecx.struct_span_err(sp, msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
748 |             let mut err = ecx.struct_span_err(
    |                               ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
748 |             let mut err = ecx.struct_span_err(
    |                               ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_builtin_macros/src/cmdline_attrs.rs:27:40
   |
27 |             parse_sess.span_diagnostic.span_err(start_span.to(end_span), "invalid crate attribute");


error: diagnostics should be created using translatable messages
  --> compiler/rustc_builtin_macros/src/cmdline_attrs.rs:27:40
   |
27 |             parse_sess.span_diagnostic.span_err(start_span.to(end_span), "invalid crate attribute");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
91 |             self.handler.span_err(


error: diagnostics should be created using translatable messages
   |
   |
91 |             self.handler.span_err(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
120 |             self.handler.span_err(self.source_map.guess_head_span(item.span), msg);


error: diagnostics should be created using translatable messages
    |
    |
120 |             self.handler.span_err(self.source_map.guess_head_span(item.span), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
138 |             self.handler.span_err(self.source_map.guess_head_span(item.span), msg);


error: diagnostics should be created using translatable messages
    |
    |
138 |             self.handler.span_err(self.source_map.guess_head_span(item.span), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
156 |             self.handler.span_err(self.source_map.guess_head_span(item.span), msg);


error: diagnostics should be created using translatable messages
    |
    |
156 |             self.handler.span_err(self.source_map.guess_head_span(item.span), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
167 |                 self.handler.span_err(self.source_map.guess_head_span(item.span), msg);


error: diagnostics should be created using translatable messages
    |
    |
167 |                 self.handler.span_err(self.source_map.guess_head_span(item.span), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
202 |                         .struct_span_err(attr.span, &msg)


error: diagnostics should be created using translatable messages
    |
    |
202 |                         .struct_span_err(attr.span, &msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
227 |             self.handler.span_err(attr.span, &msg);


error: diagnostics should be created using translatable messages
    |
    |
227 |             self.handler.span_err(attr.span, &msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
241 |             self.handler.span_err(attr.span, &msg);


error: diagnostics should be created using translatable messages
    |
    |
241 |             self.handler.span_err(attr.span, &msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
387 |                 sd.struct_span_err(span, "`test_runner` argument must be a path").emit();


error: diagnostics should be created using translatable messages
    |
    |
387 |                 sd.struct_span_err(span, "`test_runner` argument must be a path").emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
391 |             sd.struct_span_err(span, "`#![test_runner(..)]` accepts exactly 1 argument").emit();


error: diagnostics should be created using translatable messages
    |
    |
391 |             sd.struct_span_err(span, "`#![test_runner(..)]` accepts exactly 1 argument").emit();

error: could not compile `rustc_builtin_macros` due to 76 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_builtin_macros` due to 76 previous errors
