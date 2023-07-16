plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.......................i..................................F.F..........

---- [ui] src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs stdout ----
diff of stderr:


416    |     ^^^^^^^^^^^^^^^^
417 
418 error: expected exactly one string literal for `code = ...`
-   --> $DIR/subdiagnostic-derive.rs:668:23
420    |
420    |
421 LL |     #[suggestion_part(code("foo"))]

423 
423 
424 error: expected exactly one string literal for `code = ...`
-   --> $DIR/subdiagnostic-derive.rs:677:23
426    |
426    |
427 LL |     #[suggestion_part(code("foo", "bar"))]

429 
429 
430 error: expected exactly one string literal for `code = ...`
-   --> $DIR/subdiagnostic-derive.rs:686:23
432    |
432    |
433 LL |     #[suggestion_part(code(3))]

435 
435 
436 error: expected exactly one string literal for `code = ...`
-   --> $DIR/subdiagnostic-derive.rs:695:23
438    |
438    |
439 LL |     #[suggestion_part(code())]

441 
441 
442 error: `code = "..."`/`code(...)` must contain only string literals
-   --> $DIR/subdiagnostic-derive.rs:704:23
444    |
444    |
445 LL |     #[suggestion_part(code = 3)]


505 LL | #[label(slug)]
506    |         ^^^^ not found in `crate::fluent_generated`
- error: aborting due to 72 previous errors
+ error[E0425]: cannot find value `parser_add_paren` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:668:24
+    |
+    |
+ LL | #[multipart_suggestion(parser_add_paren)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `parser_add_paren` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:677:24
+    |
+    |
+ LL | #[multipart_suggestion(parser_add_paren)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `parser_add_paren` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:686:24
+    |
+    |
+ LL | #[multipart_suggestion(parser_add_paren)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `parser_add_paren` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:695:24
+    |
+    |
+ LL | #[multipart_suggestion(parser_add_paren)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `parser_add_paren` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:704:24
+    |
+    |
+ LL | #[multipart_suggestion(parser_add_paren)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error: aborting due to 77 previous errors
509 
510 For more information about this error, try `rustc --explain E0425`.
511 
---
To only update this specific test, also pass `--test-args session-diagnostic/subdiagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: label without `#[primary_span]` field
   |
   |
LL | / #[label(no_crate_example)]
LL | | //~^ ERROR label without `#[primary_span]` field
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


error: `#[label(bug = ...)]` is not a valid attribute
   |
   |
LL | #[label(bug = "...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
   |
   |
LL | #[label(bug = "...")]


error: `#[label("...")]` is not a valid attribute
   |
   |
LL | #[label("...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
   |
   |
LL | #[label("...")]


error: `#[label(slug = ...)]` is not a valid attribute
   |
   |
LL | #[label(slug = 4)]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
   |
   |
LL | #[label(slug = 4)]


error: `#[label(slug(...))]` is not a valid attribute
   |
   |
LL | #[label(slug("..."))]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
   |
   |
LL | #[label(slug("..."))]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
   |
   |
LL | #[label()]


error: `#[label(code = ...)]` is not a valid attribute
   |
   |
LL | #[label(no_crate_example, code = "...")]


error: `#[label(applicability = ...)]` is not a valid attribute
   |
   |
LL | #[label(no_crate_example, applicability = "machine-applicable")]

error: unsupported type attribute for subdiagnostic enum
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:161:1
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


