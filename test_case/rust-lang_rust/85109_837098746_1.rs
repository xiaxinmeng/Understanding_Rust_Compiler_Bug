\n\nDelete the offending feature attribute.\n"},"level":"error","spans":[{"file_name":"tests/run-pass/calls.rs","byte_start":11,"byte_end":19,"line_start":1,"line_end":1,"column_start":12,"column_end":20,"is_primary":true,"text":[{"text":"#![feature(const_fn)]","highlight_start":12,"highlight_end":20}],"label":"feature has been removed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"split into finer-grained feature gates","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0557]: feature has been removed\n --> tests/run-pass/calls.rs:1:12\n  |\n1 | #![feature(const_fn)]\n  |            ^^^^^^^^ feature has been removed\n  |\n  = note: split into finer-grained feature gates\n\n"}
{"message":"For more information about this error, try `rustc --explain E0557`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0557`.\n"}

------------------------------------------

---

---- compile_test stdout ----
diff of stderr:

-error: you should consider adding a `Default` implementation for `Foo`
+error[E0557]: feature has been removed
+  --> $DIR/new_without_default.rs:1:12
    |
    |
-LL | /     pub fn new() -> Foo {
-LL | |         Foo
-LL | |     }
-   | |_____^
+LL | #![feature(const_fn)]
    |
    |
-   = note: `-D clippy::new-without-default` implied by `-D warnings`
-   |
-   |
-LL | impl Default for Foo {
-LL |     fn default() -> Self {
-LL |         Self::new()
-LL |     }
-LL | }
-   |
+   = note: split into finer-grained feature gates
 
-error: you should consider adding a `Default` implementation for `Bar`
-   |
-   |
-LL | /     pub fn new() -> Self {
-LL | |         Bar
-LL | |     }
-   | |_____^
-   |
-   |
-   |
-LL | impl Default for Bar {
-LL |     fn default() -> Self {
-LL |         Self::new()
-LL |     }
-LL | }
-   |
 
 
-error: you should consider adding a `Default` implementation for `LtKo<'c>`
-   |
-   |
-LL | /     pub fn new() -> LtKo<'c> {
-LL | |         unimplemented!()
-LL | |     }
-   | |_____^
-   |
-   |
-   |
-LL | impl<'c> Default for LtKo<'c> {
-LL |     fn default() -> Self {
-LL |         Self::new()
-LL |     }
-LL | }
-   |
-
-error: you should consider adding a `Default` implementation for `NewNotEqualToDerive`
-   |
-   |
-LL | /     pub fn new() -> Self {
-LL | |         NewNotEqualToDerive { foo: 1 }
-LL | |     }
-   | |_____^
-   |
-   |
-   |
-LL | impl Default for NewNotEqualToDerive {
-LL |     fn default() -> Self {
-LL |         Self::new()
-LL |     }
-LL | }
-   |
-
-error: you should consider adding a `Default` implementation for `FooGenerics<T>`
-   |
-   |
-LL | /     pub fn new() -> Self {
-LL | |         Self(Default::default())
-LL | |     }
-   | |_____^
-   |
-   |
-   |
-LL | impl<T> Default for FooGenerics<T> {
-LL |     fn default() -> Self {
-LL |         Self::new()
-LL |     }
-LL | }
-   |
-
-error: you should consider adding a `Default` implementation for `BarGenerics<T>`
-   |
-   |
-LL | /     pub fn new() -> Self {
-LL | |         Self(Default::default())
-LL | |     }
-   | |_____^
-   |
-   |
-   |
-LL | impl<T: Copy> Default for BarGenerics<T> {
-LL |     fn default() -> Self {
-LL |         Self::new()
-LL |     }
-LL | }
-   |
-error: aborting due to 6 previous errors
-
+For more information about this error, try `rustc --explain E0557`.
 
---
To only update this specific test, also pass `--test-args new_without_default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/new_without_default.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/new_without_default.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-c9cbd4ed51f0d395.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/new_without_default.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"feature has been removed","code":{"code":"E0557","explanation":"A feature attribute named a feature that has been removed.\n\nErroneous code example:\n\n