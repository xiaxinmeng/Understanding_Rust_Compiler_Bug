plain

---- [ui] src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs stdout ----
diff of stderr:

21 LL | #[nonsense(typeck::ambiguous_lifetime_bound, code = "E0123")]
23    |
23    |
-    = help: only `diag`, `help`, `note` and `warn_` are valid attributes
+    = help: only `diag`, `help`, `note` and `warning` are valid attributes
25 
26 error: diagnostic slug not specified


421 LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]
423 
423 
- error: cannot find attribute `warning` in this scope
-   --> $DIR/diagnostic-derive.rs:565:3
-    |
- LL | #[warning(typeck::ambiguous_lifetime_bound, code = "E0123")]
- 
430 error: cannot find attribute `lint` in this scope
431   --> $DIR/diagnostic-derive.rs:572:3
432    |
432    |

459    |                   ^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::set_arg`
460    = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 55 previous errors
+ error: aborting due to 54 previous errors
463 
464 Some errors have detailed explanations: E0277, E0425.
---
To only update this specific test, also pass `--test-args session-diagnostic/diagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: `#[derive(SessionDiagnostic)]` can only be used on structs
   |
   |
LL | / #[diag(typeck::ambiguous_lifetime_bound, code = "E0123")]
LL | | //~^ ERROR `#[derive(SessionDiagnostic)]` can only be used on structs
LL | | enum SessionDiagnosticOnEnum {
LL | |     Foo,
LL | |     Bar,
LL | | }


error: `#[diag = ...]` is not a valid attribute
   |
   |
LL | #[diag = "E0123"]


error: `#[nonsense(...)]` is not a valid attribute
   |
   |
LL | #[nonsense(typeck::ambiguous_lifetime_bound, code = "E0123")]
   |
   |
   = help: only `diag`, `help`, `note` and `warning` are valid attributes

error: diagnostic slug not specified
   |
   |
