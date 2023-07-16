plain
Uplifting rustc (stage1 -> stage3)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
.....................................................i.....F.F.........
failures:

---- [ui] tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs stdout ----
diff of stderr:
diff of stderr:

38 LL | #[label(bug = "...")]
40 
40 
- error: unexpected literal in nested attribute, expected ident
-   --> $DIR/subdiagnostic-derive.rs:94:9
-    |
- LL | #[label("...")]
- 
47 error: invalid nested attribute
48   --> $DIR/subdiagnostic-derive.rs:103:9
49    |
49    |

68 LL | #[label(slug("..."))]
70 
70 
- error: unexpected end of input, unexpected token in nested attribute, expected ident
-   --> $DIR/subdiagnostic-derive.rs:133:9
-    |
- LL | #[label()]
- 
77 error: invalid nested attribute
78   --> $DIR/subdiagnostic-derive.rs:142:27
79    |
79    |

174    |
175    = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes
- error: unexpected unsupported untagged union
-   --> $DIR/subdiagnostic-derive.rs:309:1
-    |
-    |
- LL | / union AC {
- LL | |
- LL | |     span: u32,
- LL | |     b: u64,
- LL | | }
- 
- 
187 error: a diagnostic slug must be the first argument to the attribute
189    |

331 LL |     #[suggestion_part]
332    |     ^^^^^^^^^^^^^^^^^^
332    |     ^^^^^^^^^^^^^^^^^^
333 
- error: unexpected end of input, unexpected token in nested attribute, expected ident
-   --> $DIR/subdiagnostic-derive.rs:555:23
-    |
- LL |     #[suggestion_part()]
- 
- 
340 error: `#[primary_span]` is not a valid attribute
342    |


381 LL |     #[suggestion_part()]
383 
383 
- error: unexpected end of input, unexpected token in nested attribute, expected ident
-   --> $DIR/subdiagnostic-derive.rs:575:23
-    |
- LL |     #[suggestion_part()]
- 
- error: expected `,`
-   --> $DIR/subdiagnostic-derive.rs:578:27
-    |
-    |
- LL |     #[suggestion_part(foo = "bar")]
- 
396 error: specified multiple times
397   --> $DIR/subdiagnostic-derive.rs:593:37
398    |
398    |

417 LL |     #[suggestion_part(code("foo"))]
419 
- error: unexpected token
-   --> $DIR/subdiagnostic-derive.rs:670:28
-    |
-    |
- LL |     #[suggestion_part(code("foo"))]
- 
- 
426 error: expected exactly one string literal for `code = ...`
428    |


429 LL |     #[suggestion_part(code("foo", "bar"))]
431 
- error: unexpected token
-   --> $DIR/subdiagnostic-derive.rs:680:28
-    |
-    |
- LL |     #[suggestion_part(code("foo", "bar"))]
- 
- 
438 error: expected exactly one string literal for `code = ...`
440    |


441 LL |     #[suggestion_part(code(3))]
443 
- error: unexpected token
-   --> $DIR/subdiagnostic-derive.rs:690:28
-    |
-    |
- LL |     #[suggestion_part(code(3))]
- 
- 
450 error: expected exactly one string literal for `code = ...`
452    |


453 LL |     #[suggestion_part(code())]
455 
- error: expected string literal
-   --> $DIR/subdiagnostic-derive.rs:712:30
-    |
-    |
- LL |     #[suggestion_part(code = 3)]
- 
462 error: specified multiple times
463   --> $DIR/subdiagnostic-derive.rs:754:1
464    |
464    |

513 LL | #[suggestion(no_crate_example, code = "", style("foo"))]
515 
- error: expected `,`
-   --> $DIR/subdiagnostic-derive.rs:803:48
-    |
-    |
- LL | #[suggestion(no_crate_example, code = "", style("foo"))]
- 
- 
522 error: `#[primary_span]` is not a valid attribute
524    |

