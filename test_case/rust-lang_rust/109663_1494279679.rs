plain
Uplifting rustc (stage1 -> stage3)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
.....................................................i.....F.F.........
failures:

---- [ui] tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs stdout ----
diff of stderr:
diff of stderr:

26 LL | #[label = "..."]
28 
28 
- error: `#[label(bug = ...)]` is not a valid attribute
+ error: invalid nested attribute
31    |
31    |
32 LL | #[label(bug = "...")]
-    |         ^^^^^^^^^^^
+    |         ^^^
34 
34 
- error: diagnostic slug must be first argument of a `#[label(...)]` attribute
-   --> $DIR/subdiagnostic-derive.rs:84:1
+ error: expected `,`
37    |
37    |
38 LL | #[label(bug = "...")]
+    |             ^
40 
40 
- error: `#[label("...")]` is not a valid attribute
+ error: unexpected literal in nested attribute, expected ident
43    |
43    |
44 LL | #[label("...")]
45    |         ^^^^^
46 
46 
- error: diagnostic slug must be first argument of a `#[label(...)]` attribute
-   --> $DIR/subdiagnostic-derive.rs:94:1
-    |
- LL | #[label("...")]
- 
- 
- error: `#[label(slug = ...)]` is not a valid attribute
+ error: invalid nested attribute
55    |
55    |
56 LL | #[label(slug = 4)]
-    |         ^^^^^^^^
+    |         ^^^^
58 
58 
- error: diagnostic slug must be first argument of a `#[label(...)]` attribute
-   --> $DIR/subdiagnostic-derive.rs:104:1
+ error: expected `,`
61    |
61    |
62 LL | #[label(slug = 4)]
+    |              ^
64 
64 
- error: `#[label(slug(...))]` is not a valid attribute
+ error: invalid nested attribute
67    |
67    |
68 LL | #[label(slug("..."))]
-    |         ^^^^^^^^^^^
+    |         ^^^^
70 
70 
- error: diagnostic slug must be first argument of a `#[label(...)]` attribute
-   --> $DIR/subdiagnostic-derive.rs:114:1
+ error: expected `,`
73    |
73    |
74 LL | #[label(slug("..."))]
+    |             ^
76 
76 
- error: diagnostic slug must be first argument of a `#[label(...)]` attribute
-   --> $DIR/subdiagnostic-derive.rs:134:1
+ error: unexpected end of input, unexpected token in nested attribute, expected ident
79    |
79    |
80 LL | #[label()]
+    |         ^
82 
82 
- error: `#[label(code = ...)]` is not a valid attribute
+ error: invalid nested attribute
85    |
85    |
86 LL | #[label(no_crate_example, code = "...")]
-    |                           ^^^^^^^^^^^^
+    |                           ^^^^
88 
88 
- error: `#[label(applicability = ...)]` is not a valid attribute
+ error: expected `,`
+    |
+    |
+ LL | #[label(no_crate_example, code = "...")]
+ 
+ error: invalid nested attribute
90   --> $DIR/subdiagnostic-derive.rs:152:27
91    |
91    |
92 LL | #[label(no_crate_example, applicability = "machine-applicable")]
-    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |                           ^^^^^^^^^^^^^
94 
+ error: expected `,`
+ error: expected `,`
+   --> $DIR/subdiagnostic-derive.rs:152:41
+    |
+ LL | #[label(no_crate_example, applicability = "machine-applicable")]
+ 
95 error: unsupported type attribute for subdiagnostic enum
96   --> $DIR/subdiagnostic-derive.rs:161:1
97    |
97    |

122 LL |     #[bar("...")]
124 
124 
- error: `#[label(code = ...)]` is not a valid attribute
+ error: invalid nested attribute
127    |
127    |
128 LL |     #[label(code = "...")]
-    |             ^^^^^^^^^^^^
+    |             ^^^^
130 
130 
- error: diagnostic slug must be first argument of a `#[label(...)]` attribute
-   --> $DIR/subdiagnostic-derive.rs:223:5
+ error: expected `,`
133    |
133    |
134 LL |     #[label(code = "...")]
+    |                  ^
136 
136 
137 error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`

190 LL | | }
191    | |_^
192 
192 
- error: `#[label(no_crate::example)]` is not a valid attribute
-   --> $DIR/subdiagnostic-derive.rs:325:27
+ error: a diagnostic slug must be the first argument to the attribute
195    |
195    |
196 LL | #[label(no_crate_example, no_crate::example)]
-    |
-    |
-    = help: a diagnostic slug must be the first argument to the attribute
200 
201 error: specified multiple times
202   --> $DIR/subdiagnostic-derive.rs:338:5


