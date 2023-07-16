plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
F.......F..............i..................................F.F..........
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [ui] src/test/ui-fulldeps/fluent-messages/test.rs stdout ----
diff of stderr:


41 LL |         a => "./duplicate-a.ftl",
43 
43 
+ error[E0428]: the name `a_b_key` is defined multiple times
+    |
+ LL |     fluent_messages! {
+    |     ^^^^^^^^^^^^^^^^
+    |     |
+    |     |
+    |     `a_b_key` redefined here
+    |     previous definition of the value `a_b_key` here
+    = note: os-specific message
+    = note: os-specific message
+ 
+ 
44 error: name `slug_with_hyphens_this-slug-has-hyphens` contains a '-' character
46    |

81    |
81    |
82    = help: replace any '-'s with '_'s
- error: aborting due to 9 previous errors
+ error[E0432]: unresolved import `self::fluent_generated::valid`
+   --> $DIR/test.rs:83:60
+    |
+    |
+ LL |     use self::fluent_generated::{DEFAULT_LOCALE_RESOURCES, valid::key};
+    |                                                            ^^^^^ could not find `valid` in `fluent_generated`
+ error[E0432]: unresolved import `self::fluent_generated::test_crate`
+   --> $DIR/test.rs:96:60
+    |
+    |
+ LL |     use self::fluent_generated::{DEFAULT_LOCALE_RESOURCES, test_crate::{foo, with_hyphens}};
+    |                                                            ^^^^^^^^^^ could not find `test_crate` in `fluent_generated`
+ error: aborting due to 12 previous errors
+ 
+ Some errors have detailed explanations: E0428, E0432.
+ For more information about an error, try `rustc --explain E0428`.
---
To only update this specific test, also pass `--test-args fluent-messages/test.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/fluent-messages/test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/fluent-messages/test" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/fluent-messages/test/auxiliary"
stdout: none
--- stderr -------------------------------
error: could not open Fluent resource
   |
   |
LL |         missing_absolute => "/definitely_does_not_exist.ftl",
   |
   = note: No such file or directory (os error 2)

error: could not open Fluent resource
error: could not open Fluent resource
  --> /checkout/src/test/ui-fulldeps/fluent-messages/test.rs:34:29
   |
LL |         missing_relative => "../definitely_does_not_exist.ftl",
   |
   = note: No such file or directory (os error 2)

error: could not parse Fluent resource
error: could not parse Fluent resource
  --> /checkout/src/test/ui-fulldeps/fluent-messages/test.rs:43:28
   |
LL |         missing_message => "./missing-message.ftl",
   |
   = help: see additional errors emitted

error: expected a message field for "missing_message"
error: expected a message field for "missing_message"
 --> ./missing-message.ftl:1:1
1 | missing_message =
  | ^^^^^^^^^^^^^^^^^
  |


error: overrides existing message: `a_b_key`
   |
   |
LL |         a_b => "./duplicate-a-b.ftl",
   |
help: previously defined in this resource
  --> /checkout/src/test/ui-fulldeps/fluent-messages/test.rs:52:14
   |
   |
LL |         a => "./duplicate-a.ftl",


error[E0428]: the name `a_b_key` is defined multiple times
   |
LL |     fluent_messages! {
   |     ^^^^^^^^^^^^^^^^
   |     |
   |     |
   |     `a_b_key` redefined here
   |     previous definition of the value `a_b_key` here
   |
   = note: `a_b_key` must be defined only once in the value namespace of this module
   = note: this error originates in the macro `fluent_messages` (in Nightly builds, run with -Z macro-backtrace for more info)

error: name `slug_with_hyphens_this-slug-has-hyphens` contains a '-' character
   |
   |
LL |         slug_with_hyphens => "./slug-with-hyphens.ftl",
   |
   |
   = help: replace any '-'s with '_'s

error: attribute `label-has-hyphens` contains a '-' character
   |
   |
LL |         label_with_hyphens => "./label-with-hyphens.ftl",
   |
   |
   = help: replace any '-'s with '_'s

error: name `with-hyphens` contains a '-' character
   |
   |
LL |         test_crate => "./missing-crate-name.ftl",
   |
   |
   = help: replace any '-'s with '_'s

error: name `with-hyphens` does not start with the crate name
   |
   |
LL |         test_crate => "./missing-crate-name.ftl",
   |
   |
   = help: prepend `test_crate_` to the slug name: `test_crate_with_hyphens`

error: name `test-crate_foo` contains a '-' character
   |
   |
LL |         test_crate => "./missing-crate-name.ftl",
   |
   |
   = help: replace any '-'s with '_'s
error[E0432]: unresolved import `self::fluent_generated::valid`
  --> /checkout/src/test/ui-fulldeps/fluent-messages/test.rs:83:60
   |
   |
LL |     use self::fluent_generated::{DEFAULT_LOCALE_RESOURCES, valid::key};
   |                                                            ^^^^^ could not find `valid` in `fluent_generated`
error[E0432]: unresolved import `self::fluent_generated::test_crate`
  --> /checkout/src/test/ui-fulldeps/fluent-messages/test.rs:96:60
   |
   |
LL |     use self::fluent_generated::{DEFAULT_LOCALE_RESOURCES, test_crate::{foo, with_hyphens}};
   |                                                            ^^^^^^^^^^ could not find `test_crate` in `fluent_generated`
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0428, E0432.
For more information about an error, try `rustc --explain E0428`.
For more information about an error, try `rustc --explain E0428`.
------------------------------------------


---- [ui] src/test/ui-fulldeps/internal-lints/diagnostics.rs stdout ----
diff of stderr:

- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:39:17
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
3    |
3    |
- LL |         handler.struct_err("untranslatable diagnostic")
-    |
- note: the lint level is defined here
-   --> $DIR/diagnostics.rs:6:9
-    |
-    |
- LL | #![deny(rustc::untranslatable_diagnostic)]
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | #[diag(compiletest::example)]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
12 
- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:59:14
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
15    |
15    |
- LL |         diag.note("untranslatable diagnostic");
-    |              ^^^^
+ LL | #[note(compiletest::example)]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
18 
- error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
-   --> $DIR/diagnostics.rs:76:25
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
21    |
21    |
- LL |     let _diag = handler.struct_err(fluent::compiletest::example);
-    |
- note: the lint level is defined here
-   --> $DIR/diagnostics.rs:7:9
-    |
-    |
- LL | #![deny(rustc::diagnostic_outside_of_impl)]
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |         handler.struct_err(fluent::compiletest::example)
+    |                                    ^^^^^^^^^^^ could not find `compiletest` in `fluent`
30 
- error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
-   --> $DIR/diagnostics.rs:79:25
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
33    |
33    |
- LL |     let _diag = handler.struct_err("untranslatable diagnostic");
-    |                         ^^^^^^^^^^
+ LL |         diag.note(fluent::compiletest::note);
+    |                           ^^^^^^^^^^^ could not find `compiletest` in `fluent`
36 
- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:79:25
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
39    |
39    |
- LL |     let _diag = handler.struct_err("untranslatable diagnostic");
-    |                         ^^^^^^^^^^
+ LL |     let _diag = handler.struct_err(fluent::compiletest::example);
+    |                                            ^^^^^^^^^^^ could not find `compiletest` in `fluent`
43 error: aborting due to 5 previous errors
44 

+ For more information about this error, try `rustc --explain E0433`.
---
To only update this specific test, also pass `--test-args internal-lints/diagnostics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/diagnostics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/diagnostics/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
   |
   |
LL | #[diag(compiletest::example)]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/internal-lints/diagnostics.rs:29:8
   |
   |
LL | #[note(compiletest::example)]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/internal-lints/diagnostics.rs:48:36
   |
   |
LL |         handler.struct_err(fluent::compiletest::example)
   |                                    ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/internal-lints/diagnostics.rs:71:27
   |
   |
LL |         diag.note(fluent::compiletest::note);
   |                           ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/internal-lints/diagnostics.rs:76:44
   |
   |
LL |     let _diag = handler.struct_err(fluent::compiletest::example);
   |                                            ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0433`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs stdout ----
diff of stderr:

475 LL |     #[bar("...")]
477 
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:23:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:32:13
+    |
+    |
+ LL |     #[label(parser::add_paren)]
+    |             ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:38:13
+    |
+    |
+ LL |     #[label(parser::add_paren)]
+    |             ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:47:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:141:9
+    |
+    |
+ LL | #[label(parser::add_paren, code = "...")]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:150:9
+    |
+    |
+ LL | #[label(parser::add_paren, applicability = "machine-applicable")]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:163:13
+    |
+    |
+ LL |     #[label(parser::add_paren)]
+    |             ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:233:13
+    |
+    |
+ LL |     #[label(parser::add_paren)]
+    |             ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:248:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:257:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:267:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:278:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:289:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:300:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:316:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:317:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:332:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:350:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code = "...")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:361:18
+    |
+    |
+ LL |     #[suggestion(parser::add_paren, code = "...")]
+    |                  ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:369:18
+    |
+    |
+ LL |     #[suggestion(parser::add_paren, code = "...")]
+    |                  ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:380:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code = "...", code = "...")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:391:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code = "...")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:404:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code = "...")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:414:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code = "...")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:421:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren)]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:431:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code ="...", applicability = "foo")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:439:8
+    |
+    |
+ LL | #[help(parser::add_paren)]
+    |        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:445:8
+    |
+    |
+ LL | #[note(parser::add_paren)]
+    |        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:449:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code = "...")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:456:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code ="...", applicability = "machine-applicable")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:466:13
+    |
+    |
+ LL |     #[label(parser::add_paren)]
+    |             ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:475:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code ="{var}", applicability = "machine-applicable")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:483:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code ="{var}", applicability = "machine-applicable")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:492:18
+    |
+    |
+ LL |     #[suggestion(parser::add_paren, code ="{var}", applicability = "machine-applicable")]
+    |                  ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:502:18
+    |
+    |
+ LL |     #[suggestion(parser::add_paren, code ="{var}", applicability = "machine-applicable")]
+    |                  ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:511:11
+    |
+    |
+ LL | #[warning(parser::add_paren)]
+    |           ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:515:11
+    |
+    |
+ LL | #[warning(parser::add_paren)]
+    |           ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:522:14
+    |
+    |
+ LL | #[suggestion(parser::add_paren, code = "...")]
+    |              ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:537:24
+    |
+    |
+ LL | #[multipart_suggestion(parser::add_paren, code = "...", applicability = "machine-applicable")]
+    |                        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:545:24
+    |
+    |
+ LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
+    |                        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:553:24
+    |
+    |
+ LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
+    |                        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:561:24
+    |
+    |
+ LL | #[multipart_suggestion(parser::add_paren)]
+    |                        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:570:24
+    |
+    |
+ LL | #[multipart_suggestion(parser::add_paren)]
+    |                        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:590:24
+    |
+    |
+ LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
+    |                        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:599:24
+    |
+    |
+ LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
+    |                        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:608:24
+    |
+    |
+ LL | #[multipart_suggestion(parser::add_paren)]
+    |                        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:619:24
+    |
+    |
+ LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
+    |                        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:631:24
+    |
+    |
+ LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
+    |                        ^^^^^^ could not find `parser` in `fluent`
+ error[E0433]: failed to resolve: could not find `parser` in `fluent`
+   --> $DIR/subdiagnostic-derive.rs:638:9
+    |
+    |
+ LL | #[label(parser::add_paren)]
+    |         ^^^^^^ could not find `parser` in `fluent`
+ 
478 error[E0425]: cannot find value `slug` in module `rustc_errors::fluent`
480    |


481 LL | #[label(slug)]
482    |         ^^^^ not found in `rustc_errors::fluent`
- error: aborting due to 68 previous errors
+ error: aborting due to 117 previous errors
485 
- For more information about this error, try `rustc --explain E0425`.
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
LL | #[label(parser::add_paren, code = "...")]


error: `#[label(applicability = ...)]` is not a valid attribute
   |
   |
LL | #[label(parser::add_paren, applicability = "machine-applicable")]

error: unsupported type attribute for subdiagnostic enum
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:159:1
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


error: subdiagnostic kind not specified
   |
LL |     B {
   |     ^


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
   |
   |
   = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes
error: unexpected unsupported untagged union
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:309:1
   |
   |
LL | / union AC {
LL | | //~^ ERROR unexpected unsupported untagged union
LL | |     span: u32,
LL | |     b: u64
LL | | }


error: `#[label(parser::add_paren)]` is not a valid attribute
   |
   |
LL | #[label(parser::add_paren, parser::add_paren)]
   |
   |
   = help: a diagnostic slug must be the first argument to the attribute
error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:337:5
   |
   |
LL |     #[primary_span]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:334:5
   |
   |
LL |     #[primary_span]


error: subdiagnostic kind not specified
   |
LL | struct AG {
   |        ^^


error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:380:47
   |
LL | #[suggestion(parser::add_paren, code = "...", code = "...")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:380:33
   |
   |
LL | #[suggestion(parser::add_paren, code = "...", code = "...")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:398:5
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
LL | #[suggestion(parser::add_paren)]

error: invalid applicability
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:431:46
   |
   |
LL | #[suggestion(parser::add_paren, code ="...", applicability = "foo")]


error: suggestion without `#[primary_span]` field
   |
   |
LL | / #[suggestion(parser::add_paren, code = "...")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct AR {
LL | |     var: String,
LL | | }

error: unsupported type attribute for subdiagnostic enum
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:463:1
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
LL | / #[suggestion(parser::add_paren, code = "...")]
LL | | //~^ ERROR suggestion without `#[primary_span]` field
LL | | struct BA {
LL | |     #[suggestion_part]
LL | |     var: String,
LL | | }
   | |_^


error: `#[multipart_suggestion(code = ...)]` is not a valid attribute
   |
   |
LL | #[multipart_suggestion(parser::add_paren, code = "...", applicability = "machine-applicable")]
   |
   |
   = help: only `applicability` is a valid nested attributes

error: multipart suggestion without any `#[suggestion_part(...)]` fields
   |
   |
LL | / #[multipart_suggestion(parser::add_paren, code = "...", applicability = "machine-applicable")]
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
LL | / #[multipart_suggestion(parser::add_paren)]
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
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:592:37
   |
   |
LL |     #[suggestion_part(code = "...", code = ",,,")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:592:23
   |
   |
LL |     #[suggestion_part(code = "...", code = ",,,")]


error: `#[applicability]` has no effect if all `#[suggestion]`/`#[multipart_suggestion]` attributes have a static `applicability = "..."`
   |
LL |     #[applicability]
   |     ^^^^^^^^^^^^^^^^

---

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:185:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:197:7
   |
   |
LL |     #[bar = 4]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:209:7
   |
   |
LL |     #[bar("...")]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:271:7
   |
   |
LL |     #[bar]
   |       ^^^

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:282:7
   |
LL |     #[bar = "..."]

error: cannot find attribute `bar` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:293:7
   |
   |
LL |     #[bar("...")]

error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:23:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:32:13
   |
   |
LL |     #[label(parser::add_paren)]
   |             ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:38:13
   |
   |
LL |     #[label(parser::add_paren)]
   |             ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:47:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:141:9
   |
   |
LL | #[label(parser::add_paren, code = "...")]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:150:9
   |
   |
LL | #[label(parser::add_paren, applicability = "machine-applicable")]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:163:13
   |
   |
LL |     #[label(parser::add_paren)]
   |             ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:233:13
   |
   |
LL |     #[label(parser::add_paren)]
   |             ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:248:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:257:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:267:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:278:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:289:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:300:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:316:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:317:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:332:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:350:14
   |
   |
LL | #[suggestion(parser::add_paren, code = "...")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:361:18
   |
   |
LL |     #[suggestion(parser::add_paren, code = "...")]
   |                  ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:369:18
   |
   |
LL |     #[suggestion(parser::add_paren, code = "...")]
   |                  ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:380:14
   |
   |
LL | #[suggestion(parser::add_paren, code = "...", code = "...")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:391:14
   |
   |
LL | #[suggestion(parser::add_paren, code = "...")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:404:14
   |
   |
LL | #[suggestion(parser::add_paren, code = "...")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:414:14
   |
   |
LL | #[suggestion(parser::add_paren, code = "...")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:421:14
   |
   |
LL | #[suggestion(parser::add_paren)]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:431:14
   |
   |
LL | #[suggestion(parser::add_paren, code ="...", applicability = "foo")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:439:8
   |
   |
LL | #[help(parser::add_paren)]
   |        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:445:8
   |
   |
LL | #[note(parser::add_paren)]
   |        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:449:14
   |
   |
LL | #[suggestion(parser::add_paren, code = "...")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:456:14
   |
   |
LL | #[suggestion(parser::add_paren, code ="...", applicability = "machine-applicable")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:466:13
   |
   |
LL |     #[label(parser::add_paren)]
   |             ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:475:14
   |
   |
LL | #[suggestion(parser::add_paren, code ="{var}", applicability = "machine-applicable")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:483:14
   |
   |
LL | #[suggestion(parser::add_paren, code ="{var}", applicability = "machine-applicable")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:492:18
   |
   |
LL |     #[suggestion(parser::add_paren, code ="{var}", applicability = "machine-applicable")]
   |                  ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:502:18
   |
   |
LL |     #[suggestion(parser::add_paren, code ="{var}", applicability = "machine-applicable")]
   |                  ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:511:11
   |
   |
LL | #[warning(parser::add_paren)]
   |           ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:515:11
   |
   |
LL | #[warning(parser::add_paren)]
   |           ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:522:14
   |
   |
LL | #[suggestion(parser::add_paren, code = "...")]
   |              ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:537:24
   |
   |
LL | #[multipart_suggestion(parser::add_paren, code = "...", applicability = "machine-applicable")]
   |                        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:545:24
   |
   |
LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
   |                        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:553:24
   |
   |
LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
   |                        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:561:24
   |
   |
LL | #[multipart_suggestion(parser::add_paren)]
   |                        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:570:24
   |
   |
LL | #[multipart_suggestion(parser::add_paren)]
   |                        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:590:24
   |
   |
LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
   |                        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:599:24
   |
   |
LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
   |                        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:608:24
   |
   |
LL | #[multipart_suggestion(parser::add_paren)]
   |                        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:619:24
   |
   |
LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
   |                        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:631:24
   |
   |
LL | #[multipart_suggestion(parser::add_paren, applicability = "machine-applicable")]
   |                        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:638:9
   |
   |
LL | #[label(parser::add_paren)]
   |         ^^^^^^ could not find `parser` in `fluent`

error[E0425]: cannot find value `slug` in module `rustc_errors::fluent`
   |
   |
LL | #[label(slug)]
   |         ^^^^ not found in `rustc_errors::fluent`
error: aborting due to 117 previous errors

Some errors have detailed explanations: E0425, E0433.
For more information about an error, try `rustc --explain E0425`.
---

10 LL |     Foo,
11    |     ^^^
12    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
14 
15 error: diagnostic slug not specified

18 LL |     Bar,
19    |     ^^^
20    |
20    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
22 
23 error: `#[diag = ...]` is not a valid attribute


42 LL | | struct InvalidStructAttr {}
44    |
44    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
46 
47 error: `#[diag("...")]` is not a valid attribute


61 LL | | struct InvalidLitNestedAttr {}
63    |
63    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
65 
66 error: `#[diag(nonsense(...))]` is not a valid attribute


80 LL | | struct InvalidNestedStructAttr1 {}
82    |
82    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
84 
85 error: `#[diag(nonsense = ...)]` is not a valid attribute


108 LL | | struct InvalidNestedStructAttr2 {}
110    |
110    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
112 
113 error: `#[diag(nonsense = ...)]` is not a valid attribute


134 LL | | struct InvalidNestedStructAttr3 {}
136    |
136    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
138 
139 error: `#[diag(slug = ...)]` is not a valid attribute


200 LL | struct KindNotProvided {}
202    |
202    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
204 
205 error: diagnostic slug not specified


210 LL | | struct SlugNotProvided {}
212    |
212    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
214 
215 error: the `#[primary_span]` attribute can only be applied to fields of type `Span` or `MultiSpan`


396 LL | | struct ErrorAttribute {}
398    |
398    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
400 
401 error: `#[warn_(...)]` is not a valid attribute


414 LL | | struct WarnAttribute {}
416    |
416    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
418 
419 error: `#[lint(...)]` is not a valid attribute


432 LL | | struct LintAttributeOnSessionDiag {}
434    |
434    |
-    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis::example_error)]`
+    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(hir_analysis_example_error)]`
436 
437 error: `#[lint(...)]` is not a valid attribute


