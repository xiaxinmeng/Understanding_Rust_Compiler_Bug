plain
Uplifting rustc (stage1 -> stage3)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 54 tests
.....F....................F.................F..F......
failures:

---- [ui] tests/ui-fulldeps/internal-lints/diagnostics.rs stdout ----
diff of stderr:
diff of stderr:

- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:41:17
+ error: could not open Fluent resource: No such file or directory (os error 2)
3    |
3    |
- LL |         handler.struct_err("untranslatable diagnostic")
-    |                 ^^^^^^^^^^
+ LL | fluent_messages! { "./diagnostics.ftl" }
+ 
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostics.rs:24:8
6    |
6    |
- note: the lint level is defined here
-   --> $DIR/diagnostics.rs:6:9
-    |
- LL | #![deny(rustc::untranslatable_diagnostic)]
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | #[diag(no_crate_example)]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
12 
- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:61:14
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
15    |
15    |
- LL |         diag.note("untranslatable diagnostic");
-    |              ^^^^
+ LL | #[note(no_crate_example)]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
18 
- error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
-   --> $DIR/diagnostics.rs:78:25
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
21    |
21    |
- LL |     let _diag = handler.struct_err(crate::fluent_generated::no_crate_example);
-    |
- note: the lint level is defined here
-   --> $DIR/diagnostics.rs:7:9
-    |
-    |
- LL | #![deny(rustc::diagnostic_outside_of_impl)]
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |         handler.struct_err(crate::fluent_generated::no_crate_example)
+    |                                                     ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
30 
- error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
-   --> $DIR/diagnostics.rs:81:25
+ error[E0425]: cannot find value `no_crate_note` in module `crate::fluent_generated`
33    |
33    |
- LL |     let _diag = handler.struct_err("untranslatable diagnostic");
-    |                         ^^^^^^^^^^
+ LL |         diag.note(crate::fluent_generated::no_crate_note);
+    |                                            ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
36 
- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:81:25
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
39    |
39    |
- LL |     let _diag = handler.struct_err("untranslatable diagnostic");
-    |                         ^^^^^^^^^^
+ LL |     let _diag = handler.struct_err(crate::fluent_generated::no_crate_example);
+    |                                                             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
- error: aborting due to 5 previous errors
+ error: aborting due to 6 previous errors
44 
+ For more information about this error, try `rustc --explain E0425`.
---
To only update this specific test, also pass `--test-args internal-lints/diagnostics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/internal-lints/diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/diagnostics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/diagnostics/auxiliary" "-Z" "unstable-options"
stdout: none
--- stderr -------------------------------
error: could not open Fluent resource: No such file or directory (os error 2)
  --> fake-test-src-base/internal-lints/diagnostics.rs:21:20
   |
LL | fluent_messages! { "./diagnostics.ftl" }

error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/internal-lints/diagnostics.rs:24:8
   |
   |
LL | #[diag(no_crate_example)]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/internal-lints/diagnostics.rs:31:8
   |
   |
LL | #[note(no_crate_example)]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/internal-lints/diagnostics.rs:50:53
   |
   |
LL |         handler.struct_err(crate::fluent_generated::no_crate_example)
   |                                                     ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_note` in module `crate::fluent_generated`
  --> fake-test-src-base/internal-lints/diagnostics.rs:73:44
   |
   |
LL |         diag.note(crate::fluent_generated::no_crate_note);
   |                                            ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/internal-lints/diagnostics.rs:78:61
   |
   |
LL |     let _diag = handler.struct_err(crate::fluent_generated::no_crate_example);
   |                                                             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0425`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
stdout: none
--- stderr -------------------------------
error: couldn't read /checkout/tests/ui-fulldeps/mod_dir_simple/test.rs: No such file or directory (os error 2)
   |
   |
LL | mod gravy;

error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs stdout ----
diff of stderr:

+ error: could not open Fluent resource: No such file or directory (os error 2)
+   --> $DIR/subdiagnostic-derive.rs:22:20
+    |
+ LL | fluent_messages! { "./example.ftl" }
+ 
+ 
1 error: label without `#[primary_span]` field
3    |