540 LL | | }
541    | |_^
541    | |_^
542 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL | #[label("...")]
+    |         ^^^^^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL | #[label()]
+    |         ^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+ LL | union AC {
+    | ^^^^^ maybe a missing crate `core`?
+ 
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL |     #[suggestion_part()]
+    |                       ^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL |     #[suggestion_part()]
+    |                       ^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL |     #[suggestion_part(foo = "bar")]
+    |                           ^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL |     #[suggestion_part(code("foo"))]
+    |                            ^^^^^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL |     #[suggestion_part(code("foo", "bar"))]
+    |                            ^^^^^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL |     #[suggestion_part(code(3))]
+    |                            ^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL |     #[suggestion_part(code = 3)]
+    |                              ^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "", style("foo"))]
+    |                                                ^ maybe a missing crate `core`?
543 error: cannot find attribute `foo` in this scope
544   --> $DIR/subdiagnostic-derive.rs:65:3
545    |

---
To only update this specific test, also pass `--test-args session-diagnostic/subdiagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: label without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:49:1
   |
LL | / #[label(no_crate_example)]
LL | | //~^ ERROR label without `#[primary_span]` field
LL | | struct C {
LL | |     var: String,
LL | | }


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:56:1
LL | #[label]
   | ^^^^^^^^


error: `#[foo]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:65:1
LL | #[foo]
   | ^^^^^^


error: `#[label = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:75:1
   |
LL | #[label = "..."]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:84:9
   |
   |
LL | #[label(bug = "...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:84:1
   |
LL | #[label(bug = "...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:103:9
   |
   |
LL | #[label(slug = 4)]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:103:1
   |
LL | #[label(slug = 4)]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:113:9
   |
   |
LL | #[label(slug("..."))]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:113:1
   |
LL | #[label(slug("..."))]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:142:27
   |
   |
LL | #[label(no_crate_example, code = "...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:151:27
   |
   |
LL | #[label(no_crate_example, applicability = "machine-applicable")]

error: unsupported type attribute for subdiagnostic enum
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:160:1
   |
   |
LL | #[foo]
   | ^^^^^^

error: `#[bar]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:174:5
LL |     #[bar]
   |     ^^^^^^


error: `#[bar = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:186:5
   |
LL |     #[bar = "..."]


error: `#[bar = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:198:5
   |
LL |     #[bar = 4]


error: `#[bar(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:210:5
   |
LL |     #[bar("...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:222:13
   |
   |
LL |     #[label(code = "...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:222:5
   |
LL |     #[label(code = "...")]


error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:251:5
   |
LL |     #[primary_span]


error: label without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:248:1
   |
LL | / #[label(no_crate_example)]
LL | | //~^ ERROR label without `#[primary_span]` field
LL | | struct W {
LL | |     #[primary_span]
LL | |     //~^ ERROR the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
LL | |     span: String,
LL | | }


error: `#[applicability]` is only valid on suggestions
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:261:5
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: `#[bar]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:271:5
LL |     #[bar]
   |     ^^^^^^
   |
   |
   = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes

error: `#[bar = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:282:5
   |
LL |     #[bar = "..."]


error: `#[bar(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:293:5
   |
LL |     #[bar("...")]
   |
   |
   = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes

error: a diagnostic slug must be the first argument to the attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:324:44
   |
LL | #[label(no_crate_example, no_crate::example)]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:337:5
   |
   |
LL |     #[primary_span]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:334:5
   |
   |
LL |     #[primary_span]


error: subdiagnostic kind not specified
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:343:8
LL | struct AG {
   |        ^^

error: specified multiple times
error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:380:46
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:380:32
   |
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:398:5
   |
---
   |
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^

error: the `#[applicability]` attribute can only be applied to fields of type `Applicability`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:408:5
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:421:1
   |
LL | #[suggestion(no_crate_example)]

error: invalid applicability
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:431:62
   |
   |
LL | #[suggestion(no_crate_example, code = "...", applicability = "foo")]


error: suggestion without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:449:1
   |
LL | / #[suggestion(no_crate_example, code = "...")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct AR {
LL | |     var: String,
LL | | }

error: unsupported type attribute for subdiagnostic enum
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:463:1
   |
   |
LL | #[label]
   | ^^^^^^^^

error: `var` doesn't refer to a field on this type
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:483:39
   |
