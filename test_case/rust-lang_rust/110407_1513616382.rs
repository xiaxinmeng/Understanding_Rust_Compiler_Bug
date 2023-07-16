plain
Uplifting rustc (stage1 -> stage3)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 52 tests
.........................................F.F........
failures:

---- [ui] tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs stdout ----
normalized stderr:
normalized stderr:
error: unsupported type attribute for diagnostic derive enum
  --> $DIR/diagnostic-derive.rs:44:1
   |
LL | #[diag(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
   |
LL |     Foo,
   |     ^^^
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
   |
LL |     Bar,
   |     ^^^
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: expected parentheses: #[diag(...)]
   |
   |
LL | #[diag = "E0123"]


error: `#[nonsense(...)]` is not a valid attribute
   |
   |
LL | #[nonsense(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[nonsense(no_crate_example, code = "E0123")]
LL | |
LL | |
LL | |
LL | | struct InvalidStructAttr {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
   |
   |
LL | / #[diag("E0123")]
LL | |
LL | | struct InvalidLitNestedAttr {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug must be the first argument
   |
   |
LL | #[diag(nonsense("foo"), code = "E0123", slug = "foo")]


error: diagnostic slug not specified
   |
   |
LL | / #[diag(nonsense("foo"), code = "E0123", slug = "foo")]
LL | |
LL | |
LL | | struct InvalidNestedStructAttr1 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
error: unknown argument
  --> $DIR/diagnostic-derive.rs:83:8
   |
   |
LL | #[diag(nonsense = "...", code = "E0123", slug = "foo")]
   |
   |
   = note: only the `code` parameter is valid after the slug

error: diagnostic slug not specified
   |
   |
LL | / #[diag(nonsense = "...", code = "E0123", slug = "foo")]
LL | |
LL | |
LL | | struct InvalidNestedStructAttr2 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
error: unknown argument
  --> $DIR/diagnostic-derive.rs:89:8
   |
   |
LL | #[diag(nonsense = 4, code = "E0123", slug = "foo")]
   |
   |
   = note: only the `code` parameter is valid after the slug

error: diagnostic slug not specified
   |
   |
LL | / #[diag(nonsense = 4, code = "E0123", slug = "foo")]
LL | |
LL | |
LL | | struct InvalidNestedStructAttr3 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
error: unknown argument
  --> $DIR/diagnostic-derive.rs:95:42
   |
   |
LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
   |
   |
   = note: only the `code` parameter is valid after the slug

error: `#[suggestion = ...]` is not a valid attribute
   |
   |
LL |     #[suggestion = "bar"]

error: specified multiple times
  --> $DIR/diagnostic-derive.rs:109:8
   |
   |
LL | #[diag(no_crate_example, code = "E0456")]
   |
note: previously specified here
  --> $DIR/diagnostic-derive.rs:108:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]

error: specified multiple times
  --> $DIR/diagnostic-derive.rs:109:26
   |
   |
LL | #[diag(no_crate_example, code = "E0456")]
   |
note: previously specified here
  --> $DIR/diagnostic-derive.rs:108:26
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]

error: specified multiple times
  --> $DIR/diagnostic-derive.rs:115:42
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
   |
note: previously specified here
  --> $DIR/diagnostic-derive.rs:115:26
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]


error: diagnostic slug must be the first argument
   |
   |
LL | #[diag(no_crate_example, no_crate::example, code = "E0456")]


error: diagnostic slug not specified
   |
   |
LL | struct KindNotProvided {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
   |
   |
LL | / #[diag(code = "E0456")]
LL | |
LL | | struct SlugNotProvided {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
   |
   |
LL |     #[primary_span]


error: `#[nonsense]` is not a valid attribute
   |
   |
LL |     #[nonsense]


error: the `#[label(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
   |
   |
LL |     #[label(no_crate_label)]

error: `name` doesn't refer to a field on this type
  --> $DIR/diagnostic-derive.rs:172:46
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "{name}")]


error: invalid format string: expected `'}'` but string was terminated
   |
LL | #[derive(Diagnostic)]
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ expected `'}'` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: invalid format string: unmatched `}` found
   |
LL | #[derive(Diagnostic)]
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ unmatched `}` in format string
   |
   = note: if you intended to print `}`, you can escape it using `}}`
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: the `#[label(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
   |
   |
LL |     #[label(no_crate_label)]


error: suggestion without `code = "..."`
   |
   |
LL |     #[suggestion(no_crate_suggestion)]

error: invalid nested attribute
  --> $DIR/diagnostic-derive.rs:234:18
   |
   |
LL |     #[suggestion(nonsense = "bar")]
   |
   |
   = help: only `style`, `code` and `applicability` are valid nested attributes

error: suggestion without `code = "..."`
   |
   |
LL |     #[suggestion(nonsense = "bar")]

error: invalid nested attribute
  --> $DIR/diagnostic-derive.rs:243:18
   |
   |
LL |     #[suggestion(msg = "bar")]
   |
   |
   = help: only `style`, `code` and `applicability` are valid nested attributes

error: suggestion without `code = "..."`
   |
   |
LL |     #[suggestion(msg = "bar")]

