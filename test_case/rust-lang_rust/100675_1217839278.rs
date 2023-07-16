plain

29   | ^^^^^^^^^^^^^^^^^
30   |
31 
+ error: name `key` does not start with the crate name
+    |
+    |
+ LL |         a => "./duplicate-a.ftl",
+    |
+    |
+    = help: prepend `a_` to the slug name: `a_key`
+ 
+ error: name `key` does not start with the crate name
+    |
+    |
+ LL |         b => "./duplicate-b.ftl",
+    |
+    |
+    = help: prepend `b_` to the slug name: `b_key`
32 error: overrides existing message: `key`
-   --> $DIR/test.rs:53:9
+   --> $DIR/test.rs:53:14
34    |
34    |
35 LL |         b => "./duplicate-b.ftl",
+    |              ^^^^^^^^^^^^^^^^^^^
37    |
38 help: previously defined in this resource
-   --> $DIR/test.rs:52:9
-   --> $DIR/test.rs:52:9
+   --> $DIR/test.rs:52:14
40    |
41 LL |         a => "./duplicate-a.ftl",
+    |              ^^^^^^^^^^^^^^^^^^^
43 
43 
44 error: name `this-slug-has-hyphens` contains a '-' character
-   --> $DIR/test.rs:62:9
46    |
46    |
47 LL |         slug_with_hyphens => "./slug-with-hyphens.ftl",
+    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^
49    |
49    |
50    = help: replace any '-'s with '_'s


+ error: name `this_slug_has_hyphens` does not start with the crate name
+    |
+    |
+ LL |         slug_with_hyphens => "./slug-with-hyphens.ftl",
+    |
+    |
+    = help: prepend `slug_with_hyphens_` to the slug name: `slug_with_hyphens_this_slug_has_hyphens`
+ 
+ error: name `some_slug` does not start with the crate name
+    |
+    |
+ LL |         label_with_hyphens => "./label-with-hyphens.ftl",
+    |
+    |
+    = help: prepend `label_with_hyphens_` to the slug name: `label_with_hyphens_some_slug`
+ 
52 error: attribute `label-has-hyphens` contains a '-' character
-   --> $DIR/test.rs:71:9
54    |
54    |
55 LL |         label_with_hyphens => "./label-with-hyphens.ftl",
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^
57    |
57    |
58    = help: replace any '-'s with '_'s

- error: aborting due to 6 previous errors
- error: aborting due to 6 previous errors
+ error: name `valid` does not start with the crate name
+    |
+    |
+ LL |         valid => "./valid.ftl",
+    |
+    |
+    = help: prepend `valid_` to the slug name: `valid_valid`
+ error: aborting due to 11 previous errors
61 
62 

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


error: name `key` does not start with the crate name
   |
   |
LL |         a => "./duplicate-a.ftl",
   |
   |
   = help: prepend `a_` to the slug name: `a_key`

error: name `key` does not start with the crate name
   |
   |
LL |         b => "./duplicate-b.ftl",
   |
   |
   = help: prepend `b_` to the slug name: `b_key`
error: overrides existing message: `key`
  --> /checkout/src/test/ui-fulldeps/fluent-messages/test.rs:53:14
   |
   |
LL |         b => "./duplicate-b.ftl",
   |
help: previously defined in this resource
  --> /checkout/src/test/ui-fulldeps/fluent-messages/test.rs:52:14
   |
   |
LL |         a => "./duplicate-a.ftl",


error: name `this-slug-has-hyphens` contains a '-' character
   |
   |
LL |         slug_with_hyphens => "./slug-with-hyphens.ftl",
   |
   |
   = help: replace any '-'s with '_'s

error: name `this_slug_has_hyphens` does not start with the crate name
   |
   |
LL |         slug_with_hyphens => "./slug-with-hyphens.ftl",
   |
   |
   = help: prepend `slug_with_hyphens_` to the slug name: `slug_with_hyphens_this_slug_has_hyphens`

error: name `some_slug` does not start with the crate name
   |
   |
LL |         label_with_hyphens => "./label-with-hyphens.ftl",
   |
   |
   = help: prepend `label_with_hyphens_` to the slug name: `label_with_hyphens_some_slug`

error: attribute `label-has-hyphens` contains a '-' character
   |
   |
LL |         label_with_hyphens => "./label-with-hyphens.ftl",
   |
   |
   = help: replace any '-'s with '_'s

error: name `valid` does not start with the crate name
   |
   |
LL |         valid => "./valid.ftl",
   |
   |
   = help: prepend `valid_` to the slug name: `valid_valid`
error: aborting due to 11 previous errors
------------------------------------------