457 LL | | struct LintAttributeOnLintDiag {}
459    |
459    |
-    = help: specify the slug as the first argument to the attribute, such as `#[diag(compiletest::example)]`
+    = help: specify the slug as the first argument to the attribute, such as `#[diag(compiletest_example)]`
462 error: specified multiple times
463   --> $DIR/diagnostic-derive.rs:596:57


627 LL |     #[multipart_suggestion(compiletest::suggestion)]
629 
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:31:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:35:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:49:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:93:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123", slug = "foo")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:98:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:106:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:113:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0456", code = "E0457")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:118:8
+    |
+    |
+ LL | #[diag(compiletest::example, compiletest::example, code = "E0456")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:131:8
+    |
+    |
+ LL | #[diag(compiletest::example)]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:135:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:143:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:152:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:155:13
+    |
+    |
+ LL |     #[label(compiletest::label)]
+    |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:160:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:168:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:170:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion, code = "{name}")]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:177:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:179:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion, code = "{name")]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:187:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:189:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion, code = "name}")]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:196:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:198:13
+    |
+    |
+ LL |     #[label(compiletest::label)]
+    |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:203:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:211:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:213:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion, code = "This is the suggested code")]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:214:24
+    |
+    |
+ LL |     #[suggestion_short(compiletest::suggestion, code = "This is the suggested code")]
+    |                        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:215:25
+    |
+    |
+ LL |     #[suggestion_hidden(compiletest::suggestion, code = "This is the suggested code")]
+    |                         ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:216:26
+    |
+    |
+ LL |     #[suggestion_verbose(compiletest::suggestion, code = "This is the suggested code")]
+    |                          ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:221:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:223:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion)]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:229:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:238:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:247:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:254:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:256:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion, code = "This is suggested code")]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:261:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:269:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:271:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion, code = "This is suggested code")]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:276:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:278:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion, code = "This is suggested code")]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:284:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:286:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion, code = "This is suggested code")]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:292:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:300:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:302:13
+    |
+    |
+ LL |     #[label(compiletest::label)]
+    |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:304:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion, code = "...")]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:309:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0456")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:314:13
+    |
+    |
+ LL |     #[label(compiletest::label)]
+    |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:316:13
+    |
+    |
+ LL |     #[label(compiletest::label)]
+    |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:318:18
+    |
+    |
+ LL |     #[suggestion(compiletest::suggestion, code = "{name}.clone()")]
+    |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:323:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:325:13
+    |
+    |
+ LL |     #[label(compiletest::label)]
+    |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:331:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:340:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:348:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:359:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:366:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:368:12
+    |
+    |
+ LL |     #[note(compiletest::note)]
+    |            ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:373:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:380:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:381:8
+    |
+    |
+ LL | #[note(compiletest::note)]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:387:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:394:8
+    |
+    |
+ LL | #[diag(compiletest::example, code = "E0123")]
+    |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
+ error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
+   --> $DIR/diagnostic-derive.rs:396:12
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
LL | #[diag(compiletest::example, code = "E0123")]


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
LL | #[nonsense(compiletest::example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[nonsense(compiletest::example, code = "E0123")]
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
LL | #[diag(compiletest::example, code = "E0123", slug = "foo")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