error: wrong field type for suggestion
  --> $DIR/diagnostic-derive.rs:266:5
   |
   |
LL | /     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
LL | |
LL | |     suggestion: Applicability,
   |
   |
   = help: `#[suggestion(...)]` should be applied to fields of type `Span` or `(Span, Applicability)`
error: specified multiple times
  --> $DIR/diagnostic-derive.rs:282:24
   |
   |
LL |     suggestion: (Span, Span, Applicability),
   |
note: previously specified here
  --> $DIR/diagnostic-derive.rs:282:18
   |
   |
LL |     suggestion: (Span, Span, Applicability),

error: specified multiple times
  --> $DIR/diagnostic-derive.rs:290:33
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),
   |
note: previously specified here
  --> $DIR/diagnostic-derive.rs:290:18
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),


error: `#[label = ...]` is not a valid attribute
   |
   |
LL |     #[label = "bar"]

error: specified multiple times
  --> $DIR/diagnostic-derive.rs:448:5
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "maybe-incorrect")]
   |
note: previously specified here
  --> $DIR/diagnostic-derive.rs:450:24
   |
   |
LL |     suggestion: (Span, Applicability),

error: invalid applicability
  --> $DIR/diagnostic-derive.rs:456:69
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "batman")]


error: the `#[help(...)]` attribute can only be applied to fields of type `Span`, `MultiSpan`, `bool` or `()`
   |
   |
LL |     #[help(no_crate_help)]


error: a diagnostic slug must be the first argument to the attribute
   |
   |
LL |     #[label(no_crate_label, foo)]

error: invalid nested attribute
  --> $DIR/diagnostic-derive.rs:540:29
   |
   |
LL |     #[label(no_crate_label, foo = "...")]

error: invalid nested attribute
  --> $DIR/diagnostic-derive.rs:548:29
   |
   |
LL |     #[label(no_crate_label, foo("..."))]


error: `#[primary_span]` is not a valid attribute
   |
   |
LL |     #[primary_span]
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: `#[error(...)]` is not a valid attribute
   |
   |