LL | #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]


error: `var` doesn't refer to a field on this type
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:502:43
   |
LL |     #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]


error: `#[suggestion_part]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:525:5
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^
   |
   |
   = help: `#[suggestion_part(...)]` is only valid in multipart suggestions, use `#[primary_span]` instead

error: `#[suggestion_part(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:528:5
   |
LL |     #[suggestion_part(code = "...")]
   |
   |
   = help: `#[suggestion_part(...)]` is only valid in multipart suggestions

error: suggestion without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:522:1
   |
LL | / #[suggestion(no_crate_example, code = "...")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct BA {
LL | |     #[suggestion_part]
LL | |     var: String,
LL | | }
   | |_^


error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:537:42
   |
LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
   |
   |
   = help: only `style` and `applicability` are valid nested attributes

error: multipart suggestion without any `#[suggestion_part(...)]` fields
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:537:1
   |
LL | / #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
LL | | //~^ ERROR multipart suggestion without any `#[suggestion_part(...)]` fields
LL | | //~| ERROR invalid nested attribute
LL | | struct BBa {
LL | |     var: String,
LL | | }


error: `#[suggestion_part(...)]` attribute without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:547:5
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: `#[primary_span]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:564:5
   |
LL |     #[primary_span]
   |
   |
   = help: multipart suggestions use one or more `#[suggestion_part]`s rather than one `#[primary_span]`

error: multipart suggestion without any `#[suggestion_part(...)]` fields
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:561:1
   |
LL | / #[multipart_suggestion(no_crate_example)]
LL | | //~^ ERROR multipart suggestion without any `#[suggestion_part(...)]` fields
LL | | struct BC {
LL | |     #[primary_span]
LL | |     //~^ ERROR `#[primary_span]` is not a valid attribute
LL | |     span: Span,
LL | | }


error: `#[suggestion_part(...)]` attribute without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:572:5
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: `code` is the only valid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:578:23
   |
LL |     #[suggestion_part(foo = "bar")]


error: the `#[suggestion_part(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:582:5
   |
LL |     #[suggestion_part(code = "...")]


error: the `#[suggestion_part(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:585:5
   |
LL |     #[suggestion_part()]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:593:37
   |
   |
LL |     #[suggestion_part(code = "...", code = ",,,")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:593:23
   |
   |
LL |     #[suggestion_part(code = "...", code = ",,,")]


error: `#[applicability]` has no effect if all `#[suggestion]`/`#[multipart_suggestion]` attributes have a static `applicability = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:622:5
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:670:34
   |
LL |     #[suggestion_part(code("foo"))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:680:41
   |
LL |     #[suggestion_part(code("foo", "bar"))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:690:30
   |
LL |     #[suggestion_part(code(3))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:700:29
   |
LL |     #[suggestion_part(code())]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:754:1
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:754:1
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]


error: `#[suggestion_hidden(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:763:1
   |
LL | #[suggestion_hidden(no_crate_example, code = "")]
   |
   |
   = help: Use `#[suggestion(..., style = "hidden")]` instead

error: `#[suggestion_hidden(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:771:1
   |
LL | #[suggestion_hidden(no_crate_example, code = "", style = "normal")]
   |
   |
   = help: Use `#[suggestion(..., style = "hidden")]` instead
error: invalid suggestion style
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:779:51
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "foo")]
   |
   |
   = help: valid styles are `normal`, `short`, `hidden`, `verbose` and `tool-only`

error: expected `= "xxx"`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:787:49
   |
LL | #[suggestion(no_crate_example, code = "", style = 42)]


error: a diagnostic slug must be the first argument to the attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:795:48
   |
LL | #[suggestion(no_crate_example, code = "", style)]


error: expected `= "xxx"`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:803:48
   |
LL | #[suggestion(no_crate_example, code = "", style("foo"))]


error: `#[primary_span]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:815:5
   |
LL |     #[primary_span]
   |
   = note: there must be exactly one primary span
   = note: there must be exactly one primary span
   = help: to create a suggestion with multiple spans, use `#[multipart_suggestion]` instead

error: suggestion without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:812:1
   |