error: `#[suggestion = ...]` is not a valid attribute
   |
   |
LL |     #[suggestion = "bar"]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:107:8
   |
   |
LL | #[diag(compiletest::example, code = "E0456")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:106:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:107:37
   |
   |
LL | #[diag(compiletest::example, code = "E0456")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:106:37
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:113:53
   |
   |
LL | #[diag(compiletest::example, code = "E0456", code = "E0457")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:113:37
   |
   |
LL | #[diag(compiletest::example, code = "E0456", code = "E0457")]


error: `#[diag(compiletest::example)]` is not a valid attribute
   |
   |
LL | #[diag(compiletest::example, compiletest::example, code = "E0456")]
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
LL |     #[label(compiletest::label)]

error: `name` doesn't refer to a field on this type
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:170:50
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "{name}")]


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
LL |     #[label(compiletest::label)]


error: suggestion without `code = "..."`
   |
   |
LL |     #[suggestion(compiletest::suggestion)]


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
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:263:5
   |
   |
LL | /     #[suggestion(compiletest::suggestion, code = "This is suggested code")]
LL | |     //~^ ERROR wrong field type for suggestion
LL | |     suggestion: Applicability,
   |
   |
   = help: `#[suggestion(...)]` should be applied to fields of type `Span` or `(Span, Applicability)`
