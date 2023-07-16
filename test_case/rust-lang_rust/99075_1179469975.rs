plain
---- [ui] src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs stdout ----
diff of stderr:

411    |
412    = help: normalized in stderr
413 note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
-   --> $COMPILER_DIR/rustc_errors/src/diagnostic_builder.rs:538:19
+   --> $COMPILER_DIR/rustc_errors/src/diagnostic_builder.rs:539:19
415    |
416 LL |         arg: impl IntoDiagnosticArg,
417    |                   ^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::set_arg`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/diagnostic-derive.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args session-diagnostic/diagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: `#[derive(SessionDiagnostic)]` can only be used on structs
   |
   |
LL | / #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]
LL | | //~^ ERROR `#[derive(SessionDiagnostic)]` can only be used on structs
LL | | enum SessionDiagnosticOnEnum {
LL | |     Foo,
LL | |     Bar,
LL | | }


error: `#[error = ...]` is not a valid attribute
   |
LL | #[error = "E0123"]
   | ^^^^^^^^^^^^^^^^^^


error: `#[nonsense(...)]` is not a valid attribute
   |
   |
LL | #[nonsense(typeck::ambiguous_lifetime_bound, code = "E0123")]
   |
   |
   = help: only `error`, `warning`, `help` and `note` are valid attributes

error: diagnostic kind not specified
   |
   |
LL | / #[nonsense(typeck::ambiguous_lifetime_bound, code = "E0123")]
LL | | //~^ ERROR `#[nonsense(...)]` is not a valid attribute
LL | | //~^^ ERROR diagnostic kind not specified
LL | | //~^^^ ERROR cannot find attribute `nonsense` in this scope
LL | | struct InvalidStructAttr {}
   |
   |
   = help: use the `#[error(...)]` attribute to create an error

error: `#[error("...")]` is not a valid attribute
   |
LL | #[error("E0123")]
   |         ^^^^^^^
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: diagnostic slug not specified
   |
   |
LL | / #[error("E0123")]
LL | | //~^ ERROR `#[error("...")]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidLitNestedAttr {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[error(typeck::example_error)]`

error: `#[error(nonsense(...))]` is not a valid attribute
   |
   |
LL | #[error(nonsense("foo"), code = "E0123", slug = "foo")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: diagnostic slug not specified
   |
   |
LL | / #[error(nonsense("foo"), code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[error(nonsense(...))]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr1 {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[error(typeck::example_error)]`

error: `#[error(nonsense = ...)]` is not a valid attribute
   |
   |
LL | #[error(nonsense = "...", code = "E0123", slug = "foo")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: diagnostic slug not specified
   |
   |
LL | / #[error(nonsense = "...", code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[error(nonsense = ...)]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr2 {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[error(typeck::example_error)]`

error: `#[error(nonsense = ...)]` is not a valid attribute
   |
   |
LL | #[error(nonsense = 4, code = "E0123", slug = "foo")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: diagnostic slug not specified
   |
   |
LL | / #[error(nonsense = 4, code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[error(nonsense = ...)]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr3 {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[error(typeck::example_error)]`

error: `#[error(slug = ...)]` is not a valid attribute
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

error: `#[suggestion = ...]` is not a valid attribute
   |
   |
LL |     #[suggestion = "bar"]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:103:1
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0456")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:102:1
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:103:1
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0456")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:102:1
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:103:50
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0456")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:102:50
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:111:1
   |
   |
LL | #[warning(typeck::ambiguous_lifetime_bound, code = "E0293")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:110:1
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:111:1
   |
   |
LL | #[warning(typeck::ambiguous_lifetime_bound, code = "E0293")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:110:1
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:111:52
   |
   |
LL | #[warning(typeck::ambiguous_lifetime_bound, code = "E0293")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:110:50
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:118:66
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0456", code = "E0457")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:118:50
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0456", code = "E0457")]


error: `#[error(typeck::ambiguous_lifetime_bound)]` is not a valid attribute
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, typeck::ambiguous_lifetime_bound, code = "E0456")]


error: diagnostic kind not specified
   |
   |
LL | struct KindNotProvided {} //~ ERROR diagnostic kind not specified
   |
   |
   = help: use the `#[error(...)]` attribute to create an error

error: diagnostic slug not specified
   |
   |