LL | #[error(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[error(no_crate_example, code = "E0123")]
LL | |
LL | |
LL | |
LL | | struct ErrorAttribute {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[warn_(...)]` is not a valid attribute
   |
   |
LL | #[warn_(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[warn_(no_crate_example, code = "E0123")]
LL | |
LL | |
LL | |
LL | | struct WarnAttribute {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
   |
LL | #[lint(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[lint(no_crate_example, code = "E0123")]
LL | |
LL | |
LL | |
LL | | struct LintAttributeOnSessionDiag {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
   |
LL | #[lint(no_crate_example, code = "E0123")]


error: `#[lint(...)]` is not a valid attribute
   |
   |
LL | #[lint(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[lint(no_crate_example, code = "E0123")]
LL | |
LL | |
LL | |
LL | |
LL | | struct LintAttributeOnLintDiag {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(compiletest_example)]`
error: specified multiple times
  --> $DIR/diagnostic-derive.rs:611:53
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]
   |
note: previously specified here
  --> $DIR/diagnostic-derive.rs:611:39
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]

error: wrong types for suggestion
  --> $DIR/diagnostic-derive.rs:620:24
   |
   |
LL |     suggestion: (Span, usize),
   |                        ^^^^^
   |
   = help: `#[suggestion(...)]` on a tuple field must be applied to fields of type `(Span, Applicability)`
error: wrong types for suggestion
  --> $DIR/diagnostic-derive.rs:628:17
   |
   |
LL |     suggestion: (Span,),
   |
   |
   = help: `#[suggestion(...)]` on a tuple field must be applied to fields of type `(Span, Applicability)`

error: suggestion without `code = "..."`
   |
   |
LL |     #[suggestion(no_crate_suggestion)]


error: `#[multipart_suggestion(...)]` is not a valid attribute
   |
LL | #[multipart_suggestion(no_crate_suggestion)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = help: consider creating a `Subdiagnostic` instead

error: `#[multipart_suggestion(...)]` is not a valid attribute
   |
   |
LL |     #[multipart_suggestion(no_crate_suggestion)]
   |
   |
   = help: consider creating a `Subdiagnostic` instead

error: unexpected end of input, unexpected token in nested attribute, expected ident
   |
LL | #[multipart_suggestion()]
   |                        ^
---

error: cannot find attribute `multipart_suggestion` in this scope
  --> $DIR/diagnostic-derive.rs:649:7
   |
LL |     #[multipart_suggestion(no_crate_suggestion)]

error[E0425]: cannot find value `nonsense` in module `crate::fluent_generated`
  --> $DIR/diagnostic-derive.rs:72:8
   |
   |
LL | #[diag(nonsense, code = "E0123")]
   |        ^^^^^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `__code_34` in this scope
   |
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ not found in this scope
   |
   |
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Hello: IntoDiagnosticArg` is not satisfied
   |
LL | #[derive(Diagnostic)]
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ the trait `IntoDiagnosticArg` is not implemented for `Hello`
   = help: normalized in stderr
   = help: normalized in stderr
note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
  --> $COMPILER_DIR/rustc_errors/src/diagnostic_builder.rs:LL:CC
   = note: this error originates in the derive macro `Diagnostic` which comes from the expansion of the macro `forward` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 84 previous errors

Some errors have detailed explanations: E0277, E0425.
For more information about an error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args session-diagnostic/diagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/auxiliary"
stdout: none
error: unsupported type attribute for diagnostic derive enum
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:44:1
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:47:5
LL |     Foo,
   |     ^^^
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:49:5
LL |     Bar,
   |     ^^^
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: expected parentheses: #[diag(...)]
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:55:8
   |
LL | #[diag = "E0123"]


error: `#[nonsense(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:60:1
   |
LL | #[nonsense(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:60:1
   |
LL | / #[nonsense(no_crate_example, code = "E0123")]
LL | | //~^ ERROR `#[nonsense(...)]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | //~^^^ ERROR cannot find attribute `nonsense` in this scope
LL | | struct InvalidStructAttr {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:67:1
   |
LL | / #[diag("E0123")]
LL | | //~^ ERROR diagnostic slug not specified
LL | | struct InvalidLitNestedAttr {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug must be the first argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:77:16
   |
LL | #[diag(nonsense("foo"), code = "E0123", slug = "foo")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:77:1
   |
LL | / #[diag(nonsense("foo"), code = "E0123", slug = "foo")]
LL | | //~^ ERROR diagnostic slug must be the first argument
LL | | //~| ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr1 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
error: unknown argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:83:8
   |
   |
LL | #[diag(nonsense = "...", code = "E0123", slug = "foo")]
   |
   |
   = note: only the `code` parameter is valid after the slug

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:83:1
   |
LL | / #[diag(nonsense = "...", code = "E0123", slug = "foo")]
LL | | //~^ ERROR unknown argument
LL | | //~| ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr2 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
error: unknown argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:89:8
   |
   |
LL | #[diag(nonsense = 4, code = "E0123", slug = "foo")]
   |
   |
   = note: only the `code` parameter is valid after the slug

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:89:1
   |
LL | / #[diag(nonsense = 4, code = "E0123", slug = "foo")]
LL | | //~^ ERROR unknown argument
LL | | //~| ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr3 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
error: unknown argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:95:42
   |
   |
LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
   |
   |
   = note: only the `code` parameter is valid after the slug

error: `#[suggestion = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:102:5
   |
LL |     #[suggestion = "bar"]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:109:8
   |
   |
LL | #[diag(no_crate_example, code = "E0456")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:108:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:109:26
   |
   |
LL | #[diag(no_crate_example, code = "E0456")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:108:26
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:115:42
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:115:26
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]


error: diagnostic slug must be the first argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:120:43
   |
LL | #[diag(no_crate_example, no_crate::example, code = "E0456")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:125:1
   |
LL | struct KindNotProvided {} //~ ERROR diagnostic slug not specified
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:128:1
   |
LL | / #[diag(code = "E0456")]
LL | | //~^ ERROR diagnostic slug not specified
LL | | struct SlugNotProvided {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:139:5
   |
LL |     #[primary_span]


error: `#[nonsense]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:147:5
   |
LL |     #[nonsense]


error: the `#[label(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:164:5
   |
LL |     #[label(no_crate_label)]

error: `name` doesn't refer to a field on this type
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:172:46
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "{name}")]


error: invalid format string: expected `'}'` but string was terminated
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:177:10
LL | #[derive(Diagnostic)]
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ expected `'}'` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: invalid format string: unmatched `}` found
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:187:10
LL | #[derive(Diagnostic)]
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ unmatched `}` in format string
   |
   = note: if you intended to print `}`, you can escape it using `}}`
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: the `#[label(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:207:5
   |
LL |     #[label(no_crate_label)]


error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:226:5
   |
LL |     #[suggestion(no_crate_suggestion)]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:234:18
   |
   |
LL |     #[suggestion(nonsense = "bar")]
   |
   |
   = help: only `style`, `code` and `applicability` are valid nested attributes

error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:234:5
   |
LL |     #[suggestion(nonsense = "bar")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:243:18
   |
   |
LL |     #[suggestion(msg = "bar")]
   |
   |
   = help: only `style`, `code` and `applicability` are valid nested attributes

error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:243:5
   |
LL |     #[suggestion(msg = "bar")]

error: wrong field type for suggestion
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:266:5
   |
   |
LL | /     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
LL | |     //~^ ERROR wrong field type for suggestion
LL | |     suggestion: Applicability,
   |
   |
   = help: `#[suggestion(...)]` should be applied to fields of type `Span` or `(Span, Applicability)`
error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:282:24
   |
   |
LL |     suggestion: (Span, Span, Applicability),
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:282:18
   |
   |
LL |     suggestion: (Span, Span, Applicability),

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:290:33
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:290:18
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),


error: `#[label = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:297:5
   |
LL |     #[label = "bar"]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:448:5
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "maybe-incorrect")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:450:24
   |
   |
LL |     suggestion: (Span, Applicability),

error: invalid applicability
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:456:69
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "batman")]


error: the `#[help(...)]` attribute can only be applied to fields of type `Span`, `MultiSpan`, `bool` or `()`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:523:5
   |