error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:279:24
   |
   |
LL |     suggestion: (Span, Span, Applicability),
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:279:18
   |
   |
LL |     suggestion: (Span, Span, Applicability),

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:287:33
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:287:18
   |
   |
LL |     suggestion: (Applicability, Applicability, Span),


error: `#[label = ...]` is not a valid attribute
   |
   |
LL |     #[label = "bar"]

error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:445:57
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "...", applicability = "maybe-incorrect")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:447:24
   |
   |
LL |     suggestion: (Span, Applicability),

error: invalid applicability
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:453:57
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "...", applicability = "batman")]


error: `#[label(foo)]` is not a valid attribute
   |
   |
LL |     #[label(compiletest::label, foo)]
   |
   |
   = help: a diagnostic slug must be the first argument to the attribute

error: `#[label(foo = ...)]` is not a valid attribute
   |
   |
LL |     #[label(compiletest::label, foo = "...")]


error: `#[label(foo(...))]` is not a valid attribute
   |
   |
LL |     #[label(compiletest::label, foo("..."))]


error: `#[primary_span]` is not a valid attribute
   |
   |
LL |     #[primary_span]
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: `#[error(...)]` is not a valid attribute
   |
LL | #[error(compiletest::example, code = "E0123")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: diagnostic slug not specified
   |
   |
