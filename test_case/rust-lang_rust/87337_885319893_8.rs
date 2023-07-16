test-harness,testharness,tesT_harness
36 | | /// boo
   | |_______^
   |
   |
   = help: the code block will either not be tested if not marked as a rust one or the code will be wrapped inside a main function
error: aborting due to 15 previous errors


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/assoc-item-not-in-scope.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/assoc-item-not-in-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/assoc-item-not-in-scope" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/assoc-item-not-in-scope/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `S::fmt`
   |
   |
LL | /// Link to [`S::fmt`]
   |              ^^^^^^^^ the struct `S` has no field or associated item named `fmt`
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/assoc-item-not-in-scope.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]

error: aborting due to previous error



------------------------------------------


---- [ui] rustdoc-ui/check-attr.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/check-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-attr" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown attribute `compile-fail`. Did you mean `compile_fail`?
   |
   |
LL | / /// foo
LL | | //~^ ERROR
LL | | //~^^ ERROR
LL | | //~^^^ ERROR
LL | | /// boo
LL | | /// 