576 LL |     #[bar("...")]
578 
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:25:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:34:13
+    |
+    |
+ LL |     #[label(no_crate_example)]
+    |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:40:13
+    |
+    |
+ LL |     #[label(no_crate_example)]
+    |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:49:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ 
579 error[E0425]: cannot find value `slug` in module `crate::fluent_generated`
581    |


582 LL | #[label(slug)]
583    |         ^^^^ not found in `crate::fluent_generated`
- error: aborting due to 81 previous errors
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:143:9
+    |
+    |
+ LL | #[label(no_crate_example, code = "...")]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:152:9
+    |
+    |
+ LL | #[label(no_crate_example, applicability = "machine-applicable")]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:165:13
+    |
+    |
+ LL |     #[label(no_crate_example)]
+    |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:235:13
+    |
+    |
+ LL |     #[label(no_crate_example)]
+    |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:249:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:258:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:268:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:279:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:290:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:301:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:317:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:318:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:333:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:351:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "...")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:362:18
+    |
+    |
+ LL |     #[suggestion(no_crate_example, code = "...")]
+    |                  ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:370:18
+    |
+    |
+ LL |     #[suggestion(no_crate_example, code = "...")]
+    |                  ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:381:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "...", code = "...")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:392:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "...")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:405:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "...")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:415:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "...")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:422:14
+    |
+    |
+ LL | #[suggestion(no_crate_example)]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:432:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "...", applicability = "foo")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:440:8
+    |
+    |
+ LL | #[help(no_crate_example)]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:446:8
+    |
+    |
+ LL | #[note(no_crate_example)]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:450:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "...")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:457:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:467:13
+    |
+    |
+ LL |     #[label(no_crate_example)]
+    |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:476:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:484:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:493:18
+    |
+    |
+ LL |     #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]
+    |                  ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:503:18
+    |
+    |
+ LL |     #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]
+    |                  ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:512:11
+    |
+    |
+ LL | #[warning(no_crate_example)]
+    |           ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:516:11
+    |
+    |
+ LL | #[warning(no_crate_example)]
+    |           ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:523:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "...")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:538:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:546:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:554:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:562:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:571:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:591:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:600:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:609:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:620:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:632:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:639:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:648:9
+    |
+    |
+ LL | #[label(no_crate_example)]
+    |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:659:13
+    |
+    |
+ LL |     #[label(no_crate_example)]
+    |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:668:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:677:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:686:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:695:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:704:24
+    |
+    |
+ LL | #[multipart_suggestion(no_crate_example)]
+    |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:713:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:720:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "", style = "short")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:727:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "", style = "hidden")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:734:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "", style = "verbose")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:741:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "", style = "tool-only")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:748:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:757:21
+    |
+    |
+ LL | #[suggestion_hidden(no_crate_example, code = "")]
+    |                     ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:765:21
+    |
+    |
+ LL | #[suggestion_hidden(no_crate_example, code = "", style = "normal")]
+    |                     ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:773:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "", style = "foo")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:781:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "", style = 42)]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:797:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "", style("foo"))]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/subdiagnostic-derive.rs:805:14
+    |
+    |
+ LL | #[suggestion(no_crate_example, code = "")]
+    |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error: aborting due to 150 previous errors
586 
587 For more information about this error, try `rustc --explain E0425`.
---
To only update this specific test, also pass `--test-args session-diagnostic/subdiagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: could not open Fluent resource: No such file or directory (os error 2)
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:22:20
   |
LL | fluent_messages! { "./example.ftl" }


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


error: `#[label(bug = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:84:9
   |
LL | #[label(bug = "...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:84:1
   |
LL | #[label(bug = "...")]


error: `#[label("...")]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:94:9
   |
LL | #[label("...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:94:1
   |
LL | #[label("...")]


error: `#[label(slug = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:104:9
   |
LL | #[label(slug = 4)]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:104:1
   |
LL | #[label(slug = 4)]