LL | / #[suggestion(no_crate_example, code = "")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct PrimarySpanOnVec {
LL | |     #[primary_span]
...  |
LL | |     sub: Vec<Span>,
LL | | }

error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:94:9
   |
   |
LL | #[label("...")]
   |         ^^^^^ maybe a missing crate `core`?
error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:133:9
   |
   |
LL | #[label()]
   |         ^ maybe a missing crate `core`?
error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:309:1
   |
LL | union AC {
LL | union AC {
   | ^^^^^ maybe a missing crate `core`?

error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:555:23
   |
LL |     #[suggestion_part()]
   |                       ^ maybe a missing crate `core`?
error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:575:23
   |
   |
LL |     #[suggestion_part()]
   |                       ^ maybe a missing crate `core`?
error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:578:27
   |
   |
LL |     #[suggestion_part(foo = "bar")]
   |                           ^ maybe a missing crate `core`?
error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:670:28
   |
   |
LL |     #[suggestion_part(code("foo"))]
   |                            ^^^^^ maybe a missing crate `core`?
error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:680:28
   |
   |
LL |     #[suggestion_part(code("foo", "bar"))]
   |                            ^^^^^ maybe a missing crate `core`?
error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:690:28
   |
   |
LL |     #[suggestion_part(code(3))]
   |                            ^ maybe a missing crate `core`?
error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:712:30
   |
   |
LL |     #[suggestion_part(code = 3)]
   |                              ^ maybe a missing crate `core`?
error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:803:48
   |
   |
LL | #[suggestion(no_crate_example, code = "", style("foo"))]
   |                                                ^ maybe a missing crate `core`?
error: cannot find attribute `foo` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:65:3
   |
LL | #[foo]
---

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:186:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:198:7
   |
   |
LL |     #[bar = 4]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:210:7
   |
   |
LL |     #[bar("...")]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:271:7
   |
   |
LL |     #[bar]
   |       ^^^

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:282:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:293:7
   |
   |
LL |     #[bar("...")]


error[E0425]: cannot find value `slug` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:123:9
   |
LL | #[label(slug)]
   |         ^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `__code_29` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:706:10
LL | #[derive(Subdiagnostic)]
   |          ^^^^^^^^^^^^^ not found in this scope
   |
   = note: this error originates in the derive macro `Subdiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
---
---- [ui] tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs stdout ----
diff of stderr:

20    |
21    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
22 
- error: expected parentheses: #[diag(...)]
-   --> $DIR/diagnostic-derive.rs:52:8
-    |
- LL | #[diag = "E0123"]
- 
- 
29 error: `#[nonsense(...)]` is not a valid attribute
31    |

484    |
484    |
485    = help: consider creating a `Subdiagnostic` instead
486 
- error: unexpected end of input, unexpected token in nested attribute, expected ident
-   --> $DIR/diagnostic-derive.rs:642:24
-    |
- LL | #[multipart_suggestion()]
- 
- 
493 error: `#[suggestion(...)]` is not a valid attribute
495    |


550 LL |     #[suggestion(code(foo))]
552 
- error: unexpected token
-   --> $DIR/diagnostic-derive.rs:795:23
-    |
-    |
- LL |     #[suggestion(code(foo))]
- 
- error: expected string literal
-   --> $DIR/diagnostic-derive.rs:804:25
-    |
-    |
- LL |     #[suggestion(code = 3)]
- 
- 
565 error: `#[suggestion(...)]` is not a valid attribute
567    |


572    = help: to show a suggestion consisting of multiple parts, use a `Subdiagnostic` annotated with `#[multipart_suggestion(...)]`
573    = help: to show a variable set of suggestions, use a `Vec` of `Subdiagnostic`s annotated with `#[suggestion(...)]`
574 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL | #[diag = "E0123"]
+    |        ^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL | #[multipart_suggestion()]
+    |                        ^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL |     #[suggestion(code(foo))]
+    |                       ^^^ maybe a missing crate `core`?
+ 
+ error[E0433]: failed to resolve: maybe a missing crate `core`?
+    |
+    |
+ LL |     #[suggestion(code = 3)]
+    |                         ^ maybe a missing crate `core`?
575 error: cannot find attribute `nonsense` in this scope
576   --> $DIR/diagnostic-derive.rs:57:3
577    |

---
To only update this specific test, also pass `--test-args session-diagnostic/diagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: unsupported type attribute for diagnostic derive enum
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:41:1
   |
LL | #[diag(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:44:5
LL |     Foo,
   |     ^^^
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:46:5
LL |     Bar,
   |     ^^^
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: `#[nonsense(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:57:1
   |
LL | #[nonsense(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:57:1
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
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:64:1
   |
LL | / #[diag("E0123")]
LL | | //~^ ERROR diagnostic slug not specified
LL | | struct InvalidLitNestedAttr {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug must be the first argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:74:16
   |
LL | #[diag(nonsense("foo"), code = "E0123", slug = "foo")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:74:1
   |
LL | / #[diag(nonsense("foo"), code = "E0123", slug = "foo")]
LL | | //~^ ERROR diagnostic slug must be the first argument
LL | | //~| ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr1 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
error: unknown argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:80:8
   |
   |
LL | #[diag(nonsense = "...", code = "E0123", slug = "foo")]
   |
   |
   = note: only the `code` parameter is valid after the slug

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:80:1
   |
LL | / #[diag(nonsense = "...", code = "E0123", slug = "foo")]
LL | | //~^ ERROR unknown argument
LL | | //~| ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr2 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
error: unknown argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:86:8
   |
   |
LL | #[diag(nonsense = 4, code = "E0123", slug = "foo")]
   |
   |
   = note: only the `code` parameter is valid after the slug

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:86:1
   |
LL | / #[diag(nonsense = 4, code = "E0123", slug = "foo")]
LL | | //~^ ERROR unknown argument
LL | | //~| ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr3 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
error: unknown argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:92:42
   |
   |
LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
   |
   |
   = note: only the `code` parameter is valid after the slug

error: `#[suggestion = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:99:5
   |
LL |     #[suggestion = "bar"]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:106:8
   |
   |
LL | #[diag(no_crate_example, code = "E0456")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:105:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:106:26
   |
   |
LL | #[diag(no_crate_example, code = "E0456")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:105:26
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:112:42
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:112:26
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]


error: diagnostic slug must be the first argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:117:43
   |
LL | #[diag(no_crate_example, no_crate::example, code = "E0456")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:122:1
   |
LL | struct KindNotProvided {} //~ ERROR diagnostic slug not specified
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:125:1
   |
LL | / #[diag(code = "E0456")]
LL | | //~^ ERROR diagnostic slug not specified
LL | | struct SlugNotProvided {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:136:5
   |
LL |     #[primary_span]


error: `#[nonsense]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:144:5
   |
LL |     #[nonsense]


error: the `#[label(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:161:5
   |
LL |     #[label(no_crate_label)]

error: `name` doesn't refer to a field on this type
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:169:46
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "{name}")]


error: invalid format string: expected `'}'` but string was terminated
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:174:10
LL | #[derive(Diagnostic)]
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ expected `'}'` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: invalid format string: unmatched `}` found
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:184:10
LL | #[derive(Diagnostic)]
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ unmatched `}` in format string
   |
   = note: if you intended to print `}`, you can escape it using `}}`
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: the `#[label(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:204:5
   |