217    |        ^^
218 
219 error: specified multiple times
-   --> $DIR/subdiagnostic-derive.rs:381:46
+   --> $DIR/subdiagnostic-derive.rs:381:1
221    |
222 LL | #[suggestion(no_crate_example, code = "...", code = "...")]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
224    |
225 note: previously specified here
-   --> $DIR/subdiagnostic-derive.rs:381:32
-   --> $DIR/subdiagnostic-derive.rs:381:32
+   --> $DIR/subdiagnostic-derive.rs:381:1
227    |
228 LL | #[suggestion(no_crate_example, code = "...", code = "...")]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
230 
231 error: specified multiple times
232   --> $DIR/subdiagnostic-derive.rs:399:5
232   --> $DIR/subdiagnostic-derive.rs:399:5

253    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
254 
255 error: invalid applicability
-   --> $DIR/subdiagnostic-derive.rs:432:46
+   --> $DIR/subdiagnostic-derive.rs:432:62
257    |
258 LL | #[suggestion(no_crate_example, code = "...", applicability = "foo")]
+    |                                                              ^^^^^
260 
260 
261 error: suggestion without `#[primary_span]` field

314 LL | | }
315    | |_^
316 
316 
- error: `#[multipart_suggestion(code = ...)]` is not a valid attribute
+ error: invalid nested attribute
319    |
319    |
320 LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
-    |                                          ^^^^^^^^^^^^
+    |                                          ^^^^
322    |
322    |
323    = help: only `style` and `applicability` are valid nested attributes


- error: multipart suggestion without any `#[suggestion_part(...)]` fields
-   --> $DIR/subdiagnostic-derive.rs:538:1
+ error: expected `,`
327    |
327    |
- LL | / #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
- LL | |
- LL | |
- LL | | struct BBa {
- LL | |     var: String,
- LL | | }
-    | |_^
+ LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
335 
335 
336 error: `#[suggestion_part(...)]` attribute without `code = "..."`

339 LL |     #[suggestion_part]
340    |     ^^^^^^^^^^^^^^^^^^
341 
341 
- error: `#[suggestion_part(...)]` attribute without `code = "..."`
-   --> $DIR/subdiagnostic-derive.rs:556:5
+ error: unexpected end of input, unexpected token in nested attribute, expected ident
344    |
344    |
345 LL |     #[suggestion_part()]
+    |                       ^
347 
347 
348 error: `#[primary_span]` is not a valid attribute

371 LL |     #[suggestion_part]
372    |     ^^^^^^^^^^^^^^^^^^
373 
373 
- error: `#[suggestion_part(...)]` attribute without `code = "..."`
-   --> $DIR/subdiagnostic-derive.rs:576:5
-    |
- LL |     #[suggestion_part()]
- 
- 
- error: `#[suggestion_part(foo = ...)]` is not a valid attribute
+ error: `code` is the only valid nested attribute
382    |
382    |
383 LL |     #[suggestion_part(foo = "bar")]
-    |                       ^^^^^^^^^^^
-    |
-    |
-    = help: `code` is the only valid nested attribute
387 
387 
388 error: the `#[suggestion_part(...)]` attribute can only be applied to fields of type `Span` or `MultiSpan`


397 LL |     #[suggestion_part()]
399 
399 
+ error: unexpected end of input, unexpected token in nested attribute, expected ident
+    |
+    |
+ LL |     #[suggestion_part()]
+ 
+ error: expected `,`
+   --> $DIR/subdiagnostic-derive.rs:579:27
+    |
+    |
+ LL |     #[suggestion_part(foo = "bar")]
+ 
400 error: specified multiple times
-   --> $DIR/subdiagnostic-derive.rs:593:37
+   --> $DIR/subdiagnostic-derive.rs:593:5
+   --> $DIR/subdiagnostic-derive.rs:593:5
402    |
403 LL |     #[suggestion_part(code = "...", code = ",,,")]
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
405    |
406 note: previously specified here
-   --> $DIR/subdiagnostic-derive.rs:593:23
-   --> $DIR/subdiagnostic-derive.rs:593:23
+   --> $DIR/subdiagnostic-derive.rs:593:5
408    |
409 LL |     #[suggestion_part(code = "...", code = ",,,")]
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
411 
411 
412 error: `#[applicability]` has no effect if all `#[suggestion]`/`#[multipart_suggestion]` attributes have a static `applicability = "..."`