LL |     #[help(no_crate_help)]


error: a diagnostic slug must be the first argument to the attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:532:32
   |
LL |     #[label(no_crate_label, foo)]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:540:29
   |
   |
LL |     #[label(no_crate_label, foo = "...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:548:29
   |
   |
LL |     #[label(no_crate_label, foo("..."))]


error: `#[primary_span]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:560:5
   |
LL |     #[primary_span]
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: `#[error(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:580:1
   |
LL | #[error(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:580:1
   |
LL | / #[error(no_crate_example, code = "E0123")]
LL | | //~^ ERROR `#[error(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `error` in this scope
LL | | struct ErrorAttribute {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[warn_(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:587:1
   |
LL | #[warn_(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:587:1
   |
LL | / #[warn_(no_crate_example, code = "E0123")]
LL | | //~^ ERROR `#[warn_(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `warn_` in this scope
LL | | struct WarnAttribute {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[lint(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:594:1
   |
LL | #[lint(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:594:1
   |
LL | / #[lint(no_crate_example, code = "E0123")]
LL | | //~^ ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `lint` in this scope
LL | | struct LintAttributeOnSessionDiag {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[lint(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:601:1
   |
LL | #[lint(no_crate_example, code = "E0123")]


error: `#[lint(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:601:1
   |
LL | #[lint(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:601:1
   |
LL | / #[lint(no_crate_example, code = "E0123")]
LL | | //~^ ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `lint` in this scope
LL | | struct LintAttributeOnLintDiag {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(compiletest_example)]`
error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:611:53
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:611:39
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]

error: wrong types for suggestion
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:620:24
   |
   |
LL |     suggestion: (Span, usize),
   |                        ^^^^^
   |
   = help: `#[suggestion(...)]` on a tuple field must be applied to fields of type `(Span, Applicability)`
error: wrong types for suggestion
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:628:17
   |
   |
LL |     suggestion: (Span,),
   |
   |
   = help: `#[suggestion(...)]` on a tuple field must be applied to fields of type `(Span, Applicability)`

error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:635:5
   |
LL |     #[suggestion(no_crate_suggestion)]


error: `#[multipart_suggestion(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:642:1
LL | #[multipart_suggestion(no_crate_suggestion)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = help: consider creating a `Subdiagnostic` instead

error: `#[multipart_suggestion(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:649:5
   |
LL |     #[multipart_suggestion(no_crate_suggestion)]
   |
   |
   = help: consider creating a `Subdiagnostic` instead

error: unexpected end of input, unexpected token in nested attribute, expected ident
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:645:24
LL | #[multipart_suggestion()]
   |                        ^


error: `#[suggestion(...)]` is not a valid attribute
---

error: cannot find attribute `multipart_suggestion` in this scope
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:649:7
   |
LL |     #[multipart_suggestion(no_crate_suggestion)]

error[E0425]: cannot find value `nonsense` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:72:8
   |
   |
LL | #[diag(nonsense, code = "E0123")]
   |        ^^^^^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `__code_34` in this scope
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:804:10
   |
LL | #[derive(Diagnostic)] //~ ERROR cannot find value `__code_34` in this scope
   |
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the trait bound `Hello: IntoDiagnosticArg` is not satisfied
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:341:10
LL | #[derive(Diagnostic)]
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ the trait `IntoDiagnosticArg` is not implemented for `Hello`
   |
   = help: the following other types implement trait `IntoDiagnosticArg`:
             &'a T
             &'a std::path::Path
             &'a str
             &rustc_target::spec::TargetTriple
             CString
             CString
             CguReuse
             Cow<'a, str>
           and 51 others
note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
  --> /rustc/FAKE_PREFIX/compiler/rustc_errors/src/diagnostic_builder.rs:747:5
   = note: this error originates in the derive macro `Diagnostic` which comes from the expansion of the macro `forward` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 84 previous errors

Some errors have detailed explanations: E0277, E0425.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs stdout ----
normalized stderr:
error: label without `#[primary_span]` field
   |
   |
LL | / #[label(no_crate_example)]
LL | |
LL | | struct C {
LL | |     var: String,
LL | | }


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
   |
LL | #[label]
   | ^^^^^^^^


error: `#[foo]` is not a valid attribute
   |
LL | #[foo]
   | ^^^^^^


error: `#[label = ...]` is not a valid attribute
   |
   |
LL | #[label = "..."]

error: invalid nested attribute
  --> $DIR/subdiagnostic-derive.rs:87:9
   |
   |
LL | #[label(bug = "...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
   |
   |
LL | #[label(bug = "...")]


error: unexpected literal in nested attribute, expected ident
   |
   |
LL | #[label("...")]

error: invalid nested attribute
  --> $DIR/subdiagnostic-derive.rs:106:9
   |
   |
LL | #[label(slug = 4)]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
   |
   |
LL | #[label(slug = 4)]

error: invalid nested attribute
  --> $DIR/subdiagnostic-derive.rs:116:9
   |
   |
LL | #[label(slug("..."))]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
   |
   |
LL | #[label(slug("..."))]


error: unexpected end of input, unexpected token in nested attribute, expected ident
   |
   |
LL | #[label()]

error: invalid nested attribute
  --> $DIR/subdiagnostic-derive.rs:145:27
   |
   |
LL | #[label(no_crate_example, code = "...")]

error: invalid nested attribute
  --> $DIR/subdiagnostic-derive.rs:154:27
   |
   |
LL | #[label(no_crate_example, applicability = "machine-applicable")]

error: unsupported type attribute for subdiagnostic enum
  --> $DIR/subdiagnostic-derive.rs:163:1
   |
   |
LL | #[foo]
   | ^^^^^^

error: `#[bar]` is not a valid attribute
   |
LL |     #[bar]
   |     ^^^^^^


error: `#[bar = ...]` is not a valid attribute
   |
   |
LL |     #[bar = "..."]


error: `#[bar = ...]` is not a valid attribute
   |
   |
LL |     #[bar = 4]


error: `#[bar(...)]` is not a valid attribute
   |
   |
LL |     #[bar("...")]

error: invalid nested attribute
  --> $DIR/subdiagnostic-derive.rs:225:13
   |
   |
LL |     #[label(code = "...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
   |
   |
LL |     #[label(code = "...")]


error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
   |
   |
LL |     #[primary_span]


error: label without `#[primary_span]` field
   |
   |
LL | / #[label(no_crate_example)]
LL | |
LL | | struct W {
LL | |     #[primary_span]
LL | |
LL | |     span: String,
LL | | }


error: `#[applicability]` is only valid on suggestions
   |
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: `#[bar]` is not a valid attribute
   |
LL |     #[bar]
   |     ^^^^^^
   |
   |
   = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes

error: `#[bar = ...]` is not a valid attribute
   |
   |
LL |     #[bar = "..."]


error: `#[bar(...)]` is not a valid attribute
   |
   |
LL |     #[bar("...")]
   |
   |
   = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes
error: unexpected unsupported untagged union
  --> $DIR/subdiagnostic-derive.rs:312:1
   |
   |
LL | / union AC {
LL | |
LL | |     span: u32,
LL | |     b: u64,
LL | | }


error: a diagnostic slug must be the first argument to the attribute
   |
   |
LL | #[label(no_crate_example, no_crate::example)]

error: specified multiple times
  --> $DIR/subdiagnostic-derive.rs:340:5
   |
   |
LL |     #[primary_span]
   |
note: previously specified here
  --> $DIR/subdiagnostic-derive.rs:337:5
   |
   |
LL |     #[primary_span]


error: subdiagnostic kind not specified
   |
LL | struct AG {
   |        ^^


error: specified multiple times
  --> $DIR/subdiagnostic-derive.rs:383:46
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]
   |
note: previously specified here
  --> $DIR/subdiagnostic-derive.rs:383:32
   |
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]

error: specified multiple times
  --> $DIR/subdiagnostic-derive.rs:401:5
   |
---
   |
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^

error: the `#[applicability]` attribute can only be applied to fields of type `Applicability`
   |
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: suggestion without `code = "..."`
   |
   |
LL | #[suggestion(no_crate_example)]

error: invalid applicability
  --> $DIR/subdiagnostic-derive.rs:434:62
   |
   |
LL | #[suggestion(no_crate_example, code = "...", applicability = "foo")]


error: suggestion without `#[primary_span]` field
   |
   |
LL | / #[suggestion(no_crate_example, code = "...")]
LL | |
LL | | struct AR {
LL | |     var: String,
LL | | }

error: unsupported type attribute for subdiagnostic enum
  --> $DIR/subdiagnostic-derive.rs:466:1
   |
   |
LL | #[label]
   | ^^^^^^^^

error: `var` doesn't refer to a field on this type
   |
   |
LL | #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]


error: `var` doesn't refer to a field on this type
   |
   |
LL |     #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]


error: `#[suggestion_part]` is not a valid attribute
   |
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^
   |
   |
   = help: `#[suggestion_part(...)]` is only valid in multipart suggestions, use `#[primary_span]` instead

error: `#[suggestion_part(...)]` is not a valid attribute
   |
   |
LL |     #[suggestion_part(code = "...")]
   |
   |
   = help: `#[suggestion_part(...)]` is only valid in multipart suggestions

error: suggestion without `#[primary_span]` field
   |
   |
LL | / #[suggestion(no_crate_example, code = "...")]
LL | |
LL | | struct BA {
LL | |     #[suggestion_part]
LL | |     var: String,
LL | | }
   | |_^


error: invalid nested attribute
  --> $DIR/subdiagnostic-derive.rs:540:42
   |
LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
   |
   |
   = help: only `style` and `applicability` are valid nested attributes

error: multipart suggestion without any `#[suggestion_part(...)]` fields
   |
   |
LL | / #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
LL | |
LL | |
LL | | struct BBa {
LL | |     var: String,
LL | | }


error: `#[suggestion_part(...)]` attribute without `code = "..."`
   |
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: unexpected end of input, unexpected token in nested attribute, expected ident
   |
   |
LL |     #[suggestion_part()]


error: `#[primary_span]` is not a valid attribute
   |
   |
LL |     #[primary_span]
   |
   |
   = help: multipart suggestions use one or more `#[suggestion_part]`s rather than one `#[primary_span]`

error: multipart suggestion without any `#[suggestion_part(...)]` fields
   |
   |
LL | / #[multipart_suggestion(no_crate_example)]
LL | |
LL | | struct BC {
LL | |     #[primary_span]
LL | |
LL | |     span: Span,
LL | | }


error: `#[suggestion_part(...)]` attribute without `code = "..."`
   |
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: `code` is the only valid nested attribute
   |
   |
LL |     #[suggestion_part(foo = "bar")]


error: the `#[suggestion_part(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
   |
   |
LL |     #[suggestion_part(code = "...")]


error: the `#[suggestion_part(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
   |
   |
LL |     #[suggestion_part()]


error: unexpected end of input, unexpected token in nested attribute, expected ident
   |
   |
LL |     #[suggestion_part()]

error: expected `,`
  --> $DIR/subdiagnostic-derive.rs:581:27
   |
   |
LL |     #[suggestion_part(foo = "bar")]

error: specified multiple times
  --> $DIR/subdiagnostic-derive.rs:596:37
   |
   |
LL |     #[suggestion_part(code = "...", code = ",,,")]
   |
note: previously specified here
  --> $DIR/subdiagnostic-derive.rs:596:23
   |
   |
LL |     #[suggestion_part(code = "...", code = ",,,")]


error: `#[applicability]` has no effect if all `#[suggestion]`/`#[multipart_suggestion]` attributes have a static `applicability = "..."`
   |
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: expected exactly one string literal for `code = ...`
   |
   |
LL |     #[suggestion_part(code("foo"))]

error: unexpected token
  --> $DIR/subdiagnostic-derive.rs:673:28
   |
   |
LL |     #[suggestion_part(code("foo"))]


error: expected exactly one string literal for `code = ...`
   |
   |
LL |     #[suggestion_part(code("foo", "bar"))]

error: unexpected token
  --> $DIR/subdiagnostic-derive.rs:683:28
   |
   |
LL |     #[suggestion_part(code("foo", "bar"))]


error: expected exactly one string literal for `code = ...`
   |
   |
LL |     #[suggestion_part(code(3))]

error: unexpected token
  --> $DIR/subdiagnostic-derive.rs:693:28
   |
   |
LL |     #[suggestion_part(code(3))]


error: expected exactly one string literal for `code = ...`
   |
   |
LL |     #[suggestion_part(code())]

error: expected string literal
  --> $DIR/subdiagnostic-derive.rs:715:30
   |
   |
LL |     #[suggestion_part(code = 3)]

error: specified multiple times
  --> $DIR/subdiagnostic-derive.rs:757:1
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]
   |
note: previously specified here
  --> $DIR/subdiagnostic-derive.rs:757:1
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]


error: `#[suggestion_hidden(...)]` is not a valid attribute
   |
   |
LL | #[suggestion_hidden(no_crate_example, code = "")]
   |
   |
   = help: Use `#[suggestion(..., style = "hidden")]` instead

error: `#[suggestion_hidden(...)]` is not a valid attribute
   |
   |
LL | #[suggestion_hidden(no_crate_example, code = "", style = "normal")]
   |
   |
   = help: Use `#[suggestion(..., style = "hidden")]` instead
error: invalid suggestion style
  --> $DIR/subdiagnostic-derive.rs:782:51
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "foo")]
   |
   |
   = help: valid styles are `normal`, `short`, `hidden`, `verbose` and `tool-only`

error: expected `= "xxx"`
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = 42)]


error: a diagnostic slug must be the first argument to the attribute
   |
   |
LL | #[suggestion(no_crate_example, code = "", style)]


error: expected `= "xxx"`
   |
   |
LL | #[suggestion(no_crate_example, code = "", style("foo"))]

error: expected `,`
  --> $DIR/subdiagnostic-derive.rs:806:48
   |
   |
LL | #[suggestion(no_crate_example, code = "", style("foo"))]


error: `#[primary_span]` is not a valid attribute
   |
   |
LL |     #[primary_span]
   |
   = note: there must be exactly one primary span
   = note: there must be exactly one primary span
   = help: to create a suggestion with multiple spans, use `#[multipart_suggestion]` instead

error: suggestion without `#[primary_span]` field
   |
   |
LL | / #[suggestion(no_crate_example, code = "")]
LL | |
LL | | struct PrimarySpanOnVec {
LL | |     #[primary_span]
...  |
LL | |     sub: Vec<Span>,
LL | | }

error: cannot find attribute `foo` in this scope
  --> $DIR/subdiagnostic-derive.rs:68:3
   |
---

error: cannot find attribute `bar` in this scope
  --> $DIR/subdiagnostic-derive.rs:189:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> $DIR/subdiagnostic-derive.rs:201:7
   |
   |
LL |     #[bar = 4]

error: cannot find attribute `bar` in this scope
  --> $DIR/subdiagnostic-derive.rs:213:7
   |
   |
LL |     #[bar("...")]

error: cannot find attribute `bar` in this scope
  --> $DIR/subdiagnostic-derive.rs:274:7
   |
   |
LL |     #[bar]
   |       ^^^

error: cannot find attribute `bar` in this scope
  --> $DIR/subdiagnostic-derive.rs:285:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> $DIR/subdiagnostic-derive.rs:296:7
   |
   |
LL |     #[bar("...")]


error[E0425]: cannot find value `slug` in module `crate::fluent_generated`
   |
   |
LL | #[label(slug)]
   |         ^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `__code_29` in this scope
   |
LL | #[derive(Subdiagnostic)]
   |          ^^^^^^^^^^^^^ not found in this scope
   |
---
To only update this specific test, also pass `--test-args session-diagnostic/subdiagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: label without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:52:1
   |
LL | / #[label(no_crate_example)]
LL | | //~^ ERROR label without `#[primary_span]` field
LL | | struct C {
LL | |     var: String,
LL | | }


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:59:1
LL | #[label]
   | ^^^^^^^^


error: `#[foo]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:68:1
LL | #[foo]
   | ^^^^^^


error: `#[label = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:78:1
   |
LL | #[label = "..."]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:87:9
   |
   |
LL | #[label(bug = "...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:87:1
   |
LL | #[label(bug = "...")]


error: unexpected literal in nested attribute, expected ident
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:97:9
   |
LL | #[label("...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:106:9
   |
   |
LL | #[label(slug = 4)]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:106:1
   |
LL | #[label(slug = 4)]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:116:9
   |
   |
LL | #[label(slug("..."))]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:116:1
   |
LL | #[label(slug("..."))]


error: unexpected end of input, unexpected token in nested attribute, expected ident
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:136:9
   |
LL | #[label()]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:145:27
   |
   |
LL | #[label(no_crate_example, code = "...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:154:27
   |
   |
LL | #[label(no_crate_example, applicability = "machine-applicable")]

error: unsupported type attribute for subdiagnostic enum
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:163:1
   |
   |
LL | #[foo]
   | ^^^^^^

error: `#[bar]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:177:5
LL |     #[bar]
   |     ^^^^^^


error: `#[bar = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:189:5
   |
LL |     #[bar = "..."]


error: `#[bar = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:201:5
   |
LL |     #[bar = 4]


error: `#[bar(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:213:5
   |
LL |     #[bar("...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:225:13
   |
   |
LL |     #[label(code = "...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:225:5
   |
LL |     #[label(code = "...")]


error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:254:5
   |
LL |     #[primary_span]


error: label without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:251:1
   |
LL | / #[label(no_crate_example)]
LL | | //~^ ERROR label without `#[primary_span]` field
LL | | struct W {
LL | |     #[primary_span]
LL | |     //~^ ERROR the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
LL | |     span: String,
LL | | }


error: `#[applicability]` is only valid on suggestions
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:264:5
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: `#[bar]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:274:5
LL |     #[bar]
   |     ^^^^^^
   |
   |
   = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes

error: `#[bar = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:285:5
   |
LL |     #[bar = "..."]


error: `#[bar(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:296:5
   |
LL |     #[bar("...")]
   |
   |
   = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes
error: unexpected unsupported untagged union
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:312:1
   |
   |
LL | / union AC {
LL | |     //~^ ERROR unexpected unsupported untagged union
LL | |     span: u32,
LL | |     b: u64,
LL | | }


error: a diagnostic slug must be the first argument to the attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:327:44
   |
LL | #[label(no_crate_example, no_crate::example)]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:340:5
   |
   |
LL |     #[primary_span]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:337:5
   |
   |
LL |     #[primary_span]


error: subdiagnostic kind not specified
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:346:8
LL | struct AG {
   |        ^^

error: specified multiple times
error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:383:46
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:383:32
   |
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:401:5
   |
---
   |
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^

error: the `#[applicability]` attribute can only be applied to fields of type `Applicability`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:411:5
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:424:1
   |
LL | #[suggestion(no_crate_example)]

error: invalid applicability
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:434:62
   |
   |
LL | #[suggestion(no_crate_example, code = "...", applicability = "foo")]


error: suggestion without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:452:1
   |
LL | / #[suggestion(no_crate_example, code = "...")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct AR {
LL | |     var: String,
LL | | }

