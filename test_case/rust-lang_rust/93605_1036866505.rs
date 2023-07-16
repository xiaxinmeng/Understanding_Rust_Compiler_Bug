plain
---- [ui] rustdoc-ui/intra-doc/html-as-generics-intra-doc.rs stdout ----
diff of stderr:

38    |
39 LL | #![deny(rustdoc::invalid_html_tags)]
+ help: try marking as source code
+    |
+    |
+ LL | /// This [`ExistentStruct<i32>`] thing!
41 
41 
42 error: unclosed HTML tag `i32`

44    |
44    |
45 LL | /// This [NonExistentStruct2<i32>] thing!
+    |
+ help: try marking as source code
+    |
+    |
+ LL | /// This [`NonExistentStruct2<i32>`] thing!
47 
47 
48 error: unclosed HTML tag `i32`

50    |
50    |
51 LL | /// This [NonExistentStruct3<i32>][] thing!
+    |
+ help: try marking as source code
+    |
+    |
+ LL | /// This [`NonExistentStruct3<i32>`][] thing!
53 
54 error: aborting due to 6 previous errors
55 

---
To only update this specific test, also pass `--test-args intra-doc/html-as-generics-intra-doc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/html-as-generics-intra-doc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/html-as-generics-intra-doc" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/html-as-generics-intra-doc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `NonExistentStruct`
  --> /checkout/src/test/rustdoc-ui/intra-doc/html-as-generics-intra-doc.rs:13:17
   |
LL | /// This [test][NonExistentStruct<i32>] thing!
   |                 ^^^^^^^^^^^^^^^^^^^^^^ no item named `NonExistentStruct` in scope
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/html-as-generics-intra-doc.rs:2:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `NonExistentStruct2`
  --> /checkout/src/test/rustdoc-ui/intra-doc/html-as-generics-intra-doc.rs:17:11
   |
   |
LL | /// This [NonExistentStruct2<i32>] thing!
   |           ^^^^^^^^^^^^^^^^^^^^^^^ no item named `NonExistentStruct2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `NonExistentStruct3`
  --> /checkout/src/test/rustdoc-ui/intra-doc/html-as-generics-intra-doc.rs:22:11
   |
   |
LL | /// This [NonExistentStruct3<i32>][] thing!
   |           ^^^^^^^^^^^^^^^^^^^^^^^ no item named `NonExistentStruct3` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unclosed HTML tag `i32`
   |
   |
LL | /// This [ExistentStruct<i32>] thing!
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/html-as-generics-intra-doc.rs:1:9
   |
   |
LL | #![deny(rustdoc::invalid_html_tags)]
help: try marking as source code
   |
   |
LL | /// This [`ExistentStruct<i32>`] thing!


error: unclosed HTML tag `i32`
   |
   |
LL | /// This [NonExistentStruct2<i32>] thing!
   |
help: try marking as source code
   |
   |
LL | /// This [`NonExistentStruct2<i32>`] thing!


error: unclosed HTML tag `i32`
   |
   |
LL | /// This [NonExistentStruct3<i32>][] thing!
   |
help: try marking as source code
   |
   |
LL | /// This [`NonExistentStruct3<i32>`][] thing!

error: aborting due to 6 previous errors