416    |     ^^^^^^^^^^^^^^^^
417 
417 
418 error: expected exactly one string literal for `code = ...`
-   --> $DIR/subdiagnostic-derive.rs:670:23
420    |
420    |
421 LL |     #[suggestion_part(code("foo"))]
+    |                                  ^
423 
+ error: unexpected token
+   --> $DIR/subdiagnostic-derive.rs:670:28
+   --> $DIR/subdiagnostic-derive.rs:670:28
+    |
+ LL |     #[suggestion_part(code("foo"))]
+ 
+ 
424 error: expected exactly one string literal for `code = ...`
-   --> $DIR/subdiagnostic-derive.rs:679:23
426    |
426    |
427 LL |     #[suggestion_part(code("foo", "bar"))]
+    |                                         ^
429 
+ error: unexpected token
+   --> $DIR/subdiagnostic-derive.rs:679:28
+   --> $DIR/subdiagnostic-derive.rs:679:28
+    |
+ LL |     #[suggestion_part(code("foo", "bar"))]
+ 
+ 
430 error: expected exactly one string literal for `code = ...`
-   --> $DIR/subdiagnostic-derive.rs:688:23
432    |
432    |
433 LL |     #[suggestion_part(code(3))]
+    |                              ^
435 
+ error: unexpected token
+   --> $DIR/subdiagnostic-derive.rs:688:28
+   --> $DIR/subdiagnostic-derive.rs:688:28
+    |
+ LL |     #[suggestion_part(code(3))]
+ 
+ 
436 error: expected exactly one string literal for `code = ...`
-   --> $DIR/subdiagnostic-derive.rs:697:23
438    |
438    |
439 LL |     #[suggestion_part(code())]
+    |                             ^
441 
441 
- error: `code = "..."`/`code(...)` must contain only string literals
-   --> $DIR/subdiagnostic-derive.rs:706:23
+ error: expected string literal
444    |
444    |
445 LL |     #[suggestion_part(code = 3)]
+    |                              ^
447 
448 error: specified multiple times
-   --> $DIR/subdiagnostic-derive.rs:748:61
-   --> $DIR/subdiagnostic-derive.rs:748:61
+   --> $DIR/subdiagnostic-derive.rs:748:1
450    |
451 LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
453    |
454 note: previously specified here
-   --> $DIR/subdiagnostic-derive.rs:748:43
-   --> $DIR/subdiagnostic-derive.rs:748:43
+   --> $DIR/subdiagnostic-derive.rs:748:1
456    |
457 LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
459 
459 
460 error: `#[suggestion_hidden(...)]` is not a valid attribute

481    |
481    |
482    = help: valid styles are `normal`, `short`, `hidden`, `verbose` and `tool-only`
483 
- error: `#[suggestion(style = ...)]` is not a valid attribute
-   --> $DIR/subdiagnostic-derive.rs:781:43
+ error: expected `= "xxx"`
486    |
486    |
487 LL | #[suggestion(no_crate_example, code = "", style = 42)]
+    |                                                 ^
489 
489 
- error: `#[suggestion(style)]` is not a valid attribute
-   --> $DIR/subdiagnostic-derive.rs:789:43
+ error: a diagnostic slug must be the first argument to the attribute
492    |
492    |
493 LL | #[suggestion(no_crate_example, code = "", style)]
+    |                                                ^
+ 
+ 
+ error: expected `= "xxx"`
495    |
495    |
-    = help: a diagnostic slug must be the first argument to the attribute
+ LL | #[suggestion(no_crate_example, code = "", style("foo"))]
497 
497 
- error: `#[suggestion(style(...))]` is not a valid attribute
-   --> $DIR/subdiagnostic-derive.rs:797:43
+ error: expected `,`
500    |
500    |
501 LL | #[suggestion(no_crate_example, code = "", style("foo"))]
+    |                                                ^
503 
503 
504 error: `#[primary_span]` is not a valid attribute


582 LL | #[label(slug)]
583    |         ^^^^ not found in `crate::fluent_generated`
- error: aborting due to 81 previous errors
- error: aborting due to 81 previous errors
+ error[E0425]: cannot find value `__code_29` in this scope
+    |
+ LL | #[derive(Subdiagnostic)]
+    |          ^^^^^^^^^^^^^ not found in this scope
+    |
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

error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:84:13
   |
   |
LL | #[label(bug = "...")]


error: unexpected literal in nested attribute, expected ident
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:94:9
   |
LL | #[label("...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:104:9
   |
   |
LL | #[label(slug = 4)]

error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:104:14
   |
   |
LL | #[label(slug = 4)]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:114:9
   |
   |
LL | #[label(slug("..."))]

error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:114:13
   |
   |
LL | #[label(slug("..."))]


error: unexpected end of input, unexpected token in nested attribute, expected ident
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:134:9
   |
LL | #[label()]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:143:27
   |
   |
LL | #[label(no_crate_example, code = "...")]

error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:143:32
   |
   |
LL | #[label(no_crate_example, code = "...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:152:27
   |
   |
LL | #[label(no_crate_example, applicability = "machine-applicable")]

error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:152:41
   |
   |
LL | #[label(no_crate_example, applicability = "machine-applicable")]

error: unsupported type attribute for subdiagnostic enum
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:161:1
   |
   |
LL | #[foo]
   | ^^^^^^

error: `#[bar]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:175:5
LL |     #[bar]
   |     ^^^^^^


error: `#[bar = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:187:5
   |
LL |     #[bar = "..."]


error: `#[bar = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:199:5
   |
LL |     #[bar = 4]


error: `#[bar(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:211:5
   |
LL |     #[bar("...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:223:13
   |
   |
LL |     #[label(code = "...")]