LL | / #[error(compiletest::example, code = "E0123")]
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
LL | #[warn_(compiletest::example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[warn_(compiletest::example, code = "E0123")]
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
LL | #[lint(compiletest::example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[lint(compiletest::example, code = "E0123")]
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
LL | #[lint(compiletest::example, code = "E0123")]


error: `#[lint(...)]` is not a valid attribute
   |
   |
LL | #[lint(compiletest::example, code = "E0123")]


error: diagnostic slug not specified
   |
   |
LL | / #[lint(compiletest::example, code = "E0123")]
LL | | //~^ ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR `#[lint(...)]` is not a valid attribute
LL | | //~| ERROR diagnostic slug not specified
LL | | //~| ERROR cannot find attribute `lint` in this scope
LL | | struct LintAttributeOnLintDiag {}
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(compiletest_example)]`
error: specified multiple times
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:596:57
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "...", code = ",,,")]
   |
note: previously specified here
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:596:43
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "...", code = ",,,")]

error: wrong types for suggestion
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:605:24
   |
   |
LL |     suggestion: (Span, usize),
   |
   |
   = help: `#[suggestion(...)]` on a tuple field must be applied to fields of type `(Span, Applicability)`
error: wrong types for suggestion
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:613:17
   |
   |
LL |     suggestion: (Span,),
   |
   |
   = help: `#[suggestion(...)]` on a tuple field must be applied to fields of type `(Span, Applicability)`