error: unsupported type attribute for subdiagnostic enum
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:466:1
   |
   |
LL | #[label]
   | ^^^^^^^^

error: `var` doesn't refer to a field on this type
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:486:39
   |
LL | #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]


error: `var` doesn't refer to a field on this type
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:505:43
   |
LL |     #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]


error: `#[suggestion_part]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:528:5
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^
   |
   |
   = help: `#[suggestion_part(...)]` is only valid in multipart suggestions, use `#[primary_span]` instead

error: `#[suggestion_part(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:531:5
   |
LL |     #[suggestion_part(code = "...")]
   |
   |
   = help: `#[suggestion_part(...)]` is only valid in multipart suggestions

error: suggestion without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:525:1
   |
LL | / #[suggestion(no_crate_example, code = "...")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct BA {
LL | |     #[suggestion_part]
LL | |     var: String,
LL | | }
   | |_^


error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:540:42
   |
LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
   |
   |
   = help: only `style` and `applicability` are valid nested attributes

error: multipart suggestion without any `#[suggestion_part(...)]` fields
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:540:1
   |
LL | / #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
LL | | //~^ ERROR multipart suggestion without any `#[suggestion_part(...)]` fields
LL | | //~| ERROR invalid nested attribute
LL | | struct BBa {
LL | |     var: String,
LL | | }