error: `#[label(slug(...))]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:114:9
   |
LL | #[label(slug("..."))]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:114:1
   |
LL | #[label(slug("..."))]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:134:1
   |
LL | #[label()]


error: `#[label(code = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:143:27
   |
LL | #[label(no_crate_example, code = "...")]


error: `#[label(applicability = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:152:27
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


error: `#[label(code = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:223:13
   |
LL |     #[label(code = "...")]


error: diagnostic slug must be first argument of a `#[label(...)]` attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:223:5
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


error: `#[label(no_crate::example)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:325:27
   |
LL | #[label(no_crate_example, no_crate::example)]
   |
   |
   = help: a diagnostic slug must be the first argument to the attribute
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
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:381:46
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:381:32
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
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:432:46
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


error: `#[multipart_suggestion(code = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:538:42
   |
LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
   |
   |
   = help: only `style` and `applicability` are valid nested attributes

error: multipart suggestion without any `#[suggestion_part(...)]` fields
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:538:1
   |
LL | / #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
LL | | //~^ ERROR multipart suggestion without any `#[suggestion_part(...)]` fields
LL | | //~| ERROR `#[multipart_suggestion(code = ...)]` is not a valid attribute
LL | | struct BBa {
LL | |     var: String,
LL | | }


error: `#[suggestion_part(...)]` attribute without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:548:5
LL |     #[suggestion_part]
   |     ^^^^^^^^^^^^^^^^^^


error: `#[suggestion_part(...)]` attribute without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:556:5
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


error: `#[suggestion_part(...)]` attribute without `code = "..."`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:576:5
   |
LL |     #[suggestion_part()]


error: `#[suggestion_part(foo = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:579:23
   |
LL |     #[suggestion_part(foo = "bar")]
   |
   |
   = help: `code` is the only valid nested attribute

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
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:670:23
   |
LL |     #[suggestion_part(code("foo"))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:679:23
   |
LL |     #[suggestion_part(code("foo", "bar"))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:688:23
   |
LL |     #[suggestion_part(code(3))]


error: expected exactly one string literal for `code = ...`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:697:23
   |
LL |     #[suggestion_part(code())]


error: `code = "..."`/`code(...)` must contain only string literals
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:706:23
   |
LL |     #[suggestion_part(code = 3)]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:748:61
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:748:43
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

error: `#[suggestion(style = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:781:43
   |
LL | #[suggestion(no_crate_example, code = "", style = 42)]


error: `#[suggestion(style)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:789:43
   |
LL | #[suggestion(no_crate_example, code = "", style)]
   |
   |
   = help: a diagnostic slug must be the first argument to the attribute

error: `#[suggestion(style(...))]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:797:43
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

error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:25:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:34:13
   |
   |
LL |     #[label(no_crate_example)]
   |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:40:13
   |
   |
LL |     #[label(no_crate_example)]
   |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:49:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `slug` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:124:9
   |
LL | #[label(slug)]
   |         ^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:143:9
   |
   |
LL | #[label(no_crate_example, code = "...")]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:152:9
   |
   |
LL | #[label(no_crate_example, applicability = "machine-applicable")]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:165:13
   |
   |
LL |     #[label(no_crate_example)]
   |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:235:13
   |
   |
LL |     #[label(no_crate_example)]
   |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:249:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:258:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:268:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:279:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:290:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:301:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:317:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:318:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:333:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:351:14
   |
LL | #[suggestion(no_crate_example, code = "...")]
LL | #[suggestion(no_crate_example, code = "...")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:362:18
   |
LL |     #[suggestion(no_crate_example, code = "...")]
   |                  ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:370:18
   |
   |
LL |     #[suggestion(no_crate_example, code = "...")]
   |                  ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:381:14
   |
   |
LL | #[suggestion(no_crate_example, code = "...", code = "...")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:392:14
   |
LL | #[suggestion(no_crate_example, code = "...")]
---

error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:422:14
   |
LL | #[suggestion(no_crate_example)]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:432:14
   |
   |
LL | #[suggestion(no_crate_example, code = "...", applicability = "foo")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:440:8
   |
   |