error: suggestion without `code = "..."`
   |
   |
LL |     #[suggestion(compiletest::suggestion)]
---

error: cannot find attribute `multipart_suggestion` in this scope
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:634:7
   |
LL |     #[multipart_suggestion(compiletest::suggestion)]

error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:31:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:35:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:49:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:93:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123", slug = "foo")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:98:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:106:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:113:8
   |
   |
LL | #[diag(compiletest::example, code = "E0456", code = "E0457")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:118:8
   |
   |
LL | #[diag(compiletest::example, compiletest::example, code = "E0456")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:131:8
   |
   |
LL | #[diag(compiletest::example)]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:135:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:143:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:152:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:155:13
   |
   |
LL |     #[label(compiletest::label)]
   |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:160:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:168:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:170:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "{name}")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:177:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:179:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "{name")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:187:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:189:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "name}")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:196:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:198:13
   |
   |
LL |     #[label(compiletest::label)]
   |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:203:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:211:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:213:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "This is the suggested code")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:214:24
   |
   |
LL |     #[suggestion_short(compiletest::suggestion, code = "This is the suggested code")]
   |                        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:215:25
   |
   |
LL |     #[suggestion_hidden(compiletest::suggestion, code = "This is the suggested code")]
   |                         ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:216:26
   |
   |