error: `#[suggestion_part(...)]` attribute without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:550:5
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: unexpected end of input, unexpected token in nested attribute, expected ident
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:558:23
   |
LL |     #[suggestion_part()]


error: `#[primary_span]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:567:5
   |
LL |     #[primary_span]
   |
   |
   = help: multipart suggestions use one or more `#[suggestion_part]`s rather than one `#[primary_span]`

error: multipart suggestion without any `#[suggestion_part(...)]` fields
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:564:1
   |
LL | / #[multipart_suggestion(no_crate_example)]
LL | | //~^ ERROR multipart suggestion without any `#[suggestion_part(...)]` fields
LL | | struct BC {
LL | |     #[primary_span]
LL | |     //~^ ERROR `#[primary_span]` is not a valid attribute
LL | |     span: Span,
LL | | }


error: `#[suggestion_part(...)]` attribute without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:575:5
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: `code` is the only valid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:581:23
   |
LL |     #[suggestion_part(foo = "bar")]


error: the `#[suggestion_part(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:585:5
   |
LL |     #[suggestion_part(code = "...")]


error: the `#[suggestion_part(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:588:5
   |
LL |     #[suggestion_part()]


error: unexpected end of input, unexpected token in nested attribute, expected ident
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:578:23
   |
LL |     #[suggestion_part()]