error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:223:18
   |
   |
LL |     #[label(code = "...")]


error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:252:5
   |
LL |     #[primary_span]


error: label without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:249:1
   |
LL | / #[label(no_crate_example)]
LL | | //~^ ERROR label without `#[primary_span]` field
LL | | struct W {
LL | |     #[primary_span]
LL | |     //~^ ERROR the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`
LL | |     span: String,
LL | | }


error: `#[applicability]` is only valid on suggestions
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:262:5
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: `#[bar]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:272:5
LL |     #[bar]
   |     ^^^^^^
   |
   |
   = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes

error: `#[bar = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:283:5
   |
LL |     #[bar = "..."]


error: `#[bar(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:294:5
   |
LL |     #[bar("...")]
   |
   |
   = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes
error: unexpected unsupported untagged union
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:310:1
   |
   |
LL | / union AC {
LL | |     //~^ ERROR unexpected unsupported untagged union
LL | |     span: u32,
LL | |     b: u64,
LL | | }


error: a diagnostic slug must be the first argument to the attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:325:44
   |
LL | #[label(no_crate_example, no_crate::example)]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:338:5
   |
   |
LL |     #[primary_span]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:335:5
   |
   |
LL |     #[primary_span]


error: subdiagnostic kind not specified
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:344:8
LL | struct AG {
   |        ^^

error: specified multiple times
error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:381:1
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:381:1
   |
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:399:5
   |
---
   |
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^

error: the `#[applicability]` attribute can only be applied to fields of type `Applicability`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:409:5
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^


error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:422:1
   |
LL | #[suggestion(no_crate_example)]

error: invalid applicability
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:432:62
   |
   |
LL | #[suggestion(no_crate_example, code = "...", applicability = "foo")]


error: suggestion without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:450:1
   |
LL | / #[suggestion(no_crate_example, code = "...")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct AR {
LL | |     var: String,
LL | | }

error: unsupported type attribute for subdiagnostic enum
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:464:1
   |
   |
LL | #[label]
   | ^^^^^^^^

error: `var` doesn't refer to a field on this type
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:484:39
   |
LL | #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]


error: `var` doesn't refer to a field on this type
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:503:43
   |
LL |     #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]


error: `#[suggestion_part]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:526:5
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^
   |
   |
   = help: `#[suggestion_part(...)]` is only valid in multipart suggestions, use `#[primary_span]` instead

error: `#[suggestion_part(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:529:5
   |
LL |     #[suggestion_part(code = "...")]
   |
   |
   = help: `#[suggestion_part(...)]` is only valid in multipart suggestions

error: suggestion without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:523:1
   |
LL | / #[suggestion(no_crate_example, code = "...")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct BA {
LL | |     #[suggestion_part]
LL | |     var: String,
LL | | }
   | |_^


error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:538:42
   |
LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
   |
   |
   = help: only `style` and `applicability` are valid nested attributes
error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:538:47
   |
   |
LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]


error: `#[suggestion_part(...)]` attribute without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:548:5
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: unexpected end of input, unexpected token in nested attribute, expected ident
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:556:23
   |
LL |     #[suggestion_part()]


error: `#[primary_span]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:565:5
   |
LL |     #[primary_span]
   |
   |
   = help: multipart suggestions use one or more `#[suggestion_part]`s rather than one `#[primary_span]`

error: multipart suggestion without any `#[suggestion_part(...)]` fields
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:562:1
   |
LL | / #[multipart_suggestion(no_crate_example)]
LL | | //~^ ERROR multipart suggestion without any `#[suggestion_part(...)]` fields
LL | | struct BC {
LL | |     #[primary_span]
LL | |     //~^ ERROR `#[primary_span]` is not a valid attribute
LL | |     span: Span,
LL | | }


error: `#[suggestion_part(...)]` attribute without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:573:5
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: `code` is the only valid nested attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:579:23
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


error: unexpected end of input, unexpected token in nested attribute, expected ident
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:576:23
   |
LL |     #[suggestion_part()]

error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:579:27
   |
   |
LL |     #[suggestion_part(foo = "bar")]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:593:5
   |
   |
LL |     #[suggestion_part(code = "...", code = ",,,")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:593:5
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

error: unexpected token
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:670:28
   |
   |
LL |     #[suggestion_part(code("foo"))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:679:41
   |
LL |     #[suggestion_part(code("foo", "bar"))]

error: unexpected token
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:679:28
   |
   |
LL |     #[suggestion_part(code("foo", "bar"))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:688:30
   |
LL |     #[suggestion_part(code(3))]

error: unexpected token
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:688:28
   |
   |
LL |     #[suggestion_part(code(3))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:697:29
   |
LL |     #[suggestion_part(code())]

error: expected string literal
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:706:30
   |
   |
LL |     #[suggestion_part(code = 3)]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:748:1
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:748:1
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]