error: `#[label(code = ...)]` is not a valid attribute
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
LL | | //~^ ERROR label without `#[primary_span]` field
LL | | struct W {
LL | |     #[primary_span]
LL | |     //~^ ERROR the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
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
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:310:1
   |
   |
LL | / union AC {
LL | | //~^ ERROR unexpected unsupported untagged union
LL | |     span: u32,
LL | |     b: u64
LL | | }


error: `#[label(no_crate::example)]` is not a valid attribute
   |
   |
LL | #[label(no_crate_example, no_crate::example)]
   |
   |
   = help: a diagnostic slug must be the first argument to the attribute
error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:338:5
   |
   |
LL |     #[primary_span]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:335:5
   |
   |
LL |     #[primary_span]


error: subdiagnostic kind not specified
   |
LL | struct AG {
   |        ^^


error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:381:46
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:381:32
   |
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:399:5
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
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:432:45
   |
   |
LL | #[suggestion(no_crate_example, code ="...", applicability = "foo")]


error: suggestion without `#[primary_span]` field
   |
   |
LL | / #[suggestion(no_crate_example, code = "...")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct AR {
LL | |     var: String,
LL | | }

error: unsupported type attribute for subdiagnostic enum
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:464:1
   |
   |
LL | #[label]
   | ^^^^^^^^

error: `var` doesn't refer to a field on this type
   |
   |
LL | #[suggestion(no_crate_example, code ="{var}", applicability = "machine-applicable")]


error: `var` doesn't refer to a field on this type
   |
   |
LL |     #[suggestion(no_crate_example, code ="{var}", applicability = "machine-applicable")]


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
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct BA {
LL | |     #[suggestion_part]
LL | |     var: String,
LL | | }
   | |_^


error: `#[multipart_suggestion(code = ...)]` is not a valid attribute
   |
   |
LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
   |
   |
   = help: only `applicability` is a valid nested attributes

error: multipart suggestion without any `#[suggestion_part(...)]` fields
   |
   |
LL | / #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
LL | | //~^ ERROR multipart suggestion without any `#[suggestion_part(...)]` fields
LL | | //~| ERROR `#[multipart_suggestion(code = ...)]` is not a valid attribute
LL | | struct BBa {
LL | |     var: String,
LL | | }


error: `#[suggestion_part(...)]` attribute without `code = "..."`
   |
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: `#[suggestion_part(...)]` attribute without `code = "..."`
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
LL | | //~^ ERROR multipart suggestion without any `#[suggestion_part(...)]` fields
LL | | struct BC {
LL | |     #[primary_span]
LL | |     //~^ ERROR `#[primary_span]` is not a valid attribute
LL | |     span: Span,
LL | | }


error: `#[suggestion_part(...)]` attribute without `code = "..."`
   |
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: `#[suggestion_part(...)]` attribute without `code = "..."`
   |
   |
LL |     #[suggestion_part()]


error: `#[suggestion_part(foo = ...)]` is not a valid attribute
   |
   |
LL |     #[suggestion_part(foo = "bar")]
   |
   |
   = help: `code` is the only valid nested attribute

error: the `#[suggestion_part(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
   |
   |
LL |     #[suggestion_part(code = "...")]


error: the `#[suggestion_part(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
   |
   |
LL |     #[suggestion_part()]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:593:37
   |
   |
LL |     #[suggestion_part(code = "...", code = ",,,")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:593:23
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


error: expected exactly one string literal for `code = ...`
   |
   |
LL |     #[suggestion_part(code("foo", "bar"))]


error: expected exactly one string literal for `code = ...`
   |
   |
LL |     #[suggestion_part(code(3))]


error: expected exactly one string literal for `code = ...`
   |
   |
LL |     #[suggestion_part(code())]


error: `code = "..."`/`code(...)` must contain only string literals
   |
   |
LL |     #[suggestion_part(code = 3)]

error: cannot find attribute `foo` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:65:3
   |
---

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:187:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:199:7
   |
   |
LL |     #[bar = 4]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:211:7
   |
   |
LL |     #[bar("...")]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:272:7
   |
   |
LL |     #[bar]
   |       ^^^

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:283:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:294:7
   |
   |
LL |     #[bar("...")]


error[E0425]: cannot find value `slug` in module `crate::fluent_generated`
   |
   |
LL | #[label(slug)]
   |         ^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `parser_add_paren` in module `crate::fluent_generated`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:668:24
   |
   |
LL | #[multipart_suggestion(parser_add_paren)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `parser_add_paren` in module `crate::fluent_generated`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:677:24
   |
   |
LL | #[multipart_suggestion(parser_add_paren)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `parser_add_paren` in module `crate::fluent_generated`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:686:24
   |
   |
LL | #[multipart_suggestion(parser_add_paren)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `parser_add_paren` in module `crate::fluent_generated`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:695:24
   |
   |
LL | #[multipart_suggestion(parser_add_paren)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `parser_add_paren` in module `crate::fluent_generated`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:704:24
   |
   |
LL | #[multipart_suggestion(parser_add_paren)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error: aborting due to 77 previous errors

For more information about this error, try `rustc --explain E0425`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs stdout ----
diff of stderr:

574    = help: eager subdiagnostics are not supported on lints
575 
576 error: expected at least one string literal for `code(...)`
-   --> $DIR/diagnostic-derive.rs:779:18
578    |
578    |
579 LL |     #[suggestion(code())]

581 
581 
582 error: `code(...)` must contain only string literals
-   --> $DIR/diagnostic-derive.rs:787:23
584    |
584    |
585 LL |     #[suggestion(code(foo))]

587 
587 
588 error: `code = "..."`/`code(...)` must contain only string literals
-   --> $DIR/diagnostic-derive.rs:795:18
590    |
590    |
591 LL |     #[suggestion(code = 3)]


651 LL | #[diag(nonsense, code = "E0123")]
652    |        ^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `compiletest_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:764:8
+    |
+    |
+ LL | #[diag(compiletest_example)]
+    |        ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `compiletest_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:771:8
+    |
+    |
+ LL | #[diag(compiletest_example)]
+    |        ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `compiletest_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:778:8
+    |
+    |
+ LL | #[diag(compiletest_example)]
+    |        ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `compiletest_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:786:8
+    |
+    |
+ LL | #[diag(compiletest_example)]
+    |        ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `compiletest_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:794:8
+    |
+    |
+ LL | #[diag(compiletest_example)]
+    |        ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ 
654 error[E0277]: the trait bound `Hello: IntoDiagnosticArg` is not satisfied
656    |


665    |                   ^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::set_arg`
666    = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 83 previous errors
+ error: aborting due to 88 previous errors
669 
670 Some errors have detailed explanations: E0277, E0425.
---
To only update this specific test, also pass `--test-args session-diagnostic/diagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: unsupported type attribute for diagnostic derive enum
   |
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

error: `#[diag = ...]` is not a valid attribute
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
LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

error: `#[suggestion = ...]` is not a valid attribute
   |
   |
LL |     #[suggestion = "bar"]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:109:8
   |
   |
LL | #[diag(no_crate_example, code = "E0456")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:108:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:109:33
   |
   |
LL | #[diag(no_crate_example, code = "E0456")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:108:33
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:115:49
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:115:33
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]


