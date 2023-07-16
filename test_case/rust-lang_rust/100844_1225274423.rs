plain
---- [ui] src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs stdout ----
diff of stderr:

58    |
59    = help: first argument of the attribute should be the diagnostic slug
60 
- error: diagnostic slug must be first argument of a `#[label(...)]` attribute
-   --> $DIR/subdiagnostic-derive.rs:128:1
-    |
- LL | #[label()]
- 
- 
67 error: `code` is not a valid nested attribute of a `label` attribute
69    |

108    |
108    |
109    = help: first argument of the attribute should be the diagnostic slug
110 
- error: diagnostic slug must be first argument of a `#[label(...)]` attribute
-   --> $DIR/subdiagnostic-derive.rs:217:5
-    |
- LL |     #[label(code = "...")]
- 
- 
- error: subdiagnostic kind not specified
-   --> $DIR/subdiagnostic-derive.rs:234:5
- LL |     B {
-    |     ^
- 
- 
123 error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
125    |

175    | |_^
176 
176 
177 error: specified multiple times
-   --> $DIR/subdiagnostic-derive.rs:314:1
+   --> $DIR/subdiagnostic-derive.rs:318:5
179    |
- LL | #[label(parser::add_paren)]
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     #[primary_span]
182    |
183 note: previously specified here
-   --> $DIR/subdiagnostic-derive.rs:311:1
+   --> $DIR/subdiagnostic-derive.rs:318:5
+   --> $DIR/subdiagnostic-derive.rs:318:5
185    |
- LL | #[label(parser::add_paren)]
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     #[primary_span]
188 
- error: specified multiple times
-   --> $DIR/subdiagnostic-derive.rs:314:1
-    |
-    |
- LL | #[label(parser::add_paren)]
-    |
- note: previously specified here
-   --> $DIR/subdiagnostic-derive.rs:311:1
-    |
-    |
- LL | #[label(parser::add_paren)]
- 
- 
201 error: `#[label(parser::add_paren)]` is not a valid attribute
203    |


218 LL |     #[primary_span]
220 
220 
- error: subdiagnostic kind not specified
-   --> $DIR/subdiagnostic-derive.rs:342:8
-    |
- LL | struct AG {
- 
227 error: specified multiple times
228   --> $DIR/subdiagnostic-derive.rs:379:47
229    |
229    |

395 LL | #[label(slug)]
396    |         ^^^^ not found in `rustc_errors::fluent`
- error: aborting due to 52 previous errors
+ error: aborting due to 47 previous errors
399 
400 For more information about this error, try `rustc --explain E0425`.
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
LL | / #[label(parser::add_paren)]
LL | | //~^ ERROR label without `#[primary_span]` field
LL | | struct C {
LL | |     var: String,
LL | | }


error: `#[label]` is not a valid attribute
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
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: `#[label("...")]` is not a valid attribute
   |
   |
LL | #[label("...")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: `#[label(slug = ...)]` is not a valid attribute
   |
   |
LL | #[label(slug = 4)]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: `#[label(slug(...))]` is not a valid attribute
   |
   |
LL | #[label(slug("..."))]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: `code` is not a valid nested attribute of a `label` attribute
   |
   |
LL | #[label(parser::add_paren, code = "...")]


error: `applicability` is not a valid nested attribute of a `label` attribute
   |
   |
LL | #[label(parser::add_paren, applicability = "machine-applicable")]

error: unsupported type attribute for subdiagnostic enum
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:155:1
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


error: `#[bar("...")]` is not a valid attribute
   |
   |
LL |     #[bar("...")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
   |
   |
LL |     #[primary_span]


error: label without `#[primary_span]` field
   |
   |
LL | / #[label(parser::add_paren)]
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

error: unexpected unsupported untagged union
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:304:1
   |
   |
LL | / union AC {
LL | | //~^ ERROR unexpected unsupported untagged union
LL | |     span: u32,
LL | |     b: u64
LL | | }

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:318:5
   |
   |
LL |     #[primary_span]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:318:5
   |
   |
LL |     #[primary_span]


error: `#[label(parser::add_paren)]` is not a valid attribute
   |
   |
LL | #[label(parser::add_paren, parser::add_paren)]
   |
   |
   = help: a diagnostic slug must be the first argument to the attribute
error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:336:5
   |
   |
LL |     #[primary_span]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:333:5
   |
   |
LL |     #[primary_span]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:379:47
   |
   |
LL | #[suggestion(parser::add_paren, code = "...", code = "...")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:379:33
   |
   |
LL | #[suggestion(parser::add_paren, code = "...", code = "...")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:397:5
   |
---
   |
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^

error: the `#[applicability]` attribute can only be applied to fields of type `Applicability`
   |
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: suggestion without `applicability`
   |
   |
LL | / #[suggestion(parser::add_paren, code = "...")]
LL | | //~^ ERROR suggestion without `applicability`
LL | | struct AL {
LL | |     #[primary_span]
LL | |     applicability: Span,
LL | | }
   | |_^


error: suggestion without `applicability`
   |
   |
LL | / #[suggestion(parser::add_paren, code = "...")]
LL | | //~^ ERROR suggestion without `applicability`
LL | | struct AM {
LL | |     #[primary_span]
LL | |     span: Span,
LL | | }


error: suggestion without `code = "..."`
   |
   |
LL | / #[suggestion(parser::add_paren)]
LL | | //~^ ERROR suggestion without `code = "..."`
LL | | struct AN {
LL | |     #[primary_span]
LL | |     applicability: Applicability,
LL | | }
   | |_^


error: invalid applicability
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:432:46
   |
LL | #[suggestion(parser::add_paren, code ="...", applicability = "foo")]


error: suggestion without `applicability`
   |
   |
LL | / #[suggestion(parser::add_paren, code = "...")]
LL | | //~^ ERROR suggestion without `applicability`
LL | | //~^^ ERROR suggestion without `#[primary_span]` field
LL | | struct AR {
LL | |     var: String,
LL | | }


error: suggestion without `#[primary_span]` field
   |
   |
LL | / #[suggestion(parser::add_paren, code = "...")]
LL | | //~^ ERROR suggestion without `applicability`
LL | | //~^^ ERROR suggestion without `#[primary_span]` field
LL | | struct AR {
LL | |     var: String,
LL | | }

error: unsupported type attribute for subdiagnostic enum
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:465:1
   |
   |
LL | #[label]
   | ^^^^^^^^

error: `var` doesn't refer to a field on this type
   |
   |
LL | #[suggestion(parser::add_paren, code ="{var}", applicability = "machine-applicable")]


error: `var` doesn't refer to a field on this type
   |
   |
LL |     #[suggestion(parser::add_paren, code ="{var}", applicability = "machine-applicable")]

error: cannot find attribute `foo` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:63:3
   |
---

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:181:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:193:7
   |
   |
LL |     #[bar = 4]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:205:7
   |
   |
LL |     #[bar("...")]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:266:7
   |
   |
LL |     #[bar]
   |       ^^^

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:277:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:288:7
   |
   |
LL |     #[bar("...")]


error[E0425]: cannot find value `slug` in module `rustc_errors::fluent`
   |
   |
LL | #[label(slug)]
   |         ^^^^ not found in `rustc_errors::fluent`
error: aborting due to 47 previous errors

For more information about this error, try `rustc --explain E0425`.
------------------------------------------