error: `#[suggestion_hidden(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:757:1
   |
LL | #[suggestion_hidden(no_crate_example, code = "")]
   |
   |
   = help: Use `#[suggestion(..., style = "hidden")]` instead

error: `#[suggestion_hidden(...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:765:1
   |
LL | #[suggestion_hidden(no_crate_example, code = "", style = "normal")]
   |
   |
   = help: Use `#[suggestion(..., style = "hidden")]` instead
error: invalid suggestion style
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:773:51
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "foo")]
   |
   |
   = help: valid styles are `normal`, `short`, `hidden`, `verbose` and `tool-only`

error: expected `= "xxx"`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:781:49
   |
LL | #[suggestion(no_crate_example, code = "", style = 42)]


error: a diagnostic slug must be the first argument to the attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:789:48
   |
LL | #[suggestion(no_crate_example, code = "", style)]


error: expected `= "xxx"`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:797:48
   |
LL | #[suggestion(no_crate_example, code = "", style("foo"))]

error: expected `,`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:797:48
   |
   |
LL | #[suggestion(no_crate_example, code = "", style("foo"))]


error: `#[primary_span]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:808:5
   |
LL |     #[primary_span]
   |
   = note: there must be exactly one primary span
   = note: there must be exactly one primary span
   = help: to create a suggestion with multiple spans, use `#[multipart_suggestion]` instead

error: suggestion without `#[primary_span]` field
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:805:1
   |
LL | / #[suggestion(no_crate_example, code = "")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct PrimarySpanOnVec {
LL | |     #[primary_span]
...  |
LL | |     sub: Vec<Span>,
LL | | }

error: cannot find attribute `foo` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:65:3
   |
---

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:187:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:199:7
   |
   |
LL |     #[bar = 4]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:211:7
   |
   |
LL |     #[bar("...")]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:272:7
   |
   |
LL |     #[bar]
   |       ^^^

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:283:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:294:7
   |
   |
LL |     #[bar("...")]


error[E0425]: cannot find value `slug` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:124:9
   |
LL | #[label(slug)]
   |         ^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `__code_29` in this scope
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:703:10
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
- error: `#[diag = ...]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:52:1
+ error: expected parentheses: #[diag(...)]
25    |
25    |
26 LL | #[diag = "E0123"]
+    |        ^
28 
28 
29 error: `#[nonsense(...)]` is not a valid attribute

44    |
44    |
45    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
46 
- error: `#[diag("...")]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:64:8
-    |
- LL | #[diag("E0123")]
-    |
-    = help: a diagnostic slug is required as the first argument
- 
- 
55 error: diagnostic slug not specified
57    |

63    |
63    |
64    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
65 
- error: `#[diag(nonsense(...))]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:75:8
+ error: diagnostic slug must be the first argument
68    |
68    |
69 LL | #[diag(nonsense("foo"), code = "E0123", slug = "foo")]
-    |
-    = help: a diagnostic slug is required as the first argument
+    |                ^
73 
73 
74 error: diagnostic slug not specified

82    |
82    |
83    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
84 
- error: `#[diag(nonsense = ...)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:81:8
-    |
- LL | #[diag(nonsense = "...", code = "E0123", slug = "foo")]
-    |
-    |
-    = help: only `code` is a valid nested attributes following the slug
- 
- error: `#[diag(slug = ...)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:81:42
-    |
- LL | #[diag(nonsense = "...", code = "E0123", slug = "foo")]
-    |
-    |
-    = help: only `code` is a valid nested attributes following the slug
- 
101 error: diagnostic slug not specified
103    |

110    |
110    |
111    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
112 
- error: `#[diag(nonsense = ...)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:88:8
-    |
- LL | #[diag(nonsense = 4, code = "E0123", slug = "foo")]
- 
- 
- error: `#[diag(slug = ...)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:88:38
-    |
- LL | #[diag(nonsense = 4, code = "E0123", slug = "foo")]
-    |
-    |
-    = help: only `code` is a valid nested attributes following the slug
- 
127 error: diagnostic slug not specified
129    |

136    |
136    |
137    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
138 
- error: `#[diag(slug = ...)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:95:42
-    |
- LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
-    |
-    |
-    = help: only `code` is a valid nested attributes following the slug
- 
147 error: `#[suggestion = ...]` is not a valid attribute
149    |