LL | / #[nonsense(typeck::ambiguous_lifetime_bound, code = "E0123")]
LL | | //~^ ERROR `#[nonsense(...)]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | //~^^^ ERROR cannot find attribute `nonsense` in this scope
LL | | struct InvalidStructAttr {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[diag("...")]` is not a valid attribute
   |
   |
LL | #[diag("E0123")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: diagnostic slug not specified
   |
   |
LL | / #[diag("E0123")]
LL | | //~^ ERROR `#[diag("...")]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidLitNestedAttr {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[diag(nonsense(...))]` is not a valid attribute
   |
   |
LL | #[diag(nonsense("foo"), code = "E0123", slug = "foo")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: diagnostic slug not specified
   |
   |
LL | / #[diag(nonsense("foo"), code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[diag(nonsense(...))]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr1 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[diag(nonsense = ...)]` is not a valid attribute
   |
   |
LL | #[diag(nonsense = "...", code = "E0123", slug = "foo")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: diagnostic slug not specified
   |
   |
LL | / #[diag(nonsense = "...", code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[diag(nonsense = ...)]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr2 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[diag(nonsense = ...)]` is not a valid attribute
   |
   |
LL | #[diag(nonsense = 4, code = "E0123", slug = "foo")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: diagnostic slug not specified
   |
   |
LL | / #[diag(nonsense = 4, code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[diag(nonsense = ...)]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr3 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[diag(slug = ...)]` is not a valid attribute
   |
   |
LL | #[diag(typeck::ambiguous_lifetime_bound, code = "E0123", slug = "foo")]
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
LL | #[diag(typeck::ambiguous_lifetime_bound, code = "E0456")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:102:1
   |
   |
LL | #[diag(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:103:49
   |
   |
LL | #[diag(typeck::ambiguous_lifetime_bound, code = "E0456")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:102:49
   |
   |
LL | #[diag(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:109:65
   |
   |
LL | #[diag(typeck::ambiguous_lifetime_bound, code = "E0456", code = "E0457")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:109:49
   |
   |
LL | #[diag(typeck::ambiguous_lifetime_bound, code = "E0456", code = "E0457")]


error: `#[diag(typeck::ambiguous_lifetime_bound)]` is not a valid attribute
   |
   |
LL | #[diag(typeck::ambiguous_lifetime_bound, typeck::ambiguous_lifetime_bound, code = "E0456")]


error: diagnostic slug not specified
   |
   |
LL | struct KindNotProvided {} //~ ERROR diagnostic slug not specified
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: diagnostic slug not specified
   |
   |
LL | / #[diag(code = "E0456")]
LL | | //~^ ERROR diagnostic slug not specified
LL | | struct SlugNotProvided {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
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

error: the `#[label(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
   |
   |
LL |     #[label(typeck::label)]

error: `name` doesn't refer to a field on this type
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:166:45
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

error: the `#[label(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
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
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:256:5
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
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:446:52
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
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
LL | / #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]
LL | | //~^ ERROR `#[error(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `error` in this scope
LL | | struct ErrorAttribute {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[warning(...)]` is not a valid attribute
   |
   |
LL | #[warning(typeck::ambiguous_lifetime_bound, code = "E0123")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
LL | / #[warning(typeck::ambiguous_lifetime_bound, code = "E0123")]
LL | | //~^ ERROR `#[warning(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `warning` in this scope
LL | | struct WarningAttribute {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
   |
LL | #[lint(typeck::ambiguous_lifetime_bound, code = "E0123")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
LL | / #[lint(typeck::ambiguous_lifetime_bound, code = "E0123")]
LL | | //~^ ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `lint` in this scope
LL | | struct LintAttributeOnSessionDiag {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
   |
LL | #[lint(typeck::ambiguous_lifetime_bound, code = "E0123")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
LL | / #[lint(typeck::ambiguous_lifetime_bound, code = "E0123")]
LL | | //~^ ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `lint` in this scope
LL | | struct LintAttributeOnLintDiag {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(typeck::example_error)]`
error: cannot find attribute `nonsense` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:53:3
   |
   |
LL | #[nonsense(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: cannot find attribute `nonsense` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:141:7
   |
   |
LL |     #[nonsense]

error: cannot find attribute `error` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:558:3
   |
   |
LL | #[error(typeck::ambiguous_lifetime_bound, code = "E0123")]

error: cannot find attribute `lint` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:572:3
   |
   |
LL | #[lint(typeck::ambiguous_lifetime_bound, code = "E0123")]
   |   ^^^^ help: a built-in attribute with a similar name exists: `link`
error: cannot find attribute `lint` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:579:3
   |
   |
LL | #[lint(typeck::ambiguous_lifetime_bound, code = "E0123")]
   |   ^^^^ help: a built-in attribute with a similar name exists: `link`

error[E0425]: cannot find value `nonsense` in module `rustc_errors::fluent`
   |
   |
LL | #[diag(nonsense, code = "E0123")]
   |        ^^^^^^^^ not found in `rustc_errors::fluent`

error[E0277]: the trait bound `Hello: IntoDiagnosticArg` is not satisfied
   |
LL | #[derive(SessionDiagnostic)]
LL | #[derive(SessionDiagnostic)]
   |          ^^^^^^^^^^^^^^^^^ the trait `IntoDiagnosticArg` is not implemented for `Hello`
   |
   = help: the following other types implement trait `IntoDiagnosticArg`:
             &'a str
             DiagnosticArgFromDisplay<'_>
             Ident
             Ident
             MacroRulesNormalizedIdent
             String
             Symbol
           and 17 others
           and 17 others
note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
   |
   |
LL |         arg: impl IntoDiagnosticArg,
   |                   ^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::set_arg`
   = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 54 previous errors

Some errors have detailed explanations: E0277, E0425.
For more information about an error, try `rustc --explain E0277`.