error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:581:27
   |
   |
LL |     #[suggestion_part(foo = "bar")]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:596:37
   |
   |
LL |     #[suggestion_part(code = "...", code = ",,,")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:596:23
   |
   |
LL |     #[suggestion_part(code = "...", code = ",,,")]


error: `#[applicability]` has no effect if all `#[suggestion]`/`#[multipart_suggestion]` attributes have a static `applicability = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:625:5
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:673:34
   |
LL |     #[suggestion_part(code("foo"))]

error: unexpected token
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:673:28
   |
   |
LL |     #[suggestion_part(code("foo"))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:683:41
   |
LL |     #[suggestion_part(code("foo", "bar"))]

error: unexpected token
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:683:28
   |
   |
LL |     #[suggestion_part(code("foo", "bar"))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:693:30
   |
LL |     #[suggestion_part(code(3))]

error: unexpected token
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:693:28
   |
   |
LL |     #[suggestion_part(code(3))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:703:29
   |
LL |     #[suggestion_part(code())]

error: expected string literal
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:715:30
   |
   |
LL |     #[suggestion_part(code = 3)]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:757:1
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:757:1
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]


error: `#[suggestion_hidden(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:766:1
   |
LL | #[suggestion_hidden(no_crate_example, code = "")]
   |
   |
   = help: Use `#[suggestion(..., style = "hidden")]` instead