LL |     #[suggestion_verbose(compiletest::suggestion, code = "This is the suggested code")]
   |                          ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:221:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:223:18
   |
   |
LL |     #[suggestion(compiletest::suggestion)]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:229:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:238:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:247:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:254:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:256:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "This is suggested code")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:261:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:269:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:271:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "This is suggested code")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:276:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:278:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "This is suggested code")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:284:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:286:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "This is suggested code")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:292:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:300:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:302:13
   |
   |
LL |     #[label(compiletest::label)]
   |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:304:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "...")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:309:8
   |
   |
LL | #[diag(compiletest::example, code = "E0456")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:314:13
   |
   |
LL |     #[label(compiletest::label)]
   |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:316:13
   |
   |
LL |     #[label(compiletest::label)]
   |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:318:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "{name}.clone()")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:323:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:325:13
   |
   |
LL |     #[label(compiletest::label)]
   |             ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:331:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:340:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:348:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:359:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:366:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:368:12
   |
   |
LL |     #[note(compiletest::note)]
   |            ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:373:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:380:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:381:8
   |
   |
LL | #[note(compiletest::note)]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:387:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:394:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:396:12
   |
   |
LL |     #[help(compiletest::help)]
   |            ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:401:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:408:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:409:8
   |
   |
LL | #[help(compiletest::help)]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:416:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:423:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:422:8
   |
   |
LL | #[help(compiletest::help)]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:430:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:437:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:436:8
   |
   |
LL | #[note(compiletest::note)]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:443:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:445:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "...", applicability = "maybe-incorrect")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:451:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:453:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "...", applicability = "batman")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:459:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:461:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "...", applicability = "maybe-incorrect")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:466:8
   |
   |
LL | #[diag(compiletest::example, code = "E0123")]
   |        ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:468:18
   |
   |
LL |     #[suggestion(compiletest::suggestion, code = "...")]
   |                  ^^^^^^^^^^^ could not find `compiletest` in `fluent`
error[E0433]: failed to resolve: could not find `parser` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:473:8
   |
   |
LL | #[note(parser::add_paren)]
   |        ^^^^^^ could not find `parser` in `fluent`
error[E0433]: failed to resolve: could not find `compiletest` in `fluent`
  --> /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:477:8