LL | #[help(no_crate_example)]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:446:8
   |
   |
LL | #[note(no_crate_example)]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:450:14
   |
LL | #[suggestion(no_crate_example, code = "...")]
LL | #[suggestion(no_crate_example, code = "...")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:457:14
   |
LL | #[suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:467:13
   |
   |
LL |     #[label(no_crate_example)]
   |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:476:14
   |
   |
LL | #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:484:14
   |
   |
LL | #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:493:18
   |
   |
LL |     #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]
   |                  ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:503:18
   |
   |
LL |     #[suggestion(no_crate_example, code = "{var}", applicability = "machine-applicable")]
   |                  ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:512:11
   |
   |
LL | #[warning(no_crate_example)]
   |           ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:516:11
   |
   |
LL | #[warning(no_crate_example)]
   |           ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:523:14
   |
LL | #[suggestion(no_crate_example, code = "...")]
LL | #[suggestion(no_crate_example, code = "...")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:538:24
   |
LL | #[multipart_suggestion(no_crate_example, code = "...", applicability = "machine-applicable")]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:546:24
   |
   |
LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:554:24
   |
   |
LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:562:24
   |
   |
LL | #[multipart_suggestion(no_crate_example)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:571:24
   |
   |
LL | #[multipart_suggestion(no_crate_example)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:591:24
   |
   |
LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:600:24
   |
   |
LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:609:24
   |
   |
LL | #[multipart_suggestion(no_crate_example)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:620:24
   |
   |
LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:632:24
   |
   |
LL | #[multipart_suggestion(no_crate_example, applicability = "machine-applicable")]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:639:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:648:9
   |
   |
LL | #[label(no_crate_example)]
   |         ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:659:13
   |
   |
LL |     #[label(no_crate_example)]
   |             ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:668:24
   |
   |
LL | #[multipart_suggestion(no_crate_example)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:677:24
   |
   |
LL | #[multipart_suggestion(no_crate_example)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:686:24
   |
   |
LL | #[multipart_suggestion(no_crate_example)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:695:24
   |
   |
LL | #[multipart_suggestion(no_crate_example)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:704:24
   |
   |
LL | #[multipart_suggestion(no_crate_example)]
   |                        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:713:14
   |
   |
LL | #[suggestion(no_crate_example, code = "")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:720:14
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "short")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:727:14
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:734:14
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "verbose")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:741:14
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "tool-only")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:748:14
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "hidden", style = "normal")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:757:21
   |
   |
LL | #[suggestion_hidden(no_crate_example, code = "")]
   |                     ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:765:21
   |
   |
LL | #[suggestion_hidden(no_crate_example, code = "", style = "normal")]
   |                     ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:773:14
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = "foo")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:781:14
   |
   |
LL | #[suggestion(no_crate_example, code = "", style = 42)]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:797:14
   |
   |
LL | #[suggestion(no_crate_example, code = "", style("foo"))]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/subdiagnostic-derive.rs:805:14
   |
   |
LL | #[suggestion(no_crate_example, code = "")]
   |              ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error: aborting due to 150 previous errors

For more information about this error, try `rustc --explain E0425`.
------------------------------------------
---

+ error: could not open Fluent resource: No such file or directory (os error 2)
+   --> $DIR/diagnostic-derive.rs:30:20
+    |
+ LL | fluent_messages! { "./example.ftl" }
+ 
1 error: unsupported type attribute for diagnostic derive enum
2   --> $DIR/diagnostic-derive.rs:41:1
3    |
3    |

659 LL |     #[multipart_suggestion(no_crate_suggestion)]
661 
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:33:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:37:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:51:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
662 error[E0425]: cannot find value `nonsense` in module `crate::fluent_generated`
663   --> $DIR/diagnostic-derive.rs:70:8
664    |


