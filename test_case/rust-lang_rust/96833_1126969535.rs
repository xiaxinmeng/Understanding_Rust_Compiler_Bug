plain

---- [ui] src/test/ui/generic-associated-types/issue-95305.rs stdout ----
diff of stderr:

- error[E0106]: missing lifetime specifier
+ error[E0637]: `'_` cannot be used here
3    |
3    |
4 LL | fn foo(x: &impl Foo<Item<'_> = u32>) { }
-    |                          ^^ expected named lifetime parameter
-    |
-    |
- help: consider introducing a named lifetime parameter
-    |
- LL | fn foo<'a>(x: &impl Foo<Item<'a> = u32>) { }
-    |       ++++                   ~~
+    |                          ^^ `'_` is a reserved lifetime name
- error[E0106]: missing lifetime specifier
- error[E0106]: missing lifetime specifier
+ error[E0637]: `'_` cannot be used here
14    |
14    |
15 LL | fn bar(x: &impl for<'a> Foo<Item<'a> = &'_ u32>) { }
-    |                                         ^^ expected named lifetime parameter
-    |
-    |
- help: consider using the `'a` lifetime
-    |
- LL | fn bar(x: &impl for<'a> Foo<Item<'a> = &'a u32>) { }
-    |                                         ~~
+    |                                         ^^ `'_` is a reserved lifetime name
23 error: aborting due to 2 previous errors
24 

- For more information about this error, try `rustc --explain E0106`.
---
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-95305.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-95305" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-95305/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0637]: `'_` cannot be used here
   |
   |
LL | fn foo(x: &impl Foo<Item<'_> = u32>) { }
   |                          ^^ `'_` is a reserved lifetime name

error[E0637]: `'_` cannot be used here
   |
   |
LL | fn bar(x: &impl for<'a> Foo<Item<'a> = &'_ u32>) { }
   |                                         ^^ `'_` is a reserved lifetime name
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0637`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/single-use-lifetime/fn-types.rs stdout ----
diff of stderr:

14 help: elide the single-use lifetime
15    |
16 LL -   a: for<'a> fn(&'a u32),
- LL +   a: fn(&u32),
+ LL +   a:  fn(&u32),
19 
19 
20 error[E0581]: return type references lifetime `'a`, which is not constrained by the fn input types

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/fn-types/fn-types.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args single-use-lifetime/fn-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/fn-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/fn-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/fn-types/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime parameter `'a` only used once
   |
   |
LL |   a: for<'a> fn(&'a u32), //~ ERROR `'a` only used once
   |          ^^      -- ...is used only here
   |          this lifetime...
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/single-use-lifetime/fn-types.rs:1:9
  --> /checkout/src/test/ui/single-use-lifetime/fn-types.rs:1:9
   |
LL | #![deny(single_use_lifetimes)]
help: elide the single-use lifetime
   |
   |
LL -   a: for<'a> fn(&'a u32), //~ ERROR `'a` only used once
LL +   a:  fn(&u32), //~ ERROR `'a` only used once


error[E0581]: return type references lifetime `'a`, which is not constrained by the fn input types
   |
   |
LL |   d: for<'a> fn() -> &'a u32, // OK, used only in return type.

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0581`.
For more information about this error, try `rustc --explain E0581`.
------------------------------------------


---- [ui] src/test/ui/suggestions/impl-trait-missing-lifetime.rs stdout ----
diff of stderr:

- error[E0106]: missing lifetime specifier
+ error[E0637]: `'_` cannot be used here
3    |
3    |
4 LL | fn f(_: impl Iterator<Item = &'_ ()>) {}
-    |                               ^^ expected named lifetime parameter
-    |
-    |
- help: consider introducing a named lifetime parameter
-    |
- LL | fn f<'a>(_: impl Iterator<Item = &'a ()>) {}
-    |     ++++                          ~~
+    |                               ^^ `'_` is a reserved lifetime name
12 error: aborting due to previous error
13 

- For more information about this error, try `rustc --explain E0106`.
---
To only update this specific test, also pass `--test-args suggestions/impl-trait-missing-lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/impl-trait-missing-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-missing-lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-missing-lifetime/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0637]: `'_` cannot be used here
   |
   |
LL | fn f(_: impl Iterator<Item = &'_ ()>) {} //~ ERROR missing lifetime specifier
   |                               ^^ `'_` is a reserved lifetime name
error: aborting due to previous error

For more information about this error, try `rustc --explain E0637`.
------------------------------------------
