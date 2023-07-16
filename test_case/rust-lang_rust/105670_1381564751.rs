plain
diff of stderr:

590    |                  ^^^^^^^^
591 
592 error: `#[suggestion(...)]` is not a valid attribute
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/diagnostic-derive.rs:808:5
594    |
594    |
595 LL |     #[suggestion(suggestion, code = "")]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/diagnostic-derive.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/diagnostic-derive.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args session-diagnostic/diagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: unsupported type attribute for diagnostic derive enum
   |
   |
LL | #[diag(compiletest_example, code = "E0123")]


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

error: `#[diag = ...]` is not a valid attribute
   |
   |
LL | #[diag = "E0123"]


error: `#[nonsense(...)]` is not a valid attribute
   |
   |
LL | #[nonsense(compiletest_example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[nonsense(compiletest_example, code = "E0123")]
LL | | //~^ ERROR `#[nonsense(...)]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | //~^^^ ERROR cannot find attribute `nonsense` in this scope
LL | | struct InvalidStructAttr {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[diag("...")]` is not a valid attribute
   |
   |
LL | #[diag("E0123")]
   |
   = help: a diagnostic slug is required as the first argument


error: diagnostic slug not specified
   |
   |
LL | / #[diag("E0123")]
LL | | //~^ ERROR `#[diag("...")]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidLitNestedAttr {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[diag(nonsense(...))]` is not a valid attribute
   |
   |
LL | #[diag(nonsense("foo"), code = "E0123", slug = "foo")]
   |
   = help: a diagnostic slug is required as the first argument


error: diagnostic slug not specified
   |
   |
LL | / #[diag(nonsense("foo"), code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[diag(nonsense(...))]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr1 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[diag(nonsense = ...)]` is not a valid attribute
   |
   |
LL | #[diag(nonsense = "...", code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

error: `#[diag(slug = ...)]` is not a valid attribute
   |
   |
LL | #[diag(nonsense = "...", code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

error: diagnostic slug not specified
   |
   |
LL | / #[diag(nonsense = "...", code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[diag(nonsense = ...)]` is not a valid attribute
LL | | //~| ERROR `#[diag(slug = ...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr2 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[diag(nonsense = ...)]` is not a valid attribute
   |
   |
LL | #[diag(nonsense = 4, code = "E0123", slug = "foo")]


error: `#[diag(slug = ...)]` is not a valid attribute
   |
   |
LL | #[diag(nonsense = 4, code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

error: diagnostic slug not specified
   |
   |
LL | / #[diag(nonsense = 4, code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[diag(nonsense = ...)]` is not a valid attribute
LL | | //~| ERROR `#[diag(slug = ...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr3 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[diag(slug = ...)]` is not a valid attribute
   |
   |
LL | #[diag(compiletest_example, code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

error: `#[suggestion = ...]` is not a valid attribute
   |
   |
LL |     #[suggestion = "bar"]

error: specified multiple times
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:107:8
   |
   |
LL | #[diag(compiletest_example, code = "E0456")]
   |
note: previously specified here
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:106:8
   |
   |
LL | #[diag(compiletest_example, code = "E0123")]

error: specified multiple times
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:107:36
   |
   |
LL | #[diag(compiletest_example, code = "E0456")]
   |
note: previously specified here
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:106:36
   |
   |
LL | #[diag(compiletest_example, code = "E0123")]

error: specified multiple times
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:113:52
   |
   |
LL | #[diag(compiletest_example, code = "E0456", code = "E0457")]
   |
note: previously specified here
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:113:36
   |
   |
LL | #[diag(compiletest_example, code = "E0456", code = "E0457")]


error: `#[diag(compiletest_example)]` is not a valid attribute
   |
   |
LL | #[diag(compiletest_example, compiletest_example, code = "E0456")]
   |
   |
   = help: diagnostic slug must be the first argument

error: diagnostic slug not specified
   |
   |
LL | struct KindNotProvided {} //~ ERROR diagnostic slug not specified
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
   |
   |
LL | / #[diag(code = "E0456")]
LL | | //~^ ERROR diagnostic slug not specified
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
LL |     #[label(label)]

error: `name` doesn't refer to a field on this type
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:170:37
   |
   |
LL |     #[suggestion(suggestion, code = "{name}")]


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
LL |     #[label(label)]


error: suggestion without `code = "..."`
   |
   |
LL |     #[suggestion(suggestion)]


error: `#[suggestion(nonsense = ...)]` is not a valid attribute
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


error: `#[suggestion(msg = ...)]` is not a valid attribute
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
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:264:5
   |
   |
LL | /     #[suggestion(suggestion, code = "This is suggested code")]
LL | |     //~^ ERROR wrong field type for suggestion
LL | |     suggestion: Applicability,
   |
   |
   = help: `#[suggestion(...)]` should be applied to fields of type `Span` or `(Span, Applicability)`
error: specified multiple times
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:280:24
   |
   |
LL |     suggestion: (Span, Span, Applicability),
   |
note: previously specified here
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:280:18
   |
   |
LL |     suggestion: (Span, Span, Applicability),

error: specified multiple times
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:288:33
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),
   |
note: previously specified here
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:288:18
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),


error: `#[label = ...]` is not a valid attribute
   |
   |
LL |     #[label = "bar"]

error: specified multiple times
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:446:44
   |
   |
LL |     #[suggestion(suggestion, code = "...", applicability = "maybe-incorrect")]
   |
note: previously specified here
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:448:24
   |
   |
LL |     suggestion: (Span, Applicability),

error: invalid applicability
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:454:44
   |
   |
LL |     #[suggestion(suggestion, code = "...", applicability = "batman")]


error: `#[label(foo)]` is not a valid attribute
   |
   |
LL |     #[label(label, foo)]
   |
   |
   = help: a diagnostic slug must be the first argument to the attribute

error: `#[label(foo = ...)]` is not a valid attribute
   |
   |
LL |     #[label(label, foo = "...")]


error: `#[label(foo(...))]` is not a valid attribute
   |
   |
LL |     #[label(label, foo("..."))]


error: `#[primary_span]` is not a valid attribute
   |
   |
LL |     #[primary_span]
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: `#[error(...)]` is not a valid attribute
   |
LL | #[error(compiletest_example, code = "E0123")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: diagnostic slug not specified
   |
   |
LL | / #[error(compiletest_example, code = "E0123")]
LL | | //~^ ERROR `#[error(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `error` in this scope
LL | | struct ErrorAttribute {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[warn_(...)]` is not a valid attribute
   |
   |
LL | #[warn_(compiletest_example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[warn_(compiletest_example, code = "E0123")]
LL | | //~^ ERROR `#[warn_(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `warn_` in this scope
LL | | struct WarnAttribute {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
   |
LL | #[lint(compiletest_example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[lint(compiletest_example, code = "E0123")]
LL | | //~^ ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `lint` in this scope
LL | | struct LintAttributeOnSessionDiag {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
   |
LL | #[lint(compiletest_example, code = "E0123")]


error: `#[lint(...)]` is not a valid attribute
   |
   |
LL | #[lint(compiletest_example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[lint(compiletest_example, code = "E0123")]
LL | | //~^ ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `lint` in this scope
LL | | struct LintAttributeOnLintDiag {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(compiletest_example)]`
error: specified multiple times
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:596:44
   |
   |
LL |     #[suggestion(suggestion, code = "...", code = ",,,")]
   |
note: previously specified here
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:596:30
   |
   |
LL |     #[suggestion(suggestion, code = "...", code = ",,,")]

error: wrong types for suggestion
  --> /checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:605:24
   |
   |
LL |     suggestion: (Span, usize),