error: `#[suggestion_hidden(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:774:1
   |
LL | #[suggestion_hidden(no_crate_example, code = "", style = "normal")]
   |
   |
   = help: Use `#[suggestion(..., style = "hidden")]` instead
error: invalid suggestion style
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:782:51
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "foo")]
   |
   |
   = help: valid styles are `normal`, `short`, `hidden`, `verbose` and `tool-only`

error: expected `= "xxx"`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:790:49
   |
LL | #[suggestion(no_crate_example, code = "", style = 42)]


error: a diagnostic slug must be the first argument to the attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:798:48
   |
LL | #[suggestion(no_crate_example, code = "", style)]


error: expected `= "xxx"`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:806:48
   |
LL | #[suggestion(no_crate_example, code = "", style("foo"))]

error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:806:48
   |
   |
LL | #[suggestion(no_crate_example, code = "", style("foo"))]


error: `#[primary_span]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:818:5
   |
LL |     #[primary_span]
   |
   = note: there must be exactly one primary span
   = note: there must be exactly one primary span
   = help: to create a suggestion with multiple spans, use `#[multipart_suggestion]` instead

error: suggestion without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:815:1
   |
LL | / #[suggestion(no_crate_example, code = "")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct PrimarySpanOnVec {
LL | |     #[primary_span]
...  |
LL | |     sub: Vec<Span>,
LL | | }

error: cannot find attribute `foo` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:68:3
   |
---

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:189:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:201:7
   |
   |
LL |     #[bar = 4]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:213:7
   |
   |
LL |     #[bar("...")]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:274:7
   |
   |
LL |     #[bar]
   |       ^^^

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:285:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:296:7
   |
   |
LL |     #[bar("...")]


error[E0425]: cannot find value `slug` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:126:9
   |
LL | #[label(slug)]
   |         ^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `__code_29` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:709:10
LL | #[derive(Subdiagnostic)]
   |          ^^^^^^^^^^^^^ not found in this scope
   |
   = note: this error originates in the derive macro `Subdiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