LL |     #[label(no_crate_label)]


error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:223:5
   |
LL |     #[suggestion(no_crate_suggestion)]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:231:18
   |
   |
LL |     #[suggestion(nonsense = "bar")]
   |
   |
   = help: only `style`, `code` and `applicability` are valid nested attributes

error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:231:5
   |
LL |     #[suggestion(nonsense = "bar")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:240:18
   |
   |
LL |     #[suggestion(msg = "bar")]
   |
   |
   = help: only `style`, `code` and `applicability` are valid nested attributes

error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:240:5
   |
LL |     #[suggestion(msg = "bar")]

error: wrong field type for suggestion
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:263:5
   |
   |
LL | /     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
LL | |     //~^ ERROR wrong field type for suggestion
LL | |     suggestion: Applicability,
   |
   |
   = help: `#[suggestion(...)]` should be applied to fields of type `Span` or `(Span, Applicability)`
error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:279:24
   |
   |
LL |     suggestion: (Span, Span, Applicability),
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:279:18
   |
   |
LL |     suggestion: (Span, Span, Applicability),

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:287:33
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:287:18
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),


error: `#[label = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:294:5
   |
LL |     #[label = "bar"]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:445:5
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "maybe-incorrect")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:447:24
   |
   |
LL |     suggestion: (Span, Applicability),

error: invalid applicability
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:453:69
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "batman")]


error: the `#[help(...)]` attribute can only be applied to fields of type `Span`, `bool` or `()`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:520:5
   |