665 LL | #[diag(nonsense, code = "E0123")]
666    |        ^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:95:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:100:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:108:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:115:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:120:8
+    |
+    |
+ LL | #[diag(no_crate_example, no_crate::example, code = "E0456")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:133:8
+    |
+    |
+ LL | #[diag(no_crate_example)]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:137:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:145:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:154:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ 
+ error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
+    |
+    |
+ LL |     #[label(no_crate_label)]
+    |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:162:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:170:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:172:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "{name}")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:179:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:181:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "{name")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:189:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:191:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "name}")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:198:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ 
+ error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
+    |
+    |
+ LL |     #[label(no_crate_label)]
+    |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:205:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:213:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:215:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "This is the suggested code")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:216:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "This is the suggested code", style = "normal")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:217:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "This is the suggested code", style = "short")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:218:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "This is the suggested code", style = "hidden")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:219:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "This is the suggested code", style = "verbose")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:224:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:226:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion)]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:232:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:241:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:250:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:257:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:259:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:264:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:272:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:274:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:279:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:281:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:287:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:289:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:295:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:303:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ 
+ error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
+    |
+    |
+ LL |     #[label(no_crate_label)]
+    |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:307:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "...")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:312:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0456")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ 
+ error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
+    |
+    |
+ LL |     #[label(no_crate_label)]
+    |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ 
+ error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
+    |
+    |
+ LL |     #[label(no_crate_label)]
+    |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:321:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "{name}.clone()")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:326:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ 
+ error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
+    |
+    |
+ LL |     #[label(no_crate_label)]
+    |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:334:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:343:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:351:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:362:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:369:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_note` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:371:12
+    |
+    |
+ LL |     #[note(no_crate_note)]
+    |            ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:376:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:383:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_note` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:384:8
+    |
+    |
+ LL | #[note(no_crate_note)]
+    |        ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:390:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:397:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_help` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:399:12
+    |
+    |
+ LL |     #[help(no_crate_help)]
+    |            ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:404:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:411:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_help` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:412:8
+    |
+    |
+ LL | #[help(no_crate_help)]
+    |        ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:419:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:426:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_help` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:425:8
+    |
+    |
+ LL | #[help(no_crate_help)]
+    |        ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:433:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:440:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_note` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:439:8
+    |
+    |
+ LL | #[note(no_crate_note)]
+    |        ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:446:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:448:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "maybe-incorrect")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:454:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:456:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "batman")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:462:8
+    |
+    |
+ LL | #[diag(no_crate_example, code = "E0123")]
+    |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
+ error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
+   --> $DIR/diagnostic-derive.rs:464:18
+    |
+    |
+ LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "maybe-incorrect")]
+    |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
---
To only update this specific test, also pass `--test-args session-diagnostic/diagnostic-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: could not open Fluent resource: No such file or directory (os error 2)
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:30:20
   |
LL | fluent_messages! { "./example.ftl" }

error: unsupported type attribute for diagnostic derive enum
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:41:1
   |
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

error: `#[diag = ...]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:52:1
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

error: `#[diag("...")]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:64:8
   |
LL | #[diag("E0123")]
   |
   = help: a diagnostic slug is required as the first argument


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

error: `#[diag(nonsense(...))]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:75:8
   |
LL | #[diag(nonsense("foo"), code = "E0123", slug = "foo")]
   |
   = help: a diagnostic slug is required as the first argument


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

error: `#[diag(nonsense = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:81:8
   |
LL | #[diag(nonsense = "...", code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

error: `#[diag(slug = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:81:42
   |
LL | #[diag(nonsense = "...", code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

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

error: `#[diag(nonsense = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:88:8
   |
LL | #[diag(nonsense = 4, code = "E0123", slug = "foo")]


error: `#[diag(slug = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:88:38
   |
LL | #[diag(nonsense = 4, code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

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

error: `#[diag(slug = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:95:42
   |
LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

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
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:109:33
   |
   |
LL | #[diag(no_crate_example, code = "E0456")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:108:33
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]

error: specified multiple times
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:115:49
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
   |
note: previously specified here
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:115:33
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]


error: `#[diag(no_crate::example)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:120:26
   |
LL | #[diag(no_crate_example, no_crate::example, code = "E0456")]
   |
   |
   = help: diagnostic slug must be the first argument

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


