plain
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: diagnostic with static strings only
  --> compiler/rustc_builtin_macros/src/test.rs:43:13
   |
43 | /             ecx.struct_span_err(
44 | |                 anno_item.span(),
45 | |                 "`#[test_case]` attribute is only allowed on items",
47 | |             .emit();
   | |____________________^
   |
   |
   = note: `#[deny(rustc::untranslatable_diagnostic_trivial)]` on by default
error: diagnostic with static strings only
   --> compiler/rustc_builtin_macros/src/test.rs:536:17
    |
    |
536 | /                 sd.struct_span_err(i.span, "unsafe functions cannot be used for tests")
537 | |                     .span_label(span, "`unsafe` because of this")
538 | |                     .emit();

error: diagnostic with static strings only
   --> compiler/rustc_builtin_macros/src/test.rs:542:17
    |
    |
542 | /                 sd.struct_span_err(i.span, "async functions cannot be used for tests")
543 | |                     .span_label(span, "`async` because of this")
544 | |                     .emit();

error: diagnostic with static strings only
   --> compiler/rustc_builtin_macros/src/asm.rs:208:17
    |
    |
208 |                 diag.struct_span_err(span, "explicit register arguments cannot have names").emit();

error: diagnostic with static strings only
   --> compiler/rustc_builtin_macros/src/asm.rs:243:9
    |
    |
243 | /         diag.struct_span_err(spans, "the `nomem` and `readonly` options are mutually exclusive")
244 | |             .emit();

error: diagnostic with static strings only
   --> compiler/rustc_builtin_macros/src/asm.rs:250:9
    |
    |
250 | /         diag.struct_span_err(spans, "the `pure` and `noreturn` options are mutually exclusive")
251 | |             .emit();

error: diagnostic with static strings only
   --> compiler/rustc_builtin_macros/src/asm.rs:257:9
    |
    |
257 | /         diag.struct_span_err(
258 | |             spans,
259 | |             "the `pure` option must be combined with either `nomem` or `readonly`",
261 | |         .emit();
    | |________________^

error: diagnostic with static strings only
error: diagnostic with static strings only
   --> compiler/rustc_builtin_macros/src/asm.rs:288:9
    |
288 | /         diag.struct_span_err(
289 | |             args.options_spans.clone(),
290 | |             "asm with the `pure` option must have at least one output",
292 | |         .emit();
    | |________________^

error: diagnostic with static strings only
error: diagnostic with static strings only
   --> compiler/rustc_builtin_macros/src/asm.rs:708:25
    |
708 | /                         ecx.struct_span_err(
709 | |                             span,
710 | |                             "asm template modifier must be a single character",
712 | |                         .emit();
    | |________________________________^

error: diagnostic with static strings only
error: diagnostic with static strings only
   --> compiler/rustc_builtin_macros/src/test_harness.rs:388:17
    |
388 |                 sd.struct_span_err(span, "`test_runner` argument must be a path").emit();

error: diagnostic with static strings only
   --> compiler/rustc_builtin_macros/src/test_harness.rs:392:13
    |
    |
392 |             sd.struct_span_err(span, "`#![test_runner(..)]` accepts exactly 1 argument").emit();

error: could not compile `rustc_builtin_macros` due to 11 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:05:52