LL | / #[error(code = "E0456")]
LL | | //~^ ERROR diagnostic slug not specified
LL | | struct SlugNotProvided {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[error(typeck::example_error)]`

error: the `#[primary_span]` attribute can only be applied to fields of type `Span`
   |
   |
LL |     #[primary_span]


error: `#[nonsense]` is not a valid attribute
   |
   |
LL |     #[nonsense]
   |
   |
   = help: only `skip_arg`, `primary_span`, `label`, `note`, `help` and `subdiagnostic` are valid field attributes

error: the `#[label(...)]` attribute can only be applied to fields of type `Span`
   |
   |
LL |     #[label(typeck::label)]

error: `name` doesn't refer to a field on this type
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:175:45
   |
   |
LL |     #[suggestion(typeck::suggestion, code = "{name}")]


error: invalid format string: expected `'}'` but string was terminated
   |
LL | #[derive(SessionDiagnostic)]
LL | #[derive(SessionDiagnostic)]
   |           -    ^ expected `'}'` in format string
   |           because of this opening brace
   |
   |
   = note: if you intended to print `{`, you can escape it using `{{`
   = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: invalid format string: unmatched `}` found
   |
LL | #[derive(SessionDiagnostic)]
LL | #[derive(SessionDiagnostic)]
   |               ^ unmatched `}` in format string
   |
   = note: if you intended to print `}`, you can escape it using `}}`
   = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: the `#[label(...)]` attribute can only be applied to fields of type `Span`
   |
   |
LL |     #[label(typeck::label)]


error: `#[suggestion(nonsense = ...)]` is not a valid attribute
   |
   |
LL |     #[suggestion(nonsense = "bar")]
   |
   |
   = help: only `message`, `code` and `applicability` are valid field attributes

error: `#[suggestion(msg = ...)]` is not a valid attribute
   |
   |
LL |     #[suggestion(msg = "bar")]
   |
   |
   = help: only `message`, `code` and `applicability` are valid field attributes
error: wrong field type for suggestion
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:265:5
   |
   |
LL | /     #[suggestion(typeck::suggestion, code = "This is suggested code")]
LL | |     //~^ ERROR wrong field type for suggestion
LL | |     suggestion: Applicability,
   |
   |
   = help: `#[suggestion(...)]` should be applied to fields of type `Span` or `(Span, Applicability)`

error: type of field annotated with `#[suggestion(...)]` contains more than one `Span`
   |
   |
LL | /     #[suggestion(typeck::suggestion, code = "This is suggested code")]
LL | |     //~^ ERROR type of field annotated with `#[suggestion(...)]` contains more than one `Span`
LL | |     suggestion: (Span, Span, Applicability),


error: type of field annotated with `#[suggestion(...)]` contains more than one Applicability
   |
   |
LL | /     #[suggestion(typeck::suggestion, code = "This is suggested code")]
LL | |     //~^ ERROR type of field annotated with `#[suggestion(...)]` contains more than one
LL | |     suggestion: (Applicability, Applicability, Span),


error: `#[label = ...]` is not a valid attribute
   |
   |
LL |     #[label = "bar"]


error: applicability cannot be set in both the field and attribute
   |
   |
LL |     #[suggestion(typeck::suggestion, code = "...", applicability = "maybe-incorrect")]

error: invalid applicability
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:455:52
   |
   |
LL |     #[suggestion(typeck::suggestion, code = "...", applicability = "batman")]


error: `#[label(...)]` is not a valid attribute
   |
   |
LL |     #[label(typeck::label, foo)]


error: `#[label(...)]` is not a valid attribute
   |
   |
LL |     #[label(typeck::label, foo = "...")]


error: `#[label(...)]` is not a valid attribute
   |
   |
LL |     #[label(typeck::label, foo("..."))]


error: only `#[error(..)]` and `#[warn(..)]` are supported
   |
   |
LL | / #[lint(typeck::ambiguous_lifetime_bound)]
LL | | //~^ ERROR only `#[error(..)]` and `#[warn(..)]` are supported
LL | | struct LintsBad {
LL | | }
   |
   |
   = help: use the `#[error(...)]` attribute to create a error

error: only `#[lint(..)]` is supported
   |
   |
LL | / #[error(typeck::ambiguous_lifetime_bound)]
LL | | //~^ ERROR only `#[lint(..)]` is supported
LL | | struct ErrorsBad {
LL | | }
   |
   |
   = help: use the `#[lint(...)]` attribute to create a lint
error: cannot find attribute `nonsense` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:53:3
   |
   |
LL | #[nonsense(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: cannot find attribute `nonsense` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:150:7
   |
   |
LL |     #[nonsense]


error[E0425]: cannot find value `nonsense` in module `rustc_errors::fluent`
   |
   |
LL | #[error(nonsense, code = "E0123")]
   |         ^^^^^^^^ not found in `rustc_errors::fluent`

error[E0277]: the trait bound `Hello: IntoDiagnosticArg` is not satisfied
   |
LL | #[derive(SessionDiagnostic)]
LL | #[derive(SessionDiagnostic)]
   |          ^^^^^^^^^^^^^^^^^ the trait `IntoDiagnosticArg` is not implemented for `Hello`
   |
   = help: the following other types implement trait `IntoDiagnosticArg`:
             &'a str
             Ident
             NonZeroU32
             String
             Symbol
             Symbol
             bool
             i128
           and 13 others
note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
   |
   |
LL |         arg: impl IntoDiagnosticArg,
   |                   ^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::set_arg`
   = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 48 previous errors

Some errors have detailed explanations: E0277, E0425.
For more information about an error, try `rustc --explain E0277`.