LL |     #[help(no_crate_help)]


error: a diagnostic slug must be the first argument to the attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:529:32
   |
LL |     #[label(no_crate_label, foo)]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:537:29
   |
   |
LL |     #[label(no_crate_label, foo = "...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:545:29
   |
   |
LL |     #[label(no_crate_label, foo("..."))]


error: `#[primary_span]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:557:5
   |
LL |     #[primary_span]
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: `#[error(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:577:1
   |
LL | #[error(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:577:1
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
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:584:1
   |
LL | #[warn_(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:584:1
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
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:591:1
   |
LL | #[lint(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:591:1
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
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:598:1
   |
LL | #[lint(no_crate_example, code = "E0123")]


error: `#[lint(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:598:1
   |
LL | #[lint(no_crate_example, code = "E0123")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:598:1
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
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:608:53
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:608:39
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]

error: wrong types for suggestion
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:617:24
   |
   |
LL |     suggestion: (Span, usize),
   |                        ^^^^^
   |
   = help: `#[suggestion(...)]` on a tuple field must be applied to fields of type `(Span, Applicability)`
error: wrong types for suggestion
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:625:17
   |
   |
LL |     suggestion: (Span,),
   |
   |
   = help: `#[suggestion(...)]` on a tuple field must be applied to fields of type `(Span, Applicability)`

error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:632:5
   |
LL |     #[suggestion(no_crate_suggestion)]


error: `#[multipart_suggestion(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:639:1
LL | #[multipart_suggestion(no_crate_suggestion)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = help: consider creating a `Subdiagnostic` instead

error: `#[multipart_suggestion(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:646:5
   |
LL |     #[multipart_suggestion(no_crate_suggestion)]
   |
   |
   = help: consider creating a `Subdiagnostic` instead

error: `#[suggestion(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:654:1
LL | #[suggestion(no_crate_suggestion, code = "...")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = help: `#[label]` and `#[suggestion]` can only be applied to fields

error: `#[label]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:663:1
LL | #[label]
   | ^^^^^^^^
   |
---

error: cannot find attribute `multipart_suggestion` in this scope
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:646:7
   |
LL |     #[multipart_suggestion(no_crate_suggestion)]

error[E0425]: cannot find value `nonsense` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:69:8
   |
   |
LL | #[diag(nonsense, code = "E0123")]
   |        ^^^^^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `__code_34` in this scope
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:801:10
   |
LL | #[derive(Diagnostic)] //~ ERROR cannot find value `__code_34` in this scope
   |
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the trait bound `Hello: IntoDiagnosticArg` is not satisfied
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:338:10
LL | #[derive(Diagnostic)]
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ the trait `IntoDiagnosticArg` is not implemented for `Hello`
   |
   = help: the following other types implement trait `IntoDiagnosticArg`:
             &'a T
             &'a std::path::Path
             &'a str
             &rustc_target::spec::TargetTriple
             Binder<'_, ExistentialTraitRef<'_>>
             Binder<'_, TraitRef<'_>>
             CString
             CguReuse
           and 49 others
note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
  --> /rustc/FAKE_PREFIX/compiler/rustc_errors/src/diagnostic_builder.rs:747:5
   = note: this error originates in the derive macro `Diagnostic` which comes from the expansion of the macro `forward` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 84 previous errors

Some errors have detailed explanations: E0277, E0425, E0433.
For more information about an error, try `rustc --explain E0277`.