163    |        ^^^^^^^^^^^^^^^^
164 
164 
165 error: specified multiple times
-   --> $DIR/diagnostic-derive.rs:109:33
+   --> $DIR/diagnostic-derive.rs:109:26
167    |
168 LL | #[diag(no_crate_example, code = "E0456")]
+    |                          ^^^^
170    |
171 note: previously specified here
-   --> $DIR/diagnostic-derive.rs:108:33
-   --> $DIR/diagnostic-derive.rs:108:33
+   --> $DIR/diagnostic-derive.rs:108:26
173    |
174 LL | #[diag(no_crate_example, code = "E0123")]
+    |                          ^^^^
176 
- error: specified multiple times
-   --> $DIR/diagnostic-derive.rs:115:49
-   --> $DIR/diagnostic-derive.rs:115:49
+ error: diagnostic slug must be the first argument
179    |
179    |
- LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
-    |
- note: previously specified here
-   --> $DIR/diagnostic-derive.rs:115:33
-    |
-    |
- LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
- 
- 
- error: `#[diag(no_crate::example)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:120:26
-    |
192 LL | #[diag(no_crate_example, no_crate::example, code = "E0456")]
-    |
-    |
-    = help: diagnostic slug must be the first argument
196 
196 
197 error: diagnostic slug not specified


266 LL |     #[suggestion(no_crate_suggestion)]
268 
268 
- error: `#[suggestion(nonsense = ...)]` is not a valid attribute
+ error: invalid nested attribute
271    |
271    |
272 LL |     #[suggestion(nonsense = "bar")]
-    |                  ^^^^^^^^^^^^^^^^
+    |                  ^^^^^^^^
274    |
274    |
275    = help: only `style`, `code` and `applicability` are valid nested attributes


- error: suggestion without `code = "..."`
-   --> $DIR/diagnostic-derive.rs:234:5
+ error: expected `,`
279    |
279    |
280 LL |     #[suggestion(nonsense = "bar")]
+    |                           ^
282 
282 
- error: `#[suggestion(msg = ...)]` is not a valid attribute
+ error: invalid nested attribute
285    |
285    |
286 LL |     #[suggestion(msg = "bar")]
-    |                  ^^^^^^^^^^^
+    |                  ^^^
288    |
288    |
289    = help: only `style`, `code` and `applicability` are valid nested attributes


- error: suggestion without `code = "..."`
-   --> $DIR/diagnostic-derive.rs:243:5
+ error: expected `,`
293    |
293    |
294 LL |     #[suggestion(msg = "bar")]
+    |                      ^
296 
297 error: wrong field type for suggestion
298   --> $DIR/diagnostic-derive.rs:266:5
298   --> $DIR/diagnostic-derive.rs:266:5

335    |     ^^^^^^^^^^^^^^^^
336 
337 error: specified multiple times
-   --> $DIR/diagnostic-derive.rs:448:53
+   --> $DIR/diagnostic-derive.rs:448:5
339    |
340 LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "maybe-incorrect")]
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
342    |
343 note: previously specified here
344   --> $DIR/diagnostic-derive.rs:450:24
344   --> $DIR/diagnostic-derive.rs:450:24

347    |                        ^^^^^^^^^^^^^
348 
349 error: invalid applicability
-   --> $DIR/diagnostic-derive.rs:456:53
+   --> $DIR/diagnostic-derive.rs:456:69
351    |
352 LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "batman")]
+    |                                                                     ^^^^^^^^
354 
354 
355 error: the `#[help(...)]` attribute can only be applied to fields of type `Span`, `bool` or `()`


358 LL |     #[help(no_crate_help)]
360 
360 
- error: `#[label(foo)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:532:29
+ error: a diagnostic slug must be the first argument to the attribute
363    |
363    |
364 LL |     #[label(no_crate_label, foo)]
-    |
-    |
-    = help: a diagnostic slug must be the first argument to the attribute
368 
368 
- error: `#[label(foo = ...)]` is not a valid attribute
+ error: invalid nested attribute
371    |
371    |
372 LL |     #[label(no_crate_label, foo = "...")]
-    |                             ^^^^^^^^^^^
+    |                             ^^^
374 
374 
- error: `#[label(foo(...))]` is not a valid attribute
+ error: expected `,`
+    |
+    |
+ LL |     #[label(no_crate_label, foo = "...")]
+ 
+ error: invalid nested attribute
376   --> $DIR/diagnostic-derive.rs:548:29
377    |
377    |
378 LL |     #[label(no_crate_label, foo("..."))]
-    |                             ^^^^^^^^^^
+    |                             ^^^
380 
+ error: expected `,`
+ error: expected `,`
+   --> $DIR/diagnostic-derive.rs:548:32
+    |
+ LL |     #[label(no_crate_label, foo("..."))]
+ 
+ 
381 error: `#[primary_span]` is not a valid attribute
383    |


466    = help: specify the slug as the first argument to the attribute, such as `#[diag(compiletest_example)]`
468 error: specified multiple times
-   --> $DIR/diagnostic-derive.rs:611:53
+   --> $DIR/diagnostic-derive.rs:611:5
470    |
470    |
471 LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
473    |
474 note: previously specified here
-   --> $DIR/diagnostic-derive.rs:611:39
-   --> $DIR/diagnostic-derive.rs:611:39
+   --> $DIR/diagnostic-derive.rs:611:5
476    |
477 LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
479 
480 error: wrong types for suggestion
481   --> $DIR/diagnostic-derive.rs:620:24
481   --> $DIR/diagnostic-derive.rs:620:24