error: `#[suggestion(nonsense = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:234:18
   |
LL |     #[suggestion(nonsense = "bar")]
   |
   |
   = help: only `style`, `code` and `applicability` are valid nested attributes

error: suggestion without `code = "..."`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:234:5
   |
LL |     #[suggestion(nonsense = "bar")]


error: `#[suggestion(msg = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:243:18
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
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:448:53
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
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:456:53
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "batman")]


error: the `#[help(...)]` attribute can only be applied to fields of type `Span`, `bool` or `()`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:523:5
   |
LL |     #[help(no_crate_help)]


error: `#[label(foo)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:532:29
   |
LL |     #[label(no_crate_label, foo)]
   |
   |
   = help: a diagnostic slug must be the first argument to the attribute

error: `#[label(foo = ...)]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:540:29
   |
LL |     #[label(no_crate_label, foo = "...")]


error: `#[label(foo(...))]` is not a valid attribute
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:548:29
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
---

error: cannot find attribute `multipart_suggestion` in this scope
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:649:7
   |
LL |     #[multipart_suggestion(no_crate_suggestion)]

error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:33:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:37:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:51:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `nonsense` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:70:8
   |
   |
LL | #[diag(nonsense, code = "E0123")]
   |        ^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:95:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123", slug = "foo")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:100:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:108:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:115:8
   |
   |
LL | #[diag(no_crate_example, code = "E0456", code = "E0457")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:120:8
   |
   |
LL | #[diag(no_crate_example, no_crate::example, code = "E0456")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:133:8
   |
   |
LL | #[diag(no_crate_example)]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:137:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:145:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:154:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:157:13
   |
   |
LL |     #[label(no_crate_label)]
   |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:162:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:170:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:172:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "{name}")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:179:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:181:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "{name")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:189:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:191:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "name}")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:198:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:200:13
   |
   |
LL |     #[label(no_crate_label)]
   |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:205:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:213:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:215:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "This is the suggested code")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:216:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "This is the suggested code", style = "normal")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:217:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "This is the suggested code", style = "short")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:218:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "This is the suggested code", style = "hidden")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:219:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "This is the suggested code", style = "verbose")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:224:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:226:18
   |
   |
LL |     #[suggestion(no_crate_suggestion)]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:232:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:241:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:250:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:257:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:259:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:264:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:272:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:274:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:279:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:281:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:287:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:289:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "This is suggested code")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:295:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:303:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:305:13
   |
   |
LL |     #[label(no_crate_label)]
   |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:307:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:312:8
   |
   |
LL | #[diag(no_crate_example, code = "E0456")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:317:13
   |
   |
LL |     #[label(no_crate_label)]
   |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:319:13
   |
   |
LL |     #[label(no_crate_label)]
   |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:321:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "{name}.clone()")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:326:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_label` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:328:13
   |
   |
LL |     #[label(no_crate_label)]
   |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:334:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:343:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:351:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:362:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:369:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_note` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:371:12
   |
   |
LL |     #[note(no_crate_note)]
   |            ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:376:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:383:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_note` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:384:8
   |
   |
LL | #[note(no_crate_note)]
   |        ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:390:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:397:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_help` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:399:12
   |
   |
LL |     #[help(no_crate_help)]
   |            ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:404:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:411:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_help` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:412:8
   |
   |
LL | #[help(no_crate_help)]
   |        ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:419:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:426:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_help` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:425:8
   |
   |
LL | #[help(no_crate_help)]
   |        ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:433:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:440:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_note` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:439:8
   |
   |
LL | #[note(no_crate_note)]
   |        ^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:446:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:448:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "maybe-incorrect")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:454:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:456:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "batman")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:462:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:464:18
   |
   |
LL |     #[suggestion(no_crate_suggestion, code = "...", applicability = "maybe-incorrect")]
   |                  ^^^^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_example` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:469:8
   |
   |
LL | #[diag(no_crate_example, code = "E0123")]
   |        ^^^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `no_crate_suggestion` in module `crate::fluent_generated`
  --> fake-test-src-base/session-diagnostic/diagnostic-derive.rs:471:18
