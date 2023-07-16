plain
diff of stderr:

363              rustc_middle::ty::Ty<'tcx>
364              usize
365 note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
-   --> /home/davidw/rust/rust/compiler/rustc_errors/src/diagnostic_builder.rs:531:19
367    |
367    |
368 LL |         arg: impl IntoDiagnosticArg,
369    |                   ^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::set_arg`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/diagnostic-derive.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args session-diagnostic/diagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: `#[derive(SessionDiagnostic)]` can only be used on structs
   |
   |
LL | / #[error(code = "E0123", slug = "foo")]
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
LL | #[nonsense(code = "E0123", slug = "foo")]
   |
   |
   = help: only `error` and `warning` are valid attributes

error: diagnostic kind not specified
   |
   |
LL | / #[nonsense(code = "E0123", slug = "foo")]
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


error: `slug` not specified
   |
   |
LL | / #[error("E0123")]
LL | | //~^ ERROR `#[error("...")]` is not a valid attribute
LL | | //~^^ ERROR `slug` not specified
LL | | struct InvalidLitNestedAttr {}
   |
   |
   = help: use the `#[error(slug = "...")]` attribute to set this diagnostic's slug

error: `#[error(nonsense)]` is not a valid attribute
   |
   |
LL | #[error(nonsense, code = "E0123", slug = "foo")]


error: `#[error(nonsense(...))]` is not a valid attribute
   |
   |
LL | #[error(nonsense("foo"), code = "E0123", slug = "foo")]


error: `#[error(nonsense = ...)]` is not a valid attribute
   |
   |
LL | #[error(nonsense = "...", code = "E0123", slug = "foo")]
   |
   |
   = help: only `slug` and `code` are valid nested attributes

error: `#[error(nonsense = ...)]` is not a valid attribute
   |
   |
LL | #[error(nonsense = 4, code = "E0123", slug = "foo")]


error: `#[suggestion = ...]` is not a valid attribute
   |
   |
LL |     #[suggestion = "bar"]
   |
   |
   = help: only `label`, `note` and `help` are valid field attributes
error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:93:1
   |
   |
LL | #[error(code = "E0456", slug = "bar")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:92:1
   |
   |
LL | #[error(code = "E0123", slug = "foo")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:93:16
   |
   |
LL | #[error(code = "E0456", slug = "bar")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:92:16
   |
   |
LL | #[error(code = "E0123", slug = "foo")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:93:32
   |
   |
LL | #[error(code = "E0456", slug = "bar")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:92:32
   |
   |
LL | #[error(code = "E0123", slug = "foo")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:101:1
   |
   |
LL | #[warning(code = "E0293", slug = "bar")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:100:1
   |
   |
LL | #[error(code = "E0123", slug = "foo")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:101:18
   |
   |
LL | #[warning(code = "E0293", slug = "bar")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:100:16
   |
   |
LL | #[error(code = "E0123", slug = "foo")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:101:34
   |
   |
LL | #[warning(code = "E0293", slug = "bar")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:100:32
   |
   |
LL | #[error(code = "E0123", slug = "foo")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:108:32
   |
   |
LL | #[error(code = "E0456", code = "E0457", slug = "bar")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:108:16
   |
   |
LL | #[error(code = "E0456", code = "E0457", slug = "bar")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:113:46
   |
   |
LL | #[error(code = "E0456", slug = "foo", slug = "bar")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:113:32
   |
   |
LL | #[error(code = "E0456", slug = "foo", slug = "bar")]


error: diagnostic kind not specified
   |
   |
LL | struct KindNotProvided {} //~ ERROR diagnostic kind not specified
   |
   |
   = help: use the `#[error(...)]` attribute to create an error

error: `slug` not specified
   |
   |
LL | / #[error(code = "E0456")] //~ ERROR `slug` not specified
LL | | struct SlugNotProvided {}
   |
   |
   = help: use the `#[error(slug = "...")]` attribute to set this diagnostic's slug

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

error: the `#[label = ...]` attribute can only be applied to fields of type `Span`
   |
   |
LL |     #[label = "bar"]

error: `name` doesn't refer to a field on this type
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:164:42
   |
   |
LL |     #[suggestion(message = "bar", code = "{name}")]


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

error: the `#[label = ...]` attribute can only be applied to fields of type `Span`
   |
   |
LL |     #[label = "bar"]


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
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:254:5
   |
   |
LL | /     #[suggestion(message = "bar", code = "This is suggested code")]
LL | |     //~^ ERROR wrong field type for suggestion
LL | |     suggestion: Applicability,
   |
   |
   = help: `#[suggestion(...)]` should be applied to fields of type `Span` or `(Span, Applicability)`

error: type of field annotated with `#[suggestion(...)]` contains more than one `Span`
   |
   |
LL | /     #[suggestion(message = "bar", code = "This is suggested code")]
LL | |     //~^ ERROR type of field annotated with `#[suggestion(...)]` contains more than one `Span`
LL | |     suggestion: (Span, Span, Applicability),


error: type of field annotated with `#[suggestion(...)]` contains more than one Applicability
   |
   |
LL | /     #[suggestion(message = "bar", code = "This is suggested code")]
LL | |     //~^ ERROR type of field annotated with `#[suggestion(...)]` contains more than one
LL | |     suggestion: (Applicability, Applicability, Span),


error: `#[label(...)]` is not a valid attribute
   |
   |
LL |     #[label("bar")]
   |
   |
   = help: only `suggestion{,_short,_hidden,_verbose}` are valid field attributes

error: `#[help]` must come after `#[error(..)]` or `#[warn(..)]`
   |
   |
LL | #[help]


error: `#[help = ...]` must come after `#[error(..)]` or `#[warn(..)]`
   |
   |
LL | #[help = "bar"]


error: `#[note]` must come after `#[error(..)]` or `#[warn(..)]`
   |
   |
LL | #[note]


error: `#[note = ...]` must come after `#[error(..)]` or `#[warn(..)]`
   |
   |
LL | #[note = "bar"]


error: applicability cannot be set in both the field and attribute
   |
   |
LL |     #[suggestion(message = "bar", code = "...", applicability = "maybe-incorrect")]

error: invalid applicability
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:448:49
   |
   |
LL |     #[suggestion(message = "bar", code = "...", applicability = "batman")]

error: cannot find attribute `nonsense` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:51:3
   |
   |
LL | #[nonsense(code = "E0123", slug = "foo")]

error: cannot find attribute `nonsense` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:139:7
   |
   |
LL |     #[nonsense]


error[E0277]: the trait bound `Hello: IntoDiagnosticArg` is not satisfied
   |
LL | #[derive(SessionDiagnostic)]
LL | #[derive(SessionDiagnostic)]
   |          ^^^^^^^^^^^^^^^^^ the trait `IntoDiagnosticArg` is not implemented for `Hello`
   |
   = help: the following other types implement trait `IntoDiagnosticArg`:
             &'a str
             String
             Symbol
             rustc_middle::ty::Ty<'tcx>
             usize
             usize
note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
   |
   |
LL |         arg: impl IntoDiagnosticArg,
   |                   ^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::set_arg`
   = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 43 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