508    = help: consider creating a `Subdiagnostic` instead
509 
510 error: `#[multipart_suggestion(...)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:645:1
-    |
- LL | #[multipart_suggestion()]
-    |
-    |
-    = help: consider creating a `Subdiagnostic` instead
- 
- error: `#[multipart_suggestion(...)]` is not a valid attribute
520    |
520    |
521 LL |     #[multipart_suggestion(no_crate_suggestion)]
523    |
523    |
524    = help: consider creating a `Subdiagnostic` instead
525 
+ error: unexpected end of input, unexpected token in nested attribute, expected ident
+    |
+    |
+ LL | #[multipart_suggestion()]
+ 
+ 
526 error: `#[suggestion(...)]` is not a valid attribute
528    |

539    |
539    |
540    = help: `#[label]` and `#[suggestion]` can only be applied to fields
541 
- error: `#[subdiagnostic(...)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:700:5
+ error: `eager` is the only supported nested attribute for `subdiagnostic`
544    |
544    |
545 LL |     #[subdiagnostic(bad)]
-    |
-    |
-    = help: `eager` is the only supported nested attribute for `subdiagnostic`
549 
549 
550 error: `#[subdiagnostic = ...]` is not a valid attribute


553 LL |     #[subdiagnostic = "bad"]
555 
555 
- error: `#[subdiagnostic(...)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:716:5
+ error: `eager` is the only supported nested attribute for `subdiagnostic`
558    |
558    |
559 LL |     #[subdiagnostic(bad, bad)]
-    |
-    |
-    = help: `eager` is the only supported nested attribute for `subdiagnostic`
563 
563 
- error: `#[subdiagnostic(...)]` is not a valid attribute
-   --> $DIR/diagnostic-derive.rs:724:5
+ error: `eager` is the only supported nested attribute for `subdiagnostic`
566    |
566    |
567 LL |     #[subdiagnostic("bad")]
-    |
-    |
-    = help: `eager` is the only supported nested attribute for `subdiagnostic`
571 
571 
572 error: `#[subdiagnostic(...)]` is not a valid attribute

578    = help: eager subdiagnostics are not supported on lints
579 
579 
580 error: expected at least one string literal for `code(...)`
-   --> $DIR/diagnostic-derive.rs:790:18
582    |
582    |
583 LL |     #[suggestion(code())]
+    |                       ^
585 
585 
586 error: `code(...)` must contain only string literals


589 LL |     #[suggestion(code(foo))]
591 
591 
- error: `code = "..."`/`code(...)` must contain only string literals
-   --> $DIR/diagnostic-derive.rs:806:18
+ error: unexpected token
594    |
594    |
+ LL |     #[suggestion(code(foo))]
+ 
+ error: expected string literal
+   --> $DIR/diagnostic-derive.rs:806:25
+    |
+    |
595 LL |     #[suggestion(code = 3)]
+    |                         ^
597 
597 
598 error: `#[suggestion(...)]` is not a valid attribute


665 LL | #[diag(nonsense, code = "E0123")]
666    |        ^^^^^^^^ not found in `crate::fluent_generated`
667 
+ error[E0425]: cannot find value `slug` in this scope
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
+    |
+    |
+ help: you might have meant to introduce a new binding
+    |
+ LL | #[diag(no_crate_example, code = "E0123", let slug = "foo")]
+ 
+ 
+ error[E0425]: cannot find value `code` in this scope
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
+    |
+    |
+ help: you might have meant to introduce a new binding
+    |
+ LL | #[diag(no_crate_example, code = "E0456", let code = "E0457")]
+ 
+ 
+ error[E0425]: cannot find value `__code_34` in this scope
+    |
+ LL | #[derive(Diagnostic)]
+    |          ^^^^^^^^^^ not found in this scope
+    |
+    |
+    = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ error[E0061]: this enum variant takes 1 argument but 2 arguments were supplied
+   --> $DIR/diagnostic-derive.rs:94:10
+    |
+ LL | #[derive(Diagnostic)]
+    |          ^^^^^^^^^^ unexpected argument
+ LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
+    |                                 -------- help: try using a conversion method: `.to_string()`
+    |                                 expected `String`, found `&str`
+    |
+ note: tuple variant defined here
+ note: tuple variant defined here
+   --> $COMPILER_DIR/rustc_errors/src/diagnostic.rs:153:5
+    = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error[E0061]: this enum variant takes 1 argument but 2 arguments were supplied
+   --> $DIR/diagnostic-derive.rs:114:10
+    |
+ LL | #[derive(Diagnostic)]
+ LL | #[derive(Diagnostic)]
+    |          ^^^^^^^^^^ unexpected argument
+ LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
+    |                                 -------- help: try using a conversion method: `.to_string()`
+    |                                 expected `String`, found `&str`
+    |
+ note: tuple variant defined here
+ note: tuple variant defined here
+   --> $COMPILER_DIR/rustc_errors/src/diagnostic.rs:153:5
+    = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
668 error[E0277]: the trait bound `Hello: IntoDiagnosticArg` is not satisfied
670    |