error: `#[diag(no_crate::example)]` is not a valid attribute
   |
   |
LL | #[diag(no_crate_example, no_crate::example, code = "E0456")]
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
LL |     #[label(no_crate_label)]

error: `name` doesn't refer to a field on this type
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:172:46
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


error: `#[suggestion(nonsense = ...)]` is not a valid attribute
   |
   |
LL |     #[suggestion(nonsense = "bar")]
   |
   |
   = help: only `code` and `applicability` are valid nested attributes

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
   = help: only `code` and `applicability` are valid nested attributes

error: suggestion without `code = "..."`
   |
   |
LL |     #[suggestion(msg = "bar")]

error: wrong field type for suggestion
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:265:5
   |
   |
LL | /     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
LL | |     //~^ ERROR wrong field type for suggestion
LL | |     suggestion: Applicability,
   |
   |
   = help: `#[suggestion(...)]` should be applied to fields of type `Span` or `(Span, Applicability)`
error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:281:24
   |
   |
LL |     suggestion: (Span, Span, Applicability),
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:281:18
   |
   |
LL |     suggestion: (Span, Span, Applicability),

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:289:33
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:289:18
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),


error: `#[label = ...]` is not a valid attribute
   |
   |
LL |     #[label = "bar"]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:447:53
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "maybe-incorrect")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:449:24
   |
   |
LL |     suggestion: (Span, Applicability),

error: invalid applicability
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:455:53
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "batman")]


error: `#[label(foo)]` is not a valid attribute
   |
   |
LL |     #[label(no_crate_label, foo)]
   |
   |
   = help: a diagnostic slug must be the first argument to the attribute

error: `#[label(foo = ...)]` is not a valid attribute
   |
   |
LL |     #[label(no_crate_label, foo = "...")]


error: `#[label(foo(...))]` is not a valid attribute
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
LL | #[warn_(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
   |
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
   |
   |
LL | #[lint(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
   |
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
LL | | //~^ ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `lint` in this scope
LL | | struct LintAttributeOnLintDiag {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(compiletest_example)]`
error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:597:53
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:597:39
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]

error: wrong types for suggestion
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:606:24
   |
   |
LL |     suggestion: (Span, usize),
   |
   |
   = help: `#[suggestion(...)]` on a tuple field must be applied to fields of type `(Span, Applicability)`
error: wrong types for suggestion
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:614:17
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