676   --> $COMPILER_DIR/rustc_errors/src/diagnostic_builder.rs:LL:CC
677    = note: this error originates in the derive macro `Diagnostic` which comes from the expansion of the macro `forward` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 85 previous errors
+ error: aborting due to 86 previous errors
680 
- Some errors have detailed explanations: E0277, E0425.
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

error: expected parentheses: #[diag(...)]
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:52:8
   |
LL | #[diag = "E0123"]


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
LL | | //~^ ERROR `#[diag("...")]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidLitNestedAttr {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug must be the first argument
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:75:16
   |
LL | #[diag(nonsense("foo"), code = "E0123", slug = "foo")]


error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:75:1
   |
LL | / #[diag(nonsense("foo"), code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[diag(nonsense(...))]` is not a valid attribute
LL | | //~^^ ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr1 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:81:1
   |
LL | / #[diag(nonsense = "...", code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[diag(nonsense = ...)]` is not a valid attribute
LL | | //~| ERROR `#[diag(slug = ...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr2 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

error: diagnostic slug not specified
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:88:1
   |
LL | / #[diag(nonsense = 4, code = "E0123", slug = "foo")]
LL | | //~^ ERROR `#[diag(nonsense = ...)]` is not a valid attribute
LL | | //~| ERROR `#[diag(slug = ...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | struct InvalidNestedStructAttr3 {}
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`

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
error: expected `,`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:234:27
   |
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
error: expected `,`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:243:22
   |
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


error: the `#[help(...)]` attribute can only be applied to fields of type `Span`, `bool` or `()`
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

error: expected `,`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:540:33
   |
   |
LL |     #[label(no_crate_label, foo = "...")]

error: invalid nested attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:548:29
   |
   |
LL |     #[label(no_crate_label, foo("..."))]

error: expected `,`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:548:32
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
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:611:5
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", code = ",,,")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:611:5
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
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:657:1
LL | #[suggestion(no_crate_suggestion, code = "...")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = help: `#[label]` and `#[suggestion]` can only be applied to fields

error: `#[label]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:666:1
LL | #[label]
   | ^^^^^^^^
   |
   |
   = help: `#[label]` and `#[suggestion]` can only be applied to fields

error: `eager` is the only supported nested attribute for `subdiagnostic`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:700:7
   |
LL |     #[subdiagnostic(bad)]


---

error: cannot find attribute `multipart_suggestion` in this scope
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:649:7
   |
LL |     #[multipart_suggestion(no_crate_suggestion)]

error[E0425]: cannot find value `nonsense` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:70:8
   |
   |
LL | #[diag(nonsense, code = "E0123")]
   |        ^^^^^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `slug` in this scope
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:95:42
   |
LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
   |
   |
help: you might have meant to introduce a new binding
   |
LL | #[diag(no_crate_example, code = "E0123", let slug = "foo")]

error[E0425]: cannot find value `code` in this scope
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:115:42
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
   |
   |
help: you might have meant to introduce a new binding
   |
LL | #[diag(no_crate_example, code = "E0456", let code = "E0457")]


error[E0425]: cannot find value `__code_34` in this scope
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:803:10
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ not found in this scope
   |
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0061]: this enum variant takes 1 argument but 2 arguments were supplied
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:94:10
   |
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ unexpected argument
LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
   |                                 -------- help: try using a conversion method: `.to_string()`
   |                                 expected `String`, found `&str`
   |
note: tuple variant defined here
  --> /rustc/FAKE_PREFIX/compiler/rustc_errors/src/diagnostic.rs:153:5
  --> /rustc/FAKE_PREFIX/compiler/rustc_errors/src/diagnostic.rs:153:5
   = note: this error originates in the derive macro `Diagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0061]: this enum variant takes 1 argument but 2 arguments were supplied
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:114:10
   |
LL | #[derive(Diagnostic)]
   |          ^^^^^^^^^^ unexpected argument
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
   |                                 -------- help: try using a conversion method: `.to_string()`
   |                                 expected `String`, found `&str`
   |
note: tuple variant defined here
  --> /rustc/FAKE_PREFIX/compiler/rustc_errors/src/diagnostic.rs:153:5
  --> /rustc/FAKE_PREFIX/compiler/rustc_errors/src/diagnostic.rs:153:5
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
             Binder<'_, ExistentialTraitRef<'_>>
             Binder<'_, TraitRef<'_>>
             CString
             CguReuse
           and 49 others
note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
  --> /rustc/FAKE_PREFIX/compiler/rustc_errors/src/diagnostic_builder.rs:747:5
   = note: this error originates in the derive macro `Diagnostic` which comes from the expansion of the macro `forward` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 86 previous errors

Some errors have detailed explanations: E0061, E0277, E0425.
For more information about an error, try `rustc --explain E0061`.